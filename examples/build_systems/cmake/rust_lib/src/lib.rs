// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Test function to expose a dependency of the library and make sure it shows up in bindings.
pub fn make_engine() -> base64::engine::GeneralPurpose {
    let alphabet = base64::alphabet::Alphabet::new(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
    )
    .unwrap();
    base64::engine::GeneralPurpose::new(&alphabet, base64::engine::general_purpose::PAD)
}

pub fn print_path(path: &std::path::Path) -> String {
    path.display().to_string()
}

pub struct Gymnastics {
    internal: std::string::String,
}
impl Gymnastics {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { internal: print_path(&path) }
    }

    pub fn as_str(&self) -> &str {
        &self.internal
    }
}
