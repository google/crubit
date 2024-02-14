// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/cmdline.h"

#include <fstream>
#include <initializer_list>
#include <string>
#include <utility>
#include <vector>

#include "gmock/gmock.h"
#include "testing/base/public/googletest.h"
#include "gtest/gtest.h"
#include "absl/flags/flag.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/ffi_types.h"
#include "common/status_macros.h"
#include "common/status_test_matchers.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/cmdline_flags.h"
#include "rs_bindings_from_cc/ir.h"

namespace crubit {
namespace {

using ::testing::AllOf;
using ::testing::ElementsAre;
using ::testing::HasSubstr;
using ::testing::Pair;
using ::testing::UnorderedElementsAre;

absl::StatusOr<CmdlineArgs> TestCmdlineArgs(
    std::string target, std::vector<std::string> public_headers,
    absl::string_view target_args) {
  auto args = CmdlineArgs{
      .current_target = BazelLabel(std::move(target)),
      .cc_out = "cc_out",
      .rs_out = "rs_out",
      .ir_out = "ir_out",
      .namespaces_out = "namespaces_out",
      .crubit_support_path_format = "<crubit/support/path/{header}>",
      .clang_format_exe_path = "clang_format_exe_path",
      .rustfmt_exe_path = "rustfmt_exe_path",
      .rustfmt_config_path = "rustfmt_config_path",
      .generate_source_location_in_doc_comment =
          SourceLocationDocComment::Disabled};
  std::transform(public_headers.begin(), public_headers.end(),
                 std::back_inserter(args.public_headers),
                 [](std::string header) { return HeaderName(header); });
  CRUBIT_RETURN_IF_ERROR(internal::ParseTargetArgs(target_args, args));
  CRUBIT_ASSIGN_OR_RETURN(Cmdline cmdline, Cmdline::Create(std::move(args)));
  return std::move(cmdline).args();
}

absl::StatusOr<CmdlineArgs> TestCmdlineArgs(
    std::vector<std::string> public_headers, absl::string_view target_args) {
  return TestCmdlineArgs("//:target", std::move(public_headers),
                         std::move(target_args));
}

// Returns an example valid test command line.
absl::StatusOr<CmdlineArgs> TestCmdlineArgs() {
  return TestCmdlineArgs("//:target", {"h1"},
                         R"([{"t": "//:target", "h": ["h1"]}])");
}

/// TestCmdlineArgs() above needs to be valid...
TEST(CmdlineTest, TestCmdlineArgs) { ASSERT_OK(TestCmdlineArgs().status()); }

TEST(CmdlineTest, BasicCorrectInput) {
  absl::SetFlag(&FLAGS_do_nothing, false);
  absl::SetFlag(&FLAGS_rs_out, "rs_out");
  absl::SetFlag(&FLAGS_cc_out, "cc_out");
  absl::SetFlag(&FLAGS_ir_out, "ir_out");
  absl::SetFlag(&FLAGS_crubit_support_path_format,
                "<crubit/support/path/{header}>");
  absl::SetFlag(&FLAGS_clang_format_exe_path, "clang_format_exe_path");
  absl::SetFlag(&FLAGS_rustfmt_exe_path, "rustfmt_exe_path");
  absl::SetFlag(&FLAGS_rustfmt_config_path, "rustfmt_config_path");
  absl::SetFlag(&FLAGS_public_headers, {"h1"});
  absl::SetFlag(&FLAGS_target, "//:t1");
  absl::SetFlag(&FLAGS_target_args, R"([{"t": "//:t1", "h": ["h1", "h2"]}])");
  absl::SetFlag(&FLAGS_extra_rs_srcs, {"extra_file.rs"});
  absl::SetFlag(&FLAGS_srcs_to_scan_for_instantiations,
                {"scan_for_instantiations.rs"});
  absl::SetFlag(&FLAGS_instantiations_out, "instantiations_out");
  absl::SetFlag(&FLAGS_namespaces_out, "namespaces_out");
  absl::SetFlag(&FLAGS_error_report_out, "error_report_out");
  absl::SetFlag(&FLAGS_generate_source_location_in_doc_comment,
                SourceLocationDocComment::Disabled);
  ASSERT_OK_AND_ASSIGN(Cmdline cmdline, Cmdline::FromFlags());
  const CmdlineArgs& args = cmdline.args();
  EXPECT_EQ(args.cc_out, "cc_out");
  EXPECT_EQ(args.rs_out, "rs_out");
  EXPECT_EQ(args.ir_out, "ir_out");
  EXPECT_EQ(args.namespaces_out, "namespaces_out");
  EXPECT_EQ(args.crubit_support_path_format, "<crubit/support/path/{header}>");
  EXPECT_EQ(args.clang_format_exe_path, "clang_format_exe_path");
  EXPECT_EQ(args.rustfmt_exe_path, "rustfmt_exe_path");
  EXPECT_EQ(args.rustfmt_config_path, "rustfmt_config_path");
  EXPECT_EQ(args.instantiations_out, "instantiations_out");
  EXPECT_EQ(args.error_report_out, "error_report_out");
  EXPECT_EQ(args.do_nothing, false);
  EXPECT_EQ(args.current_target.value(), "//:t1");
  EXPECT_THAT(args.public_headers, ElementsAre(HeaderName("h1")));
  EXPECT_THAT(args.extra_rs_srcs, ElementsAre("extra_file.rs"));
  EXPECT_THAT(args.srcs_to_scan_for_instantiations,
              ElementsAre("scan_for_instantiations.rs"));
  EXPECT_THAT(
      args.headers_to_targets,
      UnorderedElementsAre(Pair(HeaderName("h1"), BazelLabel("//:t1")),
                           Pair(HeaderName("h2"), BazelLabel("//:t1"))));
  EXPECT_EQ(args.generate_source_location_in_doc_comment,
            SourceLocationDocComment::Disabled);
}

TEST(CmdlineTest, TargetArgsEmpty) {
  CmdlineArgs args;
  EXPECT_THAT(internal::ParseTargetArgs("", args),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --target_args")));
}

TEST(CmdlineTest, TargetArgsInvalidJson) {
  CmdlineArgs args;
  EXPECT_THAT(
      internal::ParseTargetArgs("#!$%", args),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("--target_args"), HasSubstr("Invalid JSON"))));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfTopLevelArray) {
  CmdlineArgs args;
  EXPECT_THAT(internal::ParseTargetArgs("123", args),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr("array"))));
}

TEST(CmdlineTest, TargetArgsIntInTopLevelArray) {
  CmdlineArgs args;
  EXPECT_THAT(internal::ParseTargetArgs("[123, 456]", args),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"))));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfHeadersArray) {
  CmdlineArgs args;
  EXPECT_THAT(internal::ParseTargetArgs(R"([{"t": "//:t1", "h": 123}])", args),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr(".h"),
                             HasSubstr("array"))));
}

TEST(CmdlineTest, TargetArgsMissingTarget) {
  CmdlineArgs args;
  EXPECT_THAT(internal::ParseTargetArgs(R"([{"h": ["h1", "h2"]}])", args),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr(".t"),
                             HasSubstr("missing"))));
}

