// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use runfiles::Runfiles;
use std::env;
use std::error::Error;
use std::path::PathBuf;

const CC_BINDINGS_FROM_RS_RLOCATION: &str =
   "rules_crubit/cc_bindings_from_rs/cc_bindings_from_rs";

const CLANG_FORMAT_RLOCATION: &str =
   "clang-format";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResourceSearchConfig<'a> {
    pub env_vars: &'a [&'a str],
    pub runfile_candidates: &'a [&'a str],
    pub adjacent_candidates: &'a [&'a str],
    pub path_binaries: &'a [&'a str],
}

pub fn find_resource(config: &ResourceSearchConfig) -> Option<PathBuf> {
    for &env_var in config.env_vars {
        if let Ok(env_path) = env::var(env_var)
            && let path = PathBuf::from(&env_path)
            && path.exists()
        {
            return Some(path);
        }
    }

    if let Ok(r) = Runfiles::create() {
        for &candidate in config.runfile_candidates {
            if let Some(path) = runfiles::rlocation!(r, candidate)
                && path.exists()
            {
                return Some(path);
            }
        }
    }

    if let Ok(mut exe_path) = env::current_exe() {
        exe_path.pop();
        for &adjacent_name in config.adjacent_candidates {
            let adjacent_path = exe_path.join(adjacent_name);
            if adjacent_path.exists() {
                return Some(adjacent_path);
            }
        }
    }

    for &bin_name in config.path_binaries {
        if let Some(path) = find_in_path(bin_name) {
            return Some(path);
        }
    }

    None
}

pub fn find_in_path(name: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths).find_map(|dir| {
        let full_path = dir.join(name);
        full_path.exists().then_some(full_path)
    })
}

pub fn get_cc_bindings_from_rs_path() -> Result<PathBuf, Box<dyn Error>> {
    find_resource(&ResourceSearchConfig {
        env_vars: &["CC_BINDINGS_FROM_RS"],
        runfile_candidates: &[CC_BINDINGS_FROM_RS_RLOCATION],
        adjacent_candidates: &["cc_bindings_from_rs"],
        path_binaries: &["cc_bindings_from_rs"],
    })
    .ok_or_else(|| {
        "cc_bindings_from_rs binary not found via CC_BINDINGS_FROM_RS env var, Bazel runfiles, adjacent to executable, or in system PATH".into()
    })
}

pub fn get_clang_format_path() -> Option<PathBuf> {
    find_resource(&ResourceSearchConfig {
        env_vars: &["CLANG_FORMAT", "CRUBIT_CLANG_FORMAT_EXE_PATH"],
        runfile_candidates: &[
            CLANG_FORMAT_RLOCATION,
        ],
        adjacent_candidates: &["clang-format", "stable_clang-format"],
        path_binaries: &["clang-format"],
    })
}

pub fn get_frontend_dist_path() -> Option<PathBuf> {
    find_resource(&ResourceSearchConfig {
        runfile_candidates: &[
            "crubit_explorer/frontend/dist/frontend",
        ],
        adjacent_candidates: &["frontend/dist/frontend", "dist/frontend"],
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[gtest]
    fn test_resource_search_config_default() {
        let config = ResourceSearchConfig::default();
        expect_true!(config.env_vars.is_empty());
        expect_true!(config.runfile_candidates.is_empty());
        expect_true!(config.adjacent_candidates.is_empty());
        expect_true!(config.path_binaries.is_empty());
    }
}
