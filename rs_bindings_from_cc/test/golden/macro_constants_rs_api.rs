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
// Used only by macros, which ClangTidy doesn't expand.

// IWYU pragma: keep

// A simple integer constant.

#[macro_export]
macro_rules! INT_CONSTANT {
    () => {
        4
    };
}

// Constants which refer to other macro constants.

#[macro_export]
macro_rules! REFERS_TO_OTHER_CONSTANT {
    () => {
        7
    };
}

#[macro_export]
macro_rules! DEFINED_BEFORE_ITS_DEPENDENCY {
    () => {
        42
    };
}

#[macro_export]
macro_rules! DEFINED_AFTER_ITS_DEPENDENT {
    () => {
        21
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

// A suffix in a nested macro makes the macro typed, too.

#[macro_export]
macro_rules! REFERS_TO_UNSIGNED_CONSTANT {
    () => {
        ::ffi_11::new_c_uint(43)
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

/// Typedefs used by the casts below, instead of e.g. `uint32_t`, because goldens
/// can't include standard library headers.
pub type MacroUint32 = ::ffi_11::c_uint;

pub mod macro_constants_ns {
    pub type Byte = ::ffi_11::c_uchar;

    pub type Int64 = ::ffi_11::c_longlong;

    // NOLINT(runtime/int)
}

// namespace macro_constants_ns

// Casts. These are the casts under test, so cpplint findings about casting
// style and C integer types are suppressed with `NOLINT`.

#[macro_export]
macro_rules! C_STYLE_CAST {
    () => {
        ::ffi_11::new_c_uchar(1)
    };
}

#[macro_export]
macro_rules! C_STYLE_CAST_TO_TYPEDEF {
    () => {
        ::ffi_11::new_c_uint(2147483648)
    };
}

#[macro_export]
macro_rules! FUNCTIONAL_CAST {
    () => {
        ::ffi_11::new_c_uint(4294967295)
    };
}

#[macro_export]
macro_rules! FUNCTIONAL_CAST_TO_QUALIFIED_TYPEDEF {
    () => {
        ::ffi_11::new_c_longlong(1099511627776)
    };
}

// NOLINT(readability/casting)

#[macro_export]
macro_rules! STATIC_CAST {
    () => {
        ::ffi_11::new_c_short(7)
    };
}

#[macro_export]
macro_rules! CAST_TO_BOOL {
    () => {
        true
    };
}

#[macro_export]
macro_rules! GLOBAL_QUALIFIED_CAST {
    () => {
        ::ffi_11::new_c_uint(1)
    };
}

#[macro_export]
macro_rules! NAMESPACE_ALIAS_CAST {
    () => {
        ::ffi_11::new_c_uchar(44)
    };
}

#[macro_export]
macro_rules! NESTED_CAST {
    () => {
        ::ffi_11::new_c_int(44)
    };
}

#[macro_export]
macro_rules! NEGATED_CAST {
    () => {
        ::ffi_11::new_c_int(-1)
    };
}

// Casts bind more tightly than binary operators.

#[macro_export]
macro_rules! CAST_PRECEDENCE {
    () => {
        ::ffi_11::new_c_int(256)
    };
}

// cv-qualifiers may appear wherever C++ allows them.

#[macro_export]
macro_rules! CONST_CAST_TO_KEYWORDS {
    () => {
        ::ffi_11::new_c_uint(1)
    };
}

#[macro_export]
macro_rules! CONST_VOLATILE_CAST_TO_TYPEDEF {
    () => {
        ::ffi_11::new_c_uint(1)
    };
}

#[macro_export]
macro_rules! STATIC_CAST_TO_CONST {
    () => {
        ::ffi_11::new_c_short(1)
    };
}

// String constants.

#[macro_export]
macro_rules! STRING_CONSTANT {
    () => {
        "hello"
    };
}

#[macro_export]
macro_rules! CONCATENATED_STRING_CONSTANT {
    () => {
        "hello, world"
    };
}

#[macro_export]
macro_rules! PARENTHESIZED_STRING_CONSTANT {
    () => {
        "hello"
    };
}

#[macro_export]
macro_rules! STRING_WITH_ESCAPES {
    () => {
        "tab\tquote\"newline\n"
    };
}

#[macro_export]
macro_rules! UTF8_STRING_CONSTANT {
    () => {
        "café"
    };
}

#[macro_export]
macro_rules! NON_UTF8_STRING_CONSTANT {
    () => {
        b"\xFF"
    };
}

// The result of concatenating ordinary and UTF-8 string literals is a UTF-8
// string literal.

#[macro_export]
macro_rules! MIXED_PREFIX_STRING_CONSTANT {
    () => {
        "ab"
    };
}

#[macro_export]
macro_rules! STRING_WITH_EMBEDDED_NUL {
    () => {
        "a\0b"
    };
}

// The following macros are not supported, and are silently ignored.

// Incomplete conditionals, which fail partway through parsing.

// Casts to class types are rejected without being checked by the compiler.

// Typedefs that don't receive bindings, e.g. because their target doesn't
// enable Crubit.

// error: type alias `LocalAliasToNonCrubitTypedef` could not be bound
//   Unsupported type alias NotCrubitExposedInt

// error: type alias `__LeadingDunderInt` could not be bound
//   Skipping generating bindings for '__LeadingDunderInt' because it has a leading `__`

// error: type alias `UnknownAttrInt` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): gnu::aligned

// error: namespace `crate` could not be bound
//   Namespace name is not supported: Unescapable identifier: crate

// namespace crate

// NOLINT(runtime/int)

// NOLINT(readability/casting)

// Invalid cv-qualifiers, as in C++.

// NOLINT(readability/casting)

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
