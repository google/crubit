// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
pub mod simple_math {
    pub fn get_two() -> i32 {
        2
    }

    pub mod add {
        // This function does not cause compile errors, as long as it is not called.
        pub fn add_one() -> i32 {
            3
        }
    }
}

pub mod complex_math {
    pub mod two_only {
        pub fn get_square() -> i32 {
            4
        }
    }
}
