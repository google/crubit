// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate std;

impl std::fmt::Display for crate::DisplayInRust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*std::string::String::from_utf8_lossy(
            // SAFETY: will not pass to mutating C++ code.
            unsafe { self.rust_value.as_live().as_bytes() },
        ))
    }
}
