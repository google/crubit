// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:macro_constants_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![allow(unused)]
#![allow(deprecated)]
#![allow(unknown_lints, suspicious_runtime_symbol_definitions)]
#![deny(warnings)]
// A simple integer constant.

pub const INT_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(4);

// The types follow the C++ rules.

pub const UNSIGNED_CONSTANT: ::ffi_11::c_uint = ::ffi_11::new_c_uint(42);

pub const LONG_LONG_CONSTANT: ::ffi_11::c_longlong = ::ffi_11::new_c_longlong(1);

pub const HEX_CONSTANT: ::ffi_11::c_uint = ::ffi_11::new_c_uint(4294967295);

pub const CHAR_CONSTANT: ::ffi_11::c_char = ::ffi_11::new_c_char(120 as u8);

pub const CHAR16_CONSTANT: u16 = 120;

pub const CHAR32_CONSTANT: u32 = 120;

pub const BOOL_CONSTANT: bool = true;

pub const FALSE_CONSTANT: bool = false;

// Only the definition that is in effect at the end of the header is used.

pub const REDEFINED_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(2);

pub const kAlreadyDeclared: ::ffi_11::c_int = ::ffi_11::new_c_int(1);

// Parentheses.

pub const PARENTHESIZED_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(4);

// Operators.

pub const NEGATIVE_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(-1);

pub const UNARY_PLUS_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(1);

pub const COMPLEMENT_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(-1);

pub const NOT_CONSTANT: bool = false;

// The following macros are not supported, and are silently ignored.
