#![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, const_raw_ptr_deref, negative_impls)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;

#[repr(C)]
pub struct NontrivialCustomType {
    pub i: i32,
}

impl !Unpin for NontrivialCustomType {}

// rs_bindings_from_cc/test/golden/unsupported.h;l=5
// Error while generating bindings for item 'NontrivialCustomType::NontrivialCustomType':
// Parameter type 'struct NontrivialCustomType &&' is not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=10
// Error while generating bindings for item 'UnsupportedParamType':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a parameter

// rs_bindings_from_cc/test/golden/unsupported.h;l=11
// Error while generating bindings for item 'UnsupportedUnnamedParam':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=12
// Error while generating bindings for item 'UnsupportedReturnType':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a return type

// rs_bindings_from_cc/test/golden/unsupported.h;l=14
// Error while generating bindings for item 'MultipleReasons':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a parameter

// rs_bindings_from_cc/test/golden/unsupported.h;l=14
// Error while generating bindings for item 'MultipleReasons':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=14
// Error while generating bindings for item 'MultipleReasons':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a return type

// rs_bindings_from_cc/test/golden/unsupported.h;l=17
// Error while generating bindings for item 'ns::FunctionInNamespace':
// Items contained in namespaces are not supported yet

// rs_bindings_from_cc/test/golden/unsupported.h;l=18
// Error while generating bindings for item 'ns::StructInNamespace':
// Items contained in namespaces are not supported yet

// namespace ns

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ContainingStruct {
    /// Prevent empty C++ struct being zero-size in Rust.
    placeholder: core::mem::MaybeUninit<u8>,
}

// rs_bindings_from_cc/test/golden/unsupported.h;l=22
// Error while generating bindings for item 'ContainingStruct::NestedStruct':
// Nested classes are not supported yet

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_

const_assert_eq!(std::mem::size_of::<NontrivialCustomType>(), 4usize);
const_assert_eq!(std::mem::align_of::<NontrivialCustomType>(), 4usize);
const_assert_eq!(offset_of!(NontrivialCustomType, i) * 8, 0usize);

const_assert_eq!(std::mem::size_of::<ContainingStruct>(), 1usize);
const_assert_eq!(std::mem::align_of::<ContainingStruct>(), 1usize);
