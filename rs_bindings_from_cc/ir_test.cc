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
      "rs_type": {
          "name": "CompoundRs",
          "type_params": [{"name": "i32", "type_params": []}]
      },
      "cc_type": {
          "name": "CompoundCc",
          "is_const": false,
          "type_params": [
              {"is_const": false, "name": "int", "type_params": []}
          ]
      }
  })j");
  auto type = MappedType{.rs_type = RsType{"CompoundRs", {RsType{"i32"}}},
                         .cc_type = CcType{.name = "CompoundCc",
                                           .is_const = false,
                                           .type_params = {CcType{"int"}}}};
  EXPECT_EQ(type.ToJson(), expected);
}

TEST(IrTest, IR) {
  nlohmann::json expected = nlohmann::json::parse(
      R"j({
            "used_headers": [{ "name": "foo/bar.h" }],
            "items": [
                { "Func": {
                    "identifier": { "identifier": "hello_world" },
                    "mangled_name": "#$mangled_name$#",
                    "return_type": {
                        "rs_type": { "name": "i32", "type_params": [] },
                        "cc_type": {
                            "is_const": false,
                            "name": "int",
                            "type_params": []
                        }
                    },
                    "params": [{
                        "identifier": { "identifier": "arg" },
                        "type": {
                            "rs_type": { "name": "i32", "type_params": [] },
                            "cc_type": {
                                "is_const": false,
                                "name": "int",
                                "type_params": []
                            }
                        }
                    }],
                    "is_inline": false
                }},
                { "Record": {
                    "identifier": { "identifier": "SomeStruct" },
                    "fields": [
                    {
                        "identifier": { "identifier": "public_int" },
                        "type": {
                            "rs_type": { "name": "i32", "type_params": [] },
                            "cc_type": {
                                "is_const": false,
                                "name": "int",
                                "type_params": []
                            }
                        },
                        "access": "Public",
                        "offset": 0
                    },
                    {
                        "identifier": { "identifier": "protected_int" },
                        "type": {
                            "rs_type": { "name": "i32", "type_params": [] },
                            "cc_type": {
                                "is_const": false,
                                "name": "int",
                                "type_params": []
                            }
                        },
                        "access": "Protected",
                        "offset": 32
                    },
                    {
                        "identifier": { "identifier": "private_int" },
                        "type": {
                            "rs_type": { "name": "i32", "type_params": [] },
                            "cc_type": {
                                "is_const": false,
                                "name": "int",
                                "type_params": []
                            }
                        },
                        "access": "Private",
                        "offset": 64
                    }
                    ],
                    "size": 12,
                    "alignment": 4,
                    "copy_constructor": {
                        "definition": "Nontrivial",
                        "access": "Private"
                    },
                    "move_constructor": {
                        "definition": "Deleted",
                        "access": "Protected"
                    },
                    "is_trivial_abi": true
                }}
            ]
        })j");
  IR ir =
      {
          .used_headers = {HeaderName("foo/bar.h")},
          .items = {Func{.identifier = Identifier("hello_world"),
                         .mangled_name = "#$mangled_name$#",
                         .return_type = MappedType::Simple("i32", "int"),
                         .params = {FuncParam{MappedType::Simple("i32", "int"),
                                              Identifier("arg")}},
                         .is_inline = false},
                    Record{.identifier = Identifier("SomeStruct"),
                           .fields =
                               {
                                   Field{.identifier = Identifier("public_int"),
                                         .type = MappedType::Simple("i32",
                                                                    "int"),
                                         .access = kPublic,
                                         .offset = 0},
                                   Field{.identifier = Identifier(
                                             "protected_int"),
                                         .type = MappedType::Simple("i32",
                                                                    "int"),
                                         .access = kProtected,
                                         .offset = 32},
                                   Field{.identifier = Identifier(
                                             "private_int"),
                                         .type = MappedType::Simple("i32",
                                                                    "int"),
                                         .access = kPrivate,
                                         .offset = 64},
                               },
                           .size = 12,
                           .alignment = 4,
                           .copy_constructor =
                               SpecialMemberFunc{
                                   .definition = SpecialMemberFunc::Definition::
                                       kNontrivial,
                                   .access = kPrivate},
                           .move_constructor =
                               SpecialMemberFunc{
                                   .definition =
                                       SpecialMemberFunc::Definition::kDeleted,
                                   .access = kProtected},
                           .is_trivial_abi = true}}};
  EXPECT_EQ(ir.ToJson(), expected);
}

}  // namespace
}  // namespace rs_bindings_from_cc
