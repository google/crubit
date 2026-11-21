// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:macro_constants_cc

#![rustfmt::skip]
#![feature(cfi_encoding, custom_inner_attributes, negative_impls)]
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

/// Typedefs used by the casts below, instead of e.g. `uint32_t`, because
/// goldens can't include standard library headers.
pub type MacroUint32 = ::ffi_11::c_uint;

pub mod macro_constants_ns {
    pub type Byte = ::ffi_11::c_uchar;

    pub type Int64 = ::ffi_11::c_longlong;

    // NOLINT(google-runtime-int)
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

// error: type alias `__LeadingDunderInt` could not be bound
//   Skipping generating bindings for '__LeadingDunderInt' because it has a leading `__`

#[macro_export]
macro_rules! CAST_TO_LEADING_DUNDER_TYPEDEF {
    () => {
        ::ffi_11::new_c_uint(1)
    };
}

/// Macros can use any C++ constant expression, e.g. calls to `constexpr`
/// functions and references to `constexpr` variables.
#[inline(always)]
pub fn MacroConstantsSquare(x: ::ffi_11::c_int) -> ::ffi_11::c_int {
    unsafe { crate::detail::__rust_thunk___Z20MacroConstantsSquarei(x) }
}

pub const kMacroConstantsInt: ::ffi_11::c_int = ::ffi_11::new_c_int(6);

#[macro_export]
macro_rules! CONSTEXPR_CALL_CONSTANT {
    () => {
        49
    };
}

#[macro_export]
macro_rules! CONSTEXPR_VARIABLE_CONSTANT {
    () => {
        7
    };
}

#[macro_export]
macro_rules! CONSTEXPR_VARIABLE_REFERENCE {
    () => {
        6
    };
}

// Clang accepts some invalid C++ as an extension, and so do macro constants.

#[macro_export]
macro_rules! GNU_CONDITIONAL_CONSTANT {
    () => {
        2
    };
}

#[macro_export]
macro_rules! DUPLICATE_CONST_CAST {
    () => {
        ::ffi_11::new_c_int(1)
    };
}

// Constructs that are usually only seen inside functions work too.

#[macro_export]
macro_rules! LAMBDA_CALL {
    () => {
        1
    };
}

#[macro_export]
macro_rules! LAMBDA_WITH_LOCAL {
    () => {
        2
    };
}

#[macro_export]
macro_rules! REQUIRES_EXPRESSION {
    () => {
        true
    };
}

#[macro_export]
macro_rules! SUBSCRIPT {
    () => {
        97
    };
}

#[macro_export]
macro_rules! BRACED_FUNCTIONAL_CAST {
    () => {
        ::ffi_11::new_c_int(3)
    };
}

// `sizeof` expressions, which have type `size_t` (spelled `__size_t` by Clang).

#[macro_export]
macro_rules! SIZEOF_CONSTANT {
    () => {
        4
    };
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "20SizeofDeclaredStruct"]
#[repr(C)]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=SizeofDeclaredStruct")]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
pub struct SizeofDeclaredStruct {
    pub field: ::ffi_11::c_int,
}
unsafe impl ::cxx::ExternType for SizeofDeclaredStruct {
    type Id = ::cxx::type_id!("SizeofDeclaredStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("SizeofDeclaredStruct"),
    crate::SizeofDeclaredStruct
);

impl Default for SizeofDeclaredStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20SizeofDeclaredStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
}

#[macro_export]
macro_rules! SIZEOF_DECLARED_STRUCT {
    () => {
        4
    };
}

// Macros are parsed at the end of the header, so a struct that is declared
// after the macro is complete there, and refers to the same struct.

#[macro_export]
macro_rules! SIZEOF_LATER_DECLARED_STRUCT {
    () => {
        4
    };
}

#[derive(Clone, Copy, ::ctor::MoveAndAssignViaCopy)]
#[cfi_encoding = "25SizeofLaterDeclaredStruct"]
#[repr(C)]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_type=SizeofLaterDeclaredStruct")]
#[cfg_attr(not(doc), doc = "CRUBIT_ANNOTATE: cpp_move_constructible=")]
pub struct SizeofLaterDeclaredStruct {
    pub field: ::ffi_11::c_int,
}
unsafe impl ::cxx::ExternType for SizeofLaterDeclaredStruct {
    type Id = ::cxx::type_id!("SizeofLaterDeclaredStruct");
    type Kind = ::cxx::kind::Trivial;
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("SizeofLaterDeclaredStruct"),
    crate::SizeofLaterDeclaredStruct
);

impl Default for SizeofLaterDeclaredStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25SizeofLaterDeclaredStructC1Ev(&raw mut tmp as *mut _);
            tmp.assume_init()
        }
    }
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

