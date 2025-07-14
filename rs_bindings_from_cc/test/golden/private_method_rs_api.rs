// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:private_method_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

// Error while generating bindings for class 'Ptr':
// Class templates are not supported yet

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Outer
pub struct Outer {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Outer {}
impl !Sync for Outer {}
unsafe impl ::cxx::ExternType for Outer {
    type Id = ::cxx::type_id!("Outer");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Outer"), crate::Outer);

impl Default for Outer {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5OuterC1Ev(&raw mut tmp as *mut ::core::ffi::c_void);
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for Outer {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5OuterC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Outer {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for Outer {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN5OuteraSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Outer {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN5OuteraSEOS_(self, __param_0);
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN5OuterC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN5OuterC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::Outer>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN5OuteraSERKS_<'__return_lifetime>(
            __this: &mut crate::Outer,
            __param_0: &crate::Outer,
        ) -> &'__return_lifetime mut crate::Outer;
        pub(crate) unsafe fn __rust_thunk___ZN5OuteraSEOS_<'__return_lifetime>(
            __this: &mut crate::Outer,
            __param_0: ::ctor::RvalueReference<'_, crate::Outer>,
        ) -> &'__return_lifetime mut crate::Outer;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Outer>() == 1);
    assert!(::core::mem::align_of::<crate::Outer>() == 1);
    static_assertions::assert_impl_all!(crate::Outer: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Outer: Drop);
};
