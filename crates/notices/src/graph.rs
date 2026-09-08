//! Which packages are compiled into the binaries on the releases page.
//!
//! Not simply "everything `cargo metadata` prints". That resolve graph is deliberately
//! maximal: it carries an edge for every optional dependency whether or not a feature
//! switches it on, and the feature list it reports for a package is the union over
//! that package's normal, dev and build dependencies. Walked as it stands it names
//! about ninety packages that no released binary contains.
//!
//! So the walk below reads each node's enabled features and crosses an optional edge
//! only when one of those features turns that dependency on. The slack that is left —
//! a feature switched on only by some package's dev dependency, an edge no manifest
//! entry explains — errs towards keeping a package rather than dropping one. For an
//! attribution file that is the safe direction: naming a licence that is not in the
//! binary costs a paragraph, missing one that is breaks the licence.

use anyhow::{Context, Result};
use cargo_metadata::semver::Version;
use cargo_metadata::{
    CargoOpt, DependencyKind, Metadata, MetadataCommand, Node, Package, PackageId,
};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

/// The package whose binary is released. Everything reachable from it ships.
const RELEASED: &str = "n333-cli";

/// The feature the small edition is built with. The releases page builds every system
/// twice, and this is the second build; see `.github/workflows/release.yml`.
const SMALL_EDITION: &str = "tor";

/// A package linked into a released binary.
pub(crate) struct Shipped {
    /// The name as crates.io knows it.
    pub(crate) name: String,
    /// Kept as a version rather than a string so that 0.9.0 sorts before 0.10.0 and
    /// the generated file does not reorder itself when a dependency moves.
    pub(crate) version: Version,
    /// The SPDX expression the manifest declares, if it declares one.
    pub(crate) licence: Option<String>,
    /// The file the manifest named instead of an expression, if it named one.
    pub(crate) licence_file: Option<PathBuf>,
    /// The extracted source directory, which is the only place the licence text is.
    pub(crate) directory: PathBuf,
}

impl Shipped {
    /// What to print in the licence column. A package must declare one or the other,
    /// and saying so plainly is better than an empty cell nobody can act on.
    pub(crate) fn expression(&self) -> String {
        match (&self.licence, &self.licence_file) {
            (Some(expression), _) => expression.clone(),
            (None, Some(file)) => format!("see `{}`", file.display()),
            (None, None) => "not declared".to_owned(),
        }
    }

    /// How a package is named everywhere in the generated file.
    pub(crate) fn label(&self) -> String {
        format!("{} {}", self.name, self.version)
    }
}

/// Where this repository is, asked of cargo rather than guessed from the executable's
/// path, so that the tool works from any directory inside the checkout.
pub(crate) struct Repository {
    /// The directory the generated file belongs in.
    pub(crate) root: PathBuf,
    /// The manifest of the package that becomes the released binary.
    pub(crate) released_manifest: PathBuf,
}

/// Find the workspace and the package that is released from it.
pub(crate) fn locate() -> Result<Repository> {
    let metadata = MetadataCommand::new()
        .no_deps()
        .exec()
        .context("could not run `cargo metadata`; run this from inside the repository")?;
    let released = metadata
        .packages
        .iter()
        .find(|package| package.name == RELEASED)
        .with_context(|| format!("this workspace has no `{RELEASED}` package"))?;
    Ok(Repository {
        root: metadata.workspace_root.clone().into_std_path_buf(),
        released_manifest: released.manifest_path.clone().into_std_path_buf(),
    })
}

/// Every third-party package a released binary carries, in a fixed order.
///
/// Each system is built twice: the full client, and the small one built with
/// `--no-default-features --features tor`. Somebody downloading either is owed the
/// licences of whatever is inside it, so the answer is the union of both builds.
/// Neither build is filtered to one platform, because the binaries are built for six.
pub(crate) fn shipped(released_manifest: &Path) -> Result<Vec<Shipped>> {
    let editions = [
        Vec::new(),
        vec![
            CargoOpt::NoDefaultFeatures,
            CargoOpt::SomeFeatures(vec![SMALL_EDITION.to_owned()]),
        ],
    ];
    let mut found: BTreeMap<(String, Version), Shipped> = BTreeMap::new();
    for edition in editions {
        let metadata = resolve(released_manifest, edition)?;
        for package in linked(&metadata)? {
            // A package with no source is one of this workspace's own crates, or a
            // path dependency next to it. Neither is somebody else's work to credit.
            if package.source.is_none() {
                continue;
            }
            let directory = package
                .manifest_path
                .parent()
                .with_context(|| format!("{} has no directory", package.manifest_path))?
                .to_path_buf()
                .into_std_path_buf();
            let licence_file = package
                .license_file
                .as_ref()
                .map(|relative| directory.join(relative.as_std_path()));
            found
                .entry((package.name.to_string(), package.version.clone()))
                .or_insert_with(|| Shipped {
                    name: package.name.to_string(),
                    version: package.version.clone(),
                    licence: package.license.clone(),
                    licence_file,
                    directory,
                });
        }
    }
    Ok(found.into_values().collect())
}

