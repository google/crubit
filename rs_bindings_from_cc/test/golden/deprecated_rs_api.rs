// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:deprecated_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

#[deprecated]
#[inline(always)]
pub fn deprecated_function() {
    unsafe { crate::detail::__rust_thunk___Z19deprecated_functionv() }
}

#[deprecated = "old"]
#[inline(always)]
pub fn deprecated_function_with_message() {
    unsafe { crate::detail::__rust_thunk___Z32deprecated_function_with_messagev() }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[deprecated]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: DeprecatedStruct
pub struct DeprecatedStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for DeprecatedStruct {}
impl !Sync for DeprecatedStruct {}
unsafe impl ::cxx::ExternType for DeprecatedStruct {
    type Id = ::cxx::type_id!(":: DeprecatedStruct");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for DeprecatedStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16DeprecatedStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[deprecated = "old"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: DeprecatedStructWithMessage
pub struct DeprecatedStructWithMessage {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for DeprecatedStructWithMessage {}
impl !Sync for DeprecatedStructWithMessage {}
unsafe impl ::cxx::ExternType for DeprecatedStructWithMessage {
    type Id = ::cxx::type_id!(":: DeprecatedStructWithMessage");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for DeprecatedStructWithMessage {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN27DeprecatedStructWithMessageC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[deprecated]
///CRUBIT_ANNOTATE: cpp_type=:: DeprecatedEnum
pub struct DeprecatedEnum(::ffi_11::c_uint);
impl DeprecatedEnum {}
impl From<::ffi_11::c_uint> for DeprecatedEnum {
    fn from(value: ::ffi_11::c_uint) -> DeprecatedEnum {
        DeprecatedEnum(value)
    }
}
impl From<DeprecatedEnum> for ::ffi_11::c_uint {
    fn from(value: DeprecatedEnum) -> ::ffi_11::c_uint {
        value.0
    }
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
#[deprecated = "old"]
///CRUBIT_ANNOTATE: cpp_type=:: DeprecatedEnumWithMessage
pub struct DeprecatedEnumWithMessage(::ffi_11::c_uint);
impl DeprecatedEnumWithMessage {}
impl From<::ffi_11::c_uint> for DeprecatedEnumWithMessage {
    fn from(value: ::ffi_11::c_uint) -> DeprecatedEnumWithMessage {
        DeprecatedEnumWithMessage(value)
    }
}
impl From<DeprecatedEnumWithMessage> for ::ffi_11::c_uint {
    fn from(value: DeprecatedEnumWithMessage) -> ::ffi_11::c_uint {
        value.0
    }
}

#[deprecated]
pub mod DeprecatedNamespace {
    #[inline(always)]
    pub fn f() {
        unsafe { crate::detail::__rust_thunk___ZN19DeprecatedNamespace1fEv() }
    }
}

// namespace DeprecatedNamespace

#[deprecated = "old"]
pub mod DeprecatedNamespaceWithMessage {
    #[inline(always)]
    pub fn f() {
        unsafe { crate::detail::__rust_thunk___ZN30DeprecatedNamespaceWithMessage1fEv() }
    }
}

// namespace DeprecatedNamespaceWithMessage

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=:: DeprecatedEnumerators
pub struct DeprecatedEnumerators(::ffi_11::c_uint);
impl DeprecatedEnumerators {
    #[deprecated]
    pub const kDeprecatedEnumerator: DeprecatedEnumerators =
        DeprecatedEnumerators(::ffi_11::new_c_uint(0));
    #[deprecated = "old"]
    pub const kDeprecatedEnumeratorWithMessage: DeprecatedEnumerators =
        DeprecatedEnumerators(::ffi_11::new_c_uint(1));
}
impl From<::ffi_11::c_uint> for DeprecatedEnumerators {
    fn from(value: ::ffi_11::c_uint) -> DeprecatedEnumerators {
        DeprecatedEnumerators(value)
    }
}
impl From<DeprecatedEnumerators> for ::ffi_11::c_uint {
    fn from(value: DeprecatedEnumerators) -> ::ffi_11::c_uint {
        value.0
    }
}

#[deprecated]
pub type DeprecatedUsing = ::ffi_11::c_int;

#[deprecated = "old"]
pub type DeprecatedUsingWithMessage = ::ffi_11::c_int;

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=:: DeprecatedFields
pub struct DeprecatedFields {
    #[deprecated]
    pub no_message: ::ffi_11::c_int,
    #[deprecated = "old"]
    pub message: ::ffi_11::c_int,
}
impl !Send for DeprecatedFields {}
impl !Sync for DeprecatedFields {}
unsafe impl ::cxx::ExternType for DeprecatedFields {
    type Id = ::cxx::type_id!(":: DeprecatedFields");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for DeprecatedFields {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16DeprecatedFieldsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

unsafe extern "C" {
    #[deprecated]
    pub static mut global_var: ::ffi_11::c_int;
}

unsafe extern "C" {
    #[deprecated = "old"]
    pub static mut global_var_with_message: ::ffi_11::c_int;
}

// error: class `SomeTotalSpecialization` could not be bound
//   Class templates are not yet supported

// error: class `SomeTemplate` could not be bound
//   Class templates are not yet supported

// error: class `SomeTemplateWithMessage` could not be bound
//   Class templates are not yet supported

// error: class `SomePartialSpecialization` could not be bound
//   Class templates are not yet supported

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z19deprecated_functionv();
        pub(crate) unsafe fn __rust_thunk___Z32deprecated_function_with_messagev();
        pub(crate) unsafe fn __rust_thunk___ZN16DeprecatedStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN27DeprecatedStructWithMessageC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN19DeprecatedNamespace1fEv();
        pub(crate) unsafe fn __rust_thunk___ZN30DeprecatedNamespaceWithMessage1fEv();
        pub(crate) unsafe fn __rust_thunk___ZN16DeprecatedFieldsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::DeprecatedStruct>() == 1);
    assert!(::core::mem::align_of::<crate::DeprecatedStruct>() == 1);
    static_assertions::assert_impl_all!(crate::DeprecatedStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DeprecatedStruct: Drop);

    assert!(::core::mem::size_of::<crate::DeprecatedStructWithMessage>() == 1);
    assert!(::core::mem::align_of::<crate::DeprecatedStructWithMessage>() == 1);
    static_assertions::assert_impl_all!(crate::DeprecatedStructWithMessage: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DeprecatedStructWithMessage: Drop);

    assert!(::core::mem::size_of::<crate::DeprecatedFields>() == 8);
    assert!(::core::mem::align_of::<crate::DeprecatedFields>() == 4);
    static_assertions::assert_impl_all!(crate::DeprecatedFields: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DeprecatedFields: Drop);
    assert!(::core::mem::offset_of!(crate::DeprecatedFields, no_message) == 0);
    assert!(::core::mem::offset_of!(crate::DeprecatedFields, message) == 4);
};
