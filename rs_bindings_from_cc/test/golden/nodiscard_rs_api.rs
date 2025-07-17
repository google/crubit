// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nodiscard_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[must_use]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NoDiscard
pub struct NoDiscard {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NoDiscard {}
impl !Sync for NoDiscard {}
unsafe impl ::cxx::ExternType for NoDiscard {
    type Id = ::cxx::type_id!("NoDiscard");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("NoDiscard"), crate::NoDiscard);

impl Default for NoDiscard {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9NoDiscardC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[must_use = "You really should use this"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NoDiscardWithMessage
pub struct NoDiscardWithMessage {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NoDiscardWithMessage {}
impl !Sync for NoDiscardWithMessage {}
unsafe impl ::cxx::ExternType for NoDiscardWithMessage {
    type Id = ::cxx::type_id!("NoDiscardWithMessage");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NoDiscardWithMessage"),
    crate::NoDiscardWithMessage
);

impl Default for NoDiscardWithMessage {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20NoDiscardWithMessageC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9NoDiscardC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN20NoDiscardWithMessageC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::NoDiscard>() == 1);
    assert!(::core::mem::align_of::<crate::NoDiscard>() == 1);
    static_assertions::assert_impl_all!(crate::NoDiscard: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NoDiscard: Drop);

    assert!(::core::mem::size_of::<crate::NoDiscardWithMessage>() == 1);
    assert!(::core::mem::align_of::<crate::NoDiscardWithMessage>() == 1);
    static_assertions::assert_impl_all!(crate::NoDiscardWithMessage: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NoDiscardWithMessage: Drop);
};
