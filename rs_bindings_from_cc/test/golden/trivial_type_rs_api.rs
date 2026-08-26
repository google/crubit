// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:trivial_type_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod ns {
    /// Implicitly defined special member functions are trivial on a struct with
    /// only trivial members.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N2ns7TrivialE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=ns :: Trivial
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Trivial {
        pub trivial_field: ::ffi_11::c_int,
    }
    impl !Send for Trivial {}
    impl !Sync for Trivial {}
    unsafe impl ::cxx::ExternType for Trivial {
        type Id = ::cxx::type_id!("ns :: Trivial");
        type Kind = ::cxx::kind::Trivial;
    }
    impl Trivial {
        #[inline(always)]
        pub fn Unqualified<'__this>(&'__this mut self) {
            unsafe { self::trivial::Unqualified(self) }
        }
        #[inline(always)]
        pub fn ConstQualified<'__this>(&'__this self) {
            unsafe { self::trivial::ConstQualified(self) }
        }
        #[inline(always)]
        pub fn LvalueRefQualified<'__this>(&'__this mut self) {
            unsafe { self::trivial::LvalueRefQualified(self) }
        }
        #[inline(always)]
        pub fn ConstLvalueRefQualified<'__this>(&'__this self) {
            unsafe { self::trivial::ConstLvalueRefQualified(self) }
        }
        #[inline(always)]
        pub fn RvalueRefQualified<'__this>(&'__this mut self) {
            unsafe { self::trivial::RvalueRefQualified(self) }
        }
        #[inline(always)]
        pub fn ConstRvalueRefQualified<'__this>(&'__this self) {
            unsafe { self::trivial::ConstRvalueRefQualified(self) }
        }
    }

    impl Default for Trivial {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    pub mod trivial {
        #[inline(always)]
        pub(crate) fn Unqualified<'__this>(__this: &'__this mut crate::ns::Trivial) {
            unsafe { crate::detail::__rust_thunk___ZN2ns7Trivial11UnqualifiedEv(__this) }
        }
        #[inline(always)]
        pub(crate) fn ConstQualified<'__this>(__this: &'__this crate::ns::Trivial) {
            unsafe { crate::detail::__rust_thunk___ZNK2ns7Trivial14ConstQualifiedEv(__this) }
        }
        #[inline(always)]
        pub(crate) fn LvalueRefQualified<'__this>(__this: &'__this mut crate::ns::Trivial) {
            unsafe { crate::detail::__rust_thunk___ZNR2ns7Trivial18LvalueRefQualifiedEv(__this) }
        }
        #[inline(always)]
        pub(crate) fn ConstLvalueRefQualified<'__this>(__this: &'__this crate::ns::Trivial) {
            unsafe {
                crate::detail::__rust_thunk___ZNKR2ns7Trivial23ConstLvalueRefQualifiedEv(__this)
            }
        }
        #[inline(always)]
        pub(crate) fn RvalueRefQualified<'__this>(__this: &'__this mut crate::ns::Trivial) {
            unsafe { crate::detail::__rust_thunk___ZNO2ns7Trivial18RvalueRefQualifiedEv(__this) }
        }
        #[inline(always)]
        pub(crate) fn ConstRvalueRefQualified<'__this>(__this: &'__this crate::ns::Trivial) {
            unsafe {
                crate::detail::__rust_thunk___ZNKO2ns7Trivial23ConstRvalueRefQualifiedEv(__this)
            }
        }
    }

    #[inline(always)]
    pub fn TakesByValue(mut trivial: crate::ns::Trivial) -> crate::ns::Trivial {
        unsafe {
            let mut __crubit_return = ::core::mem::MaybeUninit::<crate::ns::Trivial>::uninit();
            crate::detail::__rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(
                &raw mut __crubit_return as *mut ::core::ffi::c_void,
                &mut trivial,
            );
            __crubit_return.assume_init()
        }
    }

    #[inline(always)]
    pub fn TakesByReference<'trivial>(
        trivial: &'trivial mut crate::ns::Trivial,
    ) -> ::cref::CMut<'trivial, crate::ns::Trivial> {
        unsafe { crate::detail::__rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE(trivial) }
    }

    #[inline(always)]
    pub fn TakesByConstReference<'trivial>(
        trivial: &'trivial crate::ns::Trivial,
    ) -> ::cref::CRef<'trivial, crate::ns::Trivial> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns21TakesByConstReferenceERKNS_7TrivialE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesByRvalueReference(
        trivial: ::ctor::RvalueReference<'_, crate::ns::Trivial>,
    ) -> ::ctor::RvalueReference<'_, crate::ns::Trivial> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns22TakesByRvalueReferenceEONS_7TrivialE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesByConstRvalueReference(
        trivial: ::ctor::ConstRvalueReference<'_, crate::ns::Trivial>,
    ) -> ::ctor::ConstRvalueReference<'_, crate::ns::Trivial> {
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
        #[link_name = "_ZN2ns7Trivial11UnqualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZN2ns7Trivial11UnqualifiedEv<'__this>(
            __this: &'__this mut crate::ns::Trivial,
        );
        #[link_name = "_ZNK2ns7Trivial14ConstQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNK2ns7Trivial14ConstQualifiedEv<'__this>(
            __this: &'__this crate::ns::Trivial,
        );
        #[link_name = "_ZNR2ns7Trivial18LvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNR2ns7Trivial18LvalueRefQualifiedEv<'__this>(
            __this: &'__this mut crate::ns::Trivial,
        );
        #[link_name = "_ZNKR2ns7Trivial23ConstLvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKR2ns7Trivial23ConstLvalueRefQualifiedEv<'__this>(
            __this: &'__this crate::ns::Trivial,
        );
        #[link_name = "_ZNO2ns7Trivial18RvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNO2ns7Trivial18RvalueRefQualifiedEv<'__this>(
            __this: &'__this mut crate::ns::Trivial,
        );
        #[link_name = "_ZNKO2ns7Trivial23ConstRvalueRefQualifiedEv"]
        pub(crate) unsafe fn __rust_thunk___ZNKO2ns7Trivial23ConstRvalueRefQualifiedEv<'__this>(
            __this: &'__this crate::ns::Trivial,
        );
        pub(crate) unsafe fn __rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(
            __return: *mut ::core::ffi::c_void,
            trivial: &mut crate::ns::Trivial,
        );
        #[link_name = "_ZN2ns16TakesByReferenceERNS_7TrivialE"]
        pub(crate) unsafe fn __rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE<'trivial>(
            trivial: &'trivial mut crate::ns::Trivial,
        ) -> ::cref::CMut<'trivial, crate::ns::Trivial>;
        #[link_name = "_ZN2ns21TakesByConstReferenceERKNS_7TrivialE"]
        pub(crate) unsafe fn __rust_thunk___ZN2ns21TakesByConstReferenceERKNS_7TrivialE<'trivial>(
            trivial: &'trivial crate::ns::Trivial,
        ) -> ::cref::CRef<'trivial, crate::ns::Trivial>;
        #[link_name = "_ZN2ns22TakesByRvalueReferenceEONS_7TrivialE"]
        pub(crate) unsafe fn __rust_thunk___ZN2ns22TakesByRvalueReferenceEONS_7TrivialE(
            trivial: ::ctor::RvalueReference<'_, crate::ns::Trivial>,
        ) -> ::ctor::RvalueReference<'_, crate::ns::Trivial>;
        #[link_name = "_ZN2ns27TakesByConstRvalueReferenceEOKNS_7TrivialE"]
        pub(crate) unsafe fn __rust_thunk___ZN2ns27TakesByConstRvalueReferenceEOKNS_7TrivialE(
            trivial: ::ctor::ConstRvalueReference<'_, crate::ns::Trivial>,
        ) -> ::ctor::ConstRvalueReference<'_, crate::ns::Trivial>;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::ns::Trivial>() == 4);
    assert!(::core::mem::align_of::<crate::ns::Trivial>() == 4);
    static_assertions::assert_impl_all!(crate::ns::Trivial: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ns::Trivial: Drop);
    assert!(::core::mem::offset_of!(crate::ns::Trivial, trivial_field) == 0);
};
