// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/generate_bindings_and_metadata.h"

#include <string>
#include <utility>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/container/flat_hash_map.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "common/ffi_types.h"
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
    "nowhere/llvm/rust/main_sysroot/bin/rustfmt";

constexpr absl::string_view kDefaultClangFormatExePath =
    "third_party/crosstool/google3_users/clang-format";

TEST(GenerateBindingsAndMetadataTest, GeneratingIR) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h"]}
  ])";

  ASSERT_OK_AND_ASSIGN(
      Cmdline cmdline,
      Cmdline::CreateForTesting(
          "//:target", "cc_out", "rs_out", "ir_out", "namespaces_out",
          /*crubit_support_path_format=*/
          "<crubit/support/path/for/test/{header}>",
          std::string(kDefaultClangFormatExePath),
          std::string(kDefaultRustfmtExePath), "nowhere/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */ {"a.h"}, std::string(kTargetsAndHeaders),
          /* extra_rs_srcs= */ {},
          /* srcs_to_scan_for_instantiations= */ {},
          /* instantiations_out= */ "",
          /* error_report_out= */ "", SourceLocationDocComment::Enabled));

  ASSERT_OK_AND_ASSIGN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(cmdline, DefaultClangArgs(),
                                  /*virtual_headers_contents_for_testing=*/
                                  {{HeaderName("a.h"), "namespace ns{}"}}));

  ASSERT_EQ(result.ir.public_headers.size(), 1);
  ASSERT_EQ(result.ir.public_headers.front().IncludePath(), "a.h");
  ASSERT_EQ(result.error_report, "");

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
          "//:target", "cc_out", "rs_out", "ir_out", "namespaces_out",
          /*crubit_support_path_format=*/
          "<crubit/support/path/for/test/{header}>",
          std::string(kDefaultClangFormatExePath),
          std::string(kDefaultRustfmtExePath), "nowhere/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */ {"a.h"}, std::string(kTargetsAndHeaders),
          /* extra_rs_srcs= */ {},
          /* srcs_to_scan_for_instantiations= */ {},
          /* instantiations_out= */ "",
          /* error_report_out= */ "", SourceLocationDocComment::Enabled));

  ASSERT_OK_AND_ASSIGN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(cmdline, DefaultClangArgs(),
                                  /*virtual_headers_contents_for_testing=*/
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
          "//:target", "cc_out", "rs_out", "ir_out", "namespaces_out",
          /*crubit_support_path_format=*/
          "<crubit/support/path/for/test/{header}>",
          std::string(kDefaultClangFormatExePath),
          std::string(kDefaultRustfmtExePath), "nowhere/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */
          {"a.h"}, std::string(kTargetsAndHeaders),
          /* extra_rs_srcs= */ {},
          /* srcs_to_scan_for_instantiations= */ {a_rs_path},
          "instantiations_out", /* error_report_out= */ "",
          SourceLocationDocComment::Enabled));

  CRUBIT_ASSIGN_OR_RETURN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(
          cmdline, DefaultClangArgs(),
          /*virtual_headers_contents_for_testing=*/
          {{HeaderName("a.h"), std::string(header_content)}}));

  return std::move(result.instantiations);
}

TEST(GenerateBindingsAndMetadataTest,
     RegularTypeAliasNotPresentInInstantiations) {
  ASSERT_OK_AND_ASSIGN(auto instantiations,
                       GetInstantiationsFor(
                           R"cc(
                             template <typename T>
                             class MyTemplate {};

                             using MyFunnyTemplate = MyTemplate<bool>;

                             template <typename T>
                             class ExpectedTemplate {};
                           )cc",
                           "cc_template!{ExpectedTemplate<bool>}"));

  ASSERT_THAT(instantiations,
              ElementsAre(Pair("ExpectedTemplate<bool>",
                               "__CcTemplateInst16ExpectedTemplateIbE")));
}

