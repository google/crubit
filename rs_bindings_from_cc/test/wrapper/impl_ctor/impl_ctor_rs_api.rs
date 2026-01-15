// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/wrapper/impl_ctor:impl_ctor
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported, wrapper

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
    pub value: ::ffi_11::c_int,
}
impl !Send for Nontrivial {}
impl !Sync for Nontrivial {}
unsafe impl ::cxx::ExternType for Nontrivial {
    type Id = ::cxx::type_id!("Nontrivial");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nontrivial"), crate::Nontrivial);

/// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=10
impl ::ctor::CtorNew<::ffi_11::c_int> for Nontrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ffi_11::c_int) -> Self::CtorType {
        let mut x = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ei(
                    dest as *mut ::core::ffi::c_void,
                    x,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(::ffi_11::c_int,)> for Nontrivial {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ffi_11::c_int,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ffi_11::c_int>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=11
impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN10NontrivialC1EOS_(
                    dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=12
impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for Nontrivial {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: ::ctor::RvalueReference<'_, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSEOS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=13
impl ::ctor::PinnedDrop for Nontrivial {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self)
    }
}

/// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=16
#[inline(always)]
pub fn Create() -> ::ctor::Ctor![crate::Nontrivial] {
    unsafe {
        ::ctor::FnCtor::new(move |dest: *mut crate::Nontrivial| {
            crate::detail::__rust_thunk___Z6Createv(dest as *mut ::core::ffi::c_void);
        })
    }
}

/// Generated from: rs_bindings_from_cc/test/wrapper/impl_ctor/impl_ctor.h;l=18
#[inline(always)]
pub fn Read(nontrivial: ::ctor::Ctor![crate::Nontrivial]) -> ::ffi_11::c_int {
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
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1Ei(
            __this: *mut ::core::ffi::c_void,
            x: ::ffi_11::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialC1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::Nontrivial>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialaSEOS_<'__return_lifetime>(
            __this: ::core::pin::Pin<&mut crate::Nontrivial>,
            __param_0: ::ctor::RvalueReference<'_, crate::Nontrivial>,
        ) -> ::core::pin::Pin<&'__return_lifetime mut crate::Nontrivial>;
        pub(crate) unsafe fn __rust_thunk___ZN10NontrivialD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        );
        pub(crate) unsafe fn __rust_thunk___Z6Createv(__return: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___Z4Read10Nontrivial(
            nontrivial: &mut crate::Nontrivial,
        ) -> ::ffi_11::c_int;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Nontrivial>() == 4);
    assert!(::core::mem::align_of::<crate::Nontrivial>() == 4);
    static_assertions::assert_impl_all!(crate::Nontrivial: Drop);
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy);
    assert!(::core::mem::offset_of!(crate::Nontrivial, value) == 0);
    static_assertions::assert_impl_all!(::ffi_11::c_int: Copy);
};
