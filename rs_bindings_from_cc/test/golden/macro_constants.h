// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_MACRO_CONSTANTS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_MACRO_CONSTANTS_H_

// Used only by macros, which ClangTidy doesn't expand.
#include "rs_bindings_from_cc/test/golden/not_crubit_exposed.h"  // IWYU pragma: keep

/// A simple integer constant.
#define INT_CONSTANT 4

// Constants which refer to other macro constants.
#define REFERS_TO_OTHER_CONSTANT (INT_CONSTANT + 3)
#define DEFINED_BEFORE_ITS_DEPENDENCY (DEFINED_AFTER_ITS_DEPENDENT * 2)
#define DEFINED_AFTER_ITS_DEPENDENT 21

// The types follow the C++ rules.
#define UNSIGNED_CONSTANT 42u
#define LONG_LONG_CONSTANT 1LL
#define HEX_CONSTANT 0xFFFFFFFF
#define CHAR_CONSTANT 'x'
#define CHAR16_CONSTANT u'x'
#define CHAR32_CONSTANT U'x'
#define BOOL_CONSTANT true
#define FALSE_CONSTANT false
// A suffix in a nested macro makes the macro typed, too.
#define REFERS_TO_UNSIGNED_CONSTANT (UNSIGNED_CONSTANT + 1)
#define COMPARISON (INT_CONSTANT > 3)

// Only the definition that is in effect at the end of the header is used.
#define REDEFINED_CONSTANT 1
#undef REDEFINED_CONSTANT
#define REDEFINED_CONSTANT 2
#define UNDEFINED_CONSTANT 1
#undef UNDEFINED_CONSTANT

// A macro whose name is already declared is skipped: the constant would
// conflict with the declaration.
constexpr int kAlreadyDeclared = 1;
#define kAlreadyDeclared 2

// Parentheses.
#define PARENTHESIZED_CONSTANT ((4))

// Operators.
#define NEGATIVE_CONSTANT -1
#define UNARY_PLUS_CONSTANT +1
#define COMPLEMENT_CONSTANT ~0
#define NOT_CONSTANT !true
#define SHIFTED_CONSTANT (1u << 31)
#define BITWISE_CONSTANT ((0xF0 | 0x0F) & ~0x3)
#define CONDITIONAL_CONSTANT (INT_CONSTANT > 3 ? 10 : 20)
#define NESTED_CONDITIONAL_CONSTANT (1 ? 1 : 0 ? 2 : 3)
#define CONDITIONAL_PRECEDENCE_CONSTANT (0 || 1 ? 2 : 3)
#define LOGICAL_CONSTANT (true && !false)

// Each binary operator. The operands are chosen so that each operator gives a
// different value.
#define MUL_CONSTANT (29 * 3)
#define DIV_CONSTANT (29 / 3)
#define REM_CONSTANT (29 % 3)
#define ADD_CONSTANT (29 + 3)
#define SUB_CONSTANT (29 - 3)
#define SHL_CONSTANT (29 << 3)
#define SHR_CONSTANT (29 >> 3)
#define AND_CONSTANT (29 & 3)
#define XOR_CONSTANT (29 ^ 3)
#define OR_CONSTANT (29 | 3)
// The results of comparing 1, 2, and 3 with 2, as the bits of an integer.
#define LT_CONSTANT ((1 < 2) | (2 < 2) << 1 | (3 < 2) << 2)
#define GT_CONSTANT ((1 > 2) | (2 > 2) << 1 | (3 > 2) << 2)
#define LE_CONSTANT ((1 <= 2) | (2 <= 2) << 1 | (3 <= 2) << 2)
#define GE_CONSTANT ((1 >= 2) | (2 >= 2) << 1 | (3 >= 2) << 2)
#define EQ_CONSTANT ((1 == 2) | (2 == 2) << 1 | (3 == 2) << 2)
#define NE_CONSTANT ((1 != 2) | (2 != 2) << 1 | (3 != 2) << 2)
#define LOGICAL_AND_CONSTANT (true && false)
#define LOGICAL_OR_CONSTANT (false || true)

// Precedence and associativity.
#define PRECEDENCE_CONSTANT (1 + 2 * 3)
#define LEFT_ASSOCIATIVE_CONSTANT (10 - 4 - 3)
#define SHIFT_VS_ADD_CONSTANT (1 << 2 + 1)

/// Typedefs used by the casts below, instead of e.g. `uint32_t`, because
/// goldens can't include standard library headers.
typedef unsigned int MacroUint32;
namespace macro_constants_ns {
typedef unsigned char Byte;
typedef long long Int64;  // NOLINT(google-runtime-int)
}  // namespace macro_constants_ns

// Casts. These are the casts under test, so cpplint findings about casting
// style and C integer types are suppressed with `NOLINT`.
#define C_STYLE_CAST ((unsigned char)1)
#define C_STYLE_CAST_TO_TYPEDEF ((MacroUint32)1 << 31)
#define FUNCTIONAL_CAST unsigned(-1)
#define FUNCTIONAL_CAST_TO_QUALIFIED_TYPEDEF \
  (macro_constants_ns::Int64(1) << 40)  // NOLINT(readability/casting)
