#![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, const_raw_ptr_deref, negative_impls)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;

#[repr(C)]
pub struct Nontrivial {
    pub field: i32,
}

impl !Unpin for Nontrivial {}

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=5
// Error while generating bindings for item 'Nontrivial::Nontrivial':
// Parameter type 'struct Nontrivial &&' is not supported

// rs_bindings_from_cc/test/golden/nontrivial_type.h;l=11
// Error while generating bindings for item 'TakesByValue':
// Non-trivial_abi type 'struct Nontrivial' is not supported by value as a parameter

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

mod detail {
    use super::*;
    extern "C" {
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) fn __rust_destructor_thunk___ZN10NontrivialD1Ev(__this: *mut Nontrivial) -> ();
    }
}

const_assert_eq!(std::mem::size_of::<Nontrivial>(), 4usize);
const_assert_eq!(std::mem::align_of::<Nontrivial>(), 4usize);
const_assert_eq!(offset_of!(Nontrivial, field) * 8, 0usize);
