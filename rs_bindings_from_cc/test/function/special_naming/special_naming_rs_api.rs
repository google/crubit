// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/special_naming:special_naming

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

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

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "\u{1}_llvm_no_mangle_marker"]
        pub(crate) unsafe fn __rust_thunk___llvm_no_mangle_marker() -> ::ffi_11::c_int;
        #[link_name = "asm$name$with$dollar$sign"]
        pub(crate) unsafe fn __rust_thunk__asm_u36_name_u36_with_u36_dollar_u36_sign(
        ) -> ::ffi_11::c_int;
    }
}