/// Ask cargo to resolve the graph the way one of the released builds resolves it.
fn resolve(manifest: &Path, edition: Vec<CargoOpt>) -> Result<Metadata> {
    let mut command = MetadataCommand::new();
    command.manifest_path(manifest);
    for option in edition {
        command.features(option);
    }
    command
        .exec()
        .with_context(|| format!("`cargo metadata` failed for {}", manifest.display()))
}

/// The packages reachable from the released binary through dependencies that are
/// actually compiled: normal ones, and optional ones a feature has switched on.
fn linked(metadata: &Metadata) -> Result<Vec<&Package>> {
    let resolve = metadata
        .resolve
        .as_ref()
        .context("`cargo metadata` returned no resolve graph")?;
    let root = resolve
        .root
        .as_ref()
        .context("`cargo metadata` returned no root package")?;
    let packages: HashMap<&PackageId, &Package> =
        metadata.packages.iter().map(|p| (&p.id, p)).collect();
    let nodes: HashMap<&PackageId, &Node> = resolve.nodes.iter().map(|n| (&n.id, n)).collect();

    let mut seen: HashSet<&PackageId> = HashSet::new();
    let mut stack: Vec<&PackageId> = vec![root];
    let mut reached: Vec<&Package> = Vec::new();
    while let Some(id) = stack.pop() {
        if !seen.insert(id) {
            continue;
        }
        let (Some(package), Some(node)) = (packages.get(id), nodes.get(id)) else {
            continue;
        };
        reached.push(package);
        let switched_on = switched_on(package, node);
        for dependency in &node.deps {
            let normal = dependency.dep_kinds.is_empty()
                || dependency
                    .dep_kinds
                    .iter()
                    .any(|kind| kind.kind == DependencyKind::Normal);
            if !normal {
                continue;
            }
            let Some(target) = packages.get(&dependency.pkg) else {
                continue;
            };
            if compiled(package, target.name.as_ref(), &switched_on) {
                stack.push(&dependency.pkg);
            }
        }
    }
    Ok(reached)
}

/// The optional dependencies of `package` that its enabled features turn on.
///
/// A feature switches a dependency on three ways: by naming it `dep:name`, by having
/// the name cargo gives an optional dependency with no explicit `dep:`, or by asking
/// for a feature of it as `name/feature`. The fourth form, `name?/feature`, is weak
/// and turns nothing on by itself, so it is passed over here; if that dependency is
/// compiled at all, one of the other three forms says so.
fn switched_on<'a>(package: &'a Package, node: &'a Node) -> HashSet<&'a str> {
    let optional: HashSet<&str> = package
        .dependencies
        .iter()
        .filter(|dependency| dependency.optional)
        .map(|dependency| dependency.rename.as_deref().unwrap_or(&dependency.name))
        .collect();
    let mut on: HashSet<&str> = HashSet::new();
    for feature in &node.features {
        let feature = feature.as_ref();
        if optional.contains(feature) {
            on.insert(feature);
        }
        for entry in package.features.get(feature).into_iter().flatten() {
            if let Some(dependency) = entry.strip_prefix("dep:") {
                on.insert(dependency);
            } else if !entry.contains("?/")
                && let Some((dependency, _)) = entry.split_once('/')
                && optional.contains(dependency)
            {
                on.insert(dependency);
            }
        }
    }
    on
}

/// Whether cargo compiles `dependency` when it compiles `package`.
fn compiled(package: &Package, dependency: &str, switched_on: &HashSet<&str>) -> bool {
    let mut named = false;
    for entry in &package.dependencies {
        if entry.kind != DependencyKind::Normal || entry.name != dependency {
            continue;
        }
        named = true;
        let key = entry.rename.as_deref().unwrap_or(&entry.name);
        if !entry.optional || switched_on.contains(key) {
            return true;
        }
    }
    // An edge cargo drew that no manifest entry accounts for is a shape this code does
    // not know about. Keep the package rather than lose its licence to it.
    !named
}
