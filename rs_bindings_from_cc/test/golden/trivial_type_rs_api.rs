// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:trivial_type_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(arbitrary_self_types, custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

pub mod ns {
    /// Implicitly defined special member functions are trivial on a struct with
    /// only trivial members.
    #[derive(Clone, Copy)]
    #[repr(C)]
    #[__crubit::annotate(cpp_type = "ns :: Trivial")]
    pub struct Trivial {
        pub trivial_field: ::core::ffi::c_int,
    }
    impl !Send for Trivial {}
    impl !Sync for Trivial {}
    forward_declare::unsafe_define!(forward_declare::symbol!("ns :: Trivial"), crate::ns::Trivial);

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

    impl<'b> ::ctor::UnpinAssign<&'b Self> for Trivial {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialaSERKS0_(self, __param_0);
            }
        }
    }

    impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for Trivial {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialaSEOS0_(self, __param_0);
            }
        }
    }

    impl Trivial {
        #[inline(always)]
        pub fn Unqualified<'a>(&'a mut self) {
            unsafe { crate::detail::__rust_thunk___ZN2ns7Trivial11UnqualifiedEv(self) }
        }
    }

    impl Trivial {
        #[inline(always)]
        pub fn ConstQualified<'a>(&'a self) {
            unsafe { crate::detail::__rust_thunk___ZNK2ns7Trivial14ConstQualifiedEv(self) }
        }
    }

    impl Trivial {
        #[inline(always)]
        pub fn LvalueRefQualified<'a>(&'a mut self) {
            unsafe { crate::detail::__rust_thunk___ZNR2ns7Trivial18LvalueRefQualifiedEv(self) }
        }
    }

    impl Trivial {
        #[inline(always)]
        pub fn ConstLvalueRefQualified<'a>(&'a self) {
            unsafe {
                crate::detail::__rust_thunk___ZNKR2ns7Trivial23ConstLvalueRefQualifiedEv(self)
            }
        }
    }

    impl Trivial {
        #[inline(always)]
        pub fn RvalueRefQualified<'a>(self: ::ctor::RvalueReference<'a, Self>) {
            unsafe { crate::detail::__rust_thunk___ZNO2ns7Trivial18RvalueRefQualifiedEv(self) }
        }
    }

    impl Trivial {
        #[inline(always)]
        pub fn ConstRvalueRefQualified<'a>(self: ::ctor::ConstRvalueReference<'a, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZNKO2ns7Trivial23ConstRvalueRefQualifiedEv(self)
            }
        }
    }

    /// This struct is trivial, and therefore trivially relocatable etc., but still
    /// not safe to pass by reference as it is not final.
    #[derive(Clone, Copy)]
    #[repr(C)]
    #[__crubit::annotate(cpp_type = "ns :: TrivialNonfinal")]
    pub struct TrivialNonfinal {
        pub trivial_field: ::core::ffi::c_int,
    }
    impl !Send for TrivialNonfinal {}
    impl !Sync for TrivialNonfinal {}
    forward_declare::unsafe_define!(
        forward_declare::symbol!("ns :: TrivialNonfinal"),
        crate::ns::TrivialNonfinal
    );

    impl Default for TrivialNonfinal {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1Ev(&mut tmp);
                tmp.assume_init()
            }
        }
    }

    impl<'b> From<::ctor::RvalueReference<'b, Self>> for TrivialNonfinal {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1EOS0_(&mut tmp, __param_0);
                tmp.assume_init()
            }
        }
    }

    impl<'b> ::ctor::UnpinAssign<&'b Self> for TrivialNonfinal {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalaSERKS0_(self, __param_0);
            }
        }
    }

    impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for TrivialNonfinal {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalaSEOS0_(self, __param_0);
            }
        }
    }

    #[inline(always)]
    pub fn TakesByValue(mut trivial: crate::ns::Trivial) -> crate::ns::Trivial {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ns::Trivial>::uninit();
            crate::detail::__rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(
                &mut __return,
                &mut trivial,
            );
            __return.assume_init()
        }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByValue(
        mut trivial: crate::ns::TrivialNonfinal,
    ) -> crate::ns::TrivialNonfinal {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ns::TrivialNonfinal>::uninit();
            crate::detail::__rust_thunk___ZN2ns27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(
                &mut __return,
                &mut trivial,
            );
            __return.assume_init()
        }
    }

    #[inline(always)]
    pub fn TakesByReference<'a>(trivial: &'a mut crate::ns::Trivial) -> &'a mut crate::ns::Trivial {
        unsafe { crate::detail::__rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE(trivial) }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByReference<'a>(
        trivial: &'a mut crate::ns::TrivialNonfinal,
    ) -> &'a mut crate::ns::TrivialNonfinal {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesByConstReference<'a>(trivial: &'a crate::ns::Trivial) -> &'a crate::ns::Trivial {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns21TakesByConstReferenceERKNS_7TrivialE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByConstReference<'a>(
        trivial: &'a crate::ns::TrivialNonfinal,
    ) -> &'a crate::ns::TrivialNonfinal {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns36TakesTrivialNonfinalByConstReferenceERKNS_15TrivialNonfinalE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesByRvalueReference<'a>(
        trivial: ::ctor::RvalueReference<'a, crate::ns::Trivial>,
    ) -> ::ctor::RvalueReference<'a, crate::ns::Trivial> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns22TakesByRvalueReferenceEONS_7TrivialE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByRvalueReference<'a>(
        trivial: ::ctor::RvalueReference<'a, crate::ns::TrivialNonfinal>,
    ) -> ::ctor::RvalueReference<'a, crate::ns::TrivialNonfinal> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns37TakesTrivialNonfinalByRvalueReferenceEONS_15TrivialNonfinalE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesByConstRvalueReference<'a>(
        trivial: ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>,
    ) -> ::ctor::ConstRvalueReference<'a, crate::ns::Trivial> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns27TakesByConstRvalueReferenceEOKNS_7TrivialE(trivial)
        }
    }

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
        #[link_name = "_ZN2ns7Trivial11UnqualifiedEv"]
        pub(crate) fn __rust_thunk___ZN2ns7Trivial11UnqualifiedEv<'a>(
            __this: &'a mut crate::ns::Trivial,
        );
        #[link_name = "_ZNK2ns7Trivial14ConstQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNK2ns7Trivial14ConstQualifiedEv<'a>(
            __this: &'a crate::ns::Trivial,
        );
        #[link_name = "_ZNR2ns7Trivial18LvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNR2ns7Trivial18LvalueRefQualifiedEv<'a>(
            __this: &'a mut crate::ns::Trivial,
        );
        #[link_name = "_ZNKR2ns7Trivial23ConstLvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNKR2ns7Trivial23ConstLvalueRefQualifiedEv<'a>(
            __this: &'a crate::ns::Trivial,
        );
        #[link_name = "_ZNO2ns7Trivial18RvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNO2ns7Trivial18RvalueRefQualifiedEv<'a>(
            __this: ::ctor::RvalueReference<'a, crate::ns::Trivial>,
        );
        #[link_name = "_ZNKO2ns7Trivial23ConstRvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNKO2ns7Trivial23ConstRvalueRefQualifiedEv<'a>(
            __this: ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1EOS0_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalaSERKS0_<'a, 'b>(
            __this: &'a mut crate::ns::TrivialNonfinal,
            __param_0: &'b crate::ns::TrivialNonfinal,
        ) -> &'a mut crate::ns::TrivialNonfinal;
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalaSEOS0_<'a, 'b>(
            __this: &'a mut crate::ns::TrivialNonfinal,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,
        ) -> &'a mut crate::ns::TrivialNonfinal;
        pub(crate) fn __rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(
            __return: &mut ::core::mem::MaybeUninit<crate::ns::Trivial>,
            trivial: &mut crate::ns::Trivial,
        );
        pub(crate) fn __rust_thunk___ZN2ns27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(
            __return: &mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
            trivial: &mut crate::ns::TrivialNonfinal,
        );
        #[link_name = "_ZN2ns16TakesByReferenceERNS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE<'a>(
            trivial: &'a mut crate::ns::Trivial,
        ) -> &'a mut crate::ns::Trivial;
        #[link_name = "_ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE<
            'a,
        >(
            trivial: &'a mut crate::ns::TrivialNonfinal,
        ) -> &'a mut crate::ns::TrivialNonfinal;
        #[link_name = "_ZN2ns21TakesByConstReferenceERKNS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns21TakesByConstReferenceERKNS_7TrivialE<'a>(
            trivial: &'a crate::ns::Trivial,
        ) -> &'a crate::ns::Trivial;
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
        #[link_name = "_ZN2ns42TakesTrivialNonfinalByConstRvalueReferenceEOKNS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN2ns42TakesTrivialNonfinalByConstRvalueReferenceEOKNS_15TrivialNonfinalE<
            'a,
        >(
            trivial: ::ctor::ConstRvalueReference<'a, crate::ns::TrivialNonfinal>,
        ) -> ::ctor::ConstRvalueReference<'a, crate::ns::TrivialNonfinal>;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::ns::Trivial>() == 4);
    assert!(::core::mem::align_of::<crate::ns::Trivial>() == 4);
    static_assertions::assert_impl_all!(crate::ns::Trivial: Clone);
    static_assertions::assert_impl_all!(crate::ns::Trivial: Copy);
    static_assertions::assert_not_impl_any!(crate::ns::Trivial: Drop);
    assert!(::core::mem::offset_of!(crate::ns::Trivial, trivial_field) == 0);
    assert!(::core::mem::size_of::<crate::ns::TrivialNonfinal>() == 4);
    assert!(::core::mem::align_of::<crate::ns::TrivialNonfinal>() == 4);
    static_assertions::assert_impl_all!(crate::ns::TrivialNonfinal: Clone);
    static_assertions::assert_impl_all!(crate::ns::TrivialNonfinal: Copy);
    static_assertions::assert_not_impl_any!(crate::ns::TrivialNonfinal: Drop);
    assert!(::core::mem::offset_of!(crate::ns::TrivialNonfinal, trivial_field) == 0);
};
