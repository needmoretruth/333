//! The command line a service runs, which has to be the node the person was testing.
//!
//! A person who tried `333 --data-dir ~/node serve --tor --no-meet` and then asked for
//! a service wants that, at every boot — not a fresh node in the default directory
//! reached the default way. So install takes the same flags `serve` takes, reads them
//! with the very same definition `serve` is read with, and writes them back out in
//! full. Reading them through [`crate::Cli`] rather than a copy of it is what keeps a
//! flag added to `serve` tomorrow from being quietly dropped here: the two lists below
//! name every field, and a new one does not compile until it is carried.

use std::path::PathBuf;

use anyhow::bail;
use clap::Parser as _;

use crate::commands::Common;
use crate::paths::NodePaths;

/// The seconds `--timeout` means when nobody gave it.
const DEFAULT_TIMEOUT: u64 = 300;

/// What a service runs, and what its hourly check runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Vigil {
    /// This program, where it lives.
    pub(crate) exe: PathBuf,
    /// The node directory, absolute, because a service has no working directory to be
    /// relative to.
    pub(crate) node: PathBuf,
    /// Everything after the program's name that keeps the vigil.
    pub(crate) serve: Vec<String>,
    /// Everything after the program's name that asks whether it is being kept.
    pub(crate) check: Vec<String>,
}

/// Read the flags given to `service install` as `serve` would read them.
///
/// The options every command shares are put back in front, from what this run was
/// already given, so that `333 --data-dir X service install --tor` and `333 service
/// install --data-dir X --tor` mean the same thing.
///
/// # Errors
/// Fails the way `serve` would fail on the same flags.
pub(crate) fn read(common: &Common, flags: &[String]) -> Result<crate::Cli, clap::Error> {
    let mut argv = vec!["333".to_owned()];
    if common.paths.root() != NodePaths::default_home().root() {
        argv.push("--data-dir".to_owned());
        argv.push(common.paths.root().display().to_string());
    }
    if common.timeout.as_secs() != DEFAULT_TIMEOUT {
        argv.push("--timeout".to_owned());
        argv.push(common.timeout.as_secs().to_string());
    }
    if common.keeping == crate::node::Keeping::Everything {
        argv.push("--keep-everything".to_owned());
    }
    if common.trust_directory_permissions {
        argv.push("--dangerously-trust-directory-permissions".to_owned());
    }
    let bridges = common
        .bridges
        .lock()
        .map_or_else(|held| held.into_inner().clone(), |held| held.clone());
    for line in bridges.lines {
        argv.push("--bridge".to_owned());
        argv.push(line);
    }
    if let Some(helper) = bridges.helper {
        argv.push("--bridge-helper".to_owned());
        argv.push(helper);
    }
    argv.push("serve".to_owned());
    argv.extend(flags.iter().cloned());
    crate::Cli::try_parse_from(argv)
}

