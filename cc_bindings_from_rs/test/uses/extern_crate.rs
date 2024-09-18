// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

mod private_mod {
    pub struct X {
        pub field: i32,
    }

    pub struct Y {
        pub field: i32,
    }
}

pub use self::private_mod::X;
pub use self::private_mod::Y;
