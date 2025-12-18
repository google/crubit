// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:trivial_type_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
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
        pub trivial_field: ::core::ffi::c_int,
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

    // Error while generating bindings for constructor 'Trivial::Trivial':
    // Can't generate bindings for Trivial::Trivial, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for Trivial::Trivial (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for constructor 'Trivial::Trivial':
    // Can't generate bindings for Trivial::Trivial, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for Trivial::Trivial (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'Trivial::operator=':
    // Can't generate bindings for Trivial::operator=, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for Trivial::operator= (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for Trivial::operator= (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'Trivial::operator=':
    // Can't generate bindings for Trivial::operator=, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for Trivial::operator= (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for Trivial::operator= (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'Trivial::RvalueRefQualified':
    // Can't generate bindings for Trivial::RvalueRefQualified, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for Trivial::RvalueRefQualified (the type of __this (parameter #0): references are not supported)

    // Error while generating bindings for function 'Trivial::ConstRvalueRefQualified':
    // Can't generate bindings for Trivial::ConstRvalueRefQualified, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for Trivial::ConstRvalueRefQualified (the type of __this (parameter #0): references are not supported)

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

    // Error while generating bindings for function 'TakesByReference':
    // Can't generate bindings for TakesByReference, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for TakesByReference (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for TakesByReference (the type of trivial (parameter #0): references are not supported)

    // Error while generating bindings for function 'TakesByConstReference':
    // Can't generate bindings for TakesByConstReference, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for TakesByConstReference (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for TakesByConstReference (the type of trivial (parameter #0): references are not supported)

    // Error while generating bindings for function 'TakesByRvalueReference':
    // Can't generate bindings for TakesByRvalueReference, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for TakesByRvalueReference (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for TakesByRvalueReference (the type of trivial (parameter #0): references are not supported)

    // Error while generating bindings for function 'TakesByConstRvalueReference':
    // Can't generate bindings for TakesByConstRvalueReference, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for TakesByConstRvalueReference (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:trivial_type_cc needs [//features:experimental] for TakesByConstRvalueReference (the type of trivial (parameter #0): references are not supported)
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
