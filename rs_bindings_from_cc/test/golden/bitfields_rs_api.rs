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

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

impl Default for WithBitfields {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13WithBitfieldsC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'WithBitfields::WithBitfields':
// Can't generate bindings for WithBitfields::WithBitfields, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:bitfields_cc needs [//features:experimental] for WithBitfields::WithBitfields (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'WithBitfields::WithBitfields':
// Can't generate bindings for WithBitfields::WithBitfields, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:bitfields_cc needs [//features:experimental] for WithBitfields::WithBitfields (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'WithBitfields::operator=':
// Can't generate bindings for WithBitfields::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:bitfields_cc needs [//features:experimental] for WithBitfields::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:bitfields_cc needs [//features:experimental] for WithBitfields::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'WithBitfields::operator=':
// Can't generate bindings for WithBitfields::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:bitfields_cc needs [//features:experimental] for WithBitfields::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:bitfields_cc needs [//features:experimental] for WithBitfields::operator= (the type of __param_0 (parameter #1): references are not supported)

/// This is a regression test for b/283835873 where the alignment of the
/// generated struct was wrong/missing.
#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

impl Default for AlignmentRegressionTest {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23AlignmentRegressionTestC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for constructor 'AlignmentRegressionTest::AlignmentRegressionTest':
// Can't generate bindings for AlignmentRegressionTest::AlignmentRegressionTest, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:bitfields_cc needs [//features:experimental] for AlignmentRegressionTest::AlignmentRegressionTest (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for constructor 'AlignmentRegressionTest::AlignmentRegressionTest':
// Can't generate bindings for AlignmentRegressionTest::AlignmentRegressionTest, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:bitfields_cc needs [//features:experimental] for AlignmentRegressionTest::AlignmentRegressionTest (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AlignmentRegressionTest::operator=':
// Can't generate bindings for AlignmentRegressionTest::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:bitfields_cc needs [//features:experimental] for AlignmentRegressionTest::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:bitfields_cc needs [//features:experimental] for AlignmentRegressionTest::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for function 'AlignmentRegressionTest::operator=':
// Can't generate bindings for AlignmentRegressionTest::operator=, because of missing required features (crubit.rs-features):
// //rs_bindings_from_cc/test/golden:bitfields_cc needs [//features:experimental] for AlignmentRegressionTest::operator= (return type: references are not supported)
// //rs_bindings_from_cc/test/golden:bitfields_cc needs [//features:experimental] for AlignmentRegressionTest::operator= (the type of __param_0 (parameter #1): references are not supported)

// Error while generating bindings for enum 'AlignmentRegressionTest::(unnamed enum at ./rs_bindings_from_cc/test/golden/bitfields.h:26:3)':
// Unnamed enums are not supported yet

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN13WithBitfieldsC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN23AlignmentRegressionTestC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
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
