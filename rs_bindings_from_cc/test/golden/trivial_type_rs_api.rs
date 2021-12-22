#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]

use memoffset_unstable_const::offset_of;

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

impl Default for Trivial {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            crate::detail::__rust_thunk___ZN7TrivialC1Ev(tmp.as_mut_ptr());
            tmp.assume_init()
        }
    }
}

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

impl Default for TrivialWithDefaulted {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            crate::detail::__rust_thunk___ZN20TrivialWithDefaultedC1Ev(tmp.as_mut_ptr());
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/trivial_type.h;l=17
// Error while generating bindings for item 'TrivialWithDefaulted::TrivialWithDefaulted':
// Parameter type 'struct TrivialWithDefaulted &&' is not supported

// rs_bindings_from_cc/test/golden/trivial_type.h;l=18
// Error while generating bindings for item 'TrivialWithDefaulted::operator=':
// Parameter type 'struct TrivialWithDefaulted &&' is not supported

/// This struct is trivial, and therefore trivially relocatable etc., but still
/// not safe to pass by reference as it is not final.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TrivialNonfinal {
    pub trivial_field: i32,
}

impl !Unpin for TrivialNonfinal {}

// rs_bindings_from_cc/test/golden/trivial_type.h;l=27
// Error while generating bindings for item 'TrivialNonfinal::TrivialNonfinal':
// Nested classes are not supported yet

impl Default for TrivialNonfinal {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            crate::detail::__rust_thunk___ZN15TrivialNonfinalC1Ev(tmp.as_mut_ptr());
            tmp.assume_init()
        }
    }
}

// rs_bindings_from_cc/test/golden/trivial_type.h;l=27
// Error while generating bindings for item 'TrivialNonfinal::TrivialNonfinal':
// Parameter type 'struct TrivialNonfinal &&' is not supported

// rs_bindings_from_cc/test/golden/trivial_type.h;l=27
// Error while generating bindings for item 'TrivialNonfinal::operator=':
// Parameter type 'struct TrivialNonfinal &&' is not supported

#[inline(always)]
pub fn TakesByValue(trivial: Trivial) {
    unsafe { crate::detail::__rust_thunk___Z12TakesByValue7Trivial(trivial) }
}

#[inline(always)]
pub fn TakesWithDefaultedByValue(trivial: TrivialWithDefaulted) {
    unsafe {
        crate::detail::__rust_thunk___Z25TakesWithDefaultedByValue20TrivialWithDefaulted(trivial)
    }
}

#[inline(always)]
pub fn TakesTrivialNonfinalByValue(trivial: TrivialNonfinal) {
    unsafe {
        crate::detail::__rust_thunk___Z27TakesTrivialNonfinalByValue15TrivialNonfinal(trivial)
    }
}

#[inline(always)]
pub fn TakesByReference(trivial: *mut Trivial) {
    unsafe { crate::detail::__rust_thunk___Z16TakesByReferenceR7Trivial(trivial) }
}

#[inline(always)]
pub fn TakesWithDefaultedByReference(trivial: *mut TrivialWithDefaulted) {
    unsafe {
        crate::detail::__rust_thunk___Z29TakesWithDefaultedByReferenceR20TrivialWithDefaulted(
            trivial,
        )
    }
}

#[inline(always)]
pub fn TakesTrivialNonfinalByReference(trivial: *mut TrivialNonfinal) {
    unsafe {
        crate::detail::__rust_thunk___Z31TakesTrivialNonfinalByReferenceR15TrivialNonfinal(trivial)
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN7TrivialC1Ev(__this: *mut Trivial);
        pub(crate) fn __rust_thunk___ZN7TrivialC1ERKS_(
            __this: *mut Trivial,
            __param_0: *const Trivial,
        );
        pub(crate) fn __rust_thunk___ZN20TrivialWithDefaultedC1Ev(
            __this: *mut TrivialWithDefaulted,
        );
        pub(crate) fn __rust_thunk___ZN20TrivialWithDefaultedC1ERKS_(
            __this: *mut TrivialWithDefaulted,
            __param_0: *const TrivialWithDefaulted,
        );
        pub(crate) fn __rust_thunk___ZN15TrivialNonfinalC1Ev(__this: *mut TrivialNonfinal);
        pub(crate) fn __rust_thunk___ZN15TrivialNonfinalC1ERKS_(
            __this: *mut TrivialNonfinal,
            __param_0: *const TrivialNonfinal,
        );
        #[link_name = "_Z12TakesByValue7Trivial"]
        pub(crate) fn __rust_thunk___Z12TakesByValue7Trivial(trivial: Trivial);
        #[link_name = "_Z25TakesWithDefaultedByValue20TrivialWithDefaulted"]
        pub(crate) fn __rust_thunk___Z25TakesWithDefaultedByValue20TrivialWithDefaulted(
            trivial: TrivialWithDefaulted,
        );
        #[link_name = "_Z27TakesTrivialNonfinalByValue15TrivialNonfinal"]
        pub(crate) fn __rust_thunk___Z27TakesTrivialNonfinalByValue15TrivialNonfinal(
            trivial: TrivialNonfinal,
        );
        #[link_name = "_Z16TakesByReferenceR7Trivial"]
        pub(crate) fn __rust_thunk___Z16TakesByReferenceR7Trivial(trivial: *mut Trivial);
        #[link_name = "_Z29TakesWithDefaultedByReferenceR20TrivialWithDefaulted"]
        pub(crate) fn __rust_thunk___Z29TakesWithDefaultedByReferenceR20TrivialWithDefaulted(
            trivial: *mut TrivialWithDefaulted,
        );
        #[link_name = "_Z31TakesTrivialNonfinalByReferenceR15TrivialNonfinal"]
        pub(crate) fn __rust_thunk___Z31TakesTrivialNonfinalByReferenceR15TrivialNonfinal(
            trivial: *mut TrivialNonfinal,
        );
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<Trivial>() == 4usize);
const _: () = assert!(std::mem::align_of::<Trivial>() == 4usize);
const _: () = assert!(offset_of!(Trivial, trivial_field) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<TrivialWithDefaulted>() == 4usize);
const _: () = assert!(std::mem::align_of::<TrivialWithDefaulted>() == 4usize);
const _: () = assert!(offset_of!(TrivialWithDefaulted, trivial_field) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<TrivialNonfinal>() == 4usize);
const _: () = assert!(std::mem::align_of::<TrivialNonfinal>() == 4usize);
const _: () = assert!(offset_of!(TrivialNonfinal, trivial_field) * 8 == 0usize);
