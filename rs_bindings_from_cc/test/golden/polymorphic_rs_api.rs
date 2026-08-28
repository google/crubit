// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:polymorphic_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, impl_trait_in_assoc_type, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "15PolymorphicBase"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=PolymorphicBase
pub struct PolymorphicBase {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for PolymorphicBase {}
impl !Sync for PolymorphicBase {}
unsafe impl ::cxx::ExternType for PolymorphicBase {
    type Id = ::cxx::type_id!("PolymorphicBase");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for PolymorphicBase {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN15PolymorphicBaseC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for PolymorphicBase {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN15PolymorphicBaseC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for PolymorphicBase {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for PolymorphicBase {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN15PolymorphicBaseaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::PinnedDrop for PolymorphicBase {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN15PolymorphicBaseD1Ev(self) }
    }
}

unsafe impl ::operator::Delete for crate::PolymorphicBase {
    #[inline(always)]
    unsafe fn delete(p: *mut Self) {
        unsafe {
            crate::detail::__crubit_operator_delete__15PolymorphicBase___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3apolymorphic_5fcc(p);
        }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "16PolymorphicBase2"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=PolymorphicBase2
pub struct PolymorphicBase2 {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 8],
}
impl !Send for PolymorphicBase2 {}
impl !Sync for PolymorphicBase2 {}
unsafe impl ::cxx::ExternType for PolymorphicBase2 {
    type Id = ::cxx::type_id!("PolymorphicBase2");
    type Kind = ::cxx::kind::Opaque;
}
impl PolymorphicBase2 {
    #[inline(always)]
    pub fn Foo<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { self::polymorphic_base2::Foo(self) }
    }
}

impl ::ctor::CtorNew<()> for PolymorphicBase2 {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN16PolymorphicBase2C1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for PolymorphicBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN16PolymorphicBase2C1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for PolymorphicBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for PolymorphicBase2 {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN16PolymorphicBase2aSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::PinnedDrop for PolymorphicBase2 {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN16PolymorphicBase2D1Ev(self) }
    }
}

unsafe impl ::operator::Delete for crate::PolymorphicBase2 {
    #[inline(always)]
    unsafe fn delete(p: *mut Self) {
        unsafe {
            crate::detail::__crubit_operator_delete__16PolymorphicBase2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3apolymorphic_5fcc(p);
        }
    }
}

pub mod polymorphic_base2 {
    #[inline(always)]
    pub(crate) fn Foo<'__this>(__this: ::core::pin::Pin<&'__this mut crate::PolymorphicBase2>) {
        unsafe { crate::detail::__rust_thunk___ZN16PolymorphicBase23FooEv(__this) }
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[cfi_encoding = "18PolymorphicDerived"]
#[repr(C, align(8))]
///CRUBIT_ANNOTATE: cpp_type=PolymorphicDerived
pub struct PolymorphicDerived {
    __non_field_data: [::core::cell::Cell<::core::mem::MaybeUninit<u8>>; 16],
}
impl !Send for PolymorphicDerived {}
impl !Sync for PolymorphicDerived {}
unsafe impl ::cxx::ExternType for PolymorphicDerived {
    type Id = ::cxx::type_id!("PolymorphicDerived");
    type Kind = ::cxx::kind::Opaque;
}

impl ::ctor::CtorNew<()> for PolymorphicDerived {
    type CtorType = ::ctor::Ctor![Self];
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN18PolymorphicDerivedC1Ev(
                    __crubit_dest as *mut ::core::ffi::c_void,
                );
            })
        }
    }
}

impl<'__param_0> ::ctor::CtorNew<&'__param_0 Self> for PolymorphicDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: &'__param_0 Self) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN18PolymorphicDerivedC1ERKS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__param_0> ::ctor::CtorNew<(&'__param_0 Self,)> for PolymorphicDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__param_0>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (&'__param_0 Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'__param_0 Self>>::ctor_new(arg)
    }
}

impl<'__unelided> ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>
    for PolymorphicDerived
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'__unelided, Self>) -> Self::CtorType {
        let mut __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |__crubit_dest: *mut Self| {
                crate::detail::__rust_thunk___ZN18PolymorphicDerivedC1EOS_(
                    __crubit_dest as *mut ::core::ffi::c_void,
                    __param_0,
                );
            })
        }
    }
}
impl<'__unelided> ::ctor::CtorNew<(::ctor::RvalueReference<'__unelided, Self>,)>
    for PolymorphicDerived
{
    type CtorType = impl ::ctor::Ctor<Output = Self, Error = ::ctor::Infallible> + use<'__unelided>;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'__unelided, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'__unelided, Self>>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for PolymorphicDerived {
    #[inline(always)]
    unsafe fn pinned_drop<'__this>(self: ::core::pin::Pin<&'__this mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN18PolymorphicDerivedD1Ev(self) }
    }
}

