// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod module {
    pub fn function() {}
    #[derive(Default)]
    pub struct Type {
        pub x: i32,
    }
}

pub use module::function;
pub use module::Type;
