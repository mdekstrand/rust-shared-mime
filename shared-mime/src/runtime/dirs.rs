//! Directories to search for runtime files.
//!
//! The [XDG Base Directory spec][BD] specifies two environment variables for
//! locating environment variables;
//!
//! 1. `XDG_DATA_HOME`, with a default value of `$HOME/.local/share`
//! 2. `XDG_DATA_DIRS`, with a default value of `/usr/local/share:/usr/share`
//!
//! The Shared Mime Info spec then specifies that these paths are processed in
//! *reverse* order, so directories appearing earlier in the path are processed
//! later and override previously-processed directories.
//!
//! [BD]:
//!     https://specifications.freedesktop.org/basedir-spec/latest/ar01s03.html

use std::{env, path::PathBuf};

use log::*;

/// Return the directories to check for MIME data, in processing order.
pub fn xdg_mime_search_dirs() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    xdg_data_dirs(&mut paths);
    xdg_home_dir(&mut paths);
    paths
}

fn xdg_data_dirs(paths: &mut Vec<PathBuf>) {
    if let Some(spec) = env::var_os("XDG_DATA_DIRS") {
        debug!("found $XDG_DATA_DIRS");
        for mut spath in env::split_paths(&spec) {
            spath.push("mime");
            paths.push(spath);
        }
    } else {
        debug!("using default XDG_DATA_DIRS");
        paths.push("/usr/share/mime".into());
        paths.push("/usr/local/share/mime".into());
    }
}

fn xdg_home_dir(paths: &mut Vec<PathBuf>) {
    if let Some(spec) = env::var_os("XDG_DATA_HOME") {
        debug!("found $XDG_DATA_HOME");
        paths.push(spec.into())
    } else if let Some(path) = env::var_os("HOME") {
        debug!("constructing XDG_DATA_HOME from $HOME");
        let mut path = PathBuf::from(path);
        path.push(".local");
        path.push("share");
        path.push("mime");
        paths.push(path);
    } else {
        warn!("no $HOME or $XDG_DATA_HOME");
    }
}
