// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:compatibility_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

/// This type renames the special member functions so that they can be
/// overridden in Rust instead -- this is proof that you can write bindings
/// that are forward-compatible, as described in
/// additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl
#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=CompatibleType
pub struct CompatibleType {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for CompatibleType {}
impl !Sync for CompatibleType {}
forward_declare::unsafe_define!(forward_declare::symbol!("CompatibleType"), crate::CompatibleType);

impl CompatibleType {
    #[inline(always)]
    pub fn renamed_default_constructor<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN14CompatibleTypeC1Ev(self) }
    }
}

impl CompatibleType {
    #[inline(always)]
    pub fn renamed_copy_constructor<'a, 'b>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: &'b Self,
    ) {
        unsafe { crate::detail::__rust_thunk___ZN14CompatibleTypeC1ERKS_(self, __param_0) }
    }
}

impl<'b> ::ctor::Assign<&'b Self> for CompatibleType {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN14CompatibleTypeaSERKS_(self, __param_0);
        }
    }
}

impl CompatibleType {
    #[inline(always)]
    pub fn renamed_constructor<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::core::ffi::c_int,
    ) {
        unsafe { crate::detail::__rust_thunk___ZN14CompatibleTypeC1Ei(self, __param_0) }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_ZN14CompatibleTypeC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN14CompatibleTypeC1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::CompatibleType>,
        );
        #[link_name = "_ZN14CompatibleTypeC1ERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN14CompatibleTypeC1ERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::CompatibleType>,
            __param_0: &'b crate::CompatibleType,
        );
        #[link_name = "_ZN14CompatibleTypeaSERKS_"]
        pub(crate) unsafe fn __rust_thunk___ZN14CompatibleTypeaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::CompatibleType>,
            __param_0: &'b crate::CompatibleType,
        ) -> ::core::pin::Pin<&'a mut crate::CompatibleType>;
        #[link_name = "_ZN14CompatibleTypeC1Ei"]
        pub(crate) unsafe fn __rust_thunk___ZN14CompatibleTypeC1Ei<'a>(
            __this: ::core::pin::Pin<&'a mut crate::CompatibleType>,
            __param_0: ::core::ffi::c_int,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::CompatibleType>() == 1);
    assert!(::core::mem::align_of::<crate::CompatibleType>() == 1);
    static_assertions::assert_not_impl_any!(crate::CompatibleType: Copy);
    static_assertions::assert_not_impl_any!(crate::CompatibleType: Drop);
};