#define STATIC_CAST static_cast<short>(7)
#define CAST_TO_BOOL ((bool)2)
#define GLOBAL_QUALIFIED_CAST ((::MacroUint32)1)
// Used only by `NAMESPACE_ALIAS_CAST`, which ClangTidy doesn't expand.
// NOLINTNEXTLINE(misc-unused-alias-decls)
namespace macro_constants_alias = macro_constants_ns;
#define NAMESPACE_ALIAS_CAST ((macro_constants_alias::Byte)300)
#define NESTED_CAST ((int)(signed char)300)
#define NEGATED_CAST (-(int)1)
// Casts bind more tightly than binary operators.
#define CAST_PRECEDENCE ((unsigned char)-1 + 1)
// cv-qualifiers may appear wherever C++ allows them.
#define CONST_CAST_TO_KEYWORDS ((unsigned const int)1)
#define CONST_VOLATILE_CAST_TO_TYPEDEF ((const MacroUint32 volatile)1)
#define STATIC_CAST_TO_CONST static_cast<const short>(1)

// The following macros are not supported, and are silently ignored.
#define EMPTY_MACRO
#define FUNCTION_LIKE_MACRO(x) ((x) + 1)
#define FLOAT_CONSTANT 1.5
#define STRING_CONSTANT "hello"
#define NOT_A_NUMBER 1.2.3
#define DIVISION_BY_ZERO (1 / 0)
#define OVERFLOW_CONSTANT (2147483647 + 1)
#define SELF_REFERENTIAL (SELF_REFERENTIAL + 1)
// Incomplete conditionals, which fail partway through parsing.
#define CONDITIONAL_WITHOUT_BRANCHES 1 ?
#define CONDITIONAL_WITHOUT_COLON 1 ? 2
#define CONDITIONAL_WITHOUT_FALSE_BRANCH (1 ? 2 :)
#define CONDITIONAL_WITHOUT_TRUE_BRANCH (1 ?: 2)
#define CONDITIONAL_WITHOUT_CONDITION (? 1 : 2)
#define WIDE_CHAR_CONSTANT L'x'
#define UTF8_CHAR_CONSTANT u8'x'
#define UNKNOWN_IDENTIFIER (not_a_macro + 1)
#define TOKEN_PASTING 1##2
#define EMPTY_DAG_0
#define EMPTY_DAG_1 EMPTY_DAG_0 EMPTY_DAG_0
#define EMPTY_DAG_2 EMPTY_DAG_1 EMPTY_DAG_1
#define EMPTY_DAG_3 EMPTY_DAG_2 EMPTY_DAG_2
#define EMPTY_DAG_4 EMPTY_DAG_3 EMPTY_DAG_3
#define EMPTY_DAG_5 EMPTY_DAG_4 EMPTY_DAG_4
#define EMPTY_DAG_6 EMPTY_DAG_5 EMPTY_DAG_5
#define EMPTY_DAG_7 EMPTY_DAG_6 EMPTY_DAG_6
#define EMPTY_DAG_8 EMPTY_DAG_7 EMPTY_DAG_7
#define EMPTY_DAG_9 EMPTY_DAG_8 EMPTY_DAG_8
#define EMPTY_DAG_10 EMPTY_DAG_9 EMPTY_DAG_9
#define EMPTY_DAG_11 EMPTY_DAG_10 EMPTY_DAG_10
#define EMPTY_DAG_12 EMPTY_DAG_11 EMPTY_DAG_11
#define EXPANSION_STEP_LIMIT (EMPTY_DAG_12 1)
#define CAST_TO_POINTER ((void*)0)
#define CAST_TO_UNKNOWN_TYPE ((not_a_type)1)
#define CAST_TO_WCHAR ((wchar_t)1)
// Casts to class types are rejected without being checked by the compiler.
#define CAST_TO_CLASS ClassForCast(1)
#define C_STYLE_CAST_TO_CLASS ((ClassForCast)1)
#define CAST_TO_CLASS_TEMPLATE ClassTemplateForCastInt(1)
// Typedefs that don't receive bindings, e.g. because their target doesn't
// enable Crubit.
#define CAST_TO_NON_CRUBIT_TYPEDEF ((NotCrubitExposedInt)1)
typedef NotCrubitExposedInt LocalAliasToNonCrubitTypedef;
#define CAST_TO_CHAINED_NON_CRUBIT_TYPEDEF ((LocalAliasToNonCrubitTypedef)1)
typedef unsigned int __LeadingDunderInt;
#define CAST_TO_LEADING_DUNDER_TYPEDEF ((__LeadingDunderInt)1)
using UnknownAttrInt [[gnu::aligned(4)]] = int;
#define CAST_TO_UNKNOWN_ATTR_TYPEDEF ((UnknownAttrInt)1)
namespace crate {
typedef int UnescapableNamespaceInt;
}  // namespace crate
#define CAST_TO_UNESCAPABLE_NAMESPACE_TYPEDEF \
  ((crate::UnescapableNamespaceInt)1)
#define INVALID_KEYWORD_COMBINATION_CAST ((long short)1)  // NOLINT(runtime/int)
#define CONFLICTING_SIGNEDNESS_CAST ((signed unsigned)1)
#define MULTI_KEYWORD_FUNCTIONAL_CAST \
  unsigned int(1)  // NOLINT(readability/casting)
// Invalid cv-qualifiers, as in C++.
#define DUPLICATE_CONST_CAST ((const const int)1)
#define CONST_INSIDE_QUALIFIED_NAME_CAST ((macro_constants_ns::const Int64)1)
#define CONST_FUNCTIONAL_CAST const int(1)  // NOLINT(readability/casting)

// The following macros have names that can't be bound, and are reported as
// errors.
#define __LEADING_DUNDER_CONSTANT 1
#define self 1
#define _ 1
#define DOLLAR$CONSTANT 1

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_MACRO_CONSTANTS_H_