pub const kAlreadyDeclared: ::ffi_11::c_int = ::ffi_11::new_c_int(1);

// error: type alias `LocalAliasToNonCrubitTypedef` could not be bound
//   Unsupported type alias NotCrubitExposedInt:
//     crubit.rs/errors/unknown_target: the type is defined in
//       rs_bindings_from_cc/test/golden/not_crubit_exposed.h
//     which is not a public header of any Crubit-enabled target that
//       //rs_bindings_from_cc/test/golden:macro_constants_cc
//     depends on directly. Either enable Crubit on the library that provides the header,
//     add that library as a direct dependency (and `#include` the header directly),
//     or move the type to a public header.

// error: type alias `UnknownAttrInt` could not be bound
//   crubit.rs/errors/unknown_attribute: unknown attribute(s): gnu::aligned

// error: namespace `crate` could not be bound
//   Namespace name is not supported: Unescapable identifier: crate

// namespace crate

// The following macros are not supported, and are reported as errors. (The
// include guard of this header isn't reported.)

// error: macro `EMPTY_MACRO` could not be bound
//   the macro is empty

// error: macro `FUNCTION_LIKE_MACRO` could not be bound
//   function-like macros are not supported

// error: macro `FLOAT_CONSTANT` could not be bound
//   macro constants of type `double` are not supported

// error: macro `WIDE_STRING_CONSTANT` could not be bound
//   only ordinary and UTF-8 string literals are supported in macro constants

// error: macro `UTF16_STRING_CONSTANT` could not be bound
//   only ordinary and UTF-8 string literals are supported in macro constants

// error: macro `UTF32_STRING_CONSTANT` could not be bound
//   only ordinary and UTF-8 string literals are supported in macro constants

// error: macro `CONCATENATED_WIDE_STRING_CONSTANT` could not be bound
//   only ordinary and UTF-8 string literals are supported in macro constants

// error: macro `USER_DEFINED_STRING_LITERAL` could not be bound
//   the macro is not a valid C++ expression: no matching literal operator for call to 'operator""_s' with arguments of types 'const char *' and '__size_t' (aka 'unsigned long'), and no matching literal operator template

// error: macro `NOT_A_NUMBER` could not be bound
//   the macro is not a valid C++ expression: invalid suffix '.3' on floating constant

// error: macro `DIVISION_BY_ZERO` could not be bound
//   the macro is not a C++ constant expression: division by zero

// error: macro `OVERFLOW_CONSTANT` could not be bound
//   the macro is not a C++ constant expression: value 2147483648 is outside the range of representable values of type 'int'

// error: macro `SELF_REFERENTIAL` could not be bound
//   the macro is not a valid C++ expression: use of undeclared identifier 'SELF_REFERENTIAL'

// A macro whose name is already declared: the constant would conflict with the
// declaration.

// error: macro `kAlreadyDeclared` could not be bound
//   a C++ declaration has the same name as the macro

// Incomplete conditionals, which fail partway through parsing.

// error: macro `CONDITIONAL_WITHOUT_BRANCHES` could not be bound
//   the macro is not a valid C++ expression: expected expression

// error: macro `CONDITIONAL_WITHOUT_COLON` could not be bound
//   the macro is not a valid C++ expression: expected ':'

// error: macro `CONDITIONAL_WITHOUT_FALSE_BRANCH` could not be bound
//   the macro is not a valid C++ expression: expected expression

// error: macro `CONDITIONAL_WITHOUT_CONDITION` could not be bound
//   the macro is not a valid C++ expression: expected expression

