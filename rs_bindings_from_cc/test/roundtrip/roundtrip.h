// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_

#include <cstdint>

#include "support/annotations.h"
#include "support/rs_std/char.h"

// Tests for round-tripping between:
// - Rust built-in `char` type
// - C++ `rs::char_` type (from `crubit/support/rs_std/char.h`)
namespace char_test {

CRUBIT_MUST_BIND inline rs::char_ NextChar(rs::char_ c) {
  return rs::char_::from_u32(std::uint32_t{c} + 1).value();
}

struct SomeStruct final {
  rs::char_ c;

  CRUBIT_MUST_BIND rs::char_ GetChar() const { return c; }
};

using CharAlias = rs::char_;
CRUBIT_MUST_BIND inline CharAlias NextCharViaTypeAlias(CharAlias c) {
  return CharAlias::from_u32(std::uint32_t{c} + 1).value();
}

namespace using_test {
using rs::char_;
}

CRUBIT_MUST_BIND
inline using_test::char_ NextCharViaImport(using_test::char_ c) {
  return using_test::char_::from_u32(std::uint32_t{c} + 1).value();
}

}  // namespace char_test

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_
