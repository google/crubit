// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc
#![rustfmt::skip]
#![feature(const_ptr_offset_from, custom_inner_attributes, negative_impls)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use memoffset_unstable_const::offset_of;

pub type __builtin_ms_va_list = *mut u8;

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

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=12
// Error while generating bindings for item 'Nontrivial::Nontrivial':
// Parameter #0 is not supported: Unsupported type 'struct Nontrivial &&'

impl Drop for Nontrivial {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self) }
    }
}

/// Nontrivial due to (inline) user-specified constructor and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[repr(C)]
pub struct NontrivialInline {
    pub field: i32,
}

impl !Unpin for NontrivialInline {}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=23
// Error while generating bindings for item 'NontrivialInline::NontrivialInline':
// Parameter #0 is not supported: Unsupported type 'struct NontrivialInline &&'

impl Drop for NontrivialInline {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN16NontrivialInlineD1Ev(self) }
    }
}

/// Nontrivial due to member variables.
///
/// This changes how the destructor / drop impl work -- instead of calling
/// the destructor for NontrivialMembers, it just calls the destructors for
/// each field.
#[repr(C)]
pub struct NontrivialMembers {
    pub nontrivial_member: std::mem::ManuallyDrop<Nontrivial>,
}

impl !Unpin for NontrivialMembers {}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=34
// Error while generating bindings for item 'NontrivialMembers::NontrivialMembers':
// Parameter #0 is not supported: Unsupported type 'struct NontrivialMembers &&'

impl Drop for NontrivialMembers {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self) }
    }
}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=38
// Error while generating bindings for item 'TakesByValue':
// Non-trivial_abi type 'struct Nontrivial' is not supported by value as parameter #0

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=39
// Error while generating bindings for item 'TakesByValueInline':
// Non-trivial_abi type 'struct NontrivialInline' is not supported by value as parameter #0

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialD1Ev<'a>(__this: &'a mut Nontrivial);
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineD1Ev<'a>(__this: &'a mut NontrivialInline);
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersD1Ev<'a>(
            __this: &'a mut NontrivialMembers,
        );
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
