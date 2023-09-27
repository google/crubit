// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/cmdline.h"

#include <string>
#include <utility>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "common/ffi_types.h"
#include "common/status_test_matchers.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"

namespace crubit {
namespace {

using ::testing::AllOf;
using ::testing::ElementsAre;
using ::testing::HasSubstr;
using ::testing::Pair;
using ::testing::UnorderedElementsAre;

namespace {

absl::StatusOr<Cmdline> TestCmdline(std::string target,
                                    std::vector<std::string> public_headers,
                                    std::string target_args) {
  return Cmdline::CreateForTesting(
      std::move(target), "cc_out", "rs_out", "ir_out", "namespaces_out",
      /*crubit_support_path_format=*/"<crubit/support/path/{header}>",
      "clang_format_exe_path", "rustfmt_exe_path", "rustfmt_config_path",

      /*do_nothing=*/false, std::move(public_headers), std::move(target_args),
      /* extra_rs_srcs= */ {},
      /* srcs_to_scan_for_instantiations= */ {},
      /* instantiations_out= */ "",
      /* error_report_out= */ "", SourceLocationDocComment::Disabled);
}

absl::StatusOr<Cmdline> TestCmdline(std::vector<std::string> public_headers,
                                    std::string target_args) {
  return TestCmdline("//:target", std::move(public_headers),
                     std::move(target_args));
}

}  // namespace

TEST(CmdlineTest, BasicCorrectInput) {
  ASSERT_OK_AND_ASSIGN(
      Cmdline cmdline,
      Cmdline::CreateForTesting(
          "//:t1", "cc_out", "rs_out", "ir_out", "namespaces_out",
          /*crubit_support_path_format=*/"<crubit/support/path/{header}>",
          "clang_format_exe_path", "rustfmt_exe_path", "rustfmt_config_path",
          /* do_nothing= */ false, {"h1"},
          R"([{"t": "//:t1", "h": ["h1", "h2"]}])", {"extra_file.rs"},
          {"scan_for_instantiations.rs"}, "instantiations_out",
          "error_report_out", SourceLocationDocComment::Disabled));
  EXPECT_EQ(cmdline.cc_out(), "cc_out");
  EXPECT_EQ(cmdline.rs_out(), "rs_out");
  EXPECT_EQ(cmdline.ir_out(), "ir_out");
  EXPECT_EQ(cmdline.namespaces_out(), "namespaces_out");
  EXPECT_EQ(cmdline.crubit_support_path_format(),
            "<crubit/support/path/{header}>");
  EXPECT_EQ(cmdline.clang_format_exe_path(), "clang_format_exe_path");
  EXPECT_EQ(cmdline.rustfmt_exe_path(), "rustfmt_exe_path");
  EXPECT_EQ(cmdline.rustfmt_config_path(), "rustfmt_config_path");
  EXPECT_EQ(cmdline.instantiations_out(), "instantiations_out");
  EXPECT_EQ(cmdline.error_report_out(), "error_report_out");
  EXPECT_EQ(cmdline.do_nothing(), false);
  EXPECT_EQ(cmdline.current_target().value(), "//:t1");
  EXPECT_THAT(cmdline.public_headers(), ElementsAre(HeaderName("h1")));
  EXPECT_THAT(cmdline.extra_rs_srcs(), ElementsAre("extra_file.rs"));
  EXPECT_THAT(cmdline.srcs_to_scan_for_instantiations(),
              ElementsAre("scan_for_instantiations.rs"));
  EXPECT_THAT(
      cmdline.headers_to_targets(),
      UnorderedElementsAre(Pair(HeaderName("h1"), BazelLabel("//:t1")),
                           Pair(HeaderName("h2"), BazelLabel("//:t1"))));
  EXPECT_EQ(cmdline.generate_source_location_in_doc_comment(),
            SourceLocationDocComment::Disabled);
}

TEST(CmdlineTest, TargetArgsEmpty) {
  ASSERT_THAT(TestCmdline({"h1"}, ""),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --target_args")));
}

TEST(CmdlineTest, TargetArgsInvalidJson) {
  ASSERT_THAT(
      TestCmdline({"h1"}, "#!$%"),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("--target_args"), HasSubstr("Invalid JSON"))));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfTopLevelArray) {
  ASSERT_THAT(TestCmdline({"h1"}, "123"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr("array"))));
}

TEST(CmdlineTest, TargetArgsIntInTopLevelArray) {
  ASSERT_THAT(TestCmdline({"h1"}, "[123, 456]"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"))));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfHeadersArray) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "//:t1", "h": 123}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr(".h"),
                             HasSubstr("array"))));
}

