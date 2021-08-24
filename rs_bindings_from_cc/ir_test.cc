// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir.h"

#include <string>

#include "testing/base/public/gunit.h"
#include "third_party/absl/strings/cord.h"
#include "third_party/json/src/json.hpp"

namespace rs_bindings_from_cc {

namespace {

TEST(IrTest, TestTypeToJson) {
  nlohmann::json expected =
      nlohmann::json::parse(R"j({ "rs_name": "i32", "cc_name": "int" })j");
  EXPECT_EQ(Type(absl::Cord("i32"), absl::Cord("int")).ToJson(), expected);
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
  EXPECT_EQ(IR({HeaderName(absl::Cord("foo/bar.h"))},
               {Func(Identifier(absl::Cord("hello_world")),
                     absl::Cord("#$mangled_name$#"),
                     Type(absl::Cord("i32"), absl::Cord("int")),
                     {FuncParam(Type(absl::Cord("i32"), absl::Cord("int")),
                                Identifier(absl::Cord("arg")))},
                     /* is_inline= */ false)})
                .ToJson(),
            expected);
}

}  // namespace
}  // namespace rs_bindings_from_cc
