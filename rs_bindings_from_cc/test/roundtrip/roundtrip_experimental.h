// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_ROUNDTRIP_ROUNDTRIP_EXPERIMENTAL_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_ROUNDTRIP_ROUNDTRIP_EXPERIMENTAL_H_

#include <cstdint>
#include <utility>

#include "support/annotations.h"
#include "support/rs_std/rs_std.h"

namespace std_experimental_test {

struct CRUBIT_MUST_BIND CRUBIT_TRIVIAL_ABI TrivialAbiStructWithString final {
  rs::std::string::String method_name;
  std::int32_t id;
};

struct CRUBIT_MUST_BIND NonTrivialAbiStructWithString final {
  rs::std::string::String method_name;
  std::int32_t id;
};

CRUBIT_MUST_BIND inline TrivialAbiStructWithString MakeStructWithString(
    rs::std::string::String s, std::int32_t id) {
  return TrivialAbiStructWithString{std::move(s), id};
}

}  // namespace std_experimental_test

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_ROUNDTRIP_ROUNDTRIP_EXPERIMENTAL_H_
