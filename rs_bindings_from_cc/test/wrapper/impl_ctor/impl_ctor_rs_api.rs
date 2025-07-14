// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/wrapper/impl_ctor:impl_ctor
// Features: supported, unsafe_types, wrapper

#![rustfmt::skip]
#![feature(
    allocator_api,
    cfg_sanitize,
    custom_inner_attributes,
    impl_trait_in_assoc_type,
    negative_impls
)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

/// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=8
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Nontrivial
pub struct Nontrivial {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub value: ::core::ffi::c_int,
}
impl !Send for Nontrivial {}
impl !Sync for Nontrivial {}
unsafe impl ::cxx::ExternType for Nontrivial {
    type Id = ::cxx::type_id!("Nontrivial");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nontrivial"), crate::Nontrivial);

// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=10
// Error while generating bindings for function 'Nontrivial::Nontrivial':
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Nontrivial
// Expected first parameter to be a `__this` reference, found *mut crate::Nontrivial

// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=11
// Error while generating bindings for function 'Nontrivial::Nontrivial':
// Expected first constructor parameter to be a mutable reference, got: *mut crate::Nontrivial
// Expected first parameter to be a `__this` reference, found *mut crate::Nontrivial

// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=12
// Error while generating bindings for function 'Nontrivial::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

/// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=13
impl ::ctor::PinnedDrop for Nontrivial {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self)
    }
}

/// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=16
#[inline(always)]
pub(crate) fn Create() -> impl ::ctor::Ctor<Output = crate::Nontrivial, Error = ::ctor::Infallible>
{
    unsafe {
        ::ctor::FnCtor::new(move |dest: *mut crate::Nontrivial| {
            crate::detail::__rust_thunk___Z6Createv(dest as *mut ::core::ffi::c_void);
        })
    }
}

/// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=18
#[inline(always)]
pub(crate) fn Read(
    nontrivial: impl ::ctor::Ctor<Output = crate::Nontrivial, Error = ::ctor::Infallible>,
) -> ::core::ffi::c_int {
    unsafe {
        crate::detail::__rust_thunk___Z4Read10Nontrivial(::core::pin::Pin::into_inner_unchecked(
            ::ctor::emplace!(nontrivial),
        ))
    }
}

#[path = "rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor_extra.rs"]
mod __crubit_mod_0;
#[allow(unused_imports)]
pub use __crubit_mod_0::*;

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        );
        pub(crate) unsafe fn __rust_thunk___Z6Createv(__return: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z4Read10Nontrivial(
            nontrivial: &mut crate::Nontrivial,
        ) -> ::core::ffi::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Nontrivial>() == 4);
    assert!(::core::mem::align_of::<crate::Nontrivial>() == 4);
    static_assertions::assert_impl_all!(crate::Nontrivial: Drop);
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy);
    assert!(::core::mem::offset_of!(crate::Nontrivial, value) == 0);
    static_assertions::assert_impl_all!(::core::ffi::c_int: Copy);
};
