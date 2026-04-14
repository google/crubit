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

// error: function `deprecated_function` could not be bound
//   [[deprecated]] attribute

// error: function `deprecated_function_with_message` could not be bound
//   [[deprecated]] attribute

// error: struct `DeprecatedStruct` could not be bound
//   [[deprecated]] attribute

// error: struct `DeprecatedStructWithMessage` could not be bound
//   [[deprecated]] attribute

// error: enum `DeprecatedEnum` could not be bound
//   [[deprecated]] attribute

// error: enum `DeprecatedEnumWithMessage` could not be bound
//   [[deprecated]] attribute

// error: namespace `DeprecatedNamespace` could not be bound
//   [[deprecated]] attribute

// namespace DeprecatedNamespace

// error: namespace `DeprecatedNamespaceWithMessage` could not be bound
//   [[deprecated]] attribute

// namespace DeprecatedNamespaceWithMessage

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=DeprecatedEnumerators
pub struct DeprecatedEnumerators(::ffi_11::c_uint);
impl DeprecatedEnumerators {
    // Omitting bindings for kDeprecatedEnumerator
    // reason: marked as deprecated; requires experimental
    // Omitting bindings for kDeprecatedEnumeratorWithMessage
    // reason: marked as deprecated; requires experimental
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

// error: type alias `DeprecatedUsing` could not be bound
//   [[deprecated]] attribute

// error: type alias `DeprecatedUsingWithMessage` could not be bound
//   [[deprecated]] attribute

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=DeprecatedFields
pub struct DeprecatedFields {
    /// Reason for representing this field as a blob of bytes:
    /// field is marked as deprecated; requires experimental features on //rs_bindings_from_cc/test/golden:deprecated_cc
    pub(crate) no_message: [::core::mem::MaybeUninit<u8>; 4],
    /// Reason for representing this field as a blob of bytes:
    /// field is marked as deprecated; requires experimental features on //rs_bindings_from_cc/test/golden:deprecated_cc
    pub(crate) message: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for DeprecatedFields {}
impl !Sync for DeprecatedFields {}
unsafe impl ::cxx::ExternType for DeprecatedFields {
    type Id = ::cxx::type_id!("DeprecatedFields");
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

// error: global variable `global_var` could not be bound
//   [[deprecated]] attribute

// error: global variable `global_var_with_message` could not be bound
//   [[deprecated]] attribute

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
        pub(crate) unsafe fn __rust_thunk___ZN16DeprecatedFieldsC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::DeprecatedFields>() == 8);
    assert!(::core::mem::align_of::<crate::DeprecatedFields>() == 4);
    static_assertions::assert_impl_all!(crate::DeprecatedFields: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DeprecatedFields: Drop);
    assert!(::core::mem::offset_of!(crate::DeprecatedFields, no_message) == 0);
    assert!(::core::mem::offset_of!(crate::DeprecatedFields, message) == 4);
};
