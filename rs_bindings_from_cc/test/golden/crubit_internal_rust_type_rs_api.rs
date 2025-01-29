// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

// Type bindings for struct MyI8Struct suppressed due to being mapped to an existing Rust type (i8)

// Note that this is potentially visited, even if the original declaration is
// skipped due to crubit_internal_rust_type.

// Type bindings for struct MyI8Class suppressed due to being mapped to an existing Rust type (i8)

// Type bindings for enum MyI8Enum suppressed due to being mapped to an existing Rust type (i8)

// Type bindings for MyI8Alias suppressed due to being mapped to an existing Rust type (i8)

// Error while generating bindings for item 'TooFewArgs':
// Invalid crubit_internal_rust_type attribute: The `crubit_internal_rust_type` attribute requires a single string literal argument, the Rust type.

// Error while generating bindings for item 'TooManyArgs':
// Invalid crubit_internal_rust_type attribute: The `crubit_internal_rust_type` attribute requires a single string literal argument, the Rust type.

// Error while generating bindings for item 'NonStringArg':
// Invalid crubit_internal_rust_type attribute: cannot evaluate argument as a string literal

// Error while generating bindings for item 'BadSameAbiAttr':
// Invalid crubit_internal_is_same_abi attribute: The `crubit_internal_same_abi` attribute takes no arguments.

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cpp_type = "TypeMapOverrideFieldTypes")]
pub struct TypeMapOverrideFieldTypes {
    pub my_i8_struct: i8,
    pub my_i8_class: i8,
    pub my_i8_enum: i8,
    pub my_i8_alias: i8,
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'TooFewArgs': No generated bindings found for 'TooFewArgs'
    pub(crate) error: [::core::mem::MaybeUninit<u8>; 1],
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
    static_assertions::assert_impl_all!(crate::TypeMapOverrideFieldTypes: Clone);
    static_assertions::assert_impl_all!(crate::TypeMapOverrideFieldTypes: Copy);
    static_assertions::assert_not_impl_any!(crate::TypeMapOverrideFieldTypes: Drop);
    assert!(::core::mem::offset_of!(crate::TypeMapOverrideFieldTypes, my_i8_struct) == 0);
    assert!(::core::mem::offset_of!(crate::TypeMapOverrideFieldTypes, my_i8_class) == 1);
    assert!(::core::mem::offset_of!(crate::TypeMapOverrideFieldTypes, my_i8_enum) == 2);
    assert!(::core::mem::offset_of!(crate::TypeMapOverrideFieldTypes, my_i8_alias) == 3);
    assert!(::core::mem::offset_of!(crate::TypeMapOverrideFieldTypes, error) == 4);
};
