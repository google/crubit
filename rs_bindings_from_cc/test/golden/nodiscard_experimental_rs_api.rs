// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nodiscard_experimental_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(rust_2024_compatibility)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[must_use]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: NoDiscard
pub struct NoDiscard {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NoDiscard {}
impl !Sync for NoDiscard {}
unsafe impl ::cxx::ExternType for NoDiscard {
    type Id = ::cxx::type_id!(":: NoDiscard");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: NoDiscard"), crate::NoDiscard);

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
///CRUBIT_ANNOTATE: cpp_type=:: NoDiscardWithMessage
pub struct NoDiscardWithMessage {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NoDiscardWithMessage {}
impl !Sync for NoDiscardWithMessage {}
unsafe impl ::cxx::ExternType for NoDiscardWithMessage {
    type Id = ::cxx::type_id!(":: NoDiscardWithMessage");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: NoDiscardWithMessage"),
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

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[must_use]
///CRUBIT_ANNOTATE: cpp_type=:: NoDiscardEnum
pub struct NoDiscardEnum(::ffi_11::c_uint);
impl NoDiscardEnum {
    pub const kConstant: NoDiscardEnum = NoDiscardEnum(::ffi_11::new_c_uint(0));
}
impl From<::ffi_11::c_uint> for NoDiscardEnum {
    fn from(value: ::ffi_11::c_uint) -> NoDiscardEnum {
        NoDiscardEnum(value)
    }
}
impl From<NoDiscardEnum> for ::ffi_11::c_uint {
    fn from(value: NoDiscardEnum) -> ::ffi_11::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[must_use = "You really should use this"]
///CRUBIT_ANNOTATE: cpp_type=:: NoDiscardEnumWithMessage
pub struct NoDiscardEnumWithMessage(::ffi_11::c_uint);
impl NoDiscardEnumWithMessage {
    pub const kConstantWithMessage: NoDiscardEnumWithMessage =
        NoDiscardEnumWithMessage(::ffi_11::new_c_uint(0));
}
impl From<::ffi_11::c_uint> for NoDiscardEnumWithMessage {
    fn from(value: ::ffi_11::c_uint) -> NoDiscardEnumWithMessage {
        NoDiscardEnumWithMessage(value)
    }
}
impl From<NoDiscardEnumWithMessage> for ::ffi_11::c_uint {
    fn from(value: NoDiscardEnumWithMessage) -> ::ffi_11::c_uint {
        value.0
    }
}

#[must_use]
#[inline(always)]
pub fn crubit_nodiscard() -> *mut ::ffi_11::c_void {
    unsafe { crate::detail::__rust_thunk___Z16crubit_nodiscardv() }
}

#[must_use = "You really should use this"]
#[inline(always)]
pub fn crubit_nodiscard_message() -> *mut ::ffi_11::c_void {
    unsafe { crate::detail::__rust_thunk___Z24crubit_nodiscard_messagev() }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: NodiscardCtor
pub struct NodiscardCtor {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NodiscardCtor {}
impl !Sync for NodiscardCtor {}
unsafe impl ::cxx::ExternType for NodiscardCtor {
    type Id = ::cxx::type_id!(":: NodiscardCtor");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!(":: NodiscardCtor"), crate::NodiscardCtor);

impl From<(::ffi_11::c_int, ::ffi_11::c_int)> for NodiscardCtor {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, ::ffi_11::c_int)) -> Self {
        let (mut x, mut y) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13NodiscardCtorC1Eii(&raw mut tmp as *mut _, x, y);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, ::ffi_11::c_int)> for NodiscardCtor {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, ::ffi_11::c_int)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, ::ffi_11::c_int)>>::from(args)
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: NodiscardCtorWithMessage
pub struct NodiscardCtorWithMessage {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for NodiscardCtorWithMessage {}
impl !Sync for NodiscardCtorWithMessage {}
unsafe impl ::cxx::ExternType for NodiscardCtorWithMessage {
    type Id = ::cxx::type_id!(":: NodiscardCtorWithMessage");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!(":: NodiscardCtorWithMessage"),
    crate::NodiscardCtorWithMessage
);

impl From<(::ffi_11::c_int, ::ffi_11::c_int)> for NodiscardCtorWithMessage {
    #[inline(always)]
    fn from(args: (::ffi_11::c_int, ::ffi_11::c_int)) -> Self {
        let (mut x, mut y) = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN24NodiscardCtorWithMessageC1Eii(
                &raw mut tmp as *mut _,
                x,
                y,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int, ::ffi_11::c_int)> for NodiscardCtorWithMessage {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int, ::ffi_11::c_int)) -> Self::CtorType {
        <Self as From<(::ffi_11::c_int, ::ffi_11::c_int)>>::from(args)
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
        pub(crate) unsafe fn __rust_thunk___Z16crubit_nodiscardv() -> *mut ::ffi_11::c_void;
        pub(crate) unsafe fn __rust_thunk___Z24crubit_nodiscard_messagev() -> *mut ::ffi_11::c_void;
        pub(crate) unsafe fn __rust_thunk___ZN13NodiscardCtorC1Eii(
            __this: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
            y: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN24NodiscardCtorWithMessageC1Eii(
            __this: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
            y: ::ffi_11::c_int,
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

    assert!(::core::mem::size_of::<crate::NodiscardCtor>() == 1);
    assert!(::core::mem::align_of::<crate::NodiscardCtor>() == 1);
    static_assertions::assert_impl_all!(crate::NodiscardCtor: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NodiscardCtor: Drop);

    assert!(::core::mem::size_of::<crate::NodiscardCtorWithMessage>() == 1);
    assert!(::core::mem::align_of::<crate::NodiscardCtorWithMessage>() == 1);
    static_assertions::assert_impl_all!(crate::NodiscardCtorWithMessage: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::NodiscardCtorWithMessage: Drop);
};
