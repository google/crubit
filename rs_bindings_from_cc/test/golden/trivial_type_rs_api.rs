// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:trivial_type_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod ns {
    /// Implicitly defined special member functions are trivial on a struct with
    /// only trivial members.
    ///
    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=13
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct Trivial {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("Trivial"), crate::ns::Trivial);

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=13
    impl Default for Trivial {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialC1Ev(&mut tmp);
                tmp.assume_init()
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=13
    impl<'b> From<::ctor::RvalueReference<'b, Self>> for Trivial {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialC1EOS0_(&mut tmp, __param_0);
                tmp.assume_init()
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=13
    impl<'b> ::ctor::UnpinAssign<&'b Self> for Trivial {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialaSERKS0_(self, __param_0);
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=13
    impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for Trivial {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialaSEOS0_(self, __param_0);
            }
        }
    }

    /// Defaulted special member functions are trivial on a struct with only trivial
    /// members.
    ///
    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=19
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct TrivialWithDefaulted {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("TrivialWithDefaulted"),
        crate::ns::TrivialWithDefaulted
    );

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=20
    impl Default for TrivialWithDefaulted {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns20TrivialWithDefaultedC1Ev(&mut tmp);
                tmp.assume_init()
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=23
    impl<'b> ::ctor::UnpinAssign<&'b Self> for TrivialWithDefaulted {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns20TrivialWithDefaultedaSERKS0_(self, __param_0);
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=24
    impl<'b> From<::ctor::RvalueReference<'b, Self>> for TrivialWithDefaulted {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns20TrivialWithDefaultedC1EOS0_(
                    &mut tmp, __param_0,
                );
                tmp.assume_init()
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=25
    impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for TrivialWithDefaulted {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns20TrivialWithDefaultedaSEOS0_(self, __param_0);
            }
        }
    }

    impl TrivialWithDefaulted {
        /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=31
        #[inline(always)]
        pub fn Unqualified<'a>(&'a mut self) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns20TrivialWithDefaulted11UnqualifiedEv(self)
            }
        }
    }

    impl TrivialWithDefaulted {
        /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=32
        #[inline(always)]
        pub fn ConstQualified<'a>(&'a self) {
            unsafe {
                crate::detail::__rust_thunk___ZNK2ns20TrivialWithDefaulted14ConstQualifiedEv(self)
            }
        }
    }

    impl TrivialWithDefaulted {
        /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=33
        #[inline(always)]
        pub fn LvalueRefQualified<'a>(&'a mut self) {
            unsafe {
                crate::detail::__rust_thunk___ZNR2ns20TrivialWithDefaulted18LvalueRefQualifiedEv(
                    self,
                )
            }
        }
    }

    impl TrivialWithDefaulted {
        /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=34
        #[inline(always)]
        pub fn ConstLvalueRefQualified<'a>(&'a self) {
            unsafe {
                crate::detail::__rust_thunk___ZNKR2ns20TrivialWithDefaulted23ConstLvalueRefQualifiedEv(self)
            }
        }
    }

    impl TrivialWithDefaulted {
        /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=35
        #[inline(always)]
        pub fn RvalueRefQualified<'a>(&'a mut self) {
            unsafe {
                crate::detail::__rust_thunk___ZNO2ns20TrivialWithDefaulted18RvalueRefQualifiedEv(
                    self,
                )
            }
        }
    }

    impl TrivialWithDefaulted {
        /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=36
        #[inline(always)]
        pub fn ConstRvalueRefQualified<'a>(&'a self) {
            unsafe {
                crate::detail::__rust_thunk___ZNKO2ns20TrivialWithDefaulted23ConstRvalueRefQualifiedEv(self)
            }
        }
    }

    /// This struct is trivial, and therefore trivially relocatable etc., but still
    /// not safe to pass by reference as it is not final.
    ///
    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=41
    #[::ctor::recursively_pinned]
    #[repr(C)]
    pub struct TrivialNonfinal {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("TrivialNonfinal"),
        crate::ns::TrivialNonfinal
    );

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=41
    impl ::ctor::CtorNew<()> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(args: ()) -> Self::CtorType {
            let () = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                        crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1Ev(
                            ::core::pin::Pin::into_inner_unchecked(dest),
                        );
                    },
                )
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=41
    impl<'b> ::ctor::CtorNew<&'b Self> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: &'b Self) -> Self::CtorType {
            let __param_0 = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                        crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1ERKS0_(
                            ::core::pin::Pin::into_inner_unchecked(dest),
                            __param_0,
                        );
                    },
                )
            }
        }
    }
    impl<'b> ::ctor::CtorNew<(&'b Self,)> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=41
    impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
            let __param_0 = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                        crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1EOS0_(
                            ::core::pin::Pin::into_inner_unchecked(dest),
                            __param_0,
                        );
                    },
                )
            }
        }
    }
    impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=41
    impl<'b> ::ctor::Assign<&'b Self> for TrivialNonfinal {
        #[inline(always)]
        fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalaSERKS0_(self, __param_0);
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=41
    impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for TrivialNonfinal {
        #[inline(always)]
        fn assign<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            __param_0: ::ctor::RvalueReference<'b, Self>,
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalaSEOS0_(self, __param_0);
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=45
    #[inline(always)]
    pub fn TakesByValue(trivial: crate::ns::Trivial) -> crate::ns::Trivial {
        unsafe { crate::detail::__rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(trivial) }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=46
    #[inline(always)]
    pub fn TakesWithDefaultedByValue(
        trivial: crate::ns::TrivialWithDefaulted,
    ) -> crate::ns::TrivialWithDefaulted {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns25TakesWithDefaultedByValueENS_20TrivialWithDefaultedE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=47
    #[inline(always)]
    pub fn TakesTrivialNonfinalByValue(
        trivial: impl ::ctor::Ctor<Output = crate::ns::TrivialNonfinal>,
    ) -> impl ::ctor::Ctor<Output = crate::ns::TrivialNonfinal> {
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<
                    &mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
                >| {
                    crate::detail::__rust_thunk___ZN2ns27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(::core::pin::Pin::into_inner_unchecked(dest),::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(trivial)));
                },
            )
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=49
    #[inline(always)]
    pub fn TakesByReference<'a>(trivial: &'a mut crate::ns::Trivial) -> &'a mut crate::ns::Trivial {
        unsafe { crate::detail::__rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE(trivial) }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=50
    #[inline(always)]
    pub fn TakesWithDefaultedByReference<'a>(
        trivial: &'a mut crate::ns::TrivialWithDefaulted,
    ) -> &'a mut crate::ns::TrivialWithDefaulted {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns29TakesWithDefaultedByReferenceERNS_20TrivialWithDefaultedE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=52
    #[inline(always)]
    pub fn TakesTrivialNonfinalByReference<'a>(
        trivial: ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
    ) -> ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=54
    #[inline(always)]
    pub fn TakesByConstReference<'a>(trivial: &'a crate::ns::Trivial) -> &'a crate::ns::Trivial {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns21TakesByConstReferenceERKNS_7TrivialE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=55
    #[inline(always)]
    pub fn TakesWithDefaultedByConstReference<'a>(
        trivial: &'a crate::ns::TrivialWithDefaulted,
    ) -> &'a crate::ns::TrivialWithDefaulted {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns34TakesWithDefaultedByConstReferenceERKNS_20TrivialWithDefaultedE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=57
    #[inline(always)]
    pub fn TakesTrivialNonfinalByConstReference<'a>(
        trivial: &'a crate::ns::TrivialNonfinal,
    ) -> &'a crate::ns::TrivialNonfinal {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns36TakesTrivialNonfinalByConstReferenceERKNS_15TrivialNonfinalE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=60
    #[inline(always)]
    pub fn TakesByRvalueReference<'a>(
        trivial: ::ctor::RvalueReference<'a, crate::ns::Trivial>,
    ) -> ::ctor::RvalueReference<'a, crate::ns::Trivial> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns22TakesByRvalueReferenceEONS_7TrivialE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=61
    #[inline(always)]
    pub fn TakesWithDefaultedByRvalueReference<'a>(
        trivial: ::ctor::RvalueReference<'a, crate::ns::TrivialWithDefaulted>,
    ) -> ::ctor::RvalueReference<'a, crate::ns::TrivialWithDefaulted> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns35TakesWithDefaultedByRvalueReferenceEONS_20TrivialWithDefaultedE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=63
    #[inline(always)]
    pub fn TakesTrivialNonfinalByRvalueReference<'a>(
        trivial: ::ctor::RvalueReference<'a, crate::ns::TrivialNonfinal>,
    ) -> ::ctor::RvalueReference<'a, crate::ns::TrivialNonfinal> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns37TakesTrivialNonfinalByRvalueReferenceEONS_15TrivialNonfinalE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=66
    #[inline(always)]
    pub fn TakesByConstRvalueReference<'a>(
        trivial: ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>,
    ) -> ::ctor::ConstRvalueReference<'a, crate::ns::Trivial> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns27TakesByConstRvalueReferenceEOKNS_7TrivialE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=67
    #[inline(always)]
    pub fn TakesWithDefaultedByConstRvalueReference<'a>(
        trivial: ::ctor::ConstRvalueReference<'a, crate::ns::TrivialWithDefaulted>,
    ) -> ::ctor::ConstRvalueReference<'a, crate::ns::TrivialWithDefaulted> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns40TakesWithDefaultedByConstRvalueReferenceEOKNS_20TrivialWithDefaultedE(trivial)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/trivial_type.h;l=69
    #[inline(always)]
    pub fn TakesTrivialNonfinalByConstRvalueReference<'a>(
        trivial: ::ctor::ConstRvalueReference<'a, crate::ns::TrivialNonfinal>,
    ) -> ::ctor::ConstRvalueReference<'a, crate::ns::TrivialNonfinal> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns42TakesTrivialNonfinalByConstRvalueReferenceEOKNS_15TrivialNonfinalE(trivial)
        }
    }
}

