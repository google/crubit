// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:definition_of_forward_declaration_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=ForwardDeclaredStruct
pub struct ForwardDeclaredStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for ForwardDeclaredStruct {}
impl !Sync for ForwardDeclaredStruct {}
unsafe impl ::cxx::ExternType for ForwardDeclaredStruct {
    type Id = ::cxx::type_id!("ForwardDeclaredStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("ForwardDeclaredStruct"),
    crate::ForwardDeclaredStruct
);

impl Default for ForwardDeclaredStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21ForwardDeclaredStructC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl From<::ctor::RvalueReference<'_, Self>> for ForwardDeclaredStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21ForwardDeclaredStructC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for ForwardDeclaredStruct {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
    }
}

impl ::ctor::UnpinAssign<&Self> for ForwardDeclaredStruct {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN21ForwardDeclaredStructaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for ForwardDeclaredStruct {
    #[inline(always)]
    fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN21ForwardDeclaredStructaSEOS_(self, __param_0);
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN21ForwardDeclaredStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21ForwardDeclaredStructC1EOS_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::ForwardDeclaredStruct>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN21ForwardDeclaredStructaSERKS_<'__return_lifetime>(
            __this: &mut crate::ForwardDeclaredStruct,
            __param_0: &crate::ForwardDeclaredStruct,
        ) -> &'__return_lifetime mut crate::ForwardDeclaredStruct;
        pub(crate) unsafe fn __rust_thunk___ZN21ForwardDeclaredStructaSEOS_<'__return_lifetime>(
            __this: &mut crate::ForwardDeclaredStruct,
            __param_0: ::ctor::RvalueReference<'_, crate::ForwardDeclaredStruct>,
        ) -> &'__return_lifetime mut crate::ForwardDeclaredStruct;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::ForwardDeclaredStruct>() == 1);
    assert!(::core::mem::align_of::<crate::ForwardDeclaredStruct>() == 1);
    static_assertions::assert_impl_all!(crate::ForwardDeclaredStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ForwardDeclaredStruct: Drop);
};
