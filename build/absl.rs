// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[cfg(unix)]
const LIB_EXTENSION: &str = "a";
#[cfg(windows)]
const LIB_EXTENSION: &str = "lib";

/// Returns the path to the absl libraries (to be used as a search path) and a
/// list of libraries to be linked.
pub fn collect_absl_libs() -> (PathBuf, Vec<OsString>) {
    assert!(cfg!(unix) || cfg!(windows));

    let absl_lib_dir = Path::new(&std::env::var_os("ABSL_LIB_DIR").expect("ABSL_LIB_DIR must be specified in the environment")).to_owned();

    let mut libs = Vec::new();

    for f in std::fs::read_dir(&absl_lib_dir).expect("unable to read ABSL_LIB_DIR") {
        let Ok(entry) = f else { continue };
        let Ok(meta) = entry.metadata() else { continue };
        if !meta.is_file() {
            continue;
        };
        let path = entry.path();
        let Some(ext) = path.extension() else {
            continue;
        };
        if ext != LIB_EXTENSION {
            continue;
        }
        let libname = if cfg!(windows) {
            // On windows, the full filename: `name.lib`.
            let Some(filename) = path.file_name() else {
                continue;
            };
            filename.to_str().expect("absl lib has non-utf8 name")
        } else {
            // On unix, drop the lib prefix and the extension: `libname.a` => `name`.
            let Some(stem) = path.file_stem() else {
                continue;
            };
            let s = stem.to_str().expect("absl lib has non-utf8 name");
            s.strip_prefix("lib").unwrap_or(s)
        };
        libs.push(OsStr::new(libname).to_owned())
    }
    libs.sort_unstable();

    (absl_lib_dir, libs)
}
