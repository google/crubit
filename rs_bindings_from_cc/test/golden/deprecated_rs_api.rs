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
#![deny(warnings)]

extern crate core as __rust_core;
// error: function `deprecated_function` could not be bound
//   [[deprecated]] attribute

// error: function `deprecated_function_with_message` could not be bound
//   [[deprecated]] attribute

// error: struct `DeprecatedStruct` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): deprecated

// error: struct `DeprecatedStructWithMessage` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): deprecated

// error: enum `DeprecatedEnum` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): deprecated

// error: enum `DeprecatedEnumWithMessage` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): deprecated

// error: namespace `DeprecatedNamespace` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): deprecated

// namespace DeprecatedNamespace

// error: namespace `DeprecatedNamespaceWithMessage` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): deprecated

// namespace DeprecatedNamespaceWithMessage

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
///CRUBIT_ANNOTATE: cpp_type=DeprecatedEnumerators
pub struct DeprecatedEnumerators(::ffi_11::c_uint);
impl DeprecatedEnumerators {
    // Omitting bindings for kDeprecatedEnumerator
    // reason: unknown attribute(s): deprecated
    // Omitting bindings for kDeprecatedEnumeratorWithMessage
    // reason: unknown attribute(s): deprecated
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
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): deprecated

// error: type alias `DeprecatedUsingWithMessage` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): deprecated

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=DeprecatedFields
pub struct DeprecatedFields {
    /// Reason for representing this field as a blob of bytes:
    /// crubit.rs/errors/unknown_attribute: unknown field attributes are only supported with experimental features enabled on //rs_bindings_from_cc/test/golden:deprecated_cc
    /// Unknown attribute: deprecated`
    pub(crate) no_message: [::__rust_core::mem::MaybeUninit<u8>; 4],
    /// Reason for representing this field as a blob of bytes:
    /// crubit.rs/errors/unknown_attribute: unknown field attributes are only supported with experimental features enabled on //rs_bindings_from_cc/test/golden:deprecated_cc
    /// Unknown attribute: deprecated`
    pub(crate) message: [::__rust_core::mem::MaybeUninit<u8>; 4],
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
        let mut tmp = ::__rust_core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN16DeprecatedFieldsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// error: global variable `global_var` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): deprecated

// error: global variable `global_var_with_message` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): deprecated

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
            __this: *mut ::__rust_core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::__rust_core::mem::size_of::<crate::DeprecatedFields>() == 8);
    assert!(::__rust_core::mem::align_of::<crate::DeprecatedFields>() == 4);
    static_assertions::assert_impl_all!(crate::DeprecatedFields: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DeprecatedFields: Drop);
    assert!(::__rust_core::mem::offset_of!(crate::DeprecatedFields, no_message) == 0);
    assert!(::__rust_core::mem::offset_of!(crate::DeprecatedFields, message) == 4);
};
