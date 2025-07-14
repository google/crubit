// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/trivial_struct:example_lib
// Features: supported, unsafe_types

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: examples/cpp/trivial_struct/example.h;l=8
#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Position
pub struct Position {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
}
impl !Send for Position {}
impl !Sync for Position {}
unsafe impl ::cxx::ExternType for Position {
    type Id = ::cxx::type_id!("Position");
    type Kind = ::cxx::kind::Trivial;
}

// Generated from: examples/cpp/trivial_struct/example.h;l=8
// Error while generating bindings for function 'Position::Position':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Position
// Expected first parameter to be a `__this` reference, found *mut crate::Position

// Generated from: examples/cpp/trivial_struct/example.h;l=8
// Error while generating bindings for function 'Position::Position':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Position
// Expected first parameter to be a `__this` reference, found *mut crate::Position

// Generated from: examples/cpp/trivial_struct/example.h;l=8
// Error while generating bindings for function 'Position::Position':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Position
// Expected first parameter to be a `__this` reference, found *mut crate::Position

// Generated from: examples/cpp/trivial_struct/example.h;l=8
// Error while generating bindings for function 'Position::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: examples/cpp/trivial_struct/example.h;l=8
// Error while generating bindings for function 'Position::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

const _: () = {
    assert!(::core::mem::size_of::<crate::Position>() == 8);
    assert!(::core::mem::align_of::<crate::Position>() == 4);
    static_assertions::assert_impl_all!(crate::Position: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Position: Drop);
    assert!(::core::mem::offset_of!(crate::Position, x) == 0);
    assert!(::core::mem::offset_of!(crate::Position, y) == 4);
};