TEST(CmdlineTest, TargetArgsMissingHeader) {
  EXPECT_THAT(TestCmdlineArgs("//:t1", {"h1"}, R"([{"t": "//:t1"}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"),
                             HasSubstr("Couldn't find header"))));
}

TEST(CmdlineTest, TargetArgsEmptyHeader) {
  CmdlineArgs args;
  EXPECT_THAT(
      internal::ParseTargetArgs(R"([{"t": "t1", "h": ["", "h2"]}])", args),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("--target_args"), HasSubstr("`h`"),
                     HasSubstr("empty string"))));
}
TEST(CmdlineTest, TargetArgsEmptyTarget) {
  CmdlineArgs args;
  EXPECT_THAT(
      internal::ParseTargetArgs(R"([{"t": "", "h": ["h1", "h2"]}])", args),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("--target_args"), HasSubstr("`t`"),
                     HasSubstr("empty string"))));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfTarget) {
  CmdlineArgs args;
  EXPECT_THAT(
      internal::ParseTargetArgs(R"([{"t": 123, "h": ["h1", "h2"]}])", args),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("--target_args"), HasSubstr(".t"),
                     HasSubstr("string"))));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfHeader) {
  CmdlineArgs args;
  EXPECT_THAT(
      internal::ParseTargetArgs(R"([{"t": "//:t1", "h": [123, "h2"]}])", args),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("--target_args"), HasSubstr(".h"),
                     HasSubstr("string"))));
}

TEST(CmdlineTest, TargetArgsDuplicateHeader) {
  CmdlineArgs args;
  ASSERT_OK(internal::ParseTargetArgs(R"([
      {"t": "//:t1", "h": ["h1"]},
      {"t": "//:t2", "h": ["h1", "h2"]} ])",
                                      args));
  EXPECT_THAT(
      args.headers_to_targets,
      UnorderedElementsAre(Pair(HeaderName("h1"), BazelLabel("//:t1")),
                           Pair(HeaderName("h2"), BazelLabel("//:t2"))));
}

