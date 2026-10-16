// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `arrays_test.cc`.

use core::sync::atomic::{AtomicUsize, Ordering};
use crubit_annotate::must_bind;

static HAS_DROP_DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Returns the number of times `HasDrop::drop` has run.
#[must_bind]
pub fn has_drop_drop_count() -> usize {
    HAS_DROP_DROP_COUNT.load(Ordering::SeqCst)
}

#[must_bind]
pub fn function_with_const_array_ptr_id(array_ptr: *const [i32; 2]) -> *const [i32; 2] {
    array_ptr
}

#[must_bind]
pub fn function_with_array_id(array: [i32; 2]) -> [i32; 2] {
    array
}

#[must_bind]
pub fn function_with_array_tuple_id(array_tup: ([i32; 2], [i32; 2])) -> ([i32; 2], [i32; 2]) {
    array_tup
}

// Will not generate a stub: tuple types cannot be used inside of compound data types,
// because std::tuple is not layout-compatible with a Rust tuple.
pub fn function_with_tuple_array_id(tup_array: [(i32, i32); 2]) -> [(i32, i32); 2] {
    tup_array
}

const NAMED_SIZE: usize = 3;

#[must_bind]
pub fn function_with_mut_array_named_size_ptr_id(
    array_ptr: *const [i32; NAMED_SIZE],
) -> *const [i32; NAMED_SIZE] {
    array_ptr
}

#[derive(Default, Clone, Copy)]
pub struct ArrayStruct {
    pub array: [i32; 2],
}

#[must_bind]
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
    fn drop(&mut self) {
        HAS_DROP_DROP_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

#[must_bind]
pub fn function_with_has_drop_array_id(array: [HasDrop; 2]) -> [HasDrop; 2] {
    dbg!(array[0].x);
    dbg!(array[1].x);
    array
}

#[must_bind]
pub fn function_with_has_drop_ret_only() -> [HasDrop; 2] {
    [HasDrop::new(1), HasDrop::new(2)]
}

#[derive(Default)]
pub struct HasDropAndDefault {
    pub x: i32,
}

static HAS_DROP_AND_DEFAULT_NONZERO_DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Returns the number of times `HasDropAndDefault::drop` has run on a value whose `x` is not 0
/// (i.e. not a default value).
#[must_bind]
pub fn has_drop_and_default_nonzero_drop_count() -> usize {
    HAS_DROP_AND_DEFAULT_NONZERO_DROP_COUNT.load(Ordering::SeqCst)
}

impl Drop for HasDropAndDefault {
    fn drop(&mut self) {
        if self.x != 0 {
            HAS_DROP_AND_DEFAULT_NONZERO_DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }
}

#[must_bind]
pub fn function_with_has_drop_and_default_array_id(
    array: [HasDropAndDefault; 2],
) -> [HasDropAndDefault; 2] {
    array
}

#[must_bind]
pub fn function_with_nested_arrays(array: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
    array
}

#[must_bind]
pub fn function_with_nested_droponly_arrays(array: [[HasDrop; 2]; 2]) -> [[HasDrop; 2]; 2] {
    array
}

#[must_bind]
pub fn function_with_nested_nested_droponly_arrays(
    array: [[[HasDrop; 2]; 2]; 2],
) -> [[[HasDrop; 2]; 2]; 2] {
    array
}

#[must_bind]
pub fn function_with_nested_drop_default_arrays(
    array: [[HasDropAndDefault; 2]; 2],
) -> [[HasDropAndDefault; 2]; 2] {
    array
}

#[must_bind]
pub fn function_with_empty_array(array: [i32; 0]) -> [i32; 0] {
    array
}
