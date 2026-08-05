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

const RUSTFMT_RLOCATION: &str =
"rustfmt";

const DOXYGEN_RLOCATION: &str =
"rules_doxygen/doxygen";

const DOXYFILE_RLOCATION: &str =
"crubit_explorer/Doxyfile";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResourceSearchConfig<'a> {
    pub env_vars: &'a [&'a str],
    pub runfile_candidates: &'a [&'a str],
    pub adjacent_candidates: &'a [&'a str],
    pub path_binaries: &'a [&'a str],
    pub allow_directory: bool,
}

impl<'a> ResourceSearchConfig<'a> {
    fn is_valid_match(&self, path: &std::path::Path) -> bool {
        path.is_file() || (self.allow_directory && path.is_dir())
    }
}

pub fn find_resource(config: &ResourceSearchConfig) -> Option<PathBuf> {
    for &env_var in config.env_vars {
        if let Ok(env_path) = env::var(env_var)
            && let path = PathBuf::from(&env_path)
            && config.is_valid_match(&path)
        {
            return Some(path);
        }
    }

    if let Ok(r) = Runfiles::create() {
        for &candidate in config.runfile_candidates {
            if let Some(path) = runfiles::rlocation!(r, candidate)
                && config.is_valid_match(&path)
            {
                return Some(path);
            }
        }
    }

    if let Ok(mut exe_path) = env::current_exe() {
        exe_path.pop();
        for &adjacent_name in config.adjacent_candidates {
            let adjacent_path = exe_path.join(adjacent_name);
            if config.is_valid_match(&adjacent_path) {
                return Some(adjacent_path);
            }
        }
    }

    if let Ok(cwd) = env::current_dir() {
        for &adjacent_name in config.adjacent_candidates {
            let cwd_path = cwd.join(adjacent_name);
            if config.is_valid_match(&cwd_path) {
                return Some(cwd_path);
            }
        }
    }

    for &bin_name in config.path_binaries {
        if let Some(path) = find_in_path(bin_name)
            && config.is_valid_match(&path)
        {
            return Some(path);
        }
    }

    None
}

pub fn find_in_path(name: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths).find_map(|dir| {
        let full_path = dir.join(name);
        full_path.is_file().then_some(full_path)
    })
}

pub fn get_cc_bindings_from_rs_path() -> Result<PathBuf, Box<dyn Error>> {
    find_resource(&ResourceSearchConfig {
        env_vars: &["CC_BINDINGS_FROM_RS"],
        runfile_candidates: &[CC_BINDINGS_FROM_RS_RLOCATION],
        adjacent_candidates: &["cc_bindings_from_rs"],
        path_binaries: &["cc_bindings_from_rs"],
        allow_directory: false,
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
        allow_directory: false,
    })
}

pub fn get_rustfmt_path() -> Option<PathBuf> {
    find_resource(&ResourceSearchConfig {
        env_vars: &["RUSTFMT", "CRUBIT_RUSTFMT_EXE_PATH"],
        runfile_candidates: &[
            RUSTFMT_RLOCATION,
        ],
        adjacent_candidates: &["rustfmt"],
        path_binaries: &["rustfmt"],
        allow_directory: false,
    })
}

pub fn get_frontend_dist_path() -> Option<PathBuf> {
    find_resource(&ResourceSearchConfig {
        runfile_candidates: &[
            "crubit_explorer/frontend/dist/frontend",
        ],
        adjacent_candidates: &["frontend/dist/frontend", "dist/frontend"],
        allow_directory: true,
        ..Default::default()
    })
}

pub fn get_doxygen_path() -> Result<PathBuf, Box<dyn Error>> {
    find_resource(&ResourceSearchConfig {
        env_vars: &["DOXYGEN"],
        runfile_candidates: &[
            "doxygen/linux/doxygen",
            "doxygen/mac/doxygen",
            "doxygen/windows/doxygen.exe",
            "doxygen/executable",
            DOXYGEN_RLOCATION,
            "rules_doxygen/doxygen",
            "doxygen/doxygen",
            "doxygen",
        ],
        adjacent_candidates: &["doxygen"],
        path_binaries: &["doxygen"],
        allow_directory: false,
    })
    .ok_or_else(|| {
        "doxygen binary not found via DOXYGEN env var, Bazel runfiles, adjacent to executable, or in system PATH".into()
    })
}

pub fn get_doxyfile_path() -> Result<PathBuf, Box<dyn Error>> {
    find_resource(&ResourceSearchConfig {
        env_vars: &["DOXYFILE"],
        runfile_candidates: &[
            DOXYFILE_RLOCATION,
            "crubit_explorer/Doxyfile",
            "Doxyfile",
        ],
        adjacent_candidates: &["Doxyfile"],
        path_binaries: &[],
        allow_directory: false,
    })
    .ok_or_else(|| {
        "Doxyfile not found via DOXYFILE env var, Bazel runfiles, or adjacent to executable".into()
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
        expect_true!(!config.allow_directory);
    }

    #[gtest]
    fn test_is_valid_match() {
        let temp_dir = tempfile::tempdir().unwrap();
        let dir_path = temp_dir.path();
        let file_path = dir_path.join("test_file");
        std::fs::write(&file_path, "hello").unwrap();

        let config_no_dir = ResourceSearchConfig { allow_directory: false, ..Default::default() };
        let config_allow_dir = ResourceSearchConfig { allow_directory: true, ..Default::default() };

        expect_true!(config_no_dir.is_valid_match(&file_path));
        expect_true!(config_allow_dir.is_valid_match(&file_path));

        expect_true!(!config_no_dir.is_valid_match(dir_path));
        expect_true!(config_allow_dir.is_valid_match(dir_path));
    }
}
