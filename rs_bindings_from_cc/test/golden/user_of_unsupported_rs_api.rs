#![rustfmt::skip]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![feature(custom_inner_attributes)]

// <unknown location>
// Error while generating bindings for item '__builtin_ms_va_list':
// Cannot generate bindings for type aliases

// namespace ns

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNSUPPORTED_H_

// rs_bindings_from_cc/test/golden/user_of_unsupported.h;l=6
// Error while generating bindings for item 'UseNontrivialCustomType':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as a parameter

// rs_bindings_from_cc/test/golden/user_of_unsupported.h;l=8
// Error while generating bindings for item 'UseUnsupportedType':
// Parameter type 'ns::StructInNamespace *' is not supported

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_UNSUPPORTED_H_

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());
