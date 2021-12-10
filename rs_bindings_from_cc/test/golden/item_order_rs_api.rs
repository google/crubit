#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes)]

use memoffset_unstable_const::offset_of;

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
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/item_order.h;l=4
// Error while generating bindings for item 'FirstStruct::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/item_order.h;l=4
// Error while generating bindings for item 'FirstStruct::FirstStruct':
// Parameter type 'struct FirstStruct &&' is not supported

// rs_bindings_from_cc/test/golden/item_order.h;l=4
// Error while generating bindings for item 'FirstStruct::operator=':
// Parameter type 'struct FirstStruct &&' is not supported

#[inline(always)]
pub fn first_func() -> i32 {
    unsafe { crate::detail::__rust_thunk___Z10first_funcv() }
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
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/item_order.h;l=10
// Error while generating bindings for item 'SecondStruct::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/item_order.h;l=10
// Error while generating bindings for item 'SecondStruct::SecondStruct':
// Parameter type 'struct SecondStruct &&' is not supported

// rs_bindings_from_cc/test/golden/item_order.h;l=10
// Error while generating bindings for item 'SecondStruct::operator=':
// Parameter type 'struct SecondStruct &&' is not supported

#[inline(always)]
pub fn second_func() -> i32 {
    unsafe { crate::detail::__rust_thunk___Z11second_funcv() }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ITEM_ORDER_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN11FirstStructC1Ev(__this: *mut FirstStruct) -> ();
        pub(crate) fn __rust_thunk___Z10first_funcv() -> i32;
        pub(crate) fn __rust_thunk___ZN12SecondStructC1Ev(__this: *mut SecondStruct) -> ();
        pub(crate) fn __rust_thunk___Z11second_funcv() -> i32;
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<FirstStruct>() == 4usize);
const _: () = assert!(std::mem::align_of::<FirstStruct>() == 4usize);
const _: () = assert!(offset_of!(FirstStruct, field) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<SecondStruct>() == 4usize);
const _: () = assert!(std::mem::align_of::<SecondStruct>() == 4usize);
const _: () = assert!(offset_of!(SecondStruct, field) * 8 == 0usize);