// error: macro `WIDE_CHAR_CONSTANT` could not be bound
//   macro constants of type `wchar_t` are not supported

// error: macro `UTF8_CHAR_CONSTANT` could not be bound
//   macro constants of type `char8_t` are not supported

// error: macro `UNKNOWN_IDENTIFIER` could not be bound
//   the macro is not a valid C++ expression: use of undeclared identifier 'not_a_macro'

// error: macro `TOKEN_PASTING` could not be bound
//   the `#` and `##` preprocessor operators are not supported

// error: macro `EMPTY_DAG_0` could not be bound
//   the macro is empty

// error: macro `EMPTY_DAG_1` could not be bound
//   the macro is empty

// error: macro `EMPTY_DAG_2` could not be bound
//   the macro is empty

// error: macro `EMPTY_DAG_3` could not be bound
//   the macro is empty

// error: macro `EMPTY_DAG_4` could not be bound
//   the macro is empty

// error: macro `EMPTY_DAG_5` could not be bound
//   the macro is empty

// error: macro `EMPTY_DAG_6` could not be bound
//   the macro is empty

// error: macro `EMPTY_DAG_7` could not be bound
//   the macro is empty

// error: macro `EMPTY_DAG_8` could not be bound
//   the macro is empty

// error: macro `EMPTY_DAG_9` could not be bound
//   the macro is empty

// error: macro `EMPTY_DAG_10` could not be bound
//   the macro is empty

// error: macro `EMPTY_DAG_11` could not be bound
//   the macro is empty

// error: macro `EMPTY_DAG_12` could not be bound
//   the macro's expansion expands too many other macros

// error: macro `EXPANSION_STEP_LIMIT` could not be bound
//   the macro's expansion expands too many other macros

// error: macro `CAST_TO_POINTER` could not be bound
//   macro constants of type `void *` are not supported

// error: macro `BUILTIN_FUNCTION_NOT_CALLED` could not be bound
//   the macro is not a valid C++ expression: builtin functions must be directly called

// Clang can fold this as an extension, but it isn't a C++ constant expression.

// error: macro `FOLDED_POINTER_TO_INT64` could not be bound
//   the macro is not a C++ constant expression: cast that performs the conversions of a reinterpret_cast is not allowed in a constant expression

// Parsing this declares `struct MacroDeclaredStruct`, which isn't imported.

// error: macro `SIZEOF_UNDECLARED_STRUCT` could not be bound
//   the macro is not a valid C++ expression: invalid application of 'sizeof' to an incomplete type 'struct MacroDeclaredStruct'

// Constructs that aren't allowed outside a function, or in a constant
// expression.

// error: macro `STATEMENT_EXPRESSION` could not be bound
//   the macro is not a valid C++ expression: statement expression not allowed at file scope

// error: macro `STATEMENT_EXPRESSION_WITH_DECLARATION` could not be bound
//   the macro is not a valid C++ expression: statement expression not allowed at file scope

// error: macro `TRAILING_SEMICOLON` could not be bound
//   the macro is not a single C++ expression

// error: macro `ASM_STATEMENT` could not be bound
//   the macro is not a valid C++ expression: expected expression

// error: macro `ATTRIBUTE` could not be bound
//   the macro is not a valid C++ expression: expected variable name or 'this' in lambda capture list

// error: macro `COMPOUND_LITERAL` could not be bound
//   the macro is not a C++ constant expression: subexpression not valid in a constant expression

// error: macro `LAMBDA_WITH_STATIC_LOCAL` could not be bound
//   the macro is not a C++ constant expression: non-constexpr function 'operator()' cannot be used in a constant expression

// error: macro `LAMBDA_WITH_CAPTURE_DEFAULT` could not be bound
//   the macro is not a valid C++ expression: non-local lambda expression cannot have a capture-default

// error: macro `CAST_TO_UNKNOWN_TYPE` could not be bound
//   the macro is not a valid C++ expression: use of undeclared identifier 'not_a_type'

// error: macro `CAST_TO_WCHAR` could not be bound
//   macro constants of type `wchar_t` are not supported

