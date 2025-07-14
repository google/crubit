// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:bitfields_cc

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

#[derive(Clone, Copy)]
#[repr(C, align(4))]
///CRUBIT_ANNOTATE: cpp_type=WithBitfields
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
unsafe impl ::cxx::ExternType for WithBitfields {
    type Id = ::cxx::type_id!("WithBitfields");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(forward_declare::symbol!("WithBitfields"), crate::WithBitfields);
impl WithBitfields {
    pub fn f7(&self) -> &::core::ffi::c_char {
        unsafe {
            let ptr = (self as *const Self as *const u8).offset(27);
            &*(ptr as *const ::core::ffi::c_char)
        }
    }
}

impl Default for WithBitfields {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13WithBitfieldsC1Ev(
                &raw mut tmp as *mut ::core::ffi::c_void,
            );
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for WithBitfields {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13WithBitfieldsC1EOS_(
                &raw mut tmp as *mut ::core::ffi::c_void,
                __param_0,
            );
            tmp.assume_init()
        }
    }
}
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for WithBitfields {
    type CtorType = Self;
    type Error = ::ctor::Infallible;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        <Self as From<::ctor::RvalueReference<'b, Self>>>::from(args)
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
///CRUBIT_ANNOTATE: cpp_type=AlignmentRegressionTest
pub struct AlignmentRegressionTest {
    // code_point : 31 bits
    // status : 1 bits
    __bitfields0: [::core::mem::MaybeUninit<u8>; 4],
}
impl !Send for AlignmentRegressionTest {}
impl !Sync for AlignmentRegressionTest {}
unsafe impl ::cxx::ExternType for AlignmentRegressionTest {
    type Id = ::cxx::type_id!("AlignmentRegressionTest");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AlignmentRegressionTest"),
    crate::AlignmentRegressionTest
);

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN23AlignmentRegressionTestC1Ev {}
impl<'error> Default for AlignmentRegressionTest
where
    &'error (): BindingFailedFor_ZN23AlignmentRegressionTestC1Ev,
{
    #[inline(always)]
    fn default() -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
    }
}

// Error while generating bindings for function 'AlignmentRegressionTest::AlignmentRegressionTest':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.

#[diagnostic::on_unimplemented(
    message = "binding generation for function failed\nUnsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347."
)]
pub trait BindingFailedFor_ZN23AlignmentRegressionTestC1EOS_ {}
impl<'error, 'b> From<::ctor::RvalueReference<'b, Self>> for AlignmentRegressionTest
where
    &'error (): BindingFailedFor_ZN23AlignmentRegressionTestC1EOS_,
{
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        #![allow(unused_variables)]
        unreachable!(
            "This impl can never be instantiated. \
                    If this message appears at runtime, please report a <internal link>."
        )
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

// Error while generating bindings for enum 'AlignmentRegressionTest::(unnamed enum at ./rs_bindings_from_cc/test/golden/bitfields.h:26:3)':
// Unnamed enums are not supported yet

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN13WithBitfieldsC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN13WithBitfieldsC1EOS_<'b>(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'b, crate::WithBitfields>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN13WithBitfieldsaSERKS_<'a, 'b>(
            __this: &'a mut crate::WithBitfields,
            __param_0: &'b crate::WithBitfields,
        ) -> &'a mut crate::WithBitfields;
        pub(crate) unsafe fn __rust_thunk___ZN13WithBitfieldsaSEOS_<'a, 'b>(
            __this: &'a mut crate::WithBitfields,
            __param_0: ::ctor::RvalueReference<'b, crate::WithBitfields>,
        ) -> &'a mut crate::WithBitfields;
        pub(crate) unsafe fn __rust_thunk___ZN23AlignmentRegressionTestaSERKS_<'a, 'b>(
            __this: &'a mut crate::AlignmentRegressionTest,
            __param_0: &'b crate::AlignmentRegressionTest,
        ) -> &'a mut crate::AlignmentRegressionTest;
        pub(crate) unsafe fn __rust_thunk___ZN23AlignmentRegressionTestaSEOS_<'a, 'b>(
            __this: &'a mut crate::AlignmentRegressionTest,
            __param_0: ::ctor::RvalueReference<'b, crate::AlignmentRegressionTest>,
        ) -> &'a mut crate::AlignmentRegressionTest;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::WithBitfields>() == 32);
    assert!(::core::mem::align_of::<crate::WithBitfields>() == 4);
    static_assertions::assert_impl_all!(crate::WithBitfields: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::WithBitfields: Drop);
    assert!(::core::mem::offset_of!(crate::WithBitfields, f2) == 4);
    assert!(::core::mem::offset_of!(crate::WithBitfields, f5) == 20);
    assert!(::core::mem::offset_of!(crate::WithBitfields, f7) == 27);
    assert!(::core::mem::size_of::<crate::AlignmentRegressionTest>() == 4);
    assert!(::core::mem::align_of::<crate::AlignmentRegressionTest>() == 4);
    static_assertions::assert_impl_all!(crate::AlignmentRegressionTest: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::AlignmentRegressionTest: Drop);
};
