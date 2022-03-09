// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/cmdline.h"

#include <string>
#include <vector>

#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "util/task/status_macros.h"

namespace rs_bindings_from_cc {
namespace {

using ::testing::AllOf;
using ::testing::ElementsAre;
using ::testing::HasSubstr;
using ::testing::Pair;
using ::testing::UnorderedElementsAre;
using ::testing::status::StatusIs;

namespace {

absl::StatusOr<Cmdline> TestCmdline(std::vector<std::string> public_headers,
                                    const std::string& targets_and_headers) {
  return Cmdline::CreateForTesting("cc_out", "rs_out", "ir_out",
                                   /* do_nothing= */ false, public_headers,
                                   targets_and_headers);
}

}  // namespace

TEST(CmdlineTest, BasicCorrectInput) {
  ASSERT_OK_AND_ASSIGN(
      Cmdline cmdline,
      Cmdline::CreateForTesting("cc_out", "rs_out", "ir_out",
                                /* do_nothing= */ false, {"h1"},
                                R"([{"t": "t1", "h": ["h1", "h2"]}])"));
  EXPECT_EQ(cmdline.cc_out(), "cc_out");
  EXPECT_EQ(cmdline.rs_out(), "rs_out");
  EXPECT_EQ(cmdline.ir_out(), "ir_out");
  EXPECT_EQ(cmdline.do_nothing(), false);
  EXPECT_EQ(cmdline.current_target().value(), "t1");
  EXPECT_THAT(cmdline.public_headers(), ElementsAre(HeaderName("h1")));
  EXPECT_THAT(cmdline.headers_to_targets(),
              UnorderedElementsAre(Pair(HeaderName("h1"), BlazeLabel("t1")),
                                   Pair(HeaderName("h2"), BlazeLabel("t1"))));
}

TEST(CmdlineTest, TargetsAndHeadersEmpty) {
  ASSERT_THAT(TestCmdline({"h1"}, ""),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --targets_and_headers")));
}

TEST(CmdlineTest, TargetsAndHeadersInvalidJson) {
  ASSERT_THAT(
      TestCmdline({"h1"}, "#!$%"),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("--targets_and_headers"), HasSubstr("array"))));
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
                             HasSubstr("`h`"), HasSubstr("array"))));
}

TEST(CmdlineTest, TargetsAndHeadersMissingTarget) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"h": ["h1", "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--targets_and_headers"),
                             HasSubstr("`t`"), HasSubstr("Missing"))));
}

TEST(CmdlineTest, TargetsAndHeadersMissingHeader) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "t1"}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--targets_and_headers"),
                             HasSubstr("`h`"), HasSubstr("Missing"))));
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
                             HasSubstr("`t`"), HasSubstr("string"))));
}

TEST(CmdlineTest, TargetsAndHeadersIntInsteadOfHeader) {
  ASSERT_THAT(TestCmdline({"h1"}, R"([{"t": "t1", "h": [123, "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--targets_and_headers"),
                             HasSubstr("`h`"), HasSubstr("string"))));
}

TEST(CmdlineTest, TargetsAndHeadersDuplicateHeader) {
  constexpr char kTestInput[] = R"([
      {"t": "t1", "h": ["h1"]},
      {"t": "t2", "h": ["h1"]} ])";
  ASSERT_THAT(
      TestCmdline({"h1"}, kTestInput),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("--targets_and_headers"), HasSubstr("conflict"),
                     HasSubstr("h1"), HasSubstr("t1"), HasSubstr("t2"))));
}

TEST(CmdlineTest, PublicHeadersEmpty) {
  constexpr char kTargetsAndHeaders[] = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(TestCmdline({}, kTargetsAndHeaders),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --public_headers")));
}

TEST(CmdlineTest, PublicHeadersWhereFirstHeaderMissingInMap) {
  constexpr char kTargetsAndHeaders[] = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(TestCmdline({"missing-in-map.h"}, kTargetsAndHeaders),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("missing-in-map.h"),
                             HasSubstr("Couldn't find"))));
}

TEST(CmdlineTest, PublicHeadersWhereSecondHeaderMissingInMap) {
  constexpr char kTargetsAndHeaders[] = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      TestCmdline({"a.h", "missing.h"}, kTargetsAndHeaders),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("missing.h"), HasSubstr("Couldn't find"))));
}

TEST(CmdlineTest, PublicHeadersCoveringMultipleTargets) {
  constexpr char kTargetsAndHeaders[] = R"([
    {"t": "target1", "h": ["a.h", "b.h"]},
    {"t": "target2", "h": ["c.h", "d.h"]}
  ])";
  ASSERT_THAT(TestCmdline({"a.h", "c.h"}, kTargetsAndHeaders),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("Expected all public headers to belong "
                                       "to the current target"),
                             HasSubstr("target1"), HasSubstr("target2"),
                             HasSubstr("c.h"))));
}

TEST(CmdlineTest, CcOutEmpty) {
  constexpr char kTargetsAndHeaders[] = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(Cmdline::CreateForTesting(
                  /* cc_out= */ "", "rs_out", "ir_out", /* do_nothing= */ false,
                  {"a.h"}, kTargetsAndHeaders),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --cc_out")));
}

TEST(CmdlineTest, RsOutEmpty) {
  constexpr char kTargetsAndHeaders[] = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(Cmdline::CreateForTesting("cc_out", /* rs_out= */ "", "ir_out",
                                        /* do_nothing= */ false, {"a.h"},
                                        kTargetsAndHeaders),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --rs_out")));
}

TEST(CmdlineTest, IrOutEmpty) {
  constexpr char kTargetsAndHeaders[] = R"([
    {"t": "target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_OK(Cmdline::CreateForTesting("cc_out", "rs_out", /* ir_out= */ "",
                                      /* do_nothing= */ false, {"a.h"},
                                      kTargetsAndHeaders));
}

}  // namespace
}  // namespace rs_bindings_from_cc
