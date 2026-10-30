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

pub const COMPARISON: bool = true;

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

pub const SHIFTED_CONSTANT: ::ffi_11::c_uint = ::ffi_11::new_c_uint(2147483648);

pub const BITWISE_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(252);

pub const CONDITIONAL_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(10);

pub const NESTED_CONDITIONAL_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(1);

pub const CONDITIONAL_PRECEDENCE_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(2);

pub const LOGICAL_CONSTANT: bool = true;

// Each binary operator. The operands are chosen so that each operator gives a
// different value.

pub const MUL_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(87);

pub const DIV_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(9);

pub const REM_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(2);

pub const ADD_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(32);

pub const SUB_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(26);

pub const SHL_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(232);

pub const SHR_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(3);

pub const AND_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(1);

pub const XOR_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(30);

pub const OR_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(31);

// The results of comparing 1, 2, and 3 with 2, as the bits of an integer.

pub const LT_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(1);

pub const GT_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(4);

pub const LE_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(3);

pub const GE_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(6);

pub const EQ_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(2);

pub const NE_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(5);

pub const LOGICAL_AND_CONSTANT: bool = false;

pub const LOGICAL_OR_CONSTANT: bool = true;

// Precedence and associativity.

pub const PRECEDENCE_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(7);

pub const LEFT_ASSOCIATIVE_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(3);

pub const SHIFT_VS_ADD_CONSTANT: ::ffi_11::c_int = ::ffi_11::new_c_int(8);

// The following macros are not supported, and are silently ignored.
