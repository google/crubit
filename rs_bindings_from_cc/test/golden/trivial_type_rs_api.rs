// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:trivial_type_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![deny(warnings)]

pub mod ns {
    /// Implicitly defined special member functions are trivial on a struct with
    /// only trivial members.
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=ns :: Trivial
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
        pub fn Unqualified<'a>(&'a mut self) {
            unsafe { crate::detail::__rust_thunk___ZN2ns7Trivial11UnqualifiedEv(self) }
        }
        #[inline(always)]
        pub fn ConstQualified<'a>(&'a self) {
            unsafe { crate::detail::__rust_thunk___ZNK2ns7Trivial14ConstQualifiedEv(self) }
        }
        #[inline(always)]
        pub fn LvalueRefQualified<'a>(&'a mut self) {
            unsafe { crate::detail::__rust_thunk___ZNR2ns7Trivial18LvalueRefQualifiedEv(self) }
        }
        #[inline(always)]
        pub fn ConstLvalueRefQualified<'a>(&'a self) {
            unsafe {
                crate::detail::__rust_thunk___ZNKR2ns7Trivial23ConstLvalueRefQualifiedEv(self)
            }
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

    // error: constructor `ns::Trivial::Trivial` could not be bound
    //   Unsupported parameter #1 (__param_0): references are not yet supported

    // error: constructor `ns::Trivial::Trivial` could not be bound
    //   Unsupported parameter #1 (__param_0): references are not yet supported

    // error: function `ns::Trivial::operator=` could not be bound
    //   Unsupported return type: references are not yet supported
    //   Unsupported parameter #1 (__param_0): references are not yet supported

    // error: function `ns::Trivial::operator=` could not be bound
    //   Unsupported return type: references are not yet supported
    //   Unsupported parameter #1 (__param_0): references are not yet supported

    // error: function `ns::Trivial::RvalueRefQualified` could not be bound
    //   Unsupported parameter #0 (__this): references are not yet supported

    // error: function `ns::Trivial::ConstRvalueRefQualified` could not be bound
    //   Unsupported parameter #0 (__this): references are not yet supported

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

    // error: function `ns::TakesByReference` could not be bound
    //   Unsupported return type: references are not yet supported
    //   Unsupported parameter #0 (trivial): references are not yet supported

    // error: function `ns::TakesByConstReference` could not be bound
    //   Unsupported return type: references are not yet supported
    //   Unsupported parameter #0 (trivial): references are not yet supported

    // error: function `ns::TakesByRvalueReference` could not be bound
    //   Unsupported return type: references are not yet supported
    //   Unsupported parameter #0 (trivial): references are not yet supported

    // error: function `ns::TakesByConstRvalueReference` could not be bound
    //   Unsupported return type: references are not yet supported
    //   Unsupported parameter #0 (trivial): references are not yet supported
}

// namespace ns

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN2ns7TrivialC1Ev(__this: *mut ::core::ffi::c_void);
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
        pub(crate) unsafe fn __rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(
            __return: *mut ::core::ffi::c_void,
            trivial: &mut crate::ns::Trivial,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::ns::Trivial>() == 4);
    assert!(::core::mem::align_of::<crate::ns::Trivial>() == 4);
    static_assertions::assert_impl_all!(crate::ns::Trivial: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::ns::Trivial: Drop);
    assert!(::core::mem::offset_of!(crate::ns::Trivial, trivial_field) == 0);
};
