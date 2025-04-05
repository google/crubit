// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `tuples_test.cc`.

#[derive(Default, Copy, Clone)]
pub struct TypeWithStr {
    pub str_field: &'static str,
}

impl TypeWithStr {
    pub fn create(s: &'static str) -> Self {
        Self { str_field: s }
    }

    pub fn get_str_len(&self) -> usize {
        self.str_field.len()
    }

    pub fn get_str_data(&self) -> *const u8 {
        self.str_field.as_ptr()
    }
}

// This function checks that we don't generate bindings for potentially aliasing mutable args.
pub fn should_not_generate_bindings(_: &str, _: &mut u8) {}

pub fn get_str_len(s: &str) -> usize {
    s.len()
}

pub fn get_str_data(s: &str) -> *const u8 {
    s.as_ptr()
}

pub fn foo_as_str() -> &'static str {
    "foo"
}

pub const CONST_STR_FOO: &str = "foo";

pub static STATIC_STR_FOO: &str = "foo";