impl<'__param_0> ::ctor::Assign<&'__param_0 Self> for PolymorphicDerived {
    #[inline(always)]
    fn assign<'__this>(self: ::core::pin::Pin<&'__this mut Self>, __param_0: &'__param_0 Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN18PolymorphicDerivedaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<::ctor::RvalueReference<'_, Self>> for PolymorphicDerived {
    #[inline(always)]
    fn assign<'__this>(
        self: ::core::pin::Pin<&'__this mut Self>,
        __param_0: ::ctor::RvalueReference<'_, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN18PolymorphicDerivedaSEOS_(self, __param_0);
        }
    }
}

unsafe impl ::operator::Delete for crate::PolymorphicDerived {
    #[inline(always)]
    unsafe fn delete(p: *mut Self) {
        unsafe {
            crate::detail::__crubit_operator_delete__18PolymorphicDerived___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3apolymorphic_5fcc(p);
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN15PolymorphicBaseC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15PolymorphicBaseC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::PolymorphicBase,
        );
        pub(crate) unsafe fn __rust_thunk___ZN15PolymorphicBaseaSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::PolymorphicBase>,
            __param_0: &'__param_0 crate::PolymorphicBase,
        ) -> ::core::pin::Pin<&'__this mut crate::PolymorphicBase>;
        pub(crate) unsafe fn __rust_thunk___ZN15PolymorphicBaseD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::PolymorphicBase>,
        );
        pub(crate) unsafe fn __crubit_operator_delete__15PolymorphicBase___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3apolymorphic_5fcc(
            ptr: *mut crate::PolymorphicBase,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16PolymorphicBase2C1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16PolymorphicBase2C1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::PolymorphicBase2,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16PolymorphicBase2aSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::PolymorphicBase2>,
            __param_0: &'__param_0 crate::PolymorphicBase2,
        ) -> ::core::pin::Pin<&'__this mut crate::PolymorphicBase2>;
        pub(crate) unsafe fn __rust_thunk___ZN16PolymorphicBase23FooEv<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::PolymorphicBase2>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN16PolymorphicBase2D1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::PolymorphicBase2>,
        );
        pub(crate) unsafe fn __crubit_operator_delete__16PolymorphicBase2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3apolymorphic_5fcc(
            ptr: *mut crate::PolymorphicBase2,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18PolymorphicDerivedC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18PolymorphicDerivedC1ERKS_<'__param_0>(
            __this: *mut ::core::ffi::c_void,
            __param_0: &'__param_0 crate::PolymorphicDerived,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18PolymorphicDerivedC1EOS_<'__unelided>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'__unelided, crate::PolymorphicDerived>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18PolymorphicDerivedD1Ev<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::PolymorphicDerived>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN18PolymorphicDerivedaSERKS_<'__param_0, '__this>(
            __this: ::core::pin::Pin<&'__this mut crate::PolymorphicDerived>,
            __param_0: &'__param_0 crate::PolymorphicDerived,
        ) -> ::core::pin::Pin<&'__this mut crate::PolymorphicDerived>;
        pub(crate) unsafe fn __rust_thunk___ZN18PolymorphicDerivedaSEOS_<'__this>(
            __this: ::core::pin::Pin<&'__this mut crate::PolymorphicDerived>,
            __param_0: ::ctor::RvalueReference<'_, crate::PolymorphicDerived>,
        ) -> ::core::pin::Pin<&'__this mut crate::PolymorphicDerived>;
        pub(crate) unsafe fn __crubit_operator_delete__18PolymorphicDerived___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3apolymorphic_5fcc(
            ptr: *mut crate::PolymorphicDerived,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::PolymorphicBase>() == 8);
    assert!(::core::mem::align_of::<crate::PolymorphicBase>() == 8);
    static_assertions::assert_impl_all!(crate::PolymorphicBase: Drop);
    static_assertions::assert_not_impl_any!(crate::PolymorphicBase: Copy);

    assert!(::core::mem::size_of::<crate::PolymorphicBase2>() == 8);
    assert!(::core::mem::align_of::<crate::PolymorphicBase2>() == 8);
    static_assertions::assert_impl_all!(crate::PolymorphicBase2: Drop);
    static_assertions::assert_not_impl_any!(crate::PolymorphicBase2: Copy);

    assert!(::core::mem::size_of::<crate::PolymorphicDerived>() == 16);
    assert!(::core::mem::align_of::<crate::PolymorphicDerived>() == 8);
    static_assertions::assert_impl_all!(crate::PolymorphicDerived: Drop);
    static_assertions::assert_not_impl_any!(crate::PolymorphicDerived: Copy);
};