// namespace ns

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN2ns7TrivialC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN2ns7TrivialC1EOS0_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::Trivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN2ns7TrivialaSERKS0_<'a, 'b>(
            __this: &'a mut crate::ns::Trivial,
            __param_0: &'b crate::ns::Trivial,
        ) -> &'a mut crate::ns::Trivial;
        pub(crate) fn __rust_thunk___ZN2ns7TrivialaSEOS0_<'a, 'b>(
            __this: &'a mut crate::ns::Trivial,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::Trivial>,
        ) -> &'a mut crate::ns::Trivial;
        pub(crate) fn __rust_thunk___ZN2ns20TrivialWithDefaultedC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::TrivialWithDefaulted>,
        );
        pub(crate) fn __rust_thunk___ZN2ns20TrivialWithDefaultedaSERKS0_<'a, 'b>(
            __this: &'a mut crate::ns::TrivialWithDefaulted,
            __param_0: &'b crate::ns::TrivialWithDefaulted,
        ) -> &'a mut crate::ns::TrivialWithDefaulted;
        pub(crate) fn __rust_thunk___ZN2ns20TrivialWithDefaultedC1EOS0_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::TrivialWithDefaulted>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialWithDefaulted>,
        );
        pub(crate) fn __rust_thunk___ZN2ns20TrivialWithDefaultedaSEOS0_<'a, 'b>(
            __this: &'a mut crate::ns::TrivialWithDefaulted,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialWithDefaulted>,
        ) -> &'a mut crate::ns::TrivialWithDefaulted;
        #[link_name = "_ZN2ns20TrivialWithDefaulted11UnqualifiedEv"]
        pub(crate) fn __rust_thunk___ZN2ns20TrivialWithDefaulted11UnqualifiedEv<'a>(
            __this: &'a mut crate::ns::TrivialWithDefaulted,
        );
        #[link_name = "_ZNK2ns20TrivialWithDefaulted14ConstQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNK2ns20TrivialWithDefaulted14ConstQualifiedEv<'a>(
            __this: &'a crate::ns::TrivialWithDefaulted,
        );
        #[link_name = "_ZNR2ns20TrivialWithDefaulted18LvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNR2ns20TrivialWithDefaulted18LvalueRefQualifiedEv<'a>(
            __this: &'a mut crate::ns::TrivialWithDefaulted,
        );
        #[link_name = "_ZNKR2ns20TrivialWithDefaulted23ConstLvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNKR2ns20TrivialWithDefaulted23ConstLvalueRefQualifiedEv<'a>(
            __this: &'a crate::ns::TrivialWithDefaulted,
        );
        #[link_name = "_ZNO2ns20TrivialWithDefaulted18RvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNO2ns20TrivialWithDefaulted18RvalueRefQualifiedEv<'a>(
            __this: &'a mut crate::ns::TrivialWithDefaulted,
        );
        #[link_name = "_ZNKO2ns20TrivialWithDefaulted23ConstRvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNKO2ns20TrivialWithDefaulted23ConstRvalueRefQualifiedEv<'a>(
            __this: &'a crate::ns::TrivialWithDefaulted,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1ERKS0_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
            __param_0: &'b crate::ns::TrivialNonfinal,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1EOS0_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalaSERKS0_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
            __param_0: &'b crate::ns::TrivialNonfinal,
        ) -> ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>;
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalaSEOS0_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,
        ) -> ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>;
        #[link_name = "_ZN2ns12TakesByValueENS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(
            trivial: crate::ns::Trivial,
        ) -> crate::ns::Trivial;
        #[link_name = "_ZN2ns25TakesWithDefaultedByValueENS_20TrivialWithDefaultedE"]
        pub(crate) fn __rust_thunk___ZN2ns25TakesWithDefaultedByValueENS_20TrivialWithDefaultedE(
            trivial: crate::ns::TrivialWithDefaulted,
        ) -> crate::ns::TrivialWithDefaulted;
        pub(crate) fn __rust_thunk___ZN2ns27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(
            __return: &mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
            trivial: &mut crate::ns::TrivialNonfinal,
        );
        #[link_name = "_ZN2ns16TakesByReferenceERNS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE<'a>(
            trivial: &'a mut crate::ns::Trivial,
        ) -> &'a mut crate::ns::Trivial;
        #[link_name = "_ZN2ns29TakesWithDefaultedByReferenceERNS_20TrivialWithDefaultedE"]
        pub(crate) fn __rust_thunk___ZN2ns29TakesWithDefaultedByReferenceERNS_20TrivialWithDefaultedE<
            'a,
        >(
            trivial: &'a mut crate::ns::TrivialWithDefaulted,
        ) -> &'a mut crate::ns::TrivialWithDefaulted;
        #[link_name = "_ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE<
            'a,
        >(
            trivial: ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
        ) -> ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>;
        #[link_name = "_ZN2ns21TakesByConstReferenceERKNS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns21TakesByConstReferenceERKNS_7TrivialE<'a>(
            trivial: &'a crate::ns::Trivial,
        ) -> &'a crate::ns::Trivial;
        #[link_name = "_ZN2ns34TakesWithDefaultedByConstReferenceERKNS_20TrivialWithDefaultedE"]
        pub(crate) fn __rust_thunk___ZN2ns34TakesWithDefaultedByConstReferenceERKNS_20TrivialWithDefaultedE<
            'a,
        >(
            trivial: &'a crate::ns::TrivialWithDefaulted,
        ) -> &'a crate::ns::TrivialWithDefaulted;
        #[link_name = "_ZN2ns36TakesTrivialNonfinalByConstReferenceERKNS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN2ns36TakesTrivialNonfinalByConstReferenceERKNS_15TrivialNonfinalE<
            'a,
        >(
            trivial: &'a crate::ns::TrivialNonfinal,
        ) -> &'a crate::ns::TrivialNonfinal;
        #[link_name = "_ZN2ns22TakesByRvalueReferenceEONS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns22TakesByRvalueReferenceEONS_7TrivialE<'a>(
            trivial: ::ctor::RvalueReference<'a, crate::ns::Trivial>,
        ) -> ::ctor::RvalueReference<'a, crate::ns::Trivial>;
        #[link_name = "_ZN2ns35TakesWithDefaultedByRvalueReferenceEONS_20TrivialWithDefaultedE"]
        pub(crate) fn __rust_thunk___ZN2ns35TakesWithDefaultedByRvalueReferenceEONS_20TrivialWithDefaultedE<
            'a,
        >(
            trivial: ::ctor::RvalueReference<'a, crate::ns::TrivialWithDefaulted>,
        ) -> ::ctor::RvalueReference<'a, crate::ns::TrivialWithDefaulted>;
        #[link_name = "_ZN2ns37TakesTrivialNonfinalByRvalueReferenceEONS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN2ns37TakesTrivialNonfinalByRvalueReferenceEONS_15TrivialNonfinalE<
            'a,
        >(
            trivial: ::ctor::RvalueReference<'a, crate::ns::TrivialNonfinal>,
        ) -> ::ctor::RvalueReference<'a, crate::ns::TrivialNonfinal>;
        #[link_name = "_ZN2ns27TakesByConstRvalueReferenceEOKNS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns27TakesByConstRvalueReferenceEOKNS_7TrivialE<'a>(
            trivial: ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>,
        ) -> ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>;
        #[link_name = "_ZN2ns40TakesWithDefaultedByConstRvalueReferenceEOKNS_20TrivialWithDefaultedE"]
        pub(crate) fn __rust_thunk___ZN2ns40TakesWithDefaultedByConstRvalueReferenceEOKNS_20TrivialWithDefaultedE<
            'a,
        >(
            trivial: ::ctor::ConstRvalueReference<'a, crate::ns::TrivialWithDefaulted>,
        ) -> ::ctor::ConstRvalueReference<'a, crate::ns::TrivialWithDefaulted>;
        #[link_name = "_ZN2ns42TakesTrivialNonfinalByConstRvalueReferenceEOKNS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN2ns42TakesTrivialNonfinalByConstRvalueReferenceEOKNS_15TrivialNonfinalE<
            'a,
        >(
            trivial: ::ctor::ConstRvalueReference<'a, crate::ns::TrivialNonfinal>,
        ) -> ::ctor::ConstRvalueReference<'a, crate::ns::TrivialNonfinal>;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::ns::Trivial>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::ns::Trivial>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::Trivial: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::Trivial: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::Trivial: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::ns::Trivial, trivial_field) == 0);
const _: () = assert!(::core::mem::size_of::<crate::ns::TrivialWithDefaulted>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::ns::TrivialWithDefaulted>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::TrivialWithDefaulted: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::TrivialWithDefaulted: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::TrivialWithDefaulted: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::ns::TrivialWithDefaulted, trivial_field) == 0);
const _: () = assert!(::core::mem::size_of::<crate::ns::TrivialNonfinal>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::ns::TrivialNonfinal>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::TrivialNonfinal: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::TrivialNonfinal: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::ns::TrivialNonfinal, trivial_field) == 0);
