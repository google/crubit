#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(
    const_maybe_uninit_as_ptr,
    const_ptr_offset_from,
    const_raw_ptr_deref,
    custom_inner_attributes,
    negative_impls
)]

use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;

/// Nontrivial due to (declared, but not yet defined) user-specified constructor
/// and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[repr(C)]
pub struct Nontrivial {
    pub field: i32,
}

impl !Unpin for Nontrivial {}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=9
// Error while generating bindings for item 'Nontrivial::Nontrivial':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=10
// Error while generating bindings for item 'Nontrivial::Nontrivial':
// Parameter type 'struct Nontrivial &&' is not supported

impl Drop for Nontrivial {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { crate::detail::__rust_destructor_thunk__Nontrivial(self) }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=9
// Error while generating bindings for item 'Nontrivial::Nontrivial':
// Parameter type 'const struct Nontrivial &' is not supported

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=9
// Error while generating bindings for item 'Nontrivial::operator=':
// Parameter type 'const struct Nontrivial &' is not supported

// <unknown location>
// Error while generating bindings for item 'Nontrivial::operator=':
// Return type 'struct Nontrivial &' is not supported

/// Nontrivial due to (inline) user-specified constructor and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[repr(C)]
pub struct NontrivialInline {
    pub field: i32,
}

impl !Unpin for NontrivialInline {}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=20
// Error while generating bindings for item 'NontrivialInline::NontrivialInline':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=21
// Error while generating bindings for item 'NontrivialInline::NontrivialInline':
// Parameter type 'struct NontrivialInline &&' is not supported

impl Drop for NontrivialInline {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { crate::detail::__rust_destructor_thunk__NontrivialInline(self) }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=20
// Error while generating bindings for item 'NontrivialInline::NontrivialInline':
// Parameter type 'const struct NontrivialInline &' is not supported

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=20
// Error while generating bindings for item 'NontrivialInline::operator=':
// Parameter type 'const struct NontrivialInline &' is not supported

// <unknown location>
// Error while generating bindings for item 'NontrivialInline::operator=':
// Return type 'struct NontrivialInline &' is not supported

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=27
// Error while generating bindings for item 'TakesByValue':
// Non-trivial_abi type 'struct Nontrivial' is not supported by value as a parameter

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=28
// Error while generating bindings for item 'TakesByValueInline':
// Non-trivial_abi type 'struct NontrivialInline' is not supported by value as a parameter

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

mod detail {
    use super::*;
    extern "C" {
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) fn __rust_destructor_thunk__Nontrivial(__this: *mut Nontrivial) -> ();
        pub(crate) fn __rust_destructor_thunk__NontrivialInline(
            __this: *mut NontrivialInline,
        ) -> ();
    }
}

const_assert_eq!(std::mem::size_of::<Nontrivial>(), 4usize);
const_assert_eq!(std::mem::align_of::<Nontrivial>(), 4usize);
const_assert_eq!(offset_of!(Nontrivial, field) * 8, 0usize);

const_assert_eq!(std::mem::size_of::<NontrivialInline>(), 4usize);
const_assert_eq!(std::mem::align_of::<NontrivialInline>(), 4usize);
const_assert_eq!(offset_of!(NontrivialInline, field) * 8, 0usize);
