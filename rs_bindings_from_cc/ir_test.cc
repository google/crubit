// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ir.h"

#include <iomanip>
#include <string>

#include "rs_bindings_from_cc/bazel_types.h"
#include "testing/base/public/gunit.h"
#include "third_party/absl/hash/hash_testing.h"
#include "third_party/json/src/json.hpp"

namespace rs_bindings_from_cc {

namespace {

MATCHER_P(EqualsJson, expected_json, "") {
  if (arg == expected_json) {
    return true;
  }
  *result_listener << "Diff:\n"
                   << std::setw(2) << nlohmann::json::diff(arg, expected_json);
  return false;
}

TEST(IrTest, TypeToJson) {
  nlohmann::json expected = nlohmann::json::parse(R"j({
      "rs_type": {
          "name": "CompoundRs",
          "type_args": [{"name": "i32", "type_args": []}]
      },
      "cc_type": {
          "name": "CompoundCc",
          "is_const": false,
          "type_args": [
              {"is_const": false, "name": "int", "type_args": []}
          ]
      }
  })j");
  auto type = MappedType{
      .rs_type = RsType{.name = "CompoundRs", .type_args = {RsType{"i32"}}},
      .cc_type = CcType{.name = "CompoundCc",
                        .is_const = false,
                        .type_args = {CcType{"int"}}}};
  EXPECT_THAT(type.ToJson(), EqualsJson(expected));
}

TEST(IrTest, TypeWithDeclIdToJson) {
  nlohmann::json expected = nlohmann::json::parse(R"j({
      "rs_type": {"type_args": [], "decl_id": 42},
      "cc_type": {
        "is_const": false,
        "type_args": [],
        "decl_id": 43
      }
  })j");
  auto type = MappedType{.rs_type = {RsType{"Status", DeclId(42)}},
                         .cc_type = {CcType{"Result", DeclId(43)}}};
  EXPECT_THAT(type.ToJson(), EqualsJson(expected));
}

TEST(IrTest, IR) {
  nlohmann::json expected = nlohmann::json::parse(
      R"j({
            "current_target": "//foo:bar",
            "used_headers": [{ "name": "foo/bar.h" }],
            "items": [
                { "Record": {
                    "identifier": { "identifier": "SomeStruct" },
                    "id": 42,
                    "owning_target": "//foo:bar",
                    "fields": [
                    {
                        "identifier": { "identifier": "public_int" },
                        "type": {
                            "rs_type": { "name": "i32", "type_args": [] },
                            "cc_type": {
                                "is_const": false,
                                "name": "int",
                                "type_args": []
                            }
                        },
                        "access": "Public",
                        "offset": 0
                    },
                    {
                        "identifier": { "identifier": "protected_int" },
                        "type": {
                            "rs_type": { "name": "i32", "type_args": [] },
                            "cc_type": {
                                "is_const": false,
                                "name": "int",
                                "type_args": []
                            }
                        },
                        "access": "Protected",
                        "offset": 32
                    },
                    {
                        "identifier": { "identifier": "private_int" },
                        "type": {
                            "rs_type": { "name": "i32", "type_args": [] },
                            "cc_type": {
                                "is_const": false,
                                "name": "int",
                                "type_args": []
                            }
                        },
                        "access": "Private",
                        "offset": 64
                    }
                    ],
                    "size": 12,
                    "alignment": 4,
                    "copy_constructor": {
                        "definition": "NontrivialSelf",
                        "access": "Private"
                    },
                    "move_constructor": {
                        "definition": "Deleted",
                        "access": "Protected"
                    },
                    "destructor": {
                        "definition": "Trivial",
                        "access": "Public"
                    },
                    "is_trivial_abi": true
                }}
            ]
        })j");
  IR ir = {.used_headers = {HeaderName("foo/bar.h")},
           .current_target = Label(std::string("//foo:bar")),
           .items = {Record{.identifier = Identifier("SomeStruct"),
                            .id = DeclId(42),
                            .owning_target = Label(std::string("//foo:bar")),
                            .fields =
                                {
                                    Field{
                                        .identifier = Identifier("public_int"),
                                        .type =
                                            MappedType::Simple("i32", "int"),
                                        .access = kPublic,
                                        .offset = 0},
                                    Field{.identifier =
                                              Identifier("protected_int"),
                                          .type = MappedType::Simple("i32",
                                                                     "int"),
                                          .access = kProtected,
                                          .offset = 32},
                                    Field{
                                        .identifier = Identifier("private_int"),
                                        .type = MappedType::Simple("i32",
                                                                   "int"),
                                        .access = kPrivate,
                                        .offset = 64},
                                },
                            .size = 12,
                            .alignment = 4,
                            .copy_constructor =
                                SpecialMemberFunc{
                                    .definition = SpecialMemberFunc::
                                        Definition::kNontrivialSelf,
                                    .access = kPrivate},
                            .move_constructor =
                                SpecialMemberFunc{
                                    .definition =
                                        SpecialMemberFunc::Definition::kDeleted,
                                    .access = kProtected},
                            .destructor =
                                SpecialMemberFunc{
                                    .definition =
                                        SpecialMemberFunc::Definition::kTrivial,
                                    .access = kPublic},
                            .is_trivial_abi = true}}};
  EXPECT_THAT(ir.ToJson(), EqualsJson(expected));
}

TEST(HeaderName, Hash) {
  EXPECT_TRUE(absl::VerifyTypeImplementsAbslHashCorrectly({
      HeaderName("foo.h"),
      HeaderName("bar.h"),
  }));
}

}  // namespace
}  // namespace rs_bindings_from_cc
