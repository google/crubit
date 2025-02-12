// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:c_abi_compatible_type_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

// Type bindings for struct MyI8 suppressed due to being mapped to an existing Rust type (i8)

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "X")]
pub struct X {
    pub a: ::core::ffi::c_int,
}
impl !Send for X {}
impl !Sync for X {}
forward_declare::unsafe_define!(forward_declare::symbol!("X"), crate::X);

// Error while generating bindings for item 'X::X':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::X
// Missing lifetime for `__this` parameter type: *mut crate::X

// Error while generating bindings for item 'X::X':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::X
// Missing lifetime for `__this` parameter type: *mut crate::X

// Error while generating bindings for item 'X::X':
// Parameter #0 is not supported: Unsupported type 'X &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'X::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'X::operator=':
// Parameter #0 is not supported: Unsupported type 'X &&': Unsupported type: && without lifetime

#[inline(always)]
pub fn ffi(a: i8, mut b: crate::X) -> i8 {
    unsafe { crate::detail::__rust_thunk___Z3ffi4MyI81X(a, &mut b) }
}

pub type MyTypedefDecl = ::core::ffi::c_int;

#[inline(always)]
pub unsafe fn f(a: crate::MyTypedefDecl, b: *mut ::core::ffi::c_void, c: ::core::ffi::c_int) {
    crate::detail::__rust_thunk___Z1fiPvi(a, b, c)
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z3ffi4MyI81X(a: i8, b: &mut crate::X) -> i8;
        pub(crate) unsafe fn __rust_thunk___Z1fiPvi(
            a: crate::MyTypedefDecl,
            b: *mut ::core::ffi::c_void,
            c: ::core::ffi::c_int,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);

    assert!(::core::mem::size_of::<crate::X>() == 4);
    assert!(::core::mem::align_of::<crate::X>() == 4);
    static_assertions::assert_impl_all!(crate::X: Clone);
    static_assertions::assert_impl_all!(crate::X: Copy);
    static_assertions::assert_not_impl_any!(crate::X: Drop);
    assert!(::core::mem::offset_of!(crate::X, a) == 0);
};
