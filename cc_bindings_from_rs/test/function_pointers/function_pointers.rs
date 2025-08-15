// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! This crate is used as a test input for `cc_bindings_from_rs` and the
//! generated C++ bindings are then tested via `function_pointers_test.cc`.

use crubit_annotate::must_bind;

#[must_bind]
#[derive(Copy, Clone)]
pub struct HasFnPtrField {
    pub ptr: extern "C" fn(i32) -> i32,
}

impl HasFnPtrField {
    pub fn with_add_ten() -> Self {
        Self { ptr: add_ten }
    }
}

extern "C" fn add_ten(x: i32) -> i32 {
    x + 10
}

pub const CONST_RUST_FN_PTR_ADD_TEN: fn(i32) -> i32 = |x| x + 10;
pub const CONST_C_FN_PTR_ADD_TEN: extern "C" fn(i32) -> i32 = add_ten;

#[must_bind]
pub fn call_fn_ptr_no_args_or_return(fn_ptr: extern "C" fn()) {
    fn_ptr()
}

/// Currently generates no bindings: Rust-ABI function pointers are not supported.
pub fn call_rust_fn_ptr(fn_ptr: fn()) {
    fn_ptr()
}

#[must_bind]
pub fn call_fn_ptr_with_five(fn_ptr: extern "C" fn(i32) -> i32) -> i32 {
    fn_ptr(5)
}

// TODO: b/259749023 - support generic function pointers.
pub fn call_fn_ptr_with_five_reference(fn_ptr: extern "C" fn(&i32) -> i32) -> i32 {
    fn_ptr(&5)
}

// TODO: b/259749023 - support generic function pointers.
pub fn call_fn_ptr_with_five_reference_hrtb(fn_ptr: for<'a> extern "C" fn(&'a i32) -> i32) -> i32 {
    fn_ptr(&5)
}

#[must_bind]
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct CStruct {
    pub field: i32,
}

#[must_bind]
pub fn call_fn_ptr_with_repr_c_struct_ptr_containing_seven(
    fn_ptr: unsafe extern "C" fn(*const CStruct) -> i32,
) -> i32 {
    unsafe { fn_ptr(&CStruct { field: 7 }) }
}

// Currently generates no bindings: "Type of parameter #0 requires a thunk".
pub fn call_fn_ptr_with_repr_c_struct(fn_ptr: extern "C" fn(CStruct) -> i32) -> i32 {
    fn_ptr(CStruct { field: 5 })
}

// TODO: b/259749023 - support generic function pointers.
pub fn call_fn_ptr_with_repr_c_struct_ref(fn_ptr: extern "C" fn(&CStruct) -> i32) -> i32 {
    fn_ptr(&CStruct { field: 5 })
}
