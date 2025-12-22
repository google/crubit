// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/simple:simple
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=10
#[inline(always)]
pub fn return_value() -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z12return_valuev() }
}

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=11
#[inline(always)]
pub fn return_pointer() -> *mut ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z14return_pointerv() }
}

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=12
#[inline(always)]
pub fn return_reference() -> *mut ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z16return_referencev() }
}

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=13
#[inline(always)]
pub unsafe fn take_pointer(i: *mut ::core::ffi::c_int) {
    crate::detail::__rust_thunk___Z12take_pointerPi(i)
}

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=14
#[inline(always)]
pub unsafe fn take_reference(i: *mut ::core::ffi::c_int) {
    crate::detail::__rust_thunk___Z14take_referenceRi(i)
}

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=15
#[inline(always)]
pub fn forward_pointer(i: *const ::core::ffi::c_int) -> *const ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z15forward_pointerPKi(i) }
}

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=17
#[inline(always)]
pub unsafe fn forward_reference(i: *const ::core::ffi::c_int) -> *const ::core::ffi::c_int {
    crate::detail::__rust_thunk___Z17forward_referenceRKi(i)
}

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=18
#[inline(always)]
pub fn multiply(x: ::core::ffi::c_int, y: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z8multiplyii(x, y) }
}

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=19
#[inline(always)]
pub fn multiply_with_unnamed_parameters(
    __param_0: ::core::ffi::c_int,
    __param_1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z32multiply_with_unnamed_parametersii(__param_0, __param_1)
    }
}

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=20
#[inline(always)]
pub fn multiply_with_keyword_named_parameters(
    __param_0: ::core::ffi::c_int,
    __param_1: ::core::ffi::c_int,
    __param_2: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=32
#[inline(always)]
pub fn llvm_no_mangle_marker() -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___llvm_no_mangle_marker() }
}

/// Test that we can import functions whose `__asm` name contains a dollar sign.
/// For example, the Apple SDKs use dollar signs in their symbol versioning
/// macros (e.g. `__DARWIN_EXTSN()`).
///
/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=40
#[inline(always)]
pub fn asm_name_with_dollar_sign() -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk__asm_u36_name_u36_with_u36_dollar_u36_sign() }
}

/// https://cdecl.org/?q=int+%28*get_multiply_function%28%29%29%28int%2C+int%29:
/// declare foo as function returning pointer to function (int, int) returning
/// int
///
/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=46
#[inline(always)]
pub fn get_pointer_to_multiply_function(
) -> Option<extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int> {
    unsafe { crate::detail::__rust_thunk___Z32get_pointer_to_multiply_functionv() }
}

/// Same as above, but returning a *reference* to a function.
///
/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=49
#[inline(always)]
pub fn get_reference_to_multiply_function(
) -> extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z34get_reference_to_multiply_functionv() }
}

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=51
#[inline(always)]
pub fn inline_get_pointer_to_multiply_function(
) -> Option<extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int> {
    unsafe { crate::detail::__rust_thunk___Z39inline_get_pointer_to_multiply_functionv() }
}

/// Generated from: rs_bindings_from_cc/test/function/simple/simple.h;l=56
#[inline(always)]
pub fn apply_binary_op(
    x: ::core::ffi::c_int,
    y: ::core::ffi::c_int,
    op: Option<extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    unsafe { crate::detail::__rust_thunk___Z15apply_binary_opiiPFiiiE(x, y, op) }
}

// TODO(b/217419782): Add testcases for pointers to functions that take or
// return takes/returns non-trivially-movable types by value. In particular,
// some function signatures might require going through a C++ thunk - such
// function pointers can't work without a thunk. See also
// <internal link>

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/function/simple:simple needs [//features:wrapper] for std::integral_constant<bool, false> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb0EEE is a template instantiation)

// Generated from: nowhere/llvm/src/libcxx/include/__type_traits/integral_constant.h;l=21
// Error while generating bindings for struct 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/function/simple:simple needs [//features:wrapper] for std::integral_constant<bool, true> (crate::__CcTemplateInstNSt3__u17integral_constantIbLb1EEE is a template instantiation)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z12return_valuev"]
        pub(crate) unsafe fn __rust_thunk___Z12return_valuev() -> ::core::ffi::c_int;
        #[link_name = "_Z14return_pointerv"]
        pub(crate) unsafe fn __rust_thunk___Z14return_pointerv() -> *mut ::core::ffi::c_int;
        #[link_name = "_Z16return_referencev"]
        pub(crate) unsafe fn __rust_thunk___Z16return_referencev() -> *mut ::core::ffi::c_int;
        #[link_name = "_Z12take_pointerPi"]
        pub(crate) unsafe fn __rust_thunk___Z12take_pointerPi(i: *mut ::core::ffi::c_int);
        #[link_name = "_Z14take_referenceRi"]
        pub(crate) unsafe fn __rust_thunk___Z14take_referenceRi(i: *mut ::core::ffi::c_int);
        #[link_name = "_Z15forward_pointerPKi"]
        pub(crate) unsafe fn __rust_thunk___Z15forward_pointerPKi(
            i: *const ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_int;
        #[link_name = "_Z17forward_referenceRKi"]
        pub(crate) unsafe fn __rust_thunk___Z17forward_referenceRKi(
            i: *const ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_int;
        #[link_name = "_Z8multiplyii"]
        pub(crate) unsafe fn __rust_thunk___Z8multiplyii(
            x: ::core::ffi::c_int,
            y: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[link_name = "_Z32multiply_with_unnamed_parametersii"]
        pub(crate) unsafe fn __rust_thunk___Z32multiply_with_unnamed_parametersii(
            __param_0: ::core::ffi::c_int,
            __param_1: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[link_name = "_Z38multiply_with_keyword_named_parametersiii"]
        pub(crate) unsafe fn __rust_thunk___Z38multiply_with_keyword_named_parametersiii(
            __param_0: ::core::ffi::c_int,
            __param_1: ::core::ffi::c_int,
            __param_2: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[link_name = "\u{1}_llvm_no_mangle_marker"]
        pub(crate) unsafe fn __rust_thunk___llvm_no_mangle_marker() -> ::core::ffi::c_int;
        #[link_name = "asm$name$with$dollar$sign"]
        pub(crate) unsafe fn __rust_thunk__asm_u36_name_u36_with_u36_dollar_u36_sign(
        ) -> ::core::ffi::c_int;
        #[link_name = "_Z32get_pointer_to_multiply_functionv"]
        pub(crate) unsafe fn __rust_thunk___Z32get_pointer_to_multiply_functionv(
        ) -> Option<extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>;
        #[link_name = "_Z34get_reference_to_multiply_functionv"]
        pub(crate) unsafe fn __rust_thunk___Z34get_reference_to_multiply_functionv(
        ) -> extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___Z39inline_get_pointer_to_multiply_functionv(
        ) -> Option<extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>;
        pub(crate) unsafe fn __rust_thunk___Z15apply_binary_opiiPFiiiE(
            x: ::core::ffi::c_int,
            y: ::core::ffi::c_int,
            op: Option<extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>,
        ) -> ::core::ffi::c_int;
    }
}
