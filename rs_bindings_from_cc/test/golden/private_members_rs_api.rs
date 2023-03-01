// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:private_members_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub mod test_namespace_bindings {
    /// Generated from: rs_bindings_from_cc/test/golden/private_members.h;l=11
    #[derive(Clone, Copy)]
    #[repr(C, align(4))]
    pub struct SomeClass {
        __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
        pub public_member_variable_: i32,
        /// Reason for representing this field as a blob of bytes:
        /// Types of non-public C++ fields can be elided away
        pub(crate) private_member_variable_: [::core::mem::MaybeUninit<u8>; 4],
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("SomeClass"),
        crate::test_namespace_bindings::SomeClass
    );

    /// Generated from: rs_bindings_from_cc/test/golden/private_members.h;l=11
    impl Default for SomeClass {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings9SomeClassC1Ev(&mut tmp);
                tmp.assume_init()
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/private_members.h;l=11
    impl<'b> From<::ctor::RvalueReference<'b, Self>> for SomeClass {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings9SomeClassC1EOS0_(
                    &mut tmp, __param_0,
                );
                tmp.assume_init()
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/golden/private_members.h;l=11
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

    /// Generated from: rs_bindings_from_cc/test/golden/private_members.h;l=11
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
        /// Generated from: rs_bindings_from_cc/test/golden/private_members.h;l=13
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
        /// Generated from: rs_bindings_from_cc/test/golden/private_members.h;l=14
        #[inline(always)]
        pub fn public_static_method() {
            unsafe {
                crate::detail::__rust_thunk___ZN23test_namespace_bindings9SomeClass20public_static_methodEv()
            }
        }
    }
}

// namespace test_namespace_bindings

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_PRIVATE_MEMBERS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings9SomeClassC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::test_namespace_bindings::SomeClass>,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings9SomeClassC1EOS0_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::test_namespace_bindings::SomeClass>,
            __param_0: ::ctor::RvalueReference<'b, crate::test_namespace_bindings::SomeClass>,
        );
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings9SomeClassaSERKS0_<'a, 'b>(
            __this: &'a mut crate::test_namespace_bindings::SomeClass,
            __param_0: &'b crate::test_namespace_bindings::SomeClass,
        ) -> &'a mut crate::test_namespace_bindings::SomeClass;
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings9SomeClassaSEOS0_<'a, 'b>(
            __this: &'a mut crate::test_namespace_bindings::SomeClass,
            __param_0: ::ctor::RvalueReference<'b, crate::test_namespace_bindings::SomeClass>,
        ) -> &'a mut crate::test_namespace_bindings::SomeClass;
        #[link_name = "_ZN23test_namespace_bindings9SomeClass13public_methodEv"]
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings9SomeClass13public_methodEv<'a>(
            __this: &'a mut crate::test_namespace_bindings::SomeClass,
        );
        #[link_name = "_ZN23test_namespace_bindings9SomeClass20public_static_methodEv"]
        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings9SomeClass20public_static_methodEv();
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::test_namespace_bindings::SomeClass>() == 8);
const _: () = assert!(::core::mem::align_of::<crate::test_namespace_bindings::SomeClass>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::SomeClass: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::SomeClass: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::SomeClass: Drop);
};
const _: () = assert!(
    memoffset::offset_of!(crate::test_namespace_bindings::SomeClass, public_member_variable_) == 0
);
const _: () = assert!(
    memoffset::offset_of!(crate::test_namespace_bindings::SomeClass, private_member_variable_) == 4
);
