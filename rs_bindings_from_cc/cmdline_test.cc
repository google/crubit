// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/cmdline.h"

#include <string>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "common/status_test_matchers.h"
#include "rs_bindings_from_cc/bazel_types.h"

namespace crubit {
namespace {

using ::testing::AllOf;
using ::testing::ElementsAre;
using ::testing::HasSubstr;
using ::testing::Pair;
using ::testing::UnorderedElementsAre;

namespace {

absl::StatusOr<Cmdline> TestCmdline(std::vector<std::string> public_headers,
                                    const std::string& targets_and_headers) {
  return Cmdline::CreateForTesting(
      "cc_out", "rs_out", "ir_out", "crubit_support_path", "rustfmt_exe_path",
      "rustfmt_config_path",

      /* do_nothing= */ false, std::move(public_headers),
      std::move(targets_and_headers), /* rust_sources= */ {},
      /* instantiations_out= */ "");
}

}  // namespace

TEST(CmdlineTest, BasicCorrectInput) {
  ASSERT_OK_AND_ASSIGN(Cmdline cmdline,
                       Cmdline::CreateForTesting(
                           "cc_out", "rs_out", "ir_out", "crubit_support_path",
                           "rustfmt_exe_path", "rustfmt_config_path",
                           /* do_nothing= */ false, {"h1"},
                           R"([{"t": "t1", "h": ["h1", "h2"]}])", {"lib.rs"},
                           "instantiations_out"));
  EXPECT_EQ(cmdline.cc_out(), "cc_out");
  EXPECT_EQ(cmdline.rs_out(), "rs_out");
  EXPECT_EQ(cmdline.ir_out(), "ir_out");
  EXPECT_EQ(cmdline.crubit_support_path(), "crubit_support_path");
  EXPECT_EQ(cmdline.rustfmt_exe_path(), "rustfmt_exe_path");
  EXPECT_EQ(cmdline.rustfmt_config_path(), "rustfmt_config_path");
  EXPECT_EQ(cmdline.instantiations_out(), "instantiations_out");
  EXPECT_EQ(cmdline.do_nothing(), false);
  EXPECT_EQ(cmdline.current_target().value(), "t1");
  EXPECT_THAT(cmdline.public_headers(), ElementsAre(HeaderName("h1")));
  EXPECT_THAT(cmdline.rust_sources(), ElementsAre("lib.rs"));
  EXPECT_THAT(cmdline.headers_to_targets(),
              UnorderedElementsAre(Pair(HeaderName("h1"), BazelLabel("t1")),
                                   Pair(HeaderName("h2"), BazelLabel("t1"))));
}

TEST(CmdlineTest, TargetsAndHeadersEmpty) {
  ASSERT_THAT(TestCmdline({"h1"}, ""),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --targets_and_headers")));
}

TEST(CmdlineTest, TargetsAndHeadersInvalidJson) {
  ASSERT_THAT(TestCmdline({"h1"}, "#!$%"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--targets_and_headers"),
                             HasSubstr("Invalid JSON"))));
}

TEST(CmdlineTest, TargetsAndHeadersIntInsteadOfTopLevelArray) {
  ASSERT_THAT(
      TestCmdline({"h1"}, "123"),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("--targets_and_headers"), HasSubstr("array"))));
}

TEST(CmdlineTest, TargetsAndHeadersIntInTopLevelArray) {
  ASSERT_THAT(TestCmdline({"h1"}, "[123, 456]"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--targets_and_headers"))));
}

TEST(CmdlineTest, TargetsAndHeadersIntInsteadOfHeadersArray) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "t1", "h": 123}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--targets_and_headers"),
                             HasSubstr(".h"), HasSubstr("array"))));
}

TEST(CmdlineTest, TargetsAndHeadersMissingTarget) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"h": ["h1", "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--targets_and_headers"),
                             HasSubstr(".t"), HasSubstr("missing"))));
}

TEST(CmdlineTest, TargetsAndHeadersMissingHeader) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "t1"}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--targets_and_headers"),
                             HasSubstr(".h"), HasSubstr("missing"))));
}

TEST(CmdlineTest, TargetsAndHeadersEmptyHeader) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "t1", "h": ["", "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--targets_and_headers"),
                             HasSubstr("`h`"), HasSubstr("empty string"))));
}

TEST(CmdlineTest, TargetsAndHeadersEmptyTarget) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "", "h": ["h1", "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--targets_and_headers"),
                             HasSubstr("`t`"), HasSubstr("empty string"))));
}

