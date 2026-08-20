// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::paths;

use std::ffi::OsString;
use std::path::PathBuf;

#[cfg(unix)]
const LIB_EXTENSION: &str = "a";
#[cfg(windows)]
const LIB_EXTENSION: &str = "lib";

/// Returns a list of include paths for absl headers.
pub fn collect_absl_includes() -> Vec<PathBuf> {
    paths::get_env_paths("ABSL_INCLUDE_PATH")
}

/// Returns the paths to the absl libraries (to be used as a search path) and a
/// list of libraries to be linked.
pub fn collect_absl_libs() -> (Vec<PathBuf>, Vec<OsString>) {
    assert!(cfg!(unix) || cfg!(windows));
    paths::collect_static_libs("ABSL_LIB_STATIC_PATH", LIB_EXTENSION, |_| true)
}
