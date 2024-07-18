// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Put before the real definition to make sure that the generarated C++ bindings
// is not affected by the order of the imports.
pub use test_mod::f;

pub mod test_mod {
    pub fn f() -> i32 {
        42
    }
}
