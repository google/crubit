// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:conversion_operators_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DstLocalMovable
pub struct DstLocalMovable {
    pub val: ::ffi_11::c_int,
}
impl !Send for DstLocalMovable {}
impl !Sync for DstLocalMovable {}
unsafe impl ::cxx::ExternType for DstLocalMovable {
    type Id = ::cxx::type_id!("DstLocalMovable");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for DstLocalMovable {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15DstLocalMovableC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[::ctor::recursively_pinned]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=DstLocalNonMovable
pub struct DstLocalNonMovable {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 0],
    pub val: ::ffi_11::c_int,
}
impl !Send for DstLocalNonMovable {}
impl !Sync for DstLocalNonMovable {}
unsafe impl ::cxx::ExternType for DstLocalNonMovable {
    type Id = ::cxx::type_id!("DstLocalNonMovable");
    type Kind = ::cxx::kind::Opaque;
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=Src
pub struct Src {
    pub value: ::ffi_11::c_int,
}
impl !Send for Src {}
impl !Sync for Src {}
unsafe impl ::cxx::ExternType for Src {
    type Id = ::cxx::type_id!("Src");
    type Kind = ::cxx::kind::Trivial;
}

impl Default for Src {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN3SrcC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl<'__this> ::core::convert::Into<::cref::CRef<'__this, ::ffi_11::c_int>>
    for &'__this crate::Src
{
    #[inline(always)]
    fn into(self) -> ::cref::CRef<'__this, ::ffi_11::c_int> {
        unsafe { crate::detail::__rust_thunk___ZNK3SrccvRKiEv(self) }
    }
}

impl<'__this> ::core::convert::Into<::cref::CMut<'__this, ::ffi_11::c_int>>
    for &'__this mut crate::Src
{
    #[inline(always)]
    fn into(self) -> ::cref::CMut<'__this, ::ffi_11::c_int> {
        unsafe { crate::detail::__rust_thunk___ZN3SrccvRiEv(self) }
    }
}

impl<'__this> From<&'__this crate::Src> for crate::DstLocalMovable {
    #[inline(always)]
    fn from(args: &'__this crate::Src) -> Self {
        let mut __this = args;
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZNK3Srccv15DstLocalMovableEv(
                &raw mut tmp as *mut _,
                __this,
            );
            tmp.assume_init()
        }
    }
}
impl<'__this> ::ctor::CtorNew<&'__this crate::Src> for crate::DstLocalMovable {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__this crate::Src) -> Self::CtorType {
        <Self as From<&'__this crate::Src>>::from(args)
    }
}

impl<'__this> ::ctor::CtorNew<&'__this crate::Src> for crate::DstLocalNonMovable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__this>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__this crate::Src) -> Self::CtorType {
        let mut __this = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZNK3Srccv18DstLocalNonMovableEv(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __this,
                );
            })
        }
    }
}
impl<'__this> ::ctor::CtorNew<(&'__this crate::Src,)> for crate::DstLocalNonMovable {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__this>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__this crate::Src,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__this crate::Src>>::ctor_new(arg)
    }
}

impl<'__this> ::core::convert::Into<::ffi_11::c_int> for &'__this crate::Src {
    #[inline(always)]
    fn into(self) -> ::ffi_11::c_int {
        unsafe { crate::detail::__rust_thunk___ZNK3SrccviEv(self) }
    }
}

impl<'__this> ::core::convert::Into<f64> for &'__this mut crate::Src {
    #[inline(always)]
    fn into(self) -> f64 {
        unsafe { crate::detail::__rust_thunk___ZN3SrccvdEv(self) }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN15DstLocalMovableC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN3SrcC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZNK3SrccvRKiEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK3SrccvRKiEv<'__this>(
            __this: &'__this crate::Src,
        ) -> ::cref::CRef<'__this, ::ffi_11::c_int>;
        #[link_name = "_ZN3SrccvRiEv"]
        pub(crate) unsafe fn __rust_thunk___ZN3SrccvRiEv<'__this>(
            __this: &'__this mut crate::Src,
        ) -> ::cref::CMut<'__this, ::ffi_11::c_int>;
        pub(crate) unsafe fn __rust_thunk___ZNK3Srccv15DstLocalMovableEv<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::Src,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK3Srccv18DstLocalNonMovableEv<'__this>(
            __return: *mut ::core::ffi::c_void,
            __this: &'__this crate::Src,
        );
        #[link_name = "_ZNK3SrccviEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK3SrccviEv<'__this>(
            __this: &'__this crate::Src,
        ) -> ::ffi_11::c_int;
        #[link_name = "_ZN3SrccvdEv"]
        pub(crate) unsafe fn __rust_thunk___ZN3SrccvdEv<'__this>(
            __this: &'__this mut crate::Src,
        ) -> f64;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::DstLocalMovable>() == 4);
    assert!(::core::mem::align_of::<crate::DstLocalMovable>() == 4);
    static_assertions::assert_impl_all!(crate::DstLocalMovable: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::DstLocalMovable: Drop);
    assert!(::core::mem::offset_of!(crate::DstLocalMovable, val) == 0);
    assert!(::core::mem::size_of::<crate::DstLocalNonMovable>() == 4);
    assert!(::core::mem::align_of::<crate::DstLocalNonMovable>() == 4);
    static_assertions::assert_not_impl_any!(crate::DstLocalNonMovable: Copy,Drop);
    assert!(::core::mem::offset_of!(crate::DstLocalNonMovable, val) == 0);
    assert!(::core::mem::size_of::<crate::Src>() == 4);
    assert!(::core::mem::align_of::<crate::Src>() == 4);
    static_assertions::assert_impl_all!(crate::Src: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::Src: Drop);
    assert!(::core::mem::offset_of!(crate::Src, value) == 0);
};