TEST(CmdlineTest, TargetArgsMissingTarget) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"h": ["h1", "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr(".t"),
                             HasSubstr("missing"))));
}

TEST(CmdlineTest, TargetArgsMissingHeader) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "//:t1"}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"),
                             HasSubstr("Couldn't find header"))));
}

TEST(CmdlineTest, TargetArgsEmptyHeader) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "t1", "h": ["", "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr("`h`"),
                             HasSubstr("empty string"))));
}
TEST(CmdlineTest, TargetArgsEmptyTarget) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "", "h": ["h1", "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr("`t`"),
                             HasSubstr("empty string"))));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfTarget) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": 123, "h": ["h1", "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr(".t"),
                             HasSubstr("string"))));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfHeader) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "//:t1", "h": [123, "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr(".h"),
                             HasSubstr("string"))));
}

TEST(CmdlineTest, TargetArgsDuplicateHeader) {
  for (const char* target : {"//:t1", "//:t2"}) {
    ASSERT_OK_AND_ASSIGN(Cmdline cmdline, TestCmdline(target, {"h1"}, R"([
        {"t": "//:t1", "h": ["h1"]},
        {"t": "//:t2", "h": ["h1", "h2"]} ])"));
    EXPECT_THAT(
        cmdline.headers_to_targets(),
        UnorderedElementsAre(Pair(HeaderName("h1"), BazelLabel("//:t1")),
                             Pair(HeaderName("h2"), BazelLabel("//:t2"))));
    EXPECT_EQ(cmdline.current_target().value(), target);
  }
}

TEST(CmdlineTest, PublicHeadersEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(TestCmdline({}, std::string(kTargetsAndHeaders)),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --public_headers")));
}

TEST(CmdlineTest, PublicHeadersWhereFirstHeaderMissingInMap) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      TestCmdline({"missing-in-map.h"}, std::string(kTargetsAndHeaders)),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          AllOf(HasSubstr("missing-in-map.h"), HasSubstr("Couldn't find"))));
}

TEST(CmdlineTest, PublicHeadersWhereSecondHeaderMissingInMap) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      TestCmdline({"a.h", "missing.h"}, std::string(kTargetsAndHeaders)),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("missing.h"), HasSubstr("Couldn't find"))));
}

TEST(CmdlineTest, PublicHeadersCoveringMultipleTargets) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]},
    {"t": "//:target2", "h": ["c.h", "d.h"]}
  ])";
  ASSERT_OK_AND_ASSIGN(Cmdline cmdline,
                       TestCmdline("//:target1", {"a.h", "c.h"},
                                   std::string(kTargetsAndHeaders)));
  EXPECT_EQ(cmdline.current_target().value(), "//:target1");
  EXPECT_THAT(
      cmdline.headers_to_targets(),
      UnorderedElementsAre(Pair(HeaderName("a.h"), BazelLabel("//:target1")),
                           Pair(HeaderName("b.h"), BazelLabel("//:target1")),
                           Pair(HeaderName("c.h"), BazelLabel("//:target2")),
                           Pair(HeaderName("d.h"), BazelLabel("//:target2"))));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfFeaturesArray) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "t1", "f": 123}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr(".f"),
                             HasSubstr("array"))));
}

TEST(CmdlineTest, TargetArgsEmptyFeature) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "t1", "f": ["", "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr("`f`"),
                             HasSubstr("empty string"))));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfFeature) {
  ASSERT_THAT(
      TestCmdline({"h1"}, R"([{"t": "t1", "f": [123, "experimental"]}])"),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("--target_args"), HasSubstr(".f"),
                     HasSubstr("string"))));
}

TEST(CmdlineTest, InstantiationsOutEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      (Cmdline::CreateForTesting(
          "//:target1", "cc_out", "rs_out", "ir_out", "namespaces_out",
          /*crubit_support_path_format=*/"<crubit/support/path/{header}>",
          "clang_format_exe_path", "rustfmt_exe_path", "rustfmt_config_path",
          /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
          /* extra_rs_srcs= */ {}, {"lib.rs"},
          /* instantiations_out= */ "", "error_report_out",
          SourceLocationDocComment::Enabled)),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr(
              "please specify both --rust_sources and --instantiations_out "
              "when requesting a template instantiation mode")));
}

TEST(CmdlineTest, RustSourcesEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      Cmdline::CreateForTesting(
          "//:target1", "cc_out", "rs_out", "ir_out", "namespaces_out",
          /*crubit_support_path_format=*/"<crubit/support/path/{header}>",
          "clang_format_exe_path", "rustfmt_exe_path", "rustfmt_config_path",
          /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
          /* extra_rs_srcs= */ {},
          /* srcs_to_scan_for_instantiations= */ {}, "instantiations_out",
          "error_report_out", SourceLocationDocComment::Enabled),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr(
              "please specify both --rust_sources and --instantiations_out "
              "when requesting a template instantiation mode")));
}

