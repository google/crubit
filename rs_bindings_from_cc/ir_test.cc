// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir.h"

#include <string>

#include "testing/base/public/gunit.h"
#include "third_party/json/src/json.hpp"

namespace rs_bindings_from_cc {

namespace {

TEST(IrTest, TestTypeToJson) {
  nlohmann::json expected =
      nlohmann::json::parse(R"j({ "rs_name": "i32", "cc_name": "int" })j");
  auto type = Type{std::string("i32"), std::string("int")};
  EXPECT_EQ(type.ToJson(), expected);
}

TEST(IrTest, TestIR) {
  nlohmann::json expected = nlohmann::json::parse(
      R"j({
            "used_headers": [{ "name": "foo/bar.h" }],
            "functions": [{
              "identifier": { "identifier": "hello_world" },
              "mangled_name": "#$mangled_name$#",
              "return_type": { "rs_name": "i32", "cc_name": "int" },
              "params": [
                {
                  "identifier": {"identifier": "arg" },
                  "type": { "rs_name": "i32", "cc_name": "int" }
                }
              ],
              "is_inline": false
            }]
      })j");
  EXPECT_EQ(
      IR({HeaderName(std::string("foo/bar.h"))},
         {Func{
             .identifier = Identifier(std::string("hello_world")),
             .mangled_name = std::string("#$mangled_name$#"),
             .return_type = Type{std::string("i32"), std::string("int")},
             .params = {FuncParam{Type{std::string("i32"), std::string("int")},
                                  Identifier(std::string("arg"))}},
             .is_inline = false}})
          .ToJson(),
      expected);
}

}  // namespace
}  // namespace rs_bindings_from_cc
