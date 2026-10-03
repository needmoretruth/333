//! The files under `--site`: which one a path means, and how it is sent.
//!
//! Clean URLs, as the static host before this did them: `/` is `index.html`, `/network`
//! is `network.html`, and `/map/` is `map/index.html`. A path that would leave the site
//! directory is answered as missing: any `..` or `.` segment, a backslash, a NUL, a
//! hidden name other than `.well-known`, and — checked last, on the real path — a
//! symbolic link that points outside.

use std::path::{Path, PathBuf};

use super::respond::{IMMUTABLE, NO_CACHE};

/// The file a request path names, if there is one inside `root` (already canonical).
pub(crate) fn resolve(root: &Path, request_path: &str) -> Option<PathBuf> {
    let relative = request_path.strip_prefix('/')?;
    if !relative.split('/').all(allowed) {
        return None;
    }
    candidates(relative).into_iter().find_map(|candidate| {
        let real = root.join(candidate).canonicalize().ok()?;
        (real.starts_with(root) && real.is_file()).then_some(real)
    })
}

/// May a request path contain this segment?
fn allowed(segment: &str) -> bool {
    let hidden = segment.starts_with('.') && segment != ".well-known";
    !hidden && !segment.contains(['\\', '\0'])
}

/// The files a path could mean, in the order they are tried.
fn candidates(relative: &str) -> Vec<String> {
    if relative.is_empty() || relative.ends_with('/') {
        return vec![format!("{relative}index.html")];
    }
    vec![
        relative.to_owned(),
        format!("{relative}.html"),
        format!("{relative}/index.html"),
    ]
}

/// Is this a page to be templated?
pub(crate) fn is_html(path: &Path) -> bool {
    path.extension()
        .is_some_and(|extension| extension == "html")
}

/// The content type a file is sent with.
///
/// The installers are text so a browser shows them: they are meant to be read before
/// anybody pipes them into a shell.
pub(crate) fn kind(path: &Path) -> &'static str {
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase);
    match extension.as_deref() {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js" | "mjs") => "text/javascript; charset=utf-8",
        Some("json" | "map") => "application/json; charset=utf-8",
        Some("txt" | "sh" | "ps1" | "md") => "text/plain; charset=utf-8",
        Some("xml") => "application/xml; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("avif") => "image/avif",
        Some("gif") => "image/gif",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        Some("wasm") => "application/wasm",
        Some("pdf") => "application/pdf",
        _ => "application/octet-stream",
    }
}

/// How long a file may be kept: for ever under `/assets/`, which pages name by
/// version, and checked again every time for everything else.
pub(crate) fn cache(request_path: &str) -> &'static str {
    if request_path.starts_with("/assets/") {
        IMMUTABLE
    } else {
        NO_CACHE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_path_that_climbs_out_of_the_site_is_not_found() {
        let base = std::env::temp_dir().join(format!("n333-site-statics-{}", std::process::id()));
        let root = base.join("site");
        std::fs::create_dir_all(root.join("map")).unwrap();
        std::fs::write(base.join("secret.html"), "no").unwrap();
        std::fs::write(root.join("network.html"), "yes").unwrap();
        std::fs::write(root.join("map/index.html"), "yes").unwrap();
        let root = root.canonicalize().unwrap();

        assert_eq!(resolve(&root, "/network"), Some(root.join("network.html")));
        assert_eq!(resolve(&root, "/map/"), Some(root.join("map/index.html")));
        for climbing in [
            "/../secret.html",
            "/../secret",
            "/map/../../secret",
            "/..%2fsecret",
            "/.\\..\\secret",
            "//etc/hostname",
        ] {
            assert_eq!(resolve(&root, climbing), None, "{climbing}");
        }
        std::fs::remove_dir_all(&base).unwrap();
    }
}
