// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::paths;

use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[cfg(unix)]
const LIB_EXTENSION: &str = "a";
#[cfg(windows)]
const LIB_EXTENSION: &str = "lib";

/// Returns a list of include paths for absl headers.
pub fn collect_absl_includes() -> Vec<PathBuf> {
    paths::print_env_to_string("ABSL_INCLUDE_PATH")
        .expect("ABSL_INCLUDE_PATH must be specified in the environment")
        .split(',')
        .map(|s| Path::new(s).to_owned())
        .collect()
}

/// Returns the paths to the absl libraries (to be used as a search path) and a
/// list of libraries to be linked.
pub fn collect_absl_libs() -> (Vec<PathBuf>, Vec<OsString>) {
    assert!(cfg!(unix) || cfg!(windows));

    let absl_lib_dirs: Vec<PathBuf> = std::env::var("ABSL_LIB_STATIC_PATH")
        .expect("ABSL_LIB_STATIC_PATH must be specified in the environment")
        .split(',')
        .map(|s| Path::new(&s).to_owned())
        .collect();

    let mut libs = Vec::new();

    for dir in &absl_lib_dirs {
        for f in std::fs::read_dir(dir)
            .unwrap_or_else(|_| panic!("unable to read ABSL_LIB_STATIC_PATH: {}", dir.display()))
        {
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
                // On windows, the filename without an extension: `name`.
                let Some(stem) = path.file_stem() else {
                    continue;
                };
                stem.to_str().expect("absl lib has non-utf8 name")
            } else {
                // On unix, drop the lib prefix and the extension: `libname.a` => `name`.
                let Some(stem) = path.file_stem() else {
                    continue;
                };
                let s = stem.to_str().expect("absl lib has non-utf8 name");
                s.strip_prefix("lib").unwrap_or(s)
            };
            libs.push(OsString::from(libname))
        }
    }
    libs.sort_unstable();

    (absl_lib_dirs, libs)
}
