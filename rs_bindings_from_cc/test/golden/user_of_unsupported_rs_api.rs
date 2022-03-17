// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for C++ target
// //rs_bindings_from_cc/test/golden:user_of_unsupported_cc
#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type __builtin_ms_va_list = *mut u8;

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// rs_bindings_from_cc/test/golden/user_of_unsupported.h;l=12
// Error while generating bindings for item 'UseNontrivialCustomType':
// Non-trivial_abi type 'struct NontrivialCustomType' is not supported by value as parameter #0

// rs_bindings_from_cc/test/golden/user_of_unsupported.h;l=14
// Error while generating bindings for item 'UseUnsupportedType':
// Parameter #0 is not supported: Unsupported type 'ns::StructInNamespace *': Unsupported type 'ns::StructInNamespace': No generated bindings found for 'StructInNamespace'

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USER_OF_UNSUPPORTED_H_

const _: () = assert!(std::mem::size_of::<Option<&i32>>() == std::mem::size_of::<&i32>());