// Casts to class types are rejected, because of their type.

// error: macro `CAST_TO_CLASS` could not be bound
//   macro constants of type `ClassForCast` are not supported

// error: macro `C_STYLE_CAST_TO_CLASS` could not be bound
//   macro constants of type `ClassForCast` are not supported

// error: macro `CAST_TO_CLASS_TEMPLATE` could not be bound
//   macro constants of type `ClassTemplateForCastInt` are not supported

// Typedefs that don't receive bindings, e.g. because their target doesn't
// enable Crubit.

// error: macro `CAST_TO_NON_CRUBIT_TYPEDEF` could not be bound
//   constants of type `NotCrubitExposedInt` are not supported

// error: macro `CAST_TO_CHAINED_NON_CRUBIT_TYPEDEF` could not be bound
//   constants of type `LocalAliasToNonCrubitTypedef` are not supported

// error: macro `CAST_TO_UNKNOWN_ATTR_TYPEDEF` could not be bound
//   constants of type `UnknownAttrInt` are not supported

// error: macro `CAST_TO_UNESCAPABLE_NAMESPACE_TYPEDEF` could not be bound
//   the type of the macro, `crate::UnescapableNamespaceInt`, doesn't have bindings

// error: macro `INVALID_KEYWORD_COMBINATION_CAST` could not be bound
//   the macro is not a valid C++ expression: cannot combine with previous 'long' declaration specifier

// error: macro `CONFLICTING_SIGNEDNESS_CAST` could not be bound
//   the macro is not a valid C++ expression: cannot combine with previous 'signed' declaration specifier

// error: macro `MULTI_KEYWORD_FUNCTIONAL_CAST` could not be bound
//   the macro is not a valid C++ expression: expected '(' for function-style cast or type construction

// NOLINT(readability/casting)

// Invalid cv-qualifiers, as in C++.

// error: macro `CONST_INSIDE_QUALIFIED_NAME_CAST` could not be bound
//   the macro is not a valid C++ expression: expected unqualified-id

// error: macro `CONST_FUNCTIONAL_CAST` could not be bound
//   the macro is not a valid C++ expression: expected expression

// The following macros have names that can't be bound, and are reported as
// errors.

// error: macro `__LEADING_DUNDER_CONSTANT` could not be bound
//   Skipping generating bindings for '__LEADING_DUNDER_CONSTANT' because it has a leading `__`

// error: macro `self` could not be bound
//   variable name is not supported: Unescapable identifier: self

// error: macro `_` could not be bound
//   `_` is not a valid Rust macro name

// error: macro `DOLLAR$CONSTANT` could not be bound
//   `DOLLAR$CONSTANT` is not a valid Rust macro name

mod detail {
    #[allow(unused_imports)]
    use super::*;
    unsafe extern "C" {
        pub(crate) unsafe fn __rust_thunk___Z20MacroConstantsSquarei(
            x: ::ffi_11::c_int,
        ) -> ::ffi_11::c_int;
        pub(crate) unsafe fn __rust_thunk___ZN20SizeofDeclaredStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
        pub(crate) unsafe fn __rust_thunk___ZN25SizeofLaterDeclaredStructC1Ev(
            __this: *mut ::core::ffi::c_void,
        );
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::SizeofDeclaredStruct>() == 4);
    assert!(::core::mem::align_of::<crate::SizeofDeclaredStruct>() == 4);
    static_assertions::assert_impl_all!(crate::SizeofDeclaredStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SizeofDeclaredStruct: Drop);
    assert!(::core::mem::offset_of!(crate::SizeofDeclaredStruct, field) == 0);
    assert!(::core::mem::size_of::<crate::SizeofLaterDeclaredStruct>() == 4);
    assert!(::core::mem::align_of::<crate::SizeofLaterDeclaredStruct>() == 4);
    static_assertions::assert_impl_all!(crate::SizeofLaterDeclaredStruct: Copy,Clone);
    static_assertions::assert_not_impl_any!(crate::SizeofLaterDeclaredStruct: Drop);
    assert!(::core::mem::offset_of!(crate::SizeofLaterDeclaredStruct, field) == 0);
};
