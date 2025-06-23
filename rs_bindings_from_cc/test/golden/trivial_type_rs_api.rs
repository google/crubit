// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:trivial_type_cc

#![rustfmt::skip]
#![feature(
    allocator_api,
    arbitrary_self_types,
    cfg_sanitize,
    custom_inner_attributes,
    negative_impls
)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod ns {
    /// Implicitly defined special member functions are trivial on a struct with
    /// only trivial members.
    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=ns :: Trivial
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
                crate::detail::__rust_thunk___ZN2ns7TrivialC1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    impl<'b> From<::ctor::RvalueReference<'b, Self>> for Trivial {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialC1EOS0_(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                    __param_0,
                );
                tmp.assume_init()
            }
        }
    }
    impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for Trivial {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
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

    #[inline(always)]
    pub fn TakesByValue(mut trivial: crate::ns::Trivial) -> crate::ns::Trivial {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ns::Trivial>::uninit();
            crate::detail::__rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(
                &raw mut __return as *mut ::core::ffi::c_void,
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
    pub fn TakesByConstReference<'a>(trivial: &'a crate::ns::Trivial) -> &'a crate::ns::Trivial {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns21TakesByConstReferenceERKNS_7TrivialE(trivial)
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
    pub fn TakesByConstRvalueReference<'a>(
        trivial: ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>,
    ) -> ::ctor::ConstRvalueReference<'a, crate::ns::Trivial> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns27TakesByConstRvalueReferenceEOKNS_7TrivialE(trivial)
        }
    }
}

// namespace ns

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN2ns7TrivialC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN2ns7TrivialC1EOS0_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::Trivial>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN2ns7TrivialaSERKS0_<'a, 'b>(
            __this: &'a mut crate::ns::Trivial,
            __param_0: &'b crate::ns::Trivial,
        ) -> &'a mut crate::ns::Trivial;
        pub(crate) unsafe fn __rust_thunk___ZN2ns7TrivialaSEOS0_<'a, 'b>(
            __this: &'a mut crate::ns::Trivial,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::Trivial>,
        ) -> &'a mut crate::ns::Trivial;
        #[link_name = "_ZN2ns7Trivial11UnqualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZN2ns7Trivial11UnqualifiedEv<'a>(
            __this: &'a mut crate::ns::Trivial,
        );
        #[link_name = "_ZNK2ns7Trivial14ConstQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK2ns7Trivial14ConstQualifiedEv<'a>(
            __this: &'a crate::ns::Trivial,
        );
        #[link_name = "_ZNR2ns7Trivial18LvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNR2ns7Trivial18LvalueRefQualifiedEv<'a>(
            __this: &'a mut crate::ns::Trivial,
        );
        #[link_name = "_ZNKR2ns7Trivial23ConstLvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKR2ns7Trivial23ConstLvalueRefQualifiedEv<'a>(
            __this: &'a crate::ns::Trivial,
        );
        #[link_name = "_ZNO2ns7Trivial18RvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNO2ns7Trivial18RvalueRefQualifiedEv<'a>(
            __this: ::ctor::RvalueReference<'a, crate::ns::Trivial>,
        );
        #[link_name = "_ZNKO2ns7Trivial23ConstRvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKO2ns7Trivial23ConstRvalueRefQualifiedEv<'a>(
            __this: ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(
            __return: *mut ::core::ffi::c_void,
            trivial: &mut crate::ns::Trivial,
        );
        #[link_name = "_ZN2ns16TakesByReferenceERNS_7TrivialE"]
        pub(crate) unsafe fn __rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE<'a>(
            trivial: &'a mut crate::ns::Trivial,
        ) -> &'a mut crate::ns::Trivial;
        #[link_name = "_ZN2ns21TakesByConstReferenceERKNS_7TrivialE"]
        pub(crate) unsafe fn __rust_thunk___ZN2ns21TakesByConstReferenceERKNS_7TrivialE<'a>(
            trivial: &'a crate::ns::Trivial,
        ) -> &'a crate::ns::Trivial;
        #[link_name = "_ZN2ns22TakesByRvalueReferenceEONS_7TrivialE"]
        pub(crate) unsafe fn __rust_thunk___ZN2ns22TakesByRvalueReferenceEONS_7TrivialE<'a>(
            trivial: ::ctor::RvalueReference<'a, crate::ns::Trivial>,
        ) -> ::ctor::RvalueReference<'a, crate::ns::Trivial>;
        #[link_name = "_ZN2ns27TakesByConstRvalueReferenceEOKNS_7TrivialE"]
        pub(crate) unsafe fn __rust_thunk___ZN2ns27TakesByConstRvalueReferenceEOKNS_7TrivialE<'a>(
            trivial: ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>,
        ) -> ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::ns::Trivial>() == 4);
    assert!(::core::mem::align_of::<crate::ns::Trivial>() == 4);
    static_assertions::assert_impl_all!(crate::ns::Trivial: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ns::Trivial: Drop);
    assert!(::core::mem::offset_of!(crate::ns::Trivial, trivial_field) == 0);
};
