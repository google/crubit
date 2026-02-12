// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:trivial_type_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
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

    // Error while generating bindings for constructor 'ns::Trivial::Trivial':
    // Can't generate bindings for ns::Trivial::Trivial, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::Trivial::Trivial (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for constructor 'ns::Trivial::Trivial':
    // Can't generate bindings for ns::Trivial::Trivial, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::Trivial::Trivial (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'ns::Trivial::operator=':
    // Can't generate bindings for ns::Trivial::operator=, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::Trivial::operator= (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::Trivial::operator= (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'ns::Trivial::operator=':
    // Can't generate bindings for ns::Trivial::operator=, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::Trivial::operator= (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::Trivial::operator= (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'ns::Trivial::RvalueRefQualified':
    // Can't generate bindings for ns::Trivial::RvalueRefQualified, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::Trivial::RvalueRefQualified (the type of __this (parameter #0): references are not supported)

    // Error while generating bindings for function 'ns::Trivial::ConstRvalueRefQualified':
    // Can't generate bindings for ns::Trivial::ConstRvalueRefQualified, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::Trivial::ConstRvalueRefQualified (the type of __this (parameter #0): references are not supported)

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

    // Error while generating bindings for function 'ns::TakesByReference':
    // Can't generate bindings for ns::TakesByReference, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::TakesByReference (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::TakesByReference (the type of trivial (parameter #0): references are not supported)

    // Error while generating bindings for function 'ns::TakesByConstReference':
    // Can't generate bindings for ns::TakesByConstReference, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::TakesByConstReference (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::TakesByConstReference (the type of trivial (parameter #0): references are not supported)

    // Error while generating bindings for function 'ns::TakesByRvalueReference':
    // Can't generate bindings for ns::TakesByRvalueReference, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::TakesByRvalueReference (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::TakesByRvalueReference (the type of trivial (parameter #0): references are not supported)

    // Error while generating bindings for function 'ns::TakesByConstRvalueReference':
    // Can't generate bindings for ns::TakesByConstRvalueReference, because of missing required features (crubit.rs-features):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::TakesByConstRvalueReference (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for ns::TakesByConstRvalueReference (the type of trivial (parameter #0): references are not supported)
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
