// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/regression_401857961:repro
// Features: supported

#![rustfmt::skip]
#![feature(allocator_api, cfg_sanitize, custom_inner_attributes, negative_impls)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(dead_code)]
#![deny(warnings)]

pub mod repro {
    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=10
    // Error while generating bindings for item 'repro::optional':
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

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    // Error while generating bindings for item 'Interval::Interval':
    // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
    // Expected first constructor parameter to be a mutable reference, got: *mut crate::repro::Interval
    // Missing lifetime for `__this` parameter type: *mut crate::repro::Interval

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    // Error while generating bindings for item 'Interval::Interval':
    // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported. See b/216648347.
    // Expected first constructor parameter to be a mutable reference, got: *mut crate::repro::Interval
    // Missing lifetime for `__this` parameter type: *mut crate::repro::Interval

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    // Error while generating bindings for item 'repro::Interval::Interval':
    // Parameter #0 is not supported: Unsupported type 'Interval &&': Unsupported type: && without lifetime

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    // Error while generating bindings for item 'Interval::operator=':
    // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    // Error while generating bindings for item 'repro::Interval::operator=':
    // Parameter #0 is not supported: Unsupported type 'Interval &&': Unsupported type: && without lifetime

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=19
    // Error while generating bindings for item 'repro::Nullable':
    // Class templates are not supported yet

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=24
    // Error while generating bindings for item 'crash':
    // Failed to format type of parameter 0: Can't generate bindings for repro::Nullable<repro::Interval>, because of missing required features (<internal link>):
    // //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:experimental] for repro::Nullable<repro::Interval> (crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE is a template instantiation)
    // //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:experimental] for repro::Nullable<repro::Interval> (crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE is a template instantiation)
}

// namespace repro

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_REGRESSION_401857961_REPRO_H_

// Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=11
// Error while generating bindings for item 'repro::optional<repro::Interval>':
// Can't generate bindings for repro::optional<repro::Interval>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:experimental] for repro::optional<repro::Interval> (crate::__CcTemplateInstN5repro8optionalINS_8IntervalEEE is a template instantiation)
// //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:experimental] for repro::optional<repro::Interval> (crate::__CcTemplateInstN5repro8optionalINS_8IntervalEEE is a template instantiation)

// Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=20
// Error while generating bindings for item 'repro::Nullable<repro::Interval>':
// Can't generate bindings for repro::Nullable<repro::Interval>, because of missing required features (<internal link>):
// //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:experimental] for repro::Nullable<repro::Interval> (crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE is a template instantiation)
// //rs_bindings_from_cc/test/templates/regression_401857961:repro needs [//features:experimental] for repro::Nullable<repro::Interval> (crate::__CcTemplateInstN5repro8NullableINS_8IntervalEEE is a template instantiation)

const _: () = {
    assert!(::core::mem::size_of::<crate::repro::Interval>() == 1);
    assert!(::core::mem::align_of::<crate::repro::Interval>() == 1);
    static_assertions::assert_impl_all!(crate::repro::Interval: Clone);
    static_assertions::assert_impl_all!(crate::repro::Interval: Copy);
    static_assertions::assert_not_impl_any!(crate::repro::Interval: Drop);
    assert!(::core::mem::offset_of!(crate::repro::Interval, nanos) == 0);
};
