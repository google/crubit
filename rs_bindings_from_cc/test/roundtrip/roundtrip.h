// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_

#pragma clang lifetime_elision

#include "support/rs_std/rs_char.h"

// Tests for round-tripping between:
// - Rust built-in `char` type
// - C++ `rs_std::rs_char` type (from `crubit/support/rs_std/rs_char.h`)
namespace rs_char_test {

inline rs_std::rs_char NextChar(rs_std::rs_char c) {
  return rs_std::rs_char::from_u32(std::uint32_t{c} + 1).value();
}

struct SomeStruct final {
  rs_std::rs_char c;

  rs_std::rs_char GetChar() const { return c; }
};

using CharAlias = rs_std::rs_char;
inline CharAlias NextCharViaTypeAlias(CharAlias c) {
  return CharAlias::from_u32(std::uint32_t{c} + 1).value();
}

namespace using_test {
using rs_std::rs_char;
}
inline using_test::rs_char NextCharViaImport(using_test::rs_char c) {
  return using_test::rs_char::from_u32(std::uint32_t{c} + 1).value();
}

}  // namespace rs_char_test

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_
