// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //examples/cpp/trivial_struct:example_lib
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: examples/cpp/trivial_struct/example.h;l=8
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Position
pub struct Position {
    pub x: ::ffi_11::c_int,
    pub y: ::ffi_11::c_int,
}
impl !Send for Position {}
impl !Sync for Position {}
unsafe impl ::cxx::ExternType for Position {
    type Id = ::cxx::type_id!("Position");
    type Kind = ::cxx::kind::Trivial;
}

/// Generated from: examples/cpp/trivial_struct/example.h;l=8
impl Default for Position {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN8PositionC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN8PositionC1Ev(__this: *mut ::core::ffi::c_void);
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Position>() == 8);
    assert!(::core::mem::align_of::<crate::Position>() == 4);
    static_assertions::assert_impl_all!(crate::Position: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Position: Drop);
    assert!(::core::mem::offset_of!(crate::Position, x) == 0);
    assert!(::core::mem::offset_of!(crate::Position, y) == 4);
};
