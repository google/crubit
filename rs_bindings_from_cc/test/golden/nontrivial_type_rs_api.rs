#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(
    const_maybe_uninit_as_ptr,
    const_ptr_offset_from,
    custom_inner_attributes,
    negative_impls
)]

use memoffset_unstable_const::offset_of;

/// Nontrivial due to (declared, but not yet defined) user-specified constructor
/// and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[repr(C)]
pub struct Nontrivial {
    pub field: std::mem::ManuallyDrop<i32>,
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
        unsafe { crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self) }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=9
// Error while generating bindings for item 'Nontrivial::Nontrivial':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=9
// Error while generating bindings for item 'Nontrivial::operator=':
// Empty parameter names are not supported

/// Nontrivial due to (inline) user-specified constructor and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[repr(C)]
pub struct NontrivialInline {
    pub field: std::mem::ManuallyDrop<i32>,
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
        unsafe { crate::detail::__rust_thunk___ZN16NontrivialInlineD1Ev(self) }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=20
// Error while generating bindings for item 'NontrivialInline::NontrivialInline':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=20
// Error while generating bindings for item 'NontrivialInline::operator=':
// Empty parameter names are not supported

/// Nontrivial due to member variables.
///
/// This changes how the destructor / drop impl work -- instead of calling
/// the destructor for NontrivialMembers, it just calls the destructors for
/// each field.
#[repr(C)]
pub struct NontrivialMembers {
    pub nontrivial_member: Nontrivial,
}

impl !Unpin for NontrivialMembers {}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=32
// Error while generating bindings for item 'NontrivialMembers::NontrivialMembers':
// Nested classes are not supported yet

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=32
// Error while generating bindings for item 'NontrivialMembers::NontrivialMembers':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=32
// Error while generating bindings for item 'NontrivialMembers::NontrivialMembers':
// Parameter type 'struct NontrivialMembers &&' is not supported

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=32
// Error while generating bindings for item 'NontrivialMembers::operator=':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=32
// Error while generating bindings for item 'NontrivialMembers::operator=':
// Parameter type 'struct NontrivialMembers &&' is not supported

impl Drop for NontrivialMembers {
    #[inline(always)]
    fn drop(&mut self) {}
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=36
// Error while generating bindings for item 'TakesByValue':
// Non-trivial_abi type 'struct Nontrivial' is not supported by value as a parameter

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=37
// Error while generating bindings for item 'TakesByValueInline':
// Non-trivial_abi type 'struct NontrivialInline' is not supported by value as a parameter

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

mod detail {
    use super::*;
    extern "C" {
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialD1Ev(__this: *mut Nontrivial) -> ();
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineD1Ev(__this: *mut NontrivialInline) -> ();
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1Ev(
            __this: *mut NontrivialMembers,
        ) -> ();
    }
}

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());

const _: () = assert!(std::mem::size_of::<Nontrivial>() == 4usize);
const _: () = assert!(std::mem::align_of::<Nontrivial>() == 4usize);
const _: () = assert!(offset_of!(Nontrivial, field) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<NontrivialInline>() == 4usize);
const _: () = assert!(std::mem::align_of::<NontrivialInline>() == 4usize);
const _: () = assert!(offset_of!(NontrivialInline, field) * 8 == 0usize);

const _: () = assert!(std::mem::size_of::<NontrivialMembers>() == 4usize);
const _: () = assert!(std::mem::align_of::<NontrivialMembers>() == 4usize);
const _: () = assert!(offset_of!(NontrivialMembers, nontrivial_member) * 8 == 0usize);
