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

#[macro_export]
macro_rules! INT_CONSTANT {
    () => {
        4
    };
}

// The types follow the C++ rules.

#[macro_export]
macro_rules! UNSIGNED_CONSTANT {
    () => {
        ::ffi_11::new_c_uint(42)
    };
}

#[macro_export]
macro_rules! LONG_LONG_CONSTANT {
    () => {
        ::ffi_11::new_c_longlong(1)
    };
}

#[macro_export]
macro_rules! HEX_CONSTANT {
    () => {
        4294967295
    };
}

#[macro_export]
macro_rules! CHAR_CONSTANT {
    () => {
        120
    };
}

#[macro_export]
macro_rules! CHAR16_CONSTANT {
    () => {
        120u16
    };
}

#[macro_export]
macro_rules! CHAR32_CONSTANT {
    () => {
        120u32
    };
}

#[macro_export]
macro_rules! BOOL_CONSTANT {
    () => {
        true
    };
}

#[macro_export]
macro_rules! FALSE_CONSTANT {
    () => {
        false
    };
}

#[macro_export]
macro_rules! COMPARISON {
    () => {
        true
    };
}

// Only the definition that is in effect at the end of the header is used.

#[macro_export]
macro_rules! REDEFINED_CONSTANT {
    () => {
        2
    };
}

pub const kAlreadyDeclared: ::ffi_11::c_int = ::ffi_11::new_c_int(1);

// Parentheses.

#[macro_export]
macro_rules! PARENTHESIZED_CONSTANT {
    () => {
        4
    };
}

// Operators.

#[macro_export]
macro_rules! NEGATIVE_CONSTANT {
    () => {
        -1
    };
}

#[macro_export]
macro_rules! UNARY_PLUS_CONSTANT {
    () => {
        1
    };
}

#[macro_export]
macro_rules! COMPLEMENT_CONSTANT {
    () => {
        -1
    };
}

#[macro_export]
macro_rules! NOT_CONSTANT {
    () => {
        false
    };
}

#[macro_export]
macro_rules! SHIFTED_CONSTANT {
    () => {
        ::ffi_11::new_c_uint(2147483648)
    };
}

#[macro_export]
macro_rules! BITWISE_CONSTANT {
    () => {
        252
    };
}

#[macro_export]
macro_rules! CONDITIONAL_CONSTANT {
    () => {
        10
    };
}

#[macro_export]
macro_rules! NESTED_CONDITIONAL_CONSTANT {
    () => {
        1
    };
}

#[macro_export]
macro_rules! CONDITIONAL_PRECEDENCE_CONSTANT {
    () => {
        2
    };
}

#[macro_export]
macro_rules! LOGICAL_CONSTANT {
    () => {
        true
    };
}

// Each binary operator. The operands are chosen so that each operator gives a
// different value.

#[macro_export]
macro_rules! MUL_CONSTANT {
    () => {
        87
    };
}

#[macro_export]
macro_rules! DIV_CONSTANT {
    () => {
        9
    };
}

#[macro_export]
macro_rules! REM_CONSTANT {
    () => {
        2
    };
}

#[macro_export]
macro_rules! ADD_CONSTANT {
    () => {
        32
    };
}

#[macro_export]
macro_rules! SUB_CONSTANT {
    () => {
        26
    };
}

#[macro_export]
macro_rules! SHL_CONSTANT {
    () => {
        232
    };
}

#[macro_export]
macro_rules! SHR_CONSTANT {
    () => {
        3
    };
}

#[macro_export]
macro_rules! AND_CONSTANT {
    () => {
        1
    };
}

#[macro_export]
macro_rules! XOR_CONSTANT {
    () => {
        30
    };
}

#[macro_export]
macro_rules! OR_CONSTANT {
    () => {
        31
    };
}

// The results of comparing 1, 2, and 3 with 2, as the bits of an integer.

#[macro_export]
macro_rules! LT_CONSTANT {
    () => {
        1
    };
}

#[macro_export]
macro_rules! GT_CONSTANT {
    () => {
        4
    };
}

#[macro_export]
macro_rules! LE_CONSTANT {
    () => {
        3
    };
}

#[macro_export]
macro_rules! GE_CONSTANT {
    () => {
        6
    };
}

#[macro_export]
macro_rules! EQ_CONSTANT {
    () => {
        2
    };
}

#[macro_export]
macro_rules! NE_CONSTANT {
    () => {
        5
    };
}

#[macro_export]
macro_rules! LOGICAL_AND_CONSTANT {
    () => {
        false
    };
}

#[macro_export]
macro_rules! LOGICAL_OR_CONSTANT {
    () => {
        true
    };
}

// Precedence and associativity.

#[macro_export]
macro_rules! PRECEDENCE_CONSTANT {
    () => {
        7
    };
}

#[macro_export]
macro_rules! LEFT_ASSOCIATIVE_CONSTANT {
    () => {
        3
    };
}

#[macro_export]
macro_rules! SHIFT_VS_ADD_CONSTANT {
    () => {
        8
    };
}

// The following macros are not supported, and are silently ignored.

// Incomplete conditionals, which fail partway through parsing.

// The following macros have names that can't be bound, and are reported as
// errors.

// error: global variable `__LEADING_DUNDER_CONSTANT` could not be bound
//   Skipping generating bindings for '__LEADING_DUNDER_CONSTANT' because it has a leading `__`

// error: global variable `self` could not be bound
//   variable name is not supported: Unescapable identifier: self

// error: global variable `_` could not be bound
//   `_` is not a valid Rust macro name

// error: global variable `DOLLAR$CONSTANT` could not be bound
//   `DOLLAR$CONSTANT` is not a valid Rust macro name
