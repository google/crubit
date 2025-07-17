// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:no_elided_lifetimes_cc

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

#[inline(always)]
pub unsafe fn free_function(p1: *mut ::core::ffi::c_int) -> *mut ::core::ffi::c_int {
    crate::detail::__rust_thunk___Z13free_functionRi(p1)
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=S
pub struct S {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for S {}
impl !Sync for S {}
unsafe impl ::cxx::ExternType for S {
    type Id = ::cxx::type_id!("S");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("S"), crate::S);

impl Default for S {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN1SC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

impl S {
    #[inline(always)]
    pub unsafe fn const_method(
        __this: *const Self,
        p1: *mut ::core::ffi::c_int,
        p2: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_int {
        crate::detail::__rust_thunk___ZNK1S12const_methodERiS0_(__this, p1, p2)
    }
}

impl S {
    #[inline(always)]
    pub unsafe fn method(
        __this: *mut Self,
        p1: *mut ::core::ffi::c_int,
        p2: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_int {
        crate::detail::__rust_thunk___ZN1S6methodERiS0_(__this, p1, p2)
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=TriviallyCopyableButNontriviallyDestructible
pub struct TriviallyCopyableButNontriviallyDestructible {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for TriviallyCopyableButNontriviallyDestructible {}
impl !Sync for TriviallyCopyableButNontriviallyDestructible {}
unsafe impl ::cxx::ExternType for TriviallyCopyableButNontriviallyDestructible {
    type Id = ::cxx::type_id!("TriviallyCopyableButNontriviallyDestructible");
    type Kind = ::cxx::kind::Opaque;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("TriviallyCopyableButNontriviallyDestructible"),
    crate::TriviallyCopyableButNontriviallyDestructible
);

impl ::ctor::Assign<&Self> for TriviallyCopyableButNontriviallyDestructible {
    #[inline(always)]
    fn assign(self: ::core::pin::Pin<&mut Self>, __param_0: &Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleaSERKS_(
                self, __param_0,
            );
        }
    }
}

impl<'__unelided> ::ctor::CtorNew<&'__unelided Self>
    for TriviallyCopyableButNontriviallyDestructible
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__unelided Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: *mut Self| {
                crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_(dest as*mut::core::ffi::c_void,__param_0);
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(&'__unelided Self,)>
    for TriviallyCopyableButNontriviallyDestructible
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__unelided Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__unelided Self>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for TriviallyCopyableButNontriviallyDestructible {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev(self)
    }
}

#[inline(always)]
pub unsafe fn take_pointer(p: *mut ::core::ffi::c_int) {
    crate::detail::__rust_thunk___Z12take_pointerPi(p)
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=WrappedValue
pub struct WrappedValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) value_: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for WrappedValue {}
impl !Sync for WrappedValue {}
unsafe impl ::cxx::ExternType for WrappedValue {
    type Id = ::cxx::type_id!("WrappedValue");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("WrappedValue"), crate::WrappedValue);

impl From<::core::ffi::c_int> for WrappedValue {
    #[inline(always)]
    fn from(value: ::core::ffi::c_int) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN12WrappedValueC1Ei(&raw mut tmp as *mut _, value);
            tmp.assume_init()
        }
    }
}
impl ::ctor::CtorNew<::core::ffi::c_int> for WrappedValue {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::core::ffi::c_int) -> Self::CtorType {
        <Self as From<::core::ffi::c_int>>::from(args)
    }
}

impl ::core::ops::Add<&crate::WrappedValue> for &crate::WrappedValue {
    type Output = crate::WrappedValue;
    #[inline(always)]
    fn add(self, rhs: &crate::WrappedValue) -> Self::Output {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::WrappedValue>::uninit();
            crate::detail::__rust_thunk___ZNK12WrappedValueplERKS_(
                &raw mut __return as *mut ::core::ffi::c_void,
                self,
                rhs,
            );
            __return.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        #[link_name = "_Z13free_functionRi"]
        pub(crate) unsafe fn __rust_thunk___Z13free_functionRi(
            p1: *mut ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN1SC1Ev(__this: *mut ::core::ffi::c_void);
        #[link_name = "_ZNK1S12const_methodERiS0_"]
        pub(crate) unsafe fn __rust_thunk___ZNK1S12const_methodERiS0_(
            __this: *const crate::S,
            p1: *mut ::core::ffi::c_int,
            p2: *mut ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_int;
        #[link_name = "_ZN1S6methodERiS0_"]
        pub(crate) unsafe fn __rust_thunk___ZN1S6methodERiS0_(
            __this: *mut crate::S,
            p1: *mut ::core::ffi::c_int,
            p2: *mut ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleaSERKS_<
            '__return_lifetime,
        >(
            __this: ::core::pin::Pin<&mut crate::TriviallyCopyableButNontriviallyDestructible>,
            __param_0: &crate::TriviallyCopyableButNontriviallyDestructible,
        ) -> ::core::pin::Pin<
            &'__return_lifetime mut crate::TriviallyCopyableButNontriviallyDestructible,
        >;
        pub(crate) unsafe fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleC1ERKS_<
            '__unelided,
        >(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__unelided crate::TriviallyCopyableButNontriviallyDestructible,
        );
        #[link_name = "_ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev"]
        pub(crate) unsafe fn __rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev<
            'a,
        >(
            __this: ::core::pin::Pin<&'a mut crate::TriviallyCopyableButNontriviallyDestructible>,
        );
        #[link_name = "_Z12take_pointerPi"]
        pub(crate) unsafe fn __rust_thunk___Z12take_pointerPi(p: *mut ::core::ffi::c_int);
        pub(crate) unsafe fn __rust_thunk___ZN12WrappedValueC1Ei(
            __this: *mut ::core::ffi::c_void,
            value: ::core::ffi::c_int,
        );
        pub(crate) unsafe fn __rust_thunk___ZNK12WrappedValueplERKS_(
            __return: *mut ::core::ffi::c_void,
            __this: &crate::WrappedValue,
            rhs: &crate::WrappedValue,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::S>() == 1);
    assert!(::core::mem::align_of::<crate::S>() == 1);
    static_assertions::assert_impl_all!(crate::S: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::S: Drop);

    assert!(::core::mem::size_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
    assert!(::core::mem::align_of::<crate::TriviallyCopyableButNontriviallyDestructible>() == 1);
    static_assertions::assert_impl_all!(crate::TriviallyCopyableButNontriviallyDestructible: Drop);
    static_assertions::assert_not_impl_any!(crate::TriviallyCopyableButNontriviallyDestructible: Copy);

    assert!(::core::mem::size_of::<crate::WrappedValue>() == 4);
    assert!(::core::mem::align_of::<crate::WrappedValue>() == 4);
    static_assertions::assert_impl_all!(crate::WrappedValue: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::WrappedValue: Drop);
    assert!(::core::mem::offset_of!(crate::WrappedValue, value_) == 0);
};
