// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_

#include <cstdint>
#include <utility>

#include "support/annotations.h"
#include "support/rs_std/char.h"
#include "support/rs_std/rs_std.h"

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

// Tests for round-tripping Rust stdlib types (`rs::std::string::String`, etc.)
// from `support/rs_std/rs_std.h`.
namespace std_test {

struct CRUBIT_MUST_BIND CRUBIT_TRIVIAL_ABI TrivialAbiStructWithString final {
  rs::std::string::String method_name;
  std::int32_t id;

  CRUBIT_MUST_BIND std::int32_t GetId() const { return id; }
  CRUBIT_MUST_BIND rs::std::string::String IntoMethodName() && {
    return std::move(method_name);
  }
};

struct CRUBIT_MUST_BIND NonTrivialAbiStructWithString final {
  rs::std::string::String method_name;
  std::int32_t id;

  CRUBIT_MUST_BIND rs::std::string::String IntoMethodName() && {
    return std::move(method_name);
  }
};

CRUBIT_MUST_BIND inline rs::std::string::String EchoString(
    rs::std::string::String s) {
  return s;
}

CRUBIT_MUST_BIND inline TrivialAbiStructWithString MakeStructWithString(
    rs::std::string::String s, std::int32_t id) {
  return TrivialAbiStructWithString{std::move(s), id};
}

}  // namespace std_test

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_