TEST(CmdlineTest, PublicHeadersEmpty) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(TestCmdlineArgs({}, std::string(kTargetsAndHeaders)),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --public_headers")));
}

TEST(CmdlineTest, PublicHeadersWhereFirstHeaderMissingInMap) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      TestCmdlineArgs({"missing-in-map.h"}, std::string(kTargetsAndHeaders)),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          AllOf(HasSubstr("missing-in-map.h"), HasSubstr("Couldn't find"))));
}

TEST(CmdlineTest, PublicHeadersWhereSecondHeaderMissingInMap) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(
      TestCmdlineArgs({"a.h", "missing.h"}, kTargetsAndHeaders),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("missing.h"), HasSubstr("Couldn't find"))));
}

TEST(CmdlineTest, PublicHeadersCoveringMultipleTargets) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]},
    {"t": "//:target2", "h": ["c.h", "d.h"]}
  ])";
  ASSERT_OK_AND_ASSIGN(
      CmdlineArgs args,
      TestCmdlineArgs("//:target1", {"a.h", "c.h"}, kTargetsAndHeaders));
  EXPECT_EQ(args.current_target.value(), "//:target1");
  EXPECT_THAT(
      args.headers_to_targets,
      UnorderedElementsAre(Pair(HeaderName("a.h"), BazelLabel("//:target1")),
                           Pair(HeaderName("b.h"), BazelLabel("//:target1")),
                           Pair(HeaderName("c.h"), BazelLabel("//:target2")),
                           Pair(HeaderName("d.h"), BazelLabel("//:target2"))));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfFeaturesArray) {
  ASSERT_THAT(TestCmdlineArgs({"h1"}, R"([{"t": "t1", "f": 123}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr(".f"),
                             HasSubstr("array"))));
}

TEST(CmdlineTest, TargetArgsEmptyFeature) {
  ASSERT_THAT(TestCmdlineArgs({"h1"}, R"([{"t": "t1", "f": ["", "h2"]}])"),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("--target_args"), HasSubstr("`f`"),
                             HasSubstr("empty string"))));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfFeature) {
  ASSERT_THAT(
      TestCmdlineArgs({"h1"}, R"([{"t": "t1", "f": [123, "experimental"]}])"),
      StatusIs(absl::StatusCode::kInvalidArgument,
               AllOf(HasSubstr("--target_args"), HasSubstr(".f"),
                     HasSubstr("string"))));
}

TEST(CmdlineTest, InstantiationsOutEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.srcs_to_scan_for_instantiations = {"lib.rs"};
  args.instantiations_out = "";
  EXPECT_THAT(
      Cmdline::Create(std::move(args)),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr(
              "please specify both --rust_sources and --instantiations_out "
              "when requesting a template instantiation mode")));
}

TEST(CmdlineTest, RustSourcesEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.srcs_to_scan_for_instantiations = {};
  args.instantiations_out = "instantiations_out";
  EXPECT_THAT(
      Cmdline::Create(std::move(args)),
      StatusIs(
          absl::StatusCode::kInvalidArgument,
          HasSubstr(
              "please specify both --rust_sources and --instantiations_out "
              "when requesting a template instantiation mode")));
}

