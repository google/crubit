// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Type bindings for MyI8Struct suppressed due to being mapped to an existing Rust type (i8)

// Note that this is potentially visited, even if the original declaration is
// skipped due to crubit_internal_rust_type.

// Type bindings for MyI8Class suppressed due to being mapped to an existing Rust type (i8)

// Type bindings for MyI8Enum suppressed due to being mapped to an existing Rust type (i8)

// Type bindings for MyI8Alias suppressed due to being mapped to an existing Rust type (i8)

/// Invalid annotations cause bindings to fail to be generated.
/// (It's important not to fall back to the underlying type, since the user
/// intent was to override it.)
/// Uncomment these invalid annotations to observe the build-time errors.
/// TODO: b/402989591 - Use compile-fail UI test to check these outputs.
pub type TooFewArgs = ::core::ffi::c_uchar;

pub type TooManyArgs = ::core::ffi::c_uchar;

pub type NonStringArg = ::core::ffi::c_uchar;

pub type BadSameAbiAttr = ::core::ffi::c_uchar;

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TypeMapOverrideFieldTypes
pub struct TypeMapOverrideFieldTypes {
    /// Reason for representing this field as a blob of bytes:
    /// missing features: [//features:experimental]:
    pub(crate) my_i8_struct: [::core::mem::MaybeUninit<u8>; 1],
    /// Reason for representing this field as a blob of bytes:
    /// missing features: [//features:experimental]:
    pub(crate) my_i8_class: [::core::mem::MaybeUninit<u8>; 1],
    /// Reason for representing this field as a blob of bytes:
    /// missing features: [//features:experimental]:
    pub(crate) my_i8_enum: [::core::mem::MaybeUninit<u8>; 1],
    /// Reason for representing this field as a blob of bytes:
    /// missing features: [//features:experimental]:
    pub(crate) my_i8_alias: [::core::mem::MaybeUninit<u8>; 1],
    pub error: crate::TooFewArgs,
}
impl !Send for TypeMapOverrideFieldTypes {}
impl !Sync for TypeMapOverrideFieldTypes {}
unsafe impl ::cxx::ExternType for TypeMapOverrideFieldTypes {
    type Id = ::cxx::type_id!("TypeMapOverrideFieldTypes");
    type Kind = ::cxx::kind::Trivial;
}

// Error while generating bindings for constructor 'TypeMapOverrideFieldTypes::TypeMapOverrideFieldTypes':
// Can't generate bindings for TypeMapOverrideFieldTypes::TypeMapOverrideFieldTypes, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for TypeMapOverrideFieldTypes::TypeMapOverrideFieldTypes (the type of __this (parameter #0): references are not supported)

// Error while generating bindings for constructor 'TypeMapOverrideFieldTypes::TypeMapOverrideFieldTypes':
// Can't generate bindings for TypeMapOverrideFieldTypes::TypeMapOverrideFieldTypes, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for TypeMapOverrideFieldTypes::TypeMapOverrideFieldTypes (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for TypeMapOverrideFieldTypes::TypeMapOverrideFieldTypes (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'TypeMapOverrideFieldTypes::TypeMapOverrideFieldTypes':
// Can't generate bindings for TypeMapOverrideFieldTypes::TypeMapOverrideFieldTypes, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for TypeMapOverrideFieldTypes::TypeMapOverrideFieldTypes (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for TypeMapOverrideFieldTypes::TypeMapOverrideFieldTypes (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'TypeMapOverrideFieldTypes::operator=':
// Can't generate bindings for TypeMapOverrideFieldTypes::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for TypeMapOverrideFieldTypes::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for TypeMapOverrideFieldTypes::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for TypeMapOverrideFieldTypes::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'TypeMapOverrideFieldTypes::operator=':
// Can't generate bindings for TypeMapOverrideFieldTypes::operator=, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for TypeMapOverrideFieldTypes::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for TypeMapOverrideFieldTypes::operator= (the type of __this (parameter #0): references are not supported)
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for TypeMapOverrideFieldTypes::operator= (the type of __param_0 (parameter #1): references are not supported)

const _: () = {
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<crate::TypeMapOverrideFieldTypes>() == 5);
    assert!(::core::mem::align_of::<crate::TypeMapOverrideFieldTypes>() == 1);
    static_assertions::assert_impl_all!(crate::TypeMapOverrideFieldTypes: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::TypeMapOverrideFieldTypes: Drop);
    assert!(::core::mem::offset_of!(crate::TypeMapOverrideFieldTypes, my_i8_struct) == 0);
    assert!(::core::mem::offset_of!(crate::TypeMapOverrideFieldTypes, my_i8_class) == 1);
    assert!(::core::mem::offset_of!(crate::TypeMapOverrideFieldTypes, my_i8_enum) == 2);
    assert!(::core::mem::offset_of!(crate::TypeMapOverrideFieldTypes, my_i8_alias) == 3);
    assert!(::core::mem::offset_of!(crate::TypeMapOverrideFieldTypes, error) == 4);
};