/// Write a read command line back out in full, as a service will run it.
///
/// # Errors
/// Fails on flags `serve` would refuse, and on anything a service definition could
/// not carry: a line break inside a flag cannot be written into one.
pub(crate) fn write(cli: crate::Cli, exe: PathBuf) -> anyhow::Result<Vigil> {
    let crate::Cli {
        data_dir,
        timeout,
        dangerously_trust_directory_permissions: trust,
        keep_everything,
        bridges,
        bridge_helper,
        language: _,
        count_in: _,
        command,
    } = cli;
    let crate::Command::Serve {
        bind,
        tor,
        no_direct,
        announce,
        no_mdns,
        no_router,
        meet,
        no_meet,
        plain: _,
    } = command
    else {
        bail!("only the flags `serve` takes can be given to `service install`");
    };
    if no_direct && !tor {
        bail!("nothing would be listening: --no-direct needs --tor");
    }
    let node = data_dir.unwrap_or_else(|| NodePaths::default_home().root().to_path_buf());
    let node = std::path::absolute(&node).unwrap_or(node);
    let mut shared = vec!["--data-dir".to_owned(), node.display().to_string()];
    if trust {
        shared.push("--dangerously-trust-directory-permissions".to_owned());
    }
    let mut check = shared.clone();
    check.extend(["service", "check"].map(str::to_owned));

    let mut serve = shared;
    if timeout != DEFAULT_TIMEOUT {
        serve.extend(["--timeout".to_owned(), timeout.to_string()]);
    }
    if keep_everything {
        serve.push("--keep-everything".to_owned());
    }
    for line in bridges {
        serve.extend(["--bridge".to_owned(), line]);
    }
    if let Some(helper) = bridge_helper {
        serve.extend(["--bridge-helper".to_owned(), helper]);
    }
    // --plain because there is nobody at a terminal. The screen is for when there is.
    serve.extend(["serve", "--plain"].map(str::to_owned));
    if bind != crate::typed::default_bind() {
        serve.extend(["--bind".to_owned(), bind.to_string()]);
    }
    let mut flag = |on: bool, name: &str| {
        if on {
            serve.push(name.to_owned());
        }
    };
    flag(tor, "--tor");
    flag(no_direct, "--no-direct");
    flag(no_mdns, "--no-mdns");
    flag(no_router, "--no-router");
    flag(no_meet, "--no-meet");
    if let Some(announce) = announce {
        serve.extend(["--announce".to_owned(), announce.to_string()]);
    }
    if !no_meet && meet != n333_net::meeting::THE_PLACE {
        serve.extend(["--meet".to_owned(), meet]);
    }
    if serve
        .iter()
        .chain(&check)
        .any(|arg| arg.contains(['\n', '\r']))
    {
        bail!("a flag with a line break in it cannot be written into a service");
    }
    Ok(Vigil {
        exe,
        node,
        serve,
        check,
    })
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use super::*;

    fn common(root: &str, bridges: &[&str]) -> Common {
        Common {
            paths: NodePaths::at(PathBuf::from(root)),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
            keeping: crate::node::Keeping::TheWindow,
            bridges: Arc::new(Mutex::new(n333_net::bridges::Bridges {
                lines: bridges.iter().map(|line| (*line).to_owned()).collect(),
                helper: None,
            })),
            trust_directory_permissions: false,
        }
    }

    fn vigil(common: &Common, flags: &[&str]) -> Vigil {
        let flags: Vec<String> = flags.iter().map(|flag| (*flag).to_owned()).collect();
        write(read(common, &flags).unwrap(), PathBuf::from("/usr/bin/333")).unwrap()
    }

    #[test]
    fn the_service_runs_the_same_node_with_the_same_flags() {
        let bridge = "obfs4 192.0.2.1:443 AAAA cert=x iat-mode=0";
        let vigil = vigil(
            &common("/srv/node", &[bridge]),
            &["--tor", "--no-meet", "--bind", "0.0.0.0:43333"],
        );
        assert_eq!(vigil.node, PathBuf::from("/srv/node"));
        assert_eq!(
            vigil.serve,
            [
                "--data-dir",
                "/srv/node",
                "--bridge",
                bridge,
                "serve",
                "--plain",
                "--bind",
                "0.0.0.0:43333",
                "--tor",
                "--no-meet"
            ]
        );
        assert_eq!(vigil.check, ["--data-dir", "/srv/node", "service", "check"]);
    }

    #[test]
    fn what_is_written_out_reads_back_as_the_same_vigil() {
        // The definition is only right if the program reading it at boot reads it as
        // the node that was installed; every flag has to survive the trip.
        let flags = [
            "--data-dir",
            "/srv/node",
            "--no-mdns",
            "--no-upnp",
            "--announce",
            "192.0.2.7:3333",
            "--meet",
            "meet.example",
        ];
        // From the default directory, as a person who typed --data-dir after `install`
        // would be; giving it on both sides is refused, as it is for `serve`.
        let home = NodePaths::default_home().root().display().to_string();
        let first = vigil(&common(&home, &[]), &flags);
        let mut argv = vec!["333".to_owned()];
        argv.extend(first.serve.iter().cloned());
        let again = write(crate::Cli::try_parse_from(argv).unwrap(), first.exe.clone()).unwrap();
        assert_eq!(again, first);
    }

    #[test]
    fn a_vigil_that_would_listen_nowhere_is_refused_before_it_is_installed() {
        let flags = ["--no-direct".to_owned()];
        let read = read(&common("/srv/node", &[]), &flags).unwrap();
        assert!(write(read, PathBuf::from("/usr/bin/333")).is_err());
    }

    #[test]
    fn a_flag_serve_does_not_take_is_refused_the_way_serve_refuses_it() {
        let flags = ["--no-such-flag".to_owned()];
        assert!(read(&common("/srv/node", &[]), &flags).is_err());
    }
}