TEST(CmdlineTest, CcOutEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.cc_out = "";
  EXPECT_THAT(Cmdline::Create(std::move(args)),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --cc_out")));
}

TEST(CmdlineTest, RsOutEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.rs_out = "";
  EXPECT_THAT(Cmdline::Create(std::move(args)),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --rs_out")));
}

TEST(CmdlineTest, IrOutEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.ir_out = "";
  EXPECT_OK(Cmdline::Create(std::move(args)));
}

TEST(CmdlineTest, ClangFormatExePathEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.clang_format_exe_path = "";
  EXPECT_THAT(Cmdline::Create(std::move(args)),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --clang_format_exe_path")));
}

TEST(CmdlineTest, RustfmtExePathEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.rustfmt_exe_path = "";
  EXPECT_THAT(Cmdline::Create(std::move(args)),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --rustfmt_exe_path")));
}

TEST(CmdlineTest, SupportPathEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.crubit_support_path_format = "";
  EXPECT_THAT(
      Cmdline::Create(std::move(args)),
      StatusIs(absl::StatusCode::kInvalidArgument,
               HasSubstr("please specify --crubit_support_path_format")));
}

TEST(CmdlineTest, SupportPathNoPlaceholder) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.crubit_support_path_format = "<crubit/support/path>";
  EXPECT_THAT(Cmdline::Create(std::move(args)),
              StatusIs(absl::StatusCode::kInvalidArgument,
                       HasSubstr("cannot find `{header}` placeholder in "
                                 "crubit_support_path_format")));
}

// A mutable test argv, which doesn't leak memory.
class Args {
 public:
  explicit Args(std::vector<std::string> args) {
    storage_ = std::move(args);
    for (std::string& arg : storage_) {
      argv_back_.push_back(arg.data());
    }
    argc_ = argv_back_.size();
    argv_ = argv_back_.data();
  }

  std::vector<absl::string_view> argv_vector() const {
    auto span = absl::MakeConstSpan(argv_, argc_);
    std::vector<absl::string_view> result;
    result.reserve(span.size());
    for (const char* arg : span) {
      result.push_back(arg);
    }
    return result;
  }

  char**& argv() { return argv_; }
  int& argc() { return argc_; }

 private:
  // storage_ and argv_back_ need to be separate, because the argv can have
  // new strings inserted into it (which should _not_ be freed the same way
  // as the old strings).
  std::vector<std::string> storage_;
  std::vector<char*> argv_back_;
  int argc_;
  char** argv_;
};

TEST(PreprocessTargetArgsTest, Noop) {
  Args args({"binary", "foo", "bar"});
  PreprocessTargetArgs(args.argc(), args.argv());
  EXPECT_THAT(args.argv_vector(), ElementsAre("binary", "foo", "bar"));
}

TEST(PreprocessTargetArgsTest, TargetToArg) {
  Args args({"binary", "--target_to_arg", R"({"k": "v"})", "--target_to_arg",
             R"({"k2": "v2"})", "other_args"});
  PreprocessTargetArgs(args.argc(), args.argv());
  EXPECT_THAT(
      args.argv_vector(),
      ElementsAre("binary", R"(--target_args=[{"k": "v"},{"k2": "v2"}])",
                  "other_args"));
}

std::string Paramfile(absl::string_view contents) {
  std::string path = absl::StrCat(
      FLAGS_test_tmpdir, "/",
      testing::UnitTest::GetInstance()->current_test_info()->name(), ".param");
  std::ofstream f(path);
  f << contents;
  f.close();
  return absl::StrCat("@", path);
}

TEST(ExpandParamfilesTest, Noop) {
  Args args({"binary", "foo", "bar"});
  ExpandParamfiles(args.argc(), args.argv());
  EXPECT_THAT(args.argv_vector(), ElementsAre("binary", "foo", "bar"));
}

TEST(ExpandParamfilesTest, Expand) {
  Args args({"binary", "foo", Paramfile("arg1\narg2\n"), "bar"});
  ExpandParamfiles(args.argc(), args.argv());
  EXPECT_THAT(args.argv_vector(),
              ElementsAre("binary", "foo", "arg1", "arg2", "bar"));
}

TEST(ExpandParamfilesTest, Nested) {
  Args args({"binary", Paramfile("@unexpanded")});
  ExpandParamfiles(args.argc(), args.argv());
  EXPECT_THAT(args.argv_vector(), ElementsAre("binary", "@unexpanded"));
}

TEST(ExpandParamfilesTest, Escapes) {
  std::string unescaped_contents = "\"'\n\f\v\r";
  std::string paramfile_contents = "";
  paramfile_contents.reserve(unescaped_contents.size() * 2);
  for (char c : unescaped_contents) {
    paramfile_contents += '\\';
    paramfile_contents += c;
  }
  Args args({"binary", Paramfile(paramfile_contents)});
  ExpandParamfiles(args.argc(), args.argv());
  EXPECT_THAT(args.argv_vector(), ElementsAre("binary", unescaped_contents));
}

// Backslash escapes should be read right to left -- so for instance, while
// the two bytes `\'` become the single byte `'`, the three bytes `\\'` become
// the two bytes `\'`.
TEST(ExpandParamfilesTest, BackslashEscape) {
  Args args({"binary", Paramfile(R"(\\\')")});
  ExpandParamfiles(args.argc(), args.argv());
  EXPECT_THAT(args.argv_vector(), ElementsAre("binary", R"(\')"));
}

}  // namespace
}  // namespace crubit
