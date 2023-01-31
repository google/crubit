// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_source_order_cc

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

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=8
// Error while generating bindings for item 'MyTemplate':
// Class templates are not supported yet

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=16
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TopLevel {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("TopLevel"), crate::TopLevel);

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=16
// Error while generating bindings for item 'TopLevel::TopLevel':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=16
// Error while generating bindings for item 'TopLevel::TopLevel':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=16
// Error while generating bindings for item 'TopLevel::TopLevel':
// Parameter #0 is not supported: Unsupported type 'TopLevel &&': Unsupported type: && without lifetime

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=16
// Error while generating bindings for item 'TopLevel::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=16
// Error while generating bindings for item 'TopLevel::operator=':
// Parameter #0 is not supported: Unsupported type 'TopLevel &&': Unsupported type: && without lifetime

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=18
pub type Alias1 = crate::__CcTemplateInst10MyTemplateIiE;

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=19
pub type Alias2 = crate::__CcTemplateInst10MyTemplateIfE;

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=20
pub type Alias3 = crate::__CcTemplateInst10MyTemplateI8TopLevelE;

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=21
pub type Alias4 = crate::__CcTemplateInst10MyTemplateIdE;

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=22
pub type Alias5 = crate::__CcTemplateInst10MyTemplateIbE;

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=23
pub type Alias6 = crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE;

pub mod test_namespace_bindings {
    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=26
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct Inner {
        __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("Inner"),
        crate::test_namespace_bindings::Inner
    );

    // Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=26
    // Error while generating bindings for item 'Inner::Inner':
    // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

    // Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=26
    // Error while generating bindings for item 'Inner::Inner':
    // Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

    // Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=26
    // Error while generating bindings for item 'test_namespace_bindings::Inner::Inner':
    // Parameter #0 is not supported: Unsupported type 'Inner &&': Unsupported type: && without lifetime

    // Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=26
    // Error while generating bindings for item 'Inner::operator=':
    // `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

    // Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=26
    // Error while generating bindings for item 'test_namespace_bindings::Inner::operator=':
    // Parameter #0 is not supported: Unsupported type 'Inner &&': Unsupported type: && without lifetime

    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=27
    pub type Alias7 = crate::__CcTemplateInst10MyTemplateIcE;

    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=28
    pub type Alias8 = crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE;

    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=29
    pub type Alias9 = crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE;
}

// namespace test_namespace_bindings

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_SOURCE_ORDER_H_

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInst10MyTemplateI8TopLevelE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<TopLevel>"),
    crate::__CcTemplateInst10MyTemplateI8TopLevelE
);

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<TopLevel>::MyTemplate<TopLevel>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<TopLevel>::MyTemplate<TopLevel>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<TopLevel>::MyTemplate':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<TopLevel> &&': Unsupported type: && without lifetime

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<TopLevel>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<TopLevel>::operator=':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<TopLevel> &&': Unsupported type: && without lifetime

impl __CcTemplateInst10MyTemplateI8TopLevelE {
    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=13
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: crate::TopLevel) {
        crate::detail::__rust_thunk___ZN10MyTemplateI8TopLevelE8processTES0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<test_namespace_bindings::Inner>"),
    crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE
);

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<test_namespace_bindings::Inner>::MyTemplate<test_namespace_bindings::Inner>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<test_namespace_bindings::Inner>::MyTemplate<test_namespace_bindings::Inner>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<test_namespace_bindings::Inner>::MyTemplate':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<Inner> &&': Unsupported type: && without lifetime

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<test_namespace_bindings::Inner>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<test_namespace_bindings::Inner>::operator=':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<Inner> &&': Unsupported type: && without lifetime

impl __CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE {
    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=13
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: crate::test_namespace_bindings::Inner) {
        crate::detail::__rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEE8processTES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInst10MyTemplateIS_I8TopLevelEE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<MyTemplate<TopLevel>>"),
    crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE
);

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<MyTemplate<TopLevel>>::MyTemplate<MyTemplate<TopLevel>>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<MyTemplate<TopLevel>>::MyTemplate<MyTemplate<TopLevel>>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<MyTemplate<TopLevel>>::MyTemplate':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<MyTemplate<TopLevel> > &&': Unsupported type: && without lifetime

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<MyTemplate<TopLevel>>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<MyTemplate<TopLevel>>::operator=':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<MyTemplate<TopLevel> > &&': Unsupported type: && without lifetime

