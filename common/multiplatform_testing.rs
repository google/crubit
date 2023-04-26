// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Vocabulary library for multi-platform tests which use cross-compilation.

use once_cell::sync::Lazy;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Platform {
    X86Linux,
    ArmLinux,
}

impl Platform {
    pub fn target_triple(self) -> &'static str {
        match self {
            Platform::X86Linux => "x86_64-grtev4-linux-gnu",
            Platform::ArmLinux => "aarch64-grtev4-linux-gnu",
        }
    }
}

/// Returns the platform the current test is running for with
/// multiplatform_rust_test.
pub fn test_platform() -> Platform {
    *TEST_PLATFORM.as_ref().unwrap()
}

static TEST_PLATFORM: Lazy<Result<Platform, String>> = Lazy::new(|| {
    let env = std::env::var("CRUBIT_TEST_PLATFORM")
        .map_err(|_| "multiplatform tests must use `multiplatform_rust_test`.".to_string())?;
    let platform = match env.as_str() {
        "x86_linux" => Platform::X86Linux,
        "arm_linux" => Platform::ArmLinux,
        _ => return Err(format!("Unknown platform: {env}")),
    };
    Ok(platform)
});
