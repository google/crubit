// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir.h"

#include <string>

#include "testing/base/public/gunit.h"
#include "third_party/json/src/json.hpp"

namespace rs_bindings_from_cc {

namespace {

TEST(IrTest, TypeToJson) {
  nlohmann::json expected = nlohmann::json::parse(R"j({
      "rs_name": "CompoundRs",
      "cc_name": "CompoundCc",
      "type_params": [
          { "rs_name": "i32", "cc_name": "int", "type_params": []}
      ]
  })j");
  auto type = Type{.rs_name = "CompoundRs",
                   .cc_name = "CompoundCc",
                   .type_params = {Type{"i32", "int"}}};
  EXPECT_EQ(type.ToJson(), expected);
}

TEST(IrTest, IR) {
  nlohmann::json expected = nlohmann::json::parse(
      R"j({
            "used_headers": [{ "name": "foo/bar.h" }],
            "functions": [{
              "identifier": { "identifier": "hello_world" },
              "mangled_name": "#$mangled_name$#",
              "return_type": { "rs_name": "i32", "cc_name": "int", "type_params": [] },
              "params": [
                {
                  "identifier": {"identifier": "arg" },
                  "type": { "rs_name": "i32", "cc_name": "int", "type_params": [] }
                }
              ],
              "is_inline": false
            }],
            "records": [
              {
                "identifier": {"identifier": "SomeStruct" },
                "fields": [
                  {
                    "identifier": {"identifier": "public_int" },
                    "type": {"rs_name": "i32", "cc_name": "int", "type_params": [] },
                    "access": "Public"
                  },
                  {
                    "identifier": {"identifier": "protected_int" },
                    "type": {"rs_name": "i32", "cc_name": "int", "type_params": [] },
                    "access": "Protected"
                  },
                  {
                    "identifier": {"identifier": "private_int" },
                    "type": {"rs_name": "i32", "cc_name": "int", "type_params": [] },
                    "access": "Private"
                  }
                ]
              }
            ]
      })j");
  IR ir = {
      .used_headers = {HeaderName("foo/bar.h")},
      .functions = {Func{
          .identifier = Identifier("hello_world"),
          .mangled_name = "#$mangled_name$#",
          .return_type = Type{"i32", "int"},
          .params = {FuncParam{Type{"i32", "int"}, Identifier("arg")}},
          .is_inline = false}},
      .records = {Record(Identifier("SomeStruct"),
                         {
                             Field{.identifier = Identifier("public_int"),
                                   .type = Type{"i32", "int"},
                                   .access = kPublic},
                             Field{.identifier = Identifier("protected_int"),
                                   .type = Type{"i32", "int"},
                                   .access = kProtected},
                             Field{.identifier = Identifier("private_int"),
                                   .type = Type{"i32", "int"},
                                   .access = kPrivate},
                         })}};
  EXPECT_EQ(ir.ToJson(), expected);
}

}  // namespace
}  // namespace rs_bindings_from_cc