impl __CcTemplateInst10MyTemplateIS_I8TopLevelEE {
    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=13
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: crate::__CcTemplateInst10MyTemplateI8TopLevelE) {
        crate::detail::__rust_thunk___ZN10MyTemplateIS_I8TopLevelEE8processTES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<MyTemplate<test_namespace_bindings::Inner>>"),
    crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE
);

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<MyTemplate<test_namespace_bindings::Inner>>::MyTemplate<MyTemplate<test_namespace_bindings::Inner>>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<MyTemplate<test_namespace_bindings::Inner>>::MyTemplate<MyTemplate<test_namespace_bindings::Inner>>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<MyTemplate<test_namespace_bindings::Inner>>::MyTemplate':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<MyTemplate<Inner> > &&': Unsupported type: && without lifetime

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<MyTemplate<test_namespace_bindings::Inner>>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<MyTemplate<test_namespace_bindings::Inner>>::operator=':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<MyTemplate<Inner> > &&': Unsupported type: && without lifetime

impl __CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE {
    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=13
    #[inline(always)]
    pub unsafe fn processT(
        __this: *mut Self,
        t: crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,
    ) {
        crate::detail::__rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEE8processTES2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInst10MyTemplateIbE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<bool>"),
    crate::__CcTemplateInst10MyTemplateIbE
);

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<bool>::MyTemplate<bool>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<bool>::MyTemplate<bool>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<bool>::MyTemplate':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<_Bool> &&': Unsupported type: && without lifetime

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<bool>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<bool>::operator=':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<_Bool> &&': Unsupported type: && without lifetime

impl __CcTemplateInst10MyTemplateIbE {
    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=13
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: bool) {
        crate::detail::__rust_thunk___ZN10MyTemplateIbE8processTEb__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
#[derive(Clone, Copy)]
#[repr(C)]
pub struct __CcTemplateInst10MyTemplateIcE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<char>"),
    crate::__CcTemplateInst10MyTemplateIcE
);

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<char>::MyTemplate<char>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<char>::MyTemplate<char>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<char>::MyTemplate':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<char> &&': Unsupported type: && without lifetime

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<char>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<char>::operator=':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<char> &&': Unsupported type: && without lifetime

impl __CcTemplateInst10MyTemplateIcE {
    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=13
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: u8) {
        crate::detail::__rust_thunk___ZN10MyTemplateIcE8processTEc__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct __CcTemplateInst10MyTemplateIdE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::std::mem::MaybeUninit<u8>; 8],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<double>"),
    crate::__CcTemplateInst10MyTemplateIdE
);

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<double>::MyTemplate<double>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<double>::MyTemplate<double>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<double>::MyTemplate':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<double> &&': Unsupported type: && without lifetime

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<double>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<double>::operator=':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<double> &&': Unsupported type: && without lifetime

impl __CcTemplateInst10MyTemplateIdE {
    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=13
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: f64) {
        crate::detail::__rust_thunk___ZN10MyTemplateIdE8processTEd__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct __CcTemplateInst10MyTemplateIfE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<float>"),
    crate::__CcTemplateInst10MyTemplateIfE
);

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<float>::MyTemplate<float>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<float>::MyTemplate<float>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<float>::MyTemplate':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<float> &&': Unsupported type: && without lifetime

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<float>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<float>::operator=':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<float> &&': Unsupported type: && without lifetime

impl __CcTemplateInst10MyTemplateIfE {
    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=13
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: f32) {
        crate::detail::__rust_thunk___ZN10MyTemplateIfE8processTEf__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct __CcTemplateInst10MyTemplateIiE {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) t: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyTemplate<int>"),
    crate::__CcTemplateInst10MyTemplateIiE
);

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<int>::MyTemplate<int>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<int>::MyTemplate<int>':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<int>::MyTemplate':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<int> &&': Unsupported type: && without lifetime

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<int>::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=9
// Error while generating bindings for item 'MyTemplate<int>::operator=':
// Parameter #0 is not supported: Unsupported type 'MyTemplate<int> &&': Unsupported type: && without lifetime

