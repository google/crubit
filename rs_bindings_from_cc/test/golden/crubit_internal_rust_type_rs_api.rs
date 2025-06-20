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

// Type bindings for struct MyI8Struct suppressed due to being mapped to an existing Rust type (i8)

// Note that this is potentially visited, even if the original declaration is
// skipped due to crubit_internal_rust_type.

// Type bindings for struct MyI8Class suppressed due to being mapped to an existing Rust type (i8)

// Type bindings for enum MyI8Enum suppressed due to being mapped to an existing Rust type (i8)

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

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TypeMapOverrideFieldTypes
pub struct TypeMapOverrideFieldTypes {
    pub my_i8_struct: i8,
    pub my_i8_class: i8,
    pub my_i8_enum: i8,
    pub my_i8_alias: i8,
    pub error: crate::TooFewArgs,
}
impl !Send for TypeMapOverrideFieldTypes {}
impl !Sync for TypeMapOverrideFieldTypes {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TypeMapOverrideFieldTypes"),
    crate::TypeMapOverrideFieldTypes
);

impl Default for TypeMapOverrideFieldTypes {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25TypeMapOverrideFieldTypesC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for TypeMapOverrideFieldTypes {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25TypeMapOverrideFieldTypesC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for TypeMapOverrideFieldTypes {
    type CtorType = Self;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for TypeMapOverrideFieldTypes {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN25TypeMapOverrideFieldTypesaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for TypeMapOverrideFieldTypes {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN25TypeMapOverrideFieldTypesaSEOS_(self, __param_0);
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN25TypeMapOverrideFieldTypesC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN25TypeMapOverrideFieldTypesC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::TypeMapOverrideFieldTypes>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN25TypeMapOverrideFieldTypesaSERKS_<'a, 'b>(
            __this: &'a mut crate::TypeMapOverrideFieldTypes,
            __param_0: &'b crate::TypeMapOverrideFieldTypes,
        ) -> &'a mut crate::TypeMapOverrideFieldTypes;
        pub(crate) unsafe fn __rust_thunk___ZN25TypeMapOverrideFieldTypesaSEOS_<'a, 'b>(
            __this: &'a mut crate::TypeMapOverrideFieldTypes,
            __param_0: ::ctor::RvalueReference<'b, crate::TypeMapOverrideFieldTypes>,
        ) -> &'a mut crate::TypeMapOverrideFieldTypes;
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
