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
use static_assertions::const_assert_eq;

#[repr(C)]
pub struct NontrivialCustomType {
    pub i: i32,
}

impl !Unpin for NontrivialCustomType {}

// namespace ns

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContainingStruct {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: core::mem::MaybeUninit<u8>,
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_

// rs_bindings_from_cc/test/golden/user_of_unsupported.h;l=6
// Error while generating bindings for item 'UseNontrivialCustomType':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a parameter

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_UNSUPPORTED_H_

mod detail {
    use super::*;
    extern "C" {
        pub(crate) fn __rust_constructor_thunk__ContainingStruct(
            __this: *mut ContainingStruct,
        ) -> ();
    }
}

const_assert_eq!(std::mem::size_of::<NontrivialCustomType>(), 4usize);
const_assert_eq!(std::mem::align_of::<NontrivialCustomType>(), 4usize);
const_assert_eq!(offset_of!(NontrivialCustomType, i) * 8, 0usize);

const_assert_eq!(std::mem::size_of::<ContainingStruct>(), 1usize);
const_assert_eq!(std::mem::align_of::<ContainingStruct>(), 1usize);
