// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/trivial_abi_struct:example_lib
// Features: extern_c

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(warnings)]

/// Because this class has a destructor, it will not receive Rust bindings
/// without ABSL_ATTRIBUTE_TRIVIAL_ABI.
///
/// Generated from: examples/cpp/trivial_abi_struct/example.h;l=12
#[derive(Clone)]
#[repr(C)]
#[__crubit::annotate(cc_type = "Position")]
pub struct Position {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
impl !Send for Position {}
impl !Sync for Position {}
forward_declare::unsafe_define!(forward_declare::symbol!("Position"), crate::Position);

// Generated from: examples/cpp/trivial_abi_struct/example.h;l=12
// Error while generating bindings for item 'Position::Position':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: examples/cpp/trivial_abi_struct/example.h;l=12
// Error while generating bindings for item 'Position::Position':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: examples/cpp/trivial_abi_struct/example.h;l=12
// Error while generating bindings for item 'Position::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

/// Generated from: examples/cpp/trivial_abi_struct/example.h;l=16
impl Drop for Position {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN8PositionD1Ev(self) }
    }
}

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, false>':
// Can't generate bindings for std::integral_constant<bool, false>, because of missing required features (<internal link>):
// //examples/cpp/trivial_abi_struct:example_lib needs [//features:experimental] for std::integral_constant<bool, false>

// Generated from: google3/nowhere/llvm/toolchain/include/c++/v1/__type_traits/integral_constant.h;l=21
// Error while generating bindings for item 'std::integral_constant<bool, true>':
// Can't generate bindings for std::integral_constant<bool, true>, because of missing required features (<internal link>):
// //examples/cpp/trivial_abi_struct:example_lib needs [//features:experimental] for std::integral_constant<bool, true>

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN8PositionD1Ev<'a>(__this: &'a mut crate::Position);
    }
}

const _: () = assert!(::core::mem::size_of::<crate::Position>() == 8);
const _: () = assert!(::core::mem::align_of::<crate::Position>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::Position:Clone);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Position:Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Position:Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::Position, x) == 0);
const _: () = assert!(memoffset::offset_of!(crate::Position, y) == 4);
const _: () = {
    static_assertions::assert_impl_all!(::core::ffi::c_int:Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(::core::ffi::c_int:Copy);
};