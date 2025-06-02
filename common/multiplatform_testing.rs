// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Vocabulary library for multi-platform tests which use cross-compilation.

use std::sync::LazyLock;

#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Platform {
    X86Linux,
    ArmLinux,
    X86MacOS,
    ArmMacOS,
}

impl Platform {
    pub fn target_triple(self) -> &'static str {
        match self {
            Platform::X86Linux => "x86_64-unknown-linux-gnu",
            Platform::ArmLinux => "aarch64-unknown-linux-gnu",
            Platform::X86MacOS => "x86_64-apple-darwin",
            Platform::ArmMacOS => "arm64-apple-darwin",
        }
    }
}

/// Returns the platform the current test is running for with
/// multiplatform_rust_test.
pub fn test_platform() -> Platform {
    *TEST_PLATFORM.as_ref().unwrap()
}

static TEST_PLATFORM: LazyLock<Result<Platform, String>> = LazyLock::new(|| {
    let env = std::env::var("CRUBIT_TEST_PLATFORM")
        .map_err(|_| "multiplatform tests must use `multiplatform_rust_test`.".to_string())?;
    let platform = match env.as_str() {
        "x86_linux" => Platform::X86Linux,
        "arm_linux" => Platform::ArmLinux,
        "darwin_x86_64" => Platform::X86MacOS,
        "darwin_arm64" => Platform::ArmMacOS,
        _ => return Err(format!("Unknown platform: {env}")),
    };
    Ok(platform)
});
