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
#![allow(dead_code, unused_mut)]
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
unsafe impl ::cxx::ExternType for CompatibleType {
    type Id = ::cxx::type_id!("CompatibleType");
    type Kind = ::cxx::kind::Opaque;
}
impl CompatibleType {
    #[inline(always)]
    pub fn renamed_default_constructor<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN14CompatibleTypeC1Ev(self) }
    }
    #[inline(always)]
    pub fn renamed_constructor<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ffi_11::c_int,
    ) {
        unsafe { crate::detail::__rust_thunk___ZN14CompatibleTypeC1Ei(self, __param_0) }
    }
}

// Error while generating bindings for constructor 'CompatibleType::CompatibleType':
// Can't generate bindings for CompatibleType::CompatibleType, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:compatibility_cc needs [//features:experimental] for CompatibleType::CompatibleType (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'CompatibleType::operator=':
// Can't generate bindings for CompatibleType::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:compatibility_cc needs [//features:experimental] for CompatibleType::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:compatibility_cc needs [//features:experimental] for CompatibleType::operator= (the type of __param_0 (parameter #1): references are not supported)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_ZN14CompatibleTypeC1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN14CompatibleTypeC1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::CompatibleType>,
        );
        #[link_name = "_ZN14CompatibleTypeC1Ei"]
        pub(crate) unsafe fn __rust_thunk___ZN14CompatibleTypeC1Ei<'a>(
            __this: ::core::pin::Pin<&'a mut crate::CompatibleType>,
            __param_0: ::ffi_11::c_int,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::CompatibleType>() == 1);
    assert!(::core::mem::align_of::<crate::CompatibleType>() == 1);
    static_assertions::assert_not_impl_any!(crate::CompatibleType: Copy,Drop);
};
