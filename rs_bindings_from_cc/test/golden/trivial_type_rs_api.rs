#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, custom_inner_attributes)]

use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;

/// Implicitly defined special member functions are trivial on a struct with
/// only trivial members.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Trivial {
    pub trivial_field: i32,
}

// rs_bindings_from_cc/test/golden/trivial_type.h;l=6
// Error while generating bindings for item 'Trivial::Trivial':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/trivial_type.h;l=6
// Error while generating bindings for item 'Trivial::Trivial':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/trivial_type.h;l=6
// Error while generating bindings for item 'Trivial::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/trivial_type.h;l=6
// Error while generating bindings for item 'Trivial::Trivial':
// Parameter type 'struct Trivial &&' is not supported

// rs_bindings_from_cc/test/golden/trivial_type.h;l=6
// Error while generating bindings for item 'Trivial::operator=':
// Parameter type 'struct Trivial &&' is not supported

/// Defaulted special member functions are trivial on a struct with only trivial
/// members.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TrivialWithDefaulted {
    pub trivial_field: i32,
}

// rs_bindings_from_cc/test/golden/trivial_type.h;l=12
// Error while generating bindings for item 'TrivialWithDefaulted::TrivialWithDefaulted':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/trivial_type.h;l=15
// Error while generating bindings for item 'TrivialWithDefaulted::TrivialWithDefaulted':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/trivial_type.h;l=16
// Error while generating bindings for item 'TrivialWithDefaulted::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/trivial_type.h;l=17
// Error while generating bindings for item 'TrivialWithDefaulted::TrivialWithDefaulted':
// Parameter type 'struct TrivialWithDefaulted &&' is not supported

// rs_bindings_from_cc/test/golden/trivial_type.h;l=18
// Error while generating bindings for item 'TrivialWithDefaulted::operator=':
// Parameter type 'struct TrivialWithDefaulted &&' is not supported

#[inline(always)]
pub fn TakesByValue(trivial: Trivial) -> () {
    unsafe { crate::detail::__rust_thunk__TakesByValue(trivial) }
}

#[inline(always)]
pub fn TakesWithDefaultedByValue(trivial: TrivialWithDefaulted) -> () {
    unsafe { crate::detail::__rust_thunk__TakesWithDefaultedByValue(trivial) }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_constructor_thunk__Trivial(__this: *mut Trivial) -> ();
        pub(crate) fn __rust_constructor_thunk__TrivialWithDefaulted(
            __this: *mut TrivialWithDefaulted,
        ) -> ();
        #[link_name = "_Z12TakesByValue7Trivial"]
        pub(crate) fn __rust_thunk__TakesByValue(trivial: Trivial) -> ();
        #[link_name = "_Z25TakesWithDefaultedByValue20TrivialWithDefaulted"]
        pub(crate) fn __rust_thunk__TakesWithDefaultedByValue(trivial: TrivialWithDefaulted) -> ();
    }
}

const_assert_eq!(std::mem::size_of::<Trivial>(), 4usize);
const_assert_eq!(std::mem::align_of::<Trivial>(), 4usize);
const_assert_eq!(offset_of!(Trivial, trivial_field) * 8, 0usize);

const_assert_eq!(std::mem::size_of::<TrivialWithDefaulted>(), 4usize);
const_assert_eq!(std::mem::align_of::<TrivialWithDefaulted>(), 4usize);
const_assert_eq!(offset_of!(TrivialWithDefaulted, trivial_field) * 8, 0usize);
