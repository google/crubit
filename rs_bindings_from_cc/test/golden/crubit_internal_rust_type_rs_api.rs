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
pub type TooFewArgs = ::ffi_11::c_uchar;

pub type TooManyArgs = ::ffi_11::c_uchar;

pub type NonStringArg = ::ffi_11::c_uchar;

pub type BadSameAbiAttr = ::ffi_11::c_uchar;

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ExistingRustTypeFieldTypes
pub struct ExistingRustTypeFieldTypes {
    pub my_i8_struct: i8,
    pub my_i8_class: i8,
    pub my_i8_enum: i8,
    pub my_i8_alias: i8,
    pub error: crate::TooFewArgs,
}
impl !Send for ExistingRustTypeFieldTypes {}
impl !Sync for ExistingRustTypeFieldTypes {}
unsafe impl ::cxx::ExternType for ExistingRustTypeFieldTypes {
    type Id = ::cxx::type_id!("ExistingRustTypeFieldTypes");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for ExistingRustTypeFieldTypes {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN26ExistingRustTypeFieldTypesC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'ExistingRustTypeFieldTypes::ExistingRustTypeFieldTypes':
// Can't generate bindings for ExistingRustTypeFieldTypes::ExistingRustTypeFieldTypes, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for ExistingRustTypeFieldTypes::ExistingRustTypeFieldTypes (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'ExistingRustTypeFieldTypes::ExistingRustTypeFieldTypes':
// Can't generate bindings for ExistingRustTypeFieldTypes::ExistingRustTypeFieldTypes, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for ExistingRustTypeFieldTypes::ExistingRustTypeFieldTypes (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'ExistingRustTypeFieldTypes::operator=':
// Can't generate bindings for ExistingRustTypeFieldTypes::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for ExistingRustTypeFieldTypes::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for ExistingRustTypeFieldTypes::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'ExistingRustTypeFieldTypes::operator=':
// Can't generate bindings for ExistingRustTypeFieldTypes::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for ExistingRustTypeFieldTypes::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc needs [//features:experimental] for ExistingRustTypeFieldTypes::operator= (the type of __param_0 (parameter #1): references are not supported)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN26ExistingRustTypeFieldTypesC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
    assert!(::core::mem::size_of::<crate::ExistingRustTypeFieldTypes>() == 5);
    assert!(::core::mem::align_of::<crate::ExistingRustTypeFieldTypes>() == 1);
    static_assertions::assert_impl_all!(crate::ExistingRustTypeFieldTypes: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ExistingRustTypeFieldTypes: Drop);
    assert!(::core::mem::offset_of!(crate::ExistingRustTypeFieldTypes, my_i8_struct) == 0);
    assert!(::core::mem::offset_of!(crate::ExistingRustTypeFieldTypes, my_i8_class) == 1);
    assert!(::core::mem::offset_of!(crate::ExistingRustTypeFieldTypes, my_i8_enum) == 2);
    assert!(::core::mem::offset_of!(crate::ExistingRustTypeFieldTypes, my_i8_alias) == 3);
    assert!(::core::mem::offset_of!(crate::ExistingRustTypeFieldTypes, error) == 4);
};
