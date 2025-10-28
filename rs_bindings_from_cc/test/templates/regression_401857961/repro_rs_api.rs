// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/regression_401857961:repro
// Features: do_not_hardcode_status_bridge, non_unpin_ctor, std_unique_ptr, std_vector, supported

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
    #[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
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

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    // Error while generating bindings for constructor 'Interval::Interval':
    // Default constructors do yet receive bindings. See b/452726517.
    // Expected first constructor parameter to be a mutable reference, got: *mut crate::repro::Interval
    // Expected first reference parameter `__this` to have a lifetime, found *mut crate::repro::Interval

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    // Error while generating bindings for constructor 'Interval::Interval':
    // Move and copy constructors do yet receive bindings. See b/452726517.
    // Expected first constructor parameter to be a mutable reference, got: *mut crate::repro::Interval
    // Expected first reference parameter `__this` to have a lifetime, found *mut crate::repro::Interval

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    // Error while generating bindings for constructor 'Interval::Interval':
    // Move and copy constructors do yet receive bindings. See b/452726517.
    // Expected first constructor parameter to be a mutable reference, got: *mut crate::repro::Interval
    // Expected first reference parameter `__this` to have a lifetime, found *mut crate::repro::Interval

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    // Error while generating bindings for function 'Interval::operator=':
    // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

    // Generated from: rs_bindings_from_cc/test/templates/regression_401857961/repro.h;l=15
    // Error while generating bindings for function 'Interval::operator=':
    // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

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

const _: () = {
    assert!(::core::mem::size_of::<crate::repro::Interval>() == 1);
    assert!(::core::mem::align_of::<crate::repro::Interval>() == 1);
    static_assertions::assert_impl_all!(crate::repro::Interval: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::repro::Interval: Drop);
    assert!(::core::mem::offset_of!(crate::repro::Interval, nanos) == 0);
};