TEST(CmdlineTest, CcOutEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      Cmdline::CreateForTesting(
          "//:target1",
          /* cc_out= */ "", "rs_out", "ir_out", "namespaces_out",
          /*crubit_support_path_format=*/"<crubit/support/path/{header}>",
          "clang_format_exe_path", "rustfmt_exe_path", "rustfmt_config_path",
          /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
          /* extra_rs_srcs= */ {},
          /* srcs_to_scan_for_instantiations= */ {},
          /* instantiations_out= */ "", "error_report_out",
          SourceLocationDocComment::Enabled),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("please specify --cc_out")));
}

TEST(CmdlineTest, RsOutEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      Cmdline::CreateForTesting(
          "//:target1", "cc_out", /* rs_out= */ "", "namespaces_out", "ir_out",
          /*crubit_support_path_format=*/"<crubit/support/path/{header}>",
          "clang_format_exe_path", "rustfmt_exe_path", "rustfmt_config_path",
          /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
          /* extra_rs_srcs= */ {},
          /* srcs_to_scan_for_instantiations= */ {},
          /* instantiations_out= */ "", "error_report_out",
          SourceLocationDocComment::Enabled),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("please specify --rs_out")));
}

TEST(CmdlineTest, IrOutEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_OK(Cmdline::CreateForTesting(
      "//:target1", "cc_out", "rs_out", /* ir_out= */ "", "namespaces_out",
      /* crubit_support_path_format= */ "<crubit/support/path/{header}>",
      "clang_format_exe_path", "rustfmt_exe_path", "rustfmt_config_path",
      /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
      /* extra_rs_srcs= */ {},
      /* srcs_to_scan_for_instantiations= */ {},
      /* instantiations_out= */ "", "error_report_out",
      SourceLocationDocComment::Enabled));
}

TEST(CmdlineTest, ClangFormatExePathEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      Cmdline::CreateForTesting(
          "//:target1", "cc_out", "rs_out", "ir_out", "namespaces_out",
          /*crubit_support_path_format=*/"<crubit/support/path/{header}>",
          /* clang_format_exe_path= */ "", "rustfmt_exe_path",
          "rustfmt_config_path",
          /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
          /* extra_rs_srcs= */ {},
          /* srcs_to_scan_for_instantiations= */ {},
          /* instantiations_out= */ "", "error_report_out",
          SourceLocationDocComment::Enabled),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("please specify --clang_format_exe_path")));
}

TEST(CmdlineTest, RustfmtExePathEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      Cmdline::CreateForTesting(
          "//:target1", "cc_out", "rs_out", "ir_out", "namespaces_out",
          /*crubit_support_path_format=*/"<crubit/support/path/{header}>",
          "clang_format_exe_path",
          /* rustfmt_exe_path= */ "", "rustfmt_config_path",
          /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
          /* extra_rs_srcs= */ {},
          /* srcs_to_scan_for_instantiations= */ {},
          /* instantiations_out= */ "", "error_report_out",
          SourceLocationDocComment::Enabled),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("please specify --rustfmt_exe_path")));
}

TEST(CmdlineTest, SupportPathEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      Cmdline::CreateForTesting(
          "//:target1", "cc_out", "rs_out", "ir_out", "namespaces_out",
          /*crubit_support_path_format=*/"", "clang_format_exe_path",
          /* rustfmt_exe_path= */ "", "rustfmt_config_path",
          /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
          /* extra_rs_srcs= */ {},
          /* srcs_to_scan_for_instantiations= */ {},
          /* instantiations_out= */ "", "error_report_out",
          SourceLocationDocComment::Enabled),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("please specify --crubit_support_path_format")));
}

TEST(CmdlineTest, SupportPathNoPlaceholder) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      Cmdline::CreateForTesting(
          "//:target1", "cc_out", "rs_out", "ir_out", "namespaces_out",
          /*crubit_support_path_format=*/"<crubit/support/path>",
          "clang_format_exe_path",
          /* rustfmt_exe_path= */ "", "rustfmt_config_path",
          /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
          /* extra_rs_srcs= */ {},
          /* srcs_to_scan_for_instantiations= */ {},
          /* instantiations_out= */ "", "error_report_out",
          SourceLocationDocComment::Enabled),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("cannot find `{header}` placeholder in "
                         "crubit_support_path_format")));
}

}  // namespace
}  // namespace crubit
