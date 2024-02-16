// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:bitfields_cc
// Features: experimental, extern_c, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, register_tool)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C, align(4))]
#[__crubit::annotate(cc_type = "WithBitfields")]
pub struct WithBitfields {
    // f1 : 2 bits
    __bitfields0: [::core::mem::MaybeUninit<u8>; 1],
    pub f2: ::core::ffi::c_int,
    // f3 : 4 bits
    // f4 : 8 bits
    //  : 45 bits
    __bitfields2: [::core::mem::MaybeUninit<u8>; 10],
    pub f5: ::core::ffi::c_int,
    // f6 : 23 bits
    __bitfields4: [::core::mem::MaybeUninit<u8>; 3],
    /// Reason for representing this field as a blob of bytes:
    /// `[[no_unique_address]]` attribute was present.
    pub(crate) f7: [::core::mem::MaybeUninit<u8>; 1],
    // f8 : 2 bits
    __bitfields6: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for WithBitfields {}
impl !Sync for WithBitfields {}
forward_declare::unsafe_define!(forward_declare::symbol!("WithBitfields"), crate::WithBitfields);
impl WithBitfields {
    pub fn f7(&self) -> &::core::ffi::c_char {
        unsafe { &*(&self.f7 as *const _ as *const ::core::ffi::c_char) }
    }
}

impl Default for WithBitfields {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13WithBitfieldsC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for WithBitfields {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13WithBitfieldsC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for WithBitfields {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13WithBitfieldsaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for WithBitfields {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN13WithBitfieldsaSEOS_(self, __param_0);
        }
    }
}

/// This is a regression test for b/283835873 where the alignment of the
/// generated struct was wrong/missing.
#[derive(Clone, Copy)]
#[repr(C, align(4))]
#[__crubit::annotate(cc_type = "AlignmentRegressionTest")]
pub struct AlignmentRegressionTest {
    // code_point : 31 bits
    // status : 1 bits
    __bitfields0: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AlignmentRegressionTest {}
impl !Sync for AlignmentRegressionTest {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AlignmentRegressionTest"),
    crate::AlignmentRegressionTest
);

impl Default for AlignmentRegressionTest {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23AlignmentRegressionTestC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for AlignmentRegressionTest {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23AlignmentRegressionTestC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for AlignmentRegressionTest {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN23AlignmentRegressionTestaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for AlignmentRegressionTest {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN23AlignmentRegressionTestaSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for item 'AlignmentRegressionTest::(unnamed enum at ./rs_bindings_from_cc/test/golden/bitfields.h:26:3)':
// Unnamed enums are not supported yet

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN13WithBitfieldsC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::WithBitfields>,
        );
        pub(crate) fn __rust_thunk___ZN13WithBitfieldsC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::WithBitfields>,
            __param_0: ::ctor::RvalueReference<'b, crate::WithBitfields>,
        );
        pub(crate) fn __rust_thunk___ZN13WithBitfieldsaSERKS_<'a, 'b>(
            __this: &'a mut crate::WithBitfields,
            __param_0: &'b crate::WithBitfields,
        ) -> &'a mut crate::WithBitfields;
        pub(crate) fn __rust_thunk___ZN13WithBitfieldsaSEOS_<'a, 'b>(
            __this: &'a mut crate::WithBitfields,
            __param_0: ::ctor::RvalueReference<'b, crate::WithBitfields>,
        ) -> &'a mut crate::WithBitfields;
        pub(crate) fn __rust_thunk___ZN23AlignmentRegressionTestC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::AlignmentRegressionTest>,
        );
        pub(crate) fn __rust_thunk___ZN23AlignmentRegressionTestC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::AlignmentRegressionTest>,
            __param_0: ::ctor::RvalueReference<'b, crate::AlignmentRegressionTest>,
        );
        pub(crate) fn __rust_thunk___ZN23AlignmentRegressionTestaSERKS_<'a, 'b>(
            __this: &'a mut crate::AlignmentRegressionTest,
            __param_0: &'b crate::AlignmentRegressionTest,
        ) -> &'a mut crate::AlignmentRegressionTest;
        pub(crate) fn __rust_thunk___ZN23AlignmentRegressionTestaSEOS_<'a, 'b>(
            __this: &'a mut crate::AlignmentRegressionTest,
            __param_0: ::ctor::RvalueReference<'b, crate::AlignmentRegressionTest>,
        ) -> &'a mut crate::AlignmentRegressionTest;
    }
}

const _: () = assert!(::core::mem::size_of::<crate::WithBitfields>() == 32);
const _: () = assert!(::core::mem::align_of::<crate::WithBitfields>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::WithBitfields:Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::WithBitfields:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::WithBitfields:Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::WithBitfields, f2) == 4);
const _: () = assert!(memoffset::offset_of!(crate::WithBitfields, f5) == 20);
const _: () = assert!(memoffset::offset_of!(crate::WithBitfields, f7) == 27);

const _: () = assert!(::core::mem::size_of::<crate::AlignmentRegressionTest>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::AlignmentRegressionTest>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::AlignmentRegressionTest:Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AlignmentRegressionTest:Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AlignmentRegressionTest:Drop);
};
