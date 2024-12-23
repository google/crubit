// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/trivial_struct:example_lib
// Features: supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

/// Generated from: examples/cpp/trivial_struct/example.h;l=8
#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "Position")]
pub struct Position {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
impl !Send for Position {}
impl !Sync for Position {}

// Generated from: examples/cpp/trivial_struct/example.h;l=8
// Error while generating bindings for item 'Position::Position':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: examples/cpp/trivial_struct/example.h;l=8
// Error while generating bindings for item 'Position::Position':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: examples/cpp/trivial_struct/example.h;l=8
// Error while generating bindings for item 'Position::Position':
// Parameter #0 is not supported: Unsupported type 'Position &&': Unsupported type: && without lifetime

// Generated from: examples/cpp/trivial_struct/example.h;l=8
// Error while generating bindings for item 'Position::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: examples/cpp/trivial_struct/example.h;l=8
// Error while generating bindings for item 'Position::operator=':
// Parameter #0 is not supported: Unsupported type 'Position &&': Unsupported type: && without lifetime

const _: () = {
    assert!(::core::mem::size_of::<crate::Position>() == 8);
    assert!(::core::mem::align_of::<crate::Position>() == 4);
    static_assertions::assert_impl_all!(crate::Position: Clone);
    static_assertions::assert_impl_all!(crate::Position: Copy);
    static_assertions::assert_not_impl_any!(crate::Position: Drop);
    assert!(::core::mem::offset_of!(crate::Position, x) == 0);
    assert!(::core::mem::offset_of!(crate::Position, y) == 4);
};
