// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
use base64::engine::general_purpose::PAD;
use base64::engine::{Alphabet, GeneralPurpose};
use std::path::{Path, PathBuf};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Test function to expose a dependency of the library and make sure it shows up in bindings.
pub fn make_engine() -> GeneralPurpose {
    let alphabet =
        Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/").unwrap();
    GeneralPurpose::new(&alphabet, PAD)
}

pub fn print_path(path: &Path) -> String {
    path.display().to_string()
}

pub struct Gymnastics {
    internal: String,
}
impl Gymnastics {
    pub fn new(path: PathBuf) -> Self {
        Self { internal: print_path(&path) }
    }

    pub fn as_str(&self) -> &str {
        &self.internal
    }
}
