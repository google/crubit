// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `never_test.cc`.

pub fn never_return() -> ! {
    panic!("You can't do that!")
}

#[unsafe(no_mangle)]
pub extern "C" fn extern_never_return() -> ! {
    panic!("You can't do that directly!");
}

#[derive(Default)]
pub struct NeverStruct {
    /// Having a non-ZST field avoids hitting the following error:
    /// "Zero-sized types (ZSTs) are not supported (b/258259459)"
    _non_zst_field: i32,
}

impl NeverStruct {
    pub fn associated_fn_never_return() -> ! {
        panic!("You can't do that as an associated fn!");
    }

    pub fn method_never_return(&self) -> ! {
        panic!("You can't do that as a method!");
    }
}
