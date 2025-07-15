// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/regression_401857961:repro
// Features: infer_operator_lifetimes, supported, unsafe_types

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code, unused_mut)]
#![deny(warnings)]

pub mod repro {
    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=10
    // Error while generating bindings for class 'repro::optional':
    // Class templates are not supported yet

    /// Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    #[derive(Clone, Copy)]
    #[repr(C)]
    ///CRUBIT_ANNOTATE: cpp_type=repro :: Interval
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

    /// Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    impl Default for Interval {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN5repro8IntervalC1Ev(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                );
                tmp.assume_init()
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    impl From<::ctor::RvalueReference<'_, Self>> for Interval {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'_, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN5repro8IntervalC1EOS0_(
                    &raw mut tmp as *mut ::core::ffi::c_void,
                    __param_0,
                );
                tmp.assume_init()
            }
        }
    }
    impl ::ctor::CtorNew<::ctor::RvalueReference<'_, Self>> for Interval {
        type CtorType = Self;
        type Error = ::ctor::Infallible;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'_, Self>) -> Self::CtorType {
            <Self as From<::ctor::RvalueReference<'_, Self>>>::from(args)
        }
    }

    /// Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    impl ::ctor::UnpinAssign<&Self> for Interval {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: &Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN5repro8IntervalaSERKS0_(self, __param_0);
            }
        }
    }

    /// Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    impl ::ctor::UnpinAssign<::ctor::RvalueReference<'_, Self>> for Interval {
        #[inline(always)]
        fn unpin_assign(&mut self, __param_0: ::ctor::RvalueReference<'_, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN5repro8IntervalaSEOS0_(self, __param_0);
            }
        }
    }

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=19
    // Error while generating bindings for class 'repro::Nullable':
    // Class templates are not supported yet

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=24
    // Error while generating bindings for function 'crash':
    // Can't generate bindings for crash, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:wrapper] for crash (the type of __param_0 (parameter #0): error: Can't generate bindings for repro::Nullable<repro::Interval>, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:wrapper] for repro::Nullable<repro::Interval> (crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE is a template instantiation)
    // //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:wrapper] for repro::Nullable<repro::Interval> (crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE is a template instantiation))
}

// namespace repro

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_REGRESSION_401857961_REPRO_H_

// Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=11
// Error while generating bindings for struct 'repro::optional<repro::Interval>':
// Can't generate bindings for repro::optional<repro::Interval>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:wrapper] for repro::optional<repro::Interval> (crate::__CcTemplateInstN5repro8optionalINS_8IntervalEEE is a template instantiation)
// //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:wrapper] for repro::optional<repro::Interval> (crate::__CcTemplateInstN5repro8optionalINS_8IntervalEEE is a template instantiation)

// Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=20
// Error while generating bindings for struct 'repro::Nullable<repro::Interval>':
// Can't generate bindings for repro::Nullable<repro::Interval>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:wrapper] for repro::Nullable<repro::Interval> (crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE is a template instantiation)
// //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:wrapper] for repro::Nullable<repro::Interval> (crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE is a template instantiation)

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___ZN5repro8IntervalC1Ev(__this: *mut ::core::ffi::c_void);
        pub(crate) unsafe fn __rust_thunk___ZN5repro8IntervalC1EOS0_(
            __this: *mut ::core::ffi::c_void,
            __param_0: ::ctor::RvalueReference<'_, crate::repro::Interval>,
        );
        pub(crate) unsafe fn __rust_thunk___ZN5repro8IntervalaSERKS0_<'__return_lifetime>(
            __this: &mut crate::repro::Interval,
            __param_0: &crate::repro::Interval,
        ) -> &'__return_lifetime mut crate::repro::Interval;
        pub(crate) unsafe fn __rust_thunk___ZN5repro8IntervalaSEOS0_<'__return_lifetime>(
            __this: &mut crate::repro::Interval,
            __param_0: ::ctor::RvalueReference<'_, crate::repro::Interval>,
        ) -> &'__return_lifetime mut crate::repro::Interval;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::repro::Interval>() == 1);
    assert!(::core::mem::align_of::<crate::repro::Interval>() == 1);
    static_assertions::assert_impl_all!(crate::repro::Interval: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::repro::Interval: Drop);
    assert!(::core::mem::offset_of!(crate::repro::Interval, nanos) == 0);
};
