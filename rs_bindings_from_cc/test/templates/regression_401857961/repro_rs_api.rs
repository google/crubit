// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/regression_401857961:repro

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
pub mod repro {
    // error: class `repro::optional` could not be bound
    //   Class templates are not yet supported

    /// # Safety
    ///
    /// To call a function that accepts this type, you must uphold these requirements:
    /// * Document why the following public unsafe fields of this type cannot be misused by callee:
    ///   * `nanos`: Rust type is unknown; safety requirements cannot be automatically generated: Unsupported type 'char[1]': Unsupported clang::Type class 'ConstantArray'
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
    #[cfi_encoding = "N5repro8IntervalE"]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=repro :: Interval
    ///CRUBIT_ANNOTATE: cpp_move_constructible=
    pub struct Interval {
        /// Reason for representing this field as a blob of bytes:
        /// Unsupported type 'char[1]': Unsupported clang::Type class 'ConstantArray'
        pub(crate) nanos: [::core::mem::MaybeUninit<u8>; 1],
    }
    impl !Send for Interval {}
    impl !Sync for Interval {}
    unsafe impl ::cxx::ExternType for Interval {
        type Id = ::cxx::type_id!("repro :: Interval");
        type Kind = ::cxx::kind::Trivial;
    }

    impl Default for Interval {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN5repro8IntervalC1Ev(&raw mut tmp as *mut _);
                tmp.assume_init()
            }
        }
    }

    // error: class `repro::Nullable` could not be bound
    //   Class templates are not yet supported

    #[inline(always)]
    pub fn crash(mut __param_0: crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE) {
        unsafe {
            crate::detail::__rust_thunk___ZN5repro5crashENS_8NullableINS_8IntervalEEE(
                &mut __param_0,
            )
        }
    }
}

// namespace repro

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_REGRESSION_401857961_REPRO_H_

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "__CcTemplateInstN5repro8NullableINS_8IntervalEEE"]
#[repr(C)]
///CRUBIT_ANNOTATE: cpp_type=repro :: Nullable < repro :: Interval >
///CRUBIT_ANNOTATE: cpp_move_constructible=
pub struct __CcTemplateInstN5repro8NullableINS_8IntervalEEE {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for __CcTemplateInstN5repro8NullableINS_8IntervalEEE {}
impl !Sync for __CcTemplateInstN5repro8NullableINS_8IntervalEEE {}

impl Default for __CcTemplateInstN5repro8NullableINS_8IntervalEEE {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk__c2cb6ead__ZN5repro8NullableINS_8IntervalEEC1Ev(
                &raw mut tmp as *mut _,
            );
            tmp.assume_init()
        }
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN5repro8IntervalC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN5repro5crashENS_8NullableINS_8IntervalEEE(
            __param_0: &mut crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE,
        );
        pub(crate) unsafe fn __rust_thunk__c2cb6ead__ZN5repro8NullableINS_8IntervalEEC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::repro::Interval>() == 1);
    assert!(::core::mem::align_of::<crate::repro::Interval>() == 1);
    static_assertions::assert_impl_all!(crate::repro::Interval: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::repro::Interval: Drop);
    assert!(::core::mem::offset_of!(crate::repro::Interval, nanos) == 0);
    assert!(::core::mem::size_of::<crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE>() == 1);
    assert!(
        ::core::mem::align_of::<crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE>() == 1
    );
    static_assertions::assert_impl_all!(crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE: Drop);
};
