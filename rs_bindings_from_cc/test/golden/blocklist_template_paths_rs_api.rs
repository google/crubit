// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:blocklist_template_paths_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![deny(warnings)]

// error: class `TS` could not be bound
//   Class templates are not yet supported

// error: function `RTS` could not be bound
//   Return type is not supported: Unsupported type 'TS<int>': Failed to create bindings for template specialization type TS<int>: Class template instantiation forbidden by blocklist: TS

// error: struct `TS` could not be bound
//   Class template instantiation forbidden by blocklist: TS
