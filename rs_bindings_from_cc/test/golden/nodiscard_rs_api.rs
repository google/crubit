// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nodiscard_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

extern crate core as __rust_core;
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[must_use]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NoDiscard
pub struct NoDiscard {
    __non_field_data: [::__rust_core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NoDiscard {}
impl !Sync for NoDiscard {}
unsafe impl ::cxx::ExternType for NoDiscard {
    type Id = ::cxx::type_id!("NoDiscard");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for NoDiscard {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::__rust_core::mem::MaybeUninit::<Self>::zeroed();
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
    __non_field_data: [::__rust_core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NoDiscardWithMessage {}
impl !Sync for NoDiscardWithMessage {}
unsafe impl ::cxx::ExternType for NoDiscardWithMessage {
    type Id = ::cxx::type_id!("NoDiscardWithMessage");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for NoDiscardWithMessage {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::__rust_core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20NoDiscardWithMessageC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: enum `NoDiscardEnum` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): nodiscard

// error: enum `NoDiscardEnumWithMessage` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): nodiscard

// error: function `crubit_nodiscard` could not be bound
//   [[nodiscard]] attribute

// error: function `crubit_nodiscard_message` could not be bound
//   [[nodiscard]] attribute

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NodiscardCtor
pub struct NodiscardCtor {
    __non_field_data: [::__rust_core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NodiscardCtor {}
impl !Sync for NodiscardCtor {}
unsafe impl ::cxx::ExternType for NodiscardCtor {
    type Id = ::cxx::type_id!("NodiscardCtor");
    type Kind = ::cxx::kind::Trivial;
}

// error: constructor `NodiscardCtor::NodiscardCtor` could not be bound
//   [[nodiscard]] attribute

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=NodiscardCtorWithMessage
pub struct NodiscardCtorWithMessage {
    __non_field_data: [::__rust_core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NodiscardCtorWithMessage {}
impl !Sync for NodiscardCtorWithMessage {}
unsafe impl ::cxx::ExternType for NodiscardCtorWithMessage {
    type Id = ::cxx::type_id!("NodiscardCtorWithMessage");
    type Kind = ::cxx::kind::Trivial;
}

// error: constructor `NodiscardCtorWithMessage::NodiscardCtorWithMessage` could not be bound
//   [[nodiscard]] attribute

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9NoDiscardC1Ev(
            __this: *mut ::__rust_core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN20NoDiscardWithMessageC1Ev(
            __this: *mut ::__rust_core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::__rust_core::mem::size_of::<crate::NoDiscard>() == 1);
    assert!(::__rust_core::mem::align_of::<crate::NoDiscard>() == 1);
    static_assertions::assert_impl_all!(crate::NoDiscard: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NoDiscard: Drop);

    assert!(::__rust_core::mem::size_of::<crate::NoDiscardWithMessage>() == 1);
    assert!(::__rust_core::mem::align_of::<crate::NoDiscardWithMessage>() == 1);
    static_assertions::assert_impl_all!(crate::NoDiscardWithMessage: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NoDiscardWithMessage: Drop);

    assert!(::__rust_core::mem::size_of::<crate::NodiscardCtor>() == 1);
    assert!(::__rust_core::mem::align_of::<crate::NodiscardCtor>() == 1);
    static_assertions::assert_impl_all!(crate::NodiscardCtor: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NodiscardCtor: Drop);

    assert!(::__rust_core::mem::size_of::<crate::NodiscardCtorWithMessage>() == 1);
    assert!(::__rust_core::mem::align_of::<crate::NodiscardCtorWithMessage>() == 1);
    static_assertions::assert_impl_all!(crate::NodiscardCtorWithMessage: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NodiscardCtorWithMessage: Drop);
};
