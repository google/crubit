// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `arrays_test.cc`.

pub fn function_with_const_array_ptr_id(array_ptr: *const [i32; 2]) -> *const [i32; 2] {
    array_ptr
}
// TODO: b/260128806 - Support this use.
pub fn function_with_array_id(array: [i32; 2]) -> [i32; 2] {
    array
}
// TODO: b/260128806 - Support this use.
pub fn function_with_array_tuple_id(array_tup: ([i32; 2], [i32; 2])) -> ([i32; 2], [i32; 2]) {
    array_tup
}

#[derive(Default, Clone, Copy)]
pub struct ArrayStruct {
    pub array: [i32; 2],
}

pub fn function_with_array_struct_id(array_struct: ArrayStruct) -> ArrayStruct {
    array_struct
}
