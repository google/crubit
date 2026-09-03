// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::paths;

use std::ffi::OsString;
use std::path::PathBuf;

#[cfg(unix)]
const ZLIB_NAME: &str = "z";
#[cfg(windows)]
const ZLIB_NAME: &str = "zlib";

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
    paths::get_env_paths("CLANG_INCLUDE_PATH")
}

/// Returns a list of all clang and llvm libraries to be linked, and the paths
/// where they can be found.
pub fn collect_clang_libs() -> (Vec<PathBuf>, Vec<OsString>) {
    let (clang_lib_dirs, mut libs) =
        paths::collect_static_libs("CLANG_LIB_STATIC_PATH", include_lib);

    // libclang uses functions from Version.lib on Windows.
    #[cfg(windows)]
    libs.push(OsString::from("Version"));

    // llvm depends on zlib.
    libs.push(OsString::from(ZLIB_NAME));

    libs.sort_unstable();

    (clang_lib_dirs, libs)
}
