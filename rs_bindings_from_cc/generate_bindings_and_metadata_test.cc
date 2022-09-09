// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/generate_bindings_and_metadata.h"

#include <string>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "common/status_macros.h"
#include "common/test_utils.h"
#include "rs_bindings_from_cc/cmdline.h"
#include "rs_bindings_from_cc/collect_namespaces.h"
#include "rs_bindings_from_cc/ir.h"

namespace crubit {
namespace {

using ::testing::ElementsAre;
using ::testing::IsEmpty;
using ::testing::Pair;
using ::testing::StrEq;

constexpr absl::string_view kDefaultRustfmtExePath =
    "third_party/unsupported_toolchains/rust/toolchains/nightly/bin/rustfmt";

TEST(GenerateBindingsAndMetadataTest, GeneratingIR) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h"]}
  ])";

  ASSERT_OK_AND_ASSIGN(
      Cmdline cmdline,
      Cmdline::CreateForTesting(
          "cc_out", "rs_out", "ir_out", "namespaces_out", "crubit_support_path",
          std::string(kDefaultRustfmtExePath), "nowhere/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */ {"a.h"}, std::string(kTargetsAndHeaders),
          /* rust_sources= */ {},
          /* instantiations_out= */ ""));

  ASSERT_OK_AND_ASSIGN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(cmdline, DefaultClangArgs(),
                                  /* virtual_headers_contents= */
                                  {{HeaderName("a.h"), "namespace ns{}"}}));

  ASSERT_EQ(result.ir.used_headers.size(), 1);
  ASSERT_EQ(result.ir.used_headers.front().IncludePath(), "a.h");

  // Check that IR items have the proper owning target set.
  auto item = result.ir.get_items_if<Namespace>().front();
  ASSERT_EQ(item->owning_target.value(), "target1");
}

TEST(GenerateBindingsAndMetadataTest, InstantiationsAreEmptyInNormalMode) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h"]}
  ])";
  ASSERT_OK_AND_ASSIGN(
      Cmdline cmdline,
      Cmdline::CreateForTesting(
          "cc_out", "rs_out", "ir_out", "namespaces_out", "crubit_support_path",
          std::string(kDefaultRustfmtExePath), "nowhere/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */ {"a.h"}, std::string(kTargetsAndHeaders),
          /* rust_sources= */ {},
          /* instantiations_out= */ ""));

  ASSERT_OK_AND_ASSIGN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(cmdline, DefaultClangArgs(),
                                  /* virtual_headers_contents= */
                                  {{HeaderName("a.h"), "// empty header"}}));

  ASSERT_THAT(result.instantiations, IsEmpty());
}

absl::StatusOr<absl::flat_hash_map<std::string, std::string>>
GetInstantiationsFor(absl::string_view header_content,
                     absl::string_view rust_source) {
  std::string a_rs_path = WriteFileForCurrentTest("a.rs", rust_source);
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h"]}
  ])";

  CRUBIT_ASSIGN_OR_RETURN(
      Cmdline cmdline,
      Cmdline::CreateForTesting(
          "cc_out", "rs_out", "ir_out", "namespaces_out", "crubit_support_path",
          std::string(kDefaultRustfmtExePath), "nowhere/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */
          {"a.h"}, std::string(kTargetsAndHeaders),
          /* rust_sources= */ {a_rs_path}, "instantiations_out"));

  CRUBIT_ASSIGN_OR_RETURN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(
          cmdline, DefaultClangArgs(),
          /* virtual_headers_contents= */
          {{HeaderName("a.h"), std::string(header_content)}}));

  return std::move(result.instantiations);
}

TEST(GenerateBindingsAndMetadataTest,
     InstantiationsEmptyForTypedeffedTemplates) {
  ASSERT_OK_AND_ASSIGN(auto instantiations, GetInstantiationsFor(
                                                R"cc(
                                                  template <typename T>
                                                  class MyTemplate {
                                                    T t;
                                                  };

                                                  using MyFunnyTemplate = MyTemplate<bool>;
                                                )cc",
                                                ""));

  ASSERT_THAT(instantiations, IsEmpty());
}

TEST(GenerateBindingsAndMetadataTest,
     InstantiationsEmptyForExplicitInstantiationDeclarations) {
  ASSERT_OK_AND_ASSIGN(auto instantiations, GetInstantiationsFor(
                                                R"cc(
                                                  template <typename T>
                                                  class MyTemplate {
                                                    T t;
                                                  };

                                                  extern template class MyTemplate<bool>;
                                                )cc",
                                                ""));

  ASSERT_THAT(instantiations, IsEmpty());
}

// This test only documents *currently* expected behavior, but don't take is as
// a requirement that we put all explicit class template instantiation defitions
// into the JSON file. Only the instantiations triggered by `cc_template!` are
// required to appear there. For example we may decide in the future to not
// import instantiations that are not requested from `cc_template!` and then
// this test could be safely deleted.
TEST(GenerateBindingsAndMetadataTest,
     InstantiationsGeneratedForExplicitClassTemplateInstantiationDefinitions) {
  ASSERT_OK_AND_ASSIGN(auto instantiations, GetInstantiationsFor(
                                                R"cc(
                                                  template <typename T>
                                                  class MyTemplate {
                                                    T t;
                                                  };

                                                  template class MyTemplate<bool>;

                                                  using MyTypeAlias = MyTemplate<bool>;
                                                )cc",
                                                ""));

  ASSERT_THAT(
      instantiations,
      ElementsAre(Pair("MyTemplate<bool>", "__CcTemplateInst10MyTemplateIbE")));
}

TEST(GenerateBindingsAndMetadataTest, NamespacesJsonGenerated) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h"]}
  ])";
  constexpr absl::string_view kHeaderContent = R"(
    namespace top_level_1 {
      namespace middle {
        namespace inner_1 {}
      }
      namespace middle {
        namespace inner_2 {}
      }
    }

    namespace top_level_2 {
      namespace inner_3 {}
    }

    namespace top_level_1 {}
  )";
  constexpr absl::string_view kExpected = R"({
  "namespaces": [
    {
      "namespace": {
        "children": [
          {
            "namespace": {
              "children": [
                {
                  "namespace": {
                    "children": [],
                    "name": "inner_1"
                  }
                },
                {
                  "namespace": {
                    "children": [],
                    "name": "inner_2"
                  }
                }
              ],
              "name": "middle"
            }
          }
        ],
        "name": "top_level_1"
      }
    },
    {
      "namespace": {
        "children": [
          {
            "namespace": {
              "children": [],
              "name": "inner_3"
            }
          }
        ],
        "name": "top_level_2"
      }
    }
  ]
})";

  ASSERT_OK_AND_ASSIGN(
      Cmdline cmdline,
      Cmdline::CreateForTesting(
          "cc_out", "rs_out", "ir_out", "namespaces_json",
          "crubit_support_path", std::string(kDefaultRustfmtExePath),
          "nowhere/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */ {"a.h"}, std::string(kTargetsAndHeaders),
          /* rust_sources= */ {}, /* instantiations_out= */ ""));
  ASSERT_OK_AND_ASSIGN(BindingsAndMetadata result,
                       GenerateBindingsAndMetadata(
                           cmdline, DefaultClangArgs(),
                           /* virtual_headers_contents= */
                           {{HeaderName("a.h"), std::string(kHeaderContent)}}));

  ASSERT_THAT(NamespacesAsJson(result.namespaces), StrEq(kExpected));
}

}  // namespace
}  // namespace crubit
