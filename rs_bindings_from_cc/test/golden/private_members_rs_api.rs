// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:private_members_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod test_namespace_bindings {
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[repr(C, align(4))]
    ///CRUBIT_ANNOTATE: cpp_type=test_namespace_bindings :: SomeClass
    pub struct SomeClass {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        pub public_member_variable_: ::core::ffi::c_int,
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) private_member_variable_: [::core::mem::MaybeUninit<u8>; 4],
    }
    impl !Send for SomeClass {}
    impl !Sync for SomeClass {}
    unsafe impl ::cxx::ExternType for SomeClass {
        type Id = ::cxx::type_id!("test_namespace_bindings :: SomeClass");
        type Kind = ::cxx::kind::Trivial;
    }
    impl SomeClass {
        #[inline(always)]
        pub fn public_method<'a>(&'a mut self) {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings9SomeClass13public_methodEv(
                    self,
                )
            }
        }
        #[inline(always)]
        pub fn public_static_method() {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings9SomeClass20public_static_methodEv()
            }
        }
    }

    impl Default for SomeClass {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings9SomeClassC1Ev(
                    &raw mut tmp as *mut _,
                );
                tmp.assume_init()
            }
        }
    }

    // Error while generating bindings for constructor 'SomeClass::SomeClass':
    // Can't generate bindings for SomeClass::SomeClass, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:private_members_cc needs [//features:experimental] for SomeClass::SomeClass (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for constructor 'SomeClass::SomeClass':
    // Can't generate bindings for SomeClass::SomeClass, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:private_members_cc needs [//features:experimental] for SomeClass::SomeClass (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'SomeClass::operator=':
    // Can't generate bindings for SomeClass::operator=, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:private_members_cc needs [//features:experimental] for SomeClass::operator= (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:private_members_cc needs [//features:experimental] for SomeClass::operator= (the type of __param_0 (parameter #1): references are not supported)

    // Error while generating bindings for function 'SomeClass::operator=':
    // Can't generate bindings for SomeClass::operator=, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/golden:private_members_cc needs [//features:experimental] for SomeClass::operator= (return type: references are not supported)
    // //rs_bindings_from_cc/test/golden:private_members_cc needs [//features:experimental] for SomeClass::operator= (the type of __param_0 (parameter #1): references are not supported)
}

// namespace test_namespace_bindings

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings9SomeClassC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        #[link_name = "_ZN23test_namespace_bindings9SomeClass13public_methodEv"]
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings9SomeClass13public_methodEv<
            'a,
        >(
            __this: &'a mut crate::test_namespace_bindings::SomeClass,
        );
        #[link_name = "_ZN23test_namespace_bindings9SomeClass20public_static_methodEv"]
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings9SomeClass20public_static_methodEv(
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::test_namespace_bindings::SomeClass>() == 8);
    assert!(::core::mem::align_of::<crate::test_namespace_bindings::SomeClass>() == 4);
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::SomeClass: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::SomeClass: Drop);
    assert!(
        ::core::mem::offset_of!(crate::test_namespace_bindings::SomeClass, public_member_variable_)
            == 0
    );
    assert!(
        ::core::mem::offset_of!(
            crate::test_namespace_bindings::SomeClass,
            private_member_variable_
        ) == 4
    );
};
