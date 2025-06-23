// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:overloads_unsupported_type_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Tests that no bindings are generated when an overload set includes
/// any unsupported items.
///
/// See http://b/251045039
#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=SomeClass
pub struct SomeClass {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for SomeClass {}
impl !Sync for SomeClass {}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeClass"), crate::SomeClass);

impl Default for SomeClass {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for SomeClass {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for SomeClass {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for SomeClass {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for SomeClass {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassaSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for item 'Overload':
// Cannot generate bindings for overloaded function

// Error while generating bindings for item 'Overload':
// Cannot generate bindings for overloaded function

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClassC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClassC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::SomeClass>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClassaSERKS_<'__return_lifetime>(
            __this: &mut crate::SomeClass,
            __param_0: &crate::SomeClass,
        ) -> &'__return_lifetime mut crate::SomeClass;
        pub(crate) unsafe fn __rust_thunk___ZN9SomeClassaSEOS_<'__return_lifetime>(
            __this: &mut crate::SomeClass,
            __param_0: ::ctor::RvalueReference<'_, crate::SomeClass>,
        ) -> &'__return_lifetime mut crate::SomeClass;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SomeClass>() == 1);
    assert!(::core::mem::align_of::<crate::SomeClass>() == 1);
    static_assertions::assert_impl_all!(crate::SomeClass: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SomeClass: Drop);
};
