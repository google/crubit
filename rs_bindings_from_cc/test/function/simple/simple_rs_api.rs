// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/simple:simple

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(rust_2024_compatibility)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

#[inline(always)]
pub fn return_value() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z12return_valuev() }
}

#[inline(always)]
pub fn return_pointer() -> *mut ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z14return_pointerv() }
}

#[inline(always)]
pub fn return_reference() -> *mut ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z16return_referencev() }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `i`: raw pointer
#[inline(always)]
pub unsafe fn take_pointer(i: *mut ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z12take_pointerPi(i) }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `i`: raw pointer
#[inline(always)]
pub unsafe fn take_reference(i: *mut ::ffi_11::c_int) {
    unsafe { crate::detail::__rust_thunk___Z14take_referenceRi(i) }
}

#[inline(always)]
pub fn forward_pointer(i: *const ::ffi_11::c_int) -> *const ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z15forward_pointerPKi(i) }
}

/// # Safety
///
/// The caller must ensure that the following unsafe arguments are not misused by the function:
/// * `i`: raw pointer
#[inline(always)]
pub unsafe fn forward_reference(i: *const ::ffi_11::c_int) -> *const ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z17forward_referenceRKi(i) }
}

#[inline(always)]
pub fn multiply(x: ::ffi_11::c_int, y: ::ffi_11::c_int) -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z8multiplyii(x, y) }
}

#[inline(always)]
pub fn multiply_with_unnamed_parameters(
    __param_0: ::ffi_11::c_int,
    __param_1: ::ffi_11::c_int,
) -> ::ffi_11::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z32multiply_with_unnamed_parametersii(__param_0, __param_1)
    }
}

#[inline(always)]
pub fn multiply_with_keyword_named_parameters(
    __param_0: ::ffi_11::c_int,
    __param_1: ::ffi_11::c_int,
    __param_2: ::ffi_11::c_int,
) -> ::ffi_11::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z38multiply_with_keyword_named_parametersiii(
            __param_0, __param_1, __param_2,
        )
    }
}

// LLVM identifiers use the `\\01` prefix to suppress mangling:
// https://llvm.org/docs/LangRef.html#identifiers
// Test that we can import functions that have such names.
// If `__USER_LABEL_PREFIX__` is non-empty, the Clang mangler adds the `\\01`
// prefix; otherwise, we add it here ourselves.

#[inline(always)]
pub fn llvm_no_mangle_marker() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___llvm_no_mangle_marker() }
}

/// Test that we can import functions whose `__asm` name contains a dollar sign.
/// For example, the Apple SDKs use dollar signs in their symbol versioning
/// macros (e.g. `__DARWIN_EXTSN()`).
#[inline(always)]
pub fn asm_name_with_dollar_sign() -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk__asm_u36_name_u36_with_u36_dollar_u36_sign() }
}

/// https://cdecl.org/?q=int+%28*get_multiply_function%28%29%29%28int%2C+int%29:
/// declare foo as function returning pointer to function (int, int) returning
/// int
#[inline(always)]
pub fn get_pointer_to_multiply_function(
) -> Option<extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int> {
    unsafe { crate::detail::__rust_thunk___Z32get_pointer_to_multiply_functionv() }
}

/// Same as above, but returning a *reference* to a function.
#[inline(always)]
pub fn get_reference_to_multiply_function(
) -> extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z34get_reference_to_multiply_functionv() }
}

#[inline(always)]
pub fn inline_get_pointer_to_multiply_function(
) -> Option<extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int> {
    unsafe { crate::detail::__rust_thunk___Z39inline_get_pointer_to_multiply_functionv() }
}

#[inline(always)]
pub fn apply_binary_op(
    x: ::ffi_11::c_int,
    y: ::ffi_11::c_int,
    op: Option<extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int>,
) -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z15apply_binary_opiiPFiiiE(x, y, op) }
}

// TODO(b/217419782): Add testcases for pointers to functions that take or
// return takes/returns non-trivially-movable types by value. In particular,
// some function signatures might require going through a C++ thunk - such
// function pointers can't work without a thunk. See also
// <internal link>

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z12return_valuev"]
        pub(crate) unsafe fn __rust_thunk___Z12return_valuev() -> ::ffi_11::c_int;
        #[link_name = "_Z14return_pointerv"]
        pub(crate) unsafe fn __rust_thunk___Z14return_pointerv() -> *mut ::ffi_11::c_int;
        #[link_name = "_Z16return_referencev"]
        pub(crate) unsafe fn __rust_thunk___Z16return_referencev() -> *mut ::ffi_11::c_int;
        #[link_name = "_Z12take_pointerPi"]
        pub(crate) unsafe fn __rust_thunk___Z12take_pointerPi(i: *mut ::ffi_11::c_int);
        #[link_name = "_Z14take_referenceRi"]
        pub(crate) unsafe fn __rust_thunk___Z14take_referenceRi(i: *mut ::ffi_11::c_int);
        #[link_name = "_Z15forward_pointerPKi"]
        pub(crate) unsafe fn __rust_thunk___Z15forward_pointerPKi(
            i: *const ::ffi_11::c_int,
        ) -> *const ::ffi_11::c_int;
        #[link_name = "_Z17forward_referenceRKi"]
        pub(crate) unsafe fn __rust_thunk___Z17forward_referenceRKi(
            i: *const ::ffi_11::c_int,
        ) -> *const ::ffi_11::c_int;
        #[link_name = "_Z8multiplyii"]
        pub(crate) unsafe fn __rust_thunk___Z8multiplyii(
            x: ::ffi_11::c_int,
            y: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        #[link_name = "_Z32multiply_with_unnamed_parametersii"]
        pub(crate) unsafe fn __rust_thunk___Z32multiply_with_unnamed_parametersii(
            __param_0: ::ffi_11::c_int,
            __param_1: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        #[link_name = "_Z38multiply_with_keyword_named_parametersiii"]
        pub(crate) unsafe fn __rust_thunk___Z38multiply_with_keyword_named_parametersiii(
            __param_0: ::ffi_11::c_int,
            __param_1: ::ffi_11::c_int,
            __param_2: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        #[link_name = "\u{1}_llvm_no_mangle_marker"]
        pub(crate) unsafe fn __rust_thunk___llvm_no_mangle_marker() -> ::ffi_11::c_int;
        #[link_name = "asm$name$with$dollar$sign"]
        pub(crate) unsafe fn __rust_thunk__asm_u36_name_u36_with_u36_dollar_u36_sign(
        ) -> ::ffi_11::c_int;
        #[link_name = "_Z32get_pointer_to_multiply_functionv"]
        pub(crate) unsafe fn __rust_thunk___Z32get_pointer_to_multiply_functionv(
        ) -> Option<extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int>;
        #[link_name = "_Z34get_reference_to_multiply_functionv"]
        pub(crate) unsafe fn __rust_thunk___Z34get_reference_to_multiply_functionv(
        ) -> extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___Z39inline_get_pointer_to_multiply_functionv(
        ) -> Option<extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk___Z15apply_binary_opiiPFiiiE(
            x: ::ffi_11::c_int,
            y: ::ffi_11::c_int,
            op: Option<extern "C" fn(::ffi_11::c_int, ::ffi_11::c_int) -> ::ffi_11::c_int>,
        ) -> ::ffi_11::c_int;
    }
}
