// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use has_bindings::crubit::has_bindings;

use item_exists::{type_exists, value_exists};

#[test]
fn test_void_function() {
    has_bindings::crubit_void_function();
}
#[test]
fn test_void_ptr_function() {
    let value = 1;
    let ptr = &value as *const _ as *const std::ffi::c_void;
    // Safety: the pointer is valid in both C++ and Rust, and is not dereferenced.
    let result_ptr = unsafe { has_bindings::crubit_void_ptr_identity(ptr) };
    assert_eq!(ptr, result_ptr);
}

#[test]
fn test_user_struct() {
    // TODO(b/314382764): make Struct usable from Rust (by supporting all its
    // fields), and in the process make it possible to reasonably call/test the
    // function, too.
    assert!(type_exists!(has_bindings::Struct));
    assert!(value_exists!(has_bindings::crubit_anystruct));
}
