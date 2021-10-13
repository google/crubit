#![feature(const_maybe_uninit_as_ptr, const_ptr_offset_from, const_raw_ptr_deref)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception


use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CustomType {
    pub i: i32,
}

// rs_bindings_from_cc/test/golden/unsupported.h;l=8
// Error while generating bindings for item 'UnsupportedParamType':
// Parameter type 'struct CustomType' is not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=9
// Error while generating bindings for item 'UnsupportedUnnamedParam':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=10
// Error while generating bindings for item 'UnsupportedReturnType':
// Return type 'struct CustomType' is not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=12
// Error while generating bindings for item 'MultipleReasons':
// Parameter type 'struct CustomType' is not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=12
// Error while generating bindings for item 'MultipleReasons':
// Empty parameter names are not supported

// rs_bindings_from_cc/test/golden/unsupported.h;l=12
// Error while generating bindings for item 'MultipleReasons':
// Return type 'struct CustomType' is not supported

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_

const_assert_eq!(std::mem::size_of::<CustomType>(), 4usize);
const_assert_eq!(std::mem::align_of::<CustomType>(), 4usize);
const_assert_eq!(offset_of!(CustomType, i) * 8, 0usize);