TEST(GenerateBindingsAndMetadataTest,
     ExplicitClassTemplateInstantiationDeclarationsNotPresentInInstantiations) {
  ASSERT_OK_AND_ASSIGN(auto instantiations,
                       GetInstantiationsFor(
                           R"cc(
                             template <typename T>
                             class MyTemplate {};

                             extern template class MyTemplate<bool>;

                             template <typename T>
                             class ExpectedTemplate {};
                           )cc",
                           "cc_template!{ExpectedTemplate<bool>}"));

  ASSERT_THAT(instantiations,
              ElementsAre(Pair("ExpectedTemplate<bool>",
                               "__CcTemplateInst16ExpectedTemplateIbE")));
}

TEST(GenerateBindingsAndMetadataTest,
     ExplicitClassTemplateInstantiationDefinitionsNotPresentInInstantiations) {
  ASSERT_OK_AND_ASSIGN(auto instantiations,
                       GetInstantiationsFor(
                           R"cc(
                             template <typename T>
                             class MyTemplate {};

                             template class MyTemplate<bool>;

                             template <typename T>
                             class ExpectedTemplate {};
                           )cc",
                           "cc_template!{ExpectedTemplate<bool>}"));

  ASSERT_THAT(instantiations,
              ElementsAre(Pair("ExpectedTemplate<bool>",
                               "__CcTemplateInst16ExpectedTemplateIbE")));
}

TEST(GenerateBindingsAndMetadataTest,
     RegularRecordsNotPresentInInstantiations) {
  ASSERT_OK_AND_ASSIGN(auto instantiations,
                       GetInstantiationsFor(
                           R"cc(
                             struct MyStruct {};

                             template <typename T>
                             class ExpectedTemplate {};
                           )cc",
                           "cc_template!{ExpectedTemplate<bool>}"));

  ASSERT_THAT(instantiations,
              ElementsAre(Pair("ExpectedTemplate<bool>",
                               "__CcTemplateInst16ExpectedTemplateIbE")));
}

TEST(GenerateBindingsAndMetadataTest,
     InstantiationsAreGeneratedForCcTemplateMacro) {
  ASSERT_OK_AND_ASSIGN(auto instantiations,
                       GetInstantiationsFor(
                           R"cc(
                             template <typename T>
                             class ExpectedTemplate {};
                           )cc",
                           "cc_template!{ExpectedTemplate<bool>}"));

  ASSERT_THAT(instantiations,
              ElementsAre(Pair("ExpectedTemplate<bool>",
                               "__CcTemplateInst16ExpectedTemplateIbE")));
}

TEST(GenerateBindingsAndMetadataTest, NamespacesJsonGenerated) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h"]}
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
  "label": "//:target1",
  "namespaces": [
    {
      "children": [
        {
          "children": [
            {
              "children": [],
              "name": "inner_1"
            },
            {
              "children": [],
              "name": "inner_2"
            }
          ],
          "name": "middle"
        }
      ],
      "name": "top_level_1"
    },
    {
      "children": [
        {
          "children": [],
          "name": "inner_3"
        }
      ],
      "name": "top_level_2"
    }
  ]
})";

  ASSERT_OK_AND_ASSIGN(
      Cmdline cmdline,
      Cmdline::CreateForTesting(
          "//:target1", "cc_out", "rs_out", "ir_out", "namespaces_json",
          /*crubit_support_path_format=*/
          "<crubit/support/path/for/test/{header}>",
          std::string(kDefaultClangFormatExePath),
          std::string(kDefaultRustfmtExePath), "nowhere/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */ {"a.h"}, std::string(kTargetsAndHeaders),
          /* extra_rs_srcs= */ {},
          /* srcs_to_scan_for_instantiations= */ {},
          /* instantiations_out= */ "", /* error_report_out= */ "",
          SourceLocationDocComment::Enabled));
  ASSERT_OK_AND_ASSIGN(BindingsAndMetadata result,
                       GenerateBindingsAndMetadata(
                           cmdline, DefaultClangArgs(),
                           /*virtual_headers_contents_for_testing=*/
                           {{HeaderName("a.h"), std::string(kHeaderContent)}}));

  ASSERT_THAT(NamespacesAsJson(result.namespaces), StrEq(kExpected));
}

}  // namespace
}  // namespace crubit
