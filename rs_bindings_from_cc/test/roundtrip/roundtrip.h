// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_

#include <cstdint>
#pragma clang lifetime_elision

#include "support/rs_std/char.h"

// Tests for round-tripping between:
// - Rust built-in `char` type
// - C++ `rs_std::char_` type (from `crubit/support/rs_std/char.h`)
namespace char_test {

inline rs_std::char_ NextChar(rs_std::char_ c) {
  return rs_std::char_::from_u32(std::uint32_t{c} + 1).value();
}

struct SomeStruct final {
  rs_std::char_ c;

  rs_std::char_ GetChar() const { return c; }
};

using CharAlias = rs_std::char_;
inline CharAlias NextCharViaTypeAlias(CharAlias c) {
  return CharAlias::from_u32(std::uint32_t{c} + 1).value();
}

namespace using_test {
using rs_std::char_;
}
inline using_test::char_ NextCharViaImport(using_test::char_ c) {
  return using_test::char_::from_u32(std::uint32_t{c} + 1).value();
}

}  // namespace char_test

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_
