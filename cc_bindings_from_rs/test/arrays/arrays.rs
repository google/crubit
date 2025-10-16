// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `arrays_test.cc`.

pub fn function_with_const_array_ptr_id(array_ptr: *const [i32; 2]) -> *const [i32; 2] {
    array_ptr
}

pub fn function_with_array_id(array: [i32; 2]) -> [i32; 2] {
    array
}

pub fn function_with_array_tuple_id(array_tup: ([i32; 2], [i32; 2])) -> ([i32; 2], [i32; 2]) {
    array_tup
}

// Will not generate a stub: tuple types cannot be used inside of compound data types,
// because std::tuple is not layout-compatible with a Rust tuple.
pub fn function_with_tuple_array_id(tup_array: [(i32, i32); 2]) -> [(i32, i32); 2] {
    tup_array
}

const NAMED_SIZE: usize = 3;
pub fn function_with_mut_array_named_size_ptr_id(
    array_ptr: *const [i32; NAMED_SIZE],
) -> *const [i32; NAMED_SIZE] {
    array_ptr
}

#[derive(Default, Clone, Copy)]
pub struct ArrayStruct {
    pub array: [i32; 2],
}

pub fn function_with_array_struct_id(array_struct: ArrayStruct) -> ArrayStruct {
    array_struct
}

pub struct HasDrop {
    pub x: i32,
}

impl HasDrop {
    pub fn new(x: i32) -> HasDrop {
        HasDrop { x }
    }
}

impl Drop for HasDrop {
    fn drop(&mut self) {}
}

pub fn function_with_has_drop_array_id(array: [HasDrop; 2]) -> [HasDrop; 2] {
    dbg!(array[0].x);
    dbg!(array[1].x);
    array
}

pub fn function_with_has_drop_ret_only() -> [HasDrop; 2] {
    [HasDrop::new(1), HasDrop::new(2)]
}

#[derive(Default)]
pub struct HasDropAndDefault {
    pub x: i32,
}

impl Drop for HasDropAndDefault {
    fn drop(&mut self) {}
}

pub fn function_with_has_drop_and_default_array_id(
    array: [HasDropAndDefault; 2],
) -> [HasDropAndDefault; 2] {
    array
}

// TODO: b/260128806 - we conservatively reject nested arrays.
pub fn function_with_nested_arrays(array: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
    array
}

// TODO: b/451981992 - we don't support nested arrays with types that are Drop but not Default.
pub fn function_with_nested_droponly_arrays(array: [[HasDrop; 2]; 2]) -> [[HasDrop; 2]; 2] {
    array
}

pub fn function_with_empty_array(array: [i32; 0]) -> [i32; 0] {
    array
}