TEST(CmdlineTest, TargetsAndHeadersIntInsteadOfTarget) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": 123, "h": ["h1", "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--targets_and_headers"),
                             HasSubstr(".t"), HasSubstr("string"))));
}

TEST(CmdlineTest, TargetsAndHeadersIntInsteadOfHeader) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "t1", "h": [123, "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--targets_and_headers"),
                             HasSubstr(".h"), HasSubstr("string"))));
}

TEST(CmdlineTest, TargetsAndHeadersDuplicateHeader) {
  constexpr absl::string_view kTestInput = R"([
      {"t": "t1", "h": ["h1"]},
      {"t": "t2", "h": ["h1"]} ])";
  ASSERT_THAT(
      TestCmdline({"h1"}, std::string(kTestInput)),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("--targets_and_headers"), HasSubstr("conflict"),
                     HasSubstr("h1"), HasSubstr("t1"), HasSubstr("t2"))));
}

TEST(CmdlineTest, PublicHeadersEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(TestCmdline({}, std::string(kTargetsAndHeaders)),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --public_headers")));
}

TEST(CmdlineTest, PublicHeadersWhereFirstHeaderMissingInMap) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      TestCmdline({"missing-in-map.h"}, std::string(kTargetsAndHeaders)),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          AllOf(HasSubstr("missing-in-map.h"), HasSubstr("Couldn't find"))));
}

TEST(CmdlineTest, PublicHeadersWhereSecondHeaderMissingInMap) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      TestCmdline({"a.h", "missing.h"}, std::string(kTargetsAndHeaders)),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("missing.h"), HasSubstr("Couldn't find"))));
}

TEST(CmdlineTest, PublicHeadersCoveringMultipleTargets) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h", "b.h"]},
    {"t": "target2", "h": ["c.h", "d.h"]}
  ])";
  ASSERT_THAT(TestCmdline({"a.h", "c.h"}, std::string(kTargetsAndHeaders)),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("Expected all public headers to belong "
                                       "to the current target"),
                             HasSubstr("target1"), HasSubstr("target2"),
                             HasSubstr("c.h"))));
}

TEST(CmdlineTest, InstantiationsOutEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      (Cmdline::CreateForTesting("cc_out", "rs_out", "ir_out",
                                 "crubit_support_path", "rustfmt_exe_path",
                                 "rustfmt_config_path",
                                 /* do_nothing= */ false, {"a.h"},
                                 std::string(kTargetsAndHeaders), {"lib.rs"},
                                 /* instantiations_out= */ "")),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr(
              "please specify both --rust_sources and --instantiations_out "
              "when requesting a template instantiation mode")));
}

TEST(CmdlineTest, RustSourcesEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      Cmdline::CreateForTesting(
          "cc_out", "rs_out", "ir_out", "crubit_support_path",
          "rustfmt_exe_path", "rustfmt_config_path",
          /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
          /* rust_sources= */ {}, "instantiations_out"),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr(
              "please specify both --rust_sources and --instantiations_out "
              "when requesting a template instantiation mode")));
}

TEST(CmdlineTest, CcOutEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(Cmdline::CreateForTesting(
                  /* cc_out= */ "", "rs_out", "ir_out", "crubit_support_path",
                  "rustfmt_exe_path", "rustfmt_config_path",
                  /* do_nothing= */ false, {"a.h"},
                  std::string(kTargetsAndHeaders), /* rust_sources= */ {},
                  /* instantiations_out= */ ""),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --cc_out")));
}

TEST(CmdlineTest, RsOutEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      Cmdline::CreateForTesting(
          "cc_out", /* rs_out= */ "", "ir_out", "crubit_support_path",
          "rustfmt_exe_path", "rustfmt_config_path",
          /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
          /* rust_sources= */ {},
          /* instantiations_out= */ ""),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("please specify --rs_out")));
}

TEST(CmdlineTest, IrOutEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_OK(Cmdline::CreateForTesting(
      "cc_out", "rs_out", /* ir_out= */ "", "crubit_support_path",
      "rustfmt_exe_path", "rustfmt_config_path",
      /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
      /* rust_sources= */ {},
      /* instantiations_out= */ ""));
}

TEST(CmdlineTest, RustfmtExePathEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      Cmdline::CreateForTesting(
          "cc_out", "rs_out", "ir_out", "crubit_support_path",
          /* rustfmt_exe_path= */ "", "rustfmt_config_path",
          /* do_nothing= */ false, {"a.h"}, std::string(kTargetsAndHeaders),
          /* rust_sources= */ {},
          /* instantiations_out= */ ""),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("please specify --rustfmt_exe_path")));
}

}  // namespace
}  // namespace crubit