impl __CcTemplateInst10MyTemplateIiE {
    /// Generated from: rs_bindings_from_cc/test/golden/templates_source_order.h;l=13
    #[inline(always)]
    pub unsafe fn processT(__this: *mut Self, t: i32) {
        crate::detail::__rust_thunk___ZN10MyTemplateIiE8processTEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(__this,t)
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN10MyTemplateI8TopLevelE8processTES0___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateI8TopLevelE,
            t: crate::TopLevel,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIN23test_namespace_bindings5InnerEE8processTES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,
            t: crate::test_namespace_bindings::Inner,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIS_I8TopLevelEE8processTES1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE,
            t: crate::__CcTemplateInst10MyTemplateI8TopLevelE,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIS_IN23test_namespace_bindings5InnerEEE8processTES2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE,
            t: crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIbE8processTEb__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIbE,
            t: bool,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIcE8processTEc__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIcE,
            t: u8,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIdE8processTEd__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIdE,
            t: f64,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIfE8processTEf__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIfE,
            t: f32,
        );
        pub(crate) fn __rust_thunk___ZN10MyTemplateIiE8processTEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3atemplates_5fsource_5forder_5fcc(
            __this: *mut crate::__CcTemplateInst10MyTemplateIiE,
            t: i32,
        );
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::TopLevel>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::TopLevel>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::TopLevel: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::TopLevel: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::TopLevel: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::test_namespace_bindings::Inner>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::test_namespace_bindings::Inner>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::Inner: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::test_namespace_bindings::Inner: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::test_namespace_bindings::Inner: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::__CcTemplateInst10MyTemplateI8TopLevelE>() == 1);
const _: () =
    assert!(::std::mem::align_of::<crate::__CcTemplateInst10MyTemplateI8TopLevelE>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateI8TopLevelE: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateI8TopLevelE: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateI8TopLevelE: Drop);
};
const _: () =
    assert!(memoffset::offset_of!(crate::__CcTemplateInst10MyTemplateI8TopLevelE, t) == 0);

const _: () = assert!(
    ::std::mem::size_of::<crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE>()
        == 1
);
const _: () = assert!(
    ::std::mem::align_of::<crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE>(
    ) == 1
);
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE: Clone
    );
};
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE: Drop
    );
};
const _: () = assert!(
    memoffset::offset_of!(
        crate::__CcTemplateInst10MyTemplateIN23test_namespace_bindings5InnerEE,
        t
    ) == 0
);

const _: () =
    assert!(::std::mem::size_of::<crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE>() == 1);
const _: () =
    assert!(::std::mem::align_of::<crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE: Drop
    );
};
const _: () =
    assert!(memoffset::offset_of!(crate::__CcTemplateInst10MyTemplateIS_I8TopLevelEE, t) == 0);

const _: () = assert!(
    ::std::mem::size_of::<crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE>(
    ) == 1
);
const _: () = assert!(
    ::std::mem::align_of::<
        crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE,
    >() == 1
);
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE: Clone
    );
};
const _: () = {
    static_assertions::assert_impl_all!(
        crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE: Copy
    );
};
const _: () = {
    static_assertions::assert_not_impl_any!(
        crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE: Drop
    );
};
const _: () = assert!(
    memoffset::offset_of!(
        crate::__CcTemplateInst10MyTemplateIS_IN23test_namespace_bindings5InnerEEE,
        t
    ) == 0
);

const _: () = assert!(::std::mem::size_of::<crate::__CcTemplateInst10MyTemplateIbE>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::__CcTemplateInst10MyTemplateIbE>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIbE: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIbE: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIbE: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::__CcTemplateInst10MyTemplateIbE, t) == 0);

const _: () = assert!(::std::mem::size_of::<crate::__CcTemplateInst10MyTemplateIcE>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::__CcTemplateInst10MyTemplateIcE>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIcE: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIcE: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIcE: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::__CcTemplateInst10MyTemplateIcE, t) == 0);

const _: () = assert!(::std::mem::size_of::<crate::__CcTemplateInst10MyTemplateIdE>() == 8);
const _: () = assert!(::std::mem::align_of::<crate::__CcTemplateInst10MyTemplateIdE>() == 8);
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIdE: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIdE: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIdE: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::__CcTemplateInst10MyTemplateIdE, t) == 0);

const _: () = assert!(::std::mem::size_of::<crate::__CcTemplateInst10MyTemplateIfE>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::__CcTemplateInst10MyTemplateIfE>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIfE: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIfE: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIfE: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::__CcTemplateInst10MyTemplateIfE, t) == 0);

const _: () = assert!(::std::mem::size_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::__CcTemplateInst10MyTemplateIiE>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::__CcTemplateInst10MyTemplateIiE: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::__CcTemplateInst10MyTemplateIiE: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::__CcTemplateInst10MyTemplateIiE, t) == 0);
