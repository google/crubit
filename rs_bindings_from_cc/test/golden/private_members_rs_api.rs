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
    #[derive(Clone, Copy)]
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
    forward_declare::unsafe_define!(
        forward_declare::symbol!("test_namespace_bindings :: SomeClass"),
        crate::test_namespace_bindings::SomeClass
    );

    impl Default for SomeClass {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings9SomeClassC1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    impl<'b> From<::ctor::RvalueReference<'b, Self>> for SomeClass {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings9SomeClassC1EOS0_(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                    __param_0,
                );
                tmp.assume_init()
            }
        }
    }
    impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for SomeClass {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
        }
    }

    impl<'b> ::ctor::UnpinAssign<&'b Self> for SomeClass {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings9SomeClassaSERKS0_(
                    self, __param_0,
                );
            }
        }
    }

    impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SomeClass {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings9SomeClassaSEOS0_(
                    self, __param_0,
                );
            }
        }
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
    }

    impl SomeClass {
        #[inline(always)]
        pub fn public_static_method() {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings9SomeClass20public_static_methodEv()
            }
        }
    }
}

// namespace test_namespace_bindings

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings9SomeClassC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings9SomeClassC1EOS0_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::test_namespace_bindings::SomeClass>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings9SomeClassaSERKS0_<'a, 'b>(
            __this: &'a mut crate::test_namespace_bindings::SomeClass,
            __param_0: &'b crate::test_namespace_bindings::SomeClass,
        ) -> &'a mut crate::test_namespace_bindings::SomeClass;
        pub(crate) unsafe fn __rust_thunk___ZN23test_namespace_bindings9SomeClassaSEOS0_<'a, 'b>(
            __this: &'a mut crate::test_namespace_bindings::SomeClass,
            __param_0: ::ctor::RvalueReference<'b, crate::test_namespace_bindings::SomeClass>,
        ) -> &'a mut crate::test_namespace_bindings::SomeClass;
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
