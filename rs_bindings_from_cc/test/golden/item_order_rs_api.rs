#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(
    const_maybe_uninit_as_ptr,
    const_ptr_offset_from,
    const_raw_ptr_deref,
    custom_inner_attributes
)]

use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FirstStruct {
    pub field: i32,
}

// rs_bindings_from_cc/test/golden/item_order.h;l=4
// Error while generating bindings for item 'FirstStruct::FirstStruct':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/item_order.h;l=4
// Error while generating bindings for item 'FirstStruct::FirstStruct':
// Parameter type 'const struct FirstStruct &' is not supported

// rs_bindings_from_cc/test/golden/item_order.h;l=4
// Error while generating bindings for item 'FirstStruct::operator=':
// Parameter type 'const struct FirstStruct &' is not supported

// <unknown location>
// Error while generating bindings for item 'FirstStruct::operator=':
// Return type 'struct FirstStruct &' is not supported

// rs_bindings_from_cc/test/golden/item_order.h;l=4
// Error while generating bindings for item 'FirstStruct::FirstStruct':
// Parameter type 'struct FirstStruct &&' is not supported

// rs_bindings_from_cc/test/golden/item_order.h;l=4
// Error while generating bindings for item 'FirstStruct::operator=':
// Parameter type 'struct FirstStruct &&' is not supported

// <unknown location>
// Error while generating bindings for item 'FirstStruct::operator=':
// Return type 'struct FirstStruct &' is not supported

#[inline(always)]
pub fn first_func() -> i32 {
    unsafe { crate::detail::__rust_thunk__first_func() }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SecondStruct {
    pub field: i32,
}

// rs_bindings_from_cc/test/golden/item_order.h;l=10
// Error while generating bindings for item 'SecondStruct::SecondStruct':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/item_order.h;l=10
// Error while generating bindings for item 'SecondStruct::SecondStruct':
// Parameter type 'const struct SecondStruct &' is not supported

// rs_bindings_from_cc/test/golden/item_order.h;l=10
// Error while generating bindings for item 'SecondStruct::operator=':
// Parameter type 'const struct SecondStruct &' is not supported

// <unknown location>
// Error while generating bindings for item 'SecondStruct::operator=':
// Return type 'struct SecondStruct &' is not supported

// rs_bindings_from_cc/test/golden/item_order.h;l=10
// Error while generating bindings for item 'SecondStruct::SecondStruct':
// Parameter type 'struct SecondStruct &&' is not supported

// rs_bindings_from_cc/test/golden/item_order.h;l=10
// Error while generating bindings for item 'SecondStruct::operator=':
// Parameter type 'struct SecondStruct &&' is not supported

// <unknown location>
// Error while generating bindings for item 'SecondStruct::operator=':
// Return type 'struct SecondStruct &' is not supported

#[inline(always)]
pub fn second_func() -> i32 {
    unsafe { crate::detail::__rust_thunk__second_func() }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ITEM_ORDER_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_constructor_thunk___ZN11FirstStructC1Ev(
            __this: *mut FirstStruct,
        ) -> ();
        pub(crate) fn __rust_thunk__first_func() -> i32;
        pub(crate) fn __rust_constructor_thunk___ZN12SecondStructC1Ev(
            __this: *mut SecondStruct,
        ) -> ();
        pub(crate) fn __rust_thunk__second_func() -> i32;
    }
}

const_assert_eq!(std::mem::size_of::<FirstStruct>(), 4usize);
const_assert_eq!(std::mem::align_of::<FirstStruct>(), 4usize);
const_assert_eq!(offset_of!(FirstStruct, field) * 8, 0usize);

const_assert_eq!(std::mem::size_of::<SecondStruct>(), 4usize);
const_assert_eq!(std::mem::align_of::<SecondStruct>(), 4usize);
const_assert_eq!(offset_of!(SecondStruct, field) * 8, 0usize);
