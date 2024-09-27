// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::paths;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

#[cfg(unix)]
const LIB_EXTENSION: &str = "a";
#[cfg(windows)]
const LIB_EXTENSION: &str = "lib";

fn include_lib(libname: &str) -> bool {
    if libname.ends_with("Main") {
        return false;
    }
    // Skip target backends.
    if libname.starts_with("LLVMX86")
        || libname.starts_with("LLVMWebAssem")
        || libname.starts_with("LLVMRISCV")
        || libname.starts_with("LLVMMips")
        || libname.starts_with("LLVMLoongArch")
        || libname.starts_with("LLVMARM")
        || libname.starts_with("LLVMAArch")
    {
        return false;
    }
    if libname.contains("clangd") || libname.contains("clangTidy") {
        return false;
    }
    if libname.starts_with("lld") {
        return false;
    }
    true
}

/// Returns a list of include paths for clang and llvm headers.
pub fn collect_clang_includes() -> Vec<PathBuf> {
    paths::print_env_to_string("CLANG_INCLUDE_PATH")
        .expect("CLANG_INCLUDE_PATH must be specified in the environment")
        .split(',')
        .map(|s| Path::new(s).to_owned())
        .collect()
}

/// Returns a list of all clang and llvm libraries to be linked, and the paths
/// where they can be found.
pub fn collect_clang_libs() -> (Vec<PathBuf>, Vec<OsString>) {
    assert!(cfg!(unix) || cfg!(windows));

    let mut libs = Vec::new();

    let clang_lib_dirs: Vec<PathBuf> = std::env::var("CLANG_LIB_STATIC_PATH")
        .expect("CLANG_LIB_STATIC_PATH must be specified in the environment")
        .split(',')
        .map(|s| Path::new(&s).to_owned())
        .collect();

    for dir in &clang_lib_dirs {
        for f in std::fs::read_dir(dir)
            .expect(&format!("unable to read CLANG_LIB_STATIC_PATH: {}", dir.display()))
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
            if include_lib(libname) {
                libs.push(OsStr::new(libname).to_owned())
            }
        }
    }
    libs.sort_unstable();

    (clang_lib_dirs, libs)
}
