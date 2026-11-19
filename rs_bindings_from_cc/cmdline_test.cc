// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/cmdline.h"

#include <algorithm>
#include <fstream>
#include <initializer_list>
#include <iterator>
#include <string>
#include <utility>
#include <vector>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "common/status.h"
#include "common/status_macros.h"
#include "common/status_test_matchers.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/logging.h"
#include "llvm/Support/Error.h"

namespace crubit {
namespace {

using ::testing::AllOf;
using ::testing::ElementsAre;
using ::testing::HasSubstr;
using ::testing::IsEmpty;
using ::testing::Optional;
using ::testing::Pair;
using ::testing::UnorderedElementsAre;

llvm::Expected<CmdlineArgs> TestCmdlineArgs(
    std::string target, std::vector<std::string> public_headers,
    llvm::StringRef target_args) {
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
      .is_golden_test = true};
  std::transform(public_headers.begin(), public_headers.end(),
                 std::back_inserter(args.public_headers),
                 [](std::string header) { return HeaderName(header); });
  if (auto err = internal::ParseTargetArgs(target_args, args)) {
    return std::move(err);
  }
  CRUBIT_ASSIGN_OR_RETURN(Cmdline cmdline, Cmdline::Create(std::move(args)));
  return std::move(cmdline).args();
}

llvm::Expected<CmdlineArgs> TestCmdlineArgs(
    std::vector<std::string> public_headers, llvm::StringRef target_args) {
  return TestCmdlineArgs("//:target", std::move(public_headers),
                         std::move(target_args));
}

// Returns an example valid test command line.
llvm::Expected<CmdlineArgs> TestCmdlineArgs() {
  return TestCmdlineArgs("//:target", {"h1"},
                         R"([{"t": "//:target", "h": ["h1"]}])");
}

/// TestCmdlineArgs() above needs to be valid...
TEST(CmdlineTest, TestCmdlineArgsValid) { ASSERT_OK(TestCmdlineArgs()); }

TEST(CmdlineTest, BasicCorrectInput) {
  const char* argv[] = {
      "rs_bindings_from_cc",
      "--do_nothing=false",
      "--rs_out=rs_out",
      "--cc_out",
      "cc_out",
      "--ir_out=ir_out",
      "--crubit_support_path_format=<crubit/support/path/{header}>",
      "--crubit_support_versioned_path_format=<crubit/support/internal/"
      "{header}>",
      "--clang_format_exe_path=clang_format_exe_path",
      "--rustfmt_exe_path=rustfmt_exe_path",
      "--rustfmt_config_path=rustfmt_config_path",
      "--public_headers=h1",
      "--target=//:t1",
      "--target_args=[{\"t\": \"//:t1\", \"h\": [\"h1\", \"h2\"], \"c\": "
      "\"custom_crate\"}]",
      "--extra_rs_srcs=extra_file.rs",
      "--reexported_namespaces=absl",
      "--srcs_to_scan_for_instantiations=scan_for_instantiations.rs",
      "--instantiations_out=instantiations_out",
      "--namespaces_out=namespaces_out",
      "--error_report_out=error_report_out",
      "--is_golden_test",
      "--",
      "-x",
      "c++"};
  ASSERT_OK_AND_ASSIGN(Cmdline cmdline,
                       Cmdline::FromArgv(std::size(argv), argv));
  const CmdlineArgs& args = cmdline.args();
  EXPECT_EQ(args.cc_out, "cc_out");
  EXPECT_EQ(args.rs_out, "rs_out");
  EXPECT_EQ(args.ir_out, "ir_out");
  EXPECT_EQ(args.namespaces_out, "namespaces_out");
  EXPECT_EQ(args.crubit_support_path_format, "<crubit/support/path/{header}>");
  EXPECT_EQ(args.crubit_support_versioned_path_format,
            "<crubit/support/internal/{header}>");
  EXPECT_EQ(args.clang_format_exe_path, "clang_format_exe_path");
  EXPECT_EQ(args.rustfmt_exe_path, "rustfmt_exe_path");
  EXPECT_EQ(args.rustfmt_config_path, "rustfmt_config_path");
  EXPECT_EQ(args.instantiations_out, "instantiations_out");
  EXPECT_EQ(args.error_report_out, "error_report_out");
  EXPECT_EQ(args.do_nothing, false);
  EXPECT_EQ(args.current_target.value(), "//:t1");
  EXPECT_THAT(args.public_headers, ElementsAre(HeaderName("h1")));
  EXPECT_THAT(args.extra_rs_srcs, ElementsAre("extra_file.rs"));
  EXPECT_THAT(args.reexported_namespaces, ElementsAre("absl"));
  EXPECT_THAT(args.srcs_to_scan_for_instantiations,
              ElementsAre("scan_for_instantiations.rs"));
  EXPECT_THAT(args.clang_args, ElementsAre("-x", "c++"));

  // llvm::DenseMap doesn't have an UnorderedElementsAre equivalent easily if we
  // don't convert it. We'll check individually.
  EXPECT_EQ(args.headers_to_targets.lookup(HeaderName("h1")),
            BazelLabel("//:t1"));
  EXPECT_EQ(args.headers_to_targets.lookup(HeaderName("h2")),
            BazelLabel("//:t1"));
  EXPECT_EQ(args.target_to_crate_name.lookup(BazelLabel("//:t1")),
            "custom_crate");
  EXPECT_EQ(args.is_golden_test, true);
}

// Parses `extra_args` together with the minimal set of required flags.
llvm::Expected<CmdlineArgs> ParseArgv(std::vector<std::string> extra_args) {
  std::vector<const char*> argv = {
      "rs_bindings_from_cc",
      "--rs_out=rs_out",
      "--cc_out=cc_out",
      "--public_headers=h1",
      "--crubit_support_path_format=<crubit/support/path/{header}>",
      "--target=//:t1",
      R"(--target_args=[{"t": "//:t1", "h": ["h1"]}])",
  };
  for (const std::string& arg : extra_args) {
    argv.push_back(arg.c_str());
  }
  CRUBIT_ASSIGN_OR_RETURN(Cmdline cmdline,
                          Cmdline::FromArgv(argv.size(), argv.data()));
  return std::move(cmdline).args();
}

TEST(CmdlineTest, DefaultValues) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, ParseArgv({}));
  EXPECT_EQ(args.do_nothing, false);
  EXPECT_EQ(args.is_golden_test, false);
  EXPECT_EQ(args.kythe_annotations, false);
  EXPECT_EQ(args.kythe_default_corpus, "corpus");
  EXPECT_EQ(args.lazy_import_alien_decls, false);
  EXPECT_EQ(args.check_importer_invariants, false);
  EXPECT_EQ(args.carcinize_mode, CarcinizeMode::kOff);
  EXPECT_EQ(args.do_not_bind_allowlist, std::nullopt);
  EXPECT_THAT(args.extra_rs_srcs, IsEmpty());
  EXPECT_THAT(args.clang_args, IsEmpty());
}

TEST(CmdlineTest, FlagSyntaxVariations) {
  ASSERT_OK_AND_ASSIGN(
      CmdlineArgs args,
      ParseArgv({"--ir_out", "ir_out", "--namespaces_out=namespaces_out",
                 "--is_golden_test", "--kythe_annotations=false",
                 "--check_importer_invariants=true",
                 "--lazy_import_alien_decls=True", "--carcinize=strict"}));
  EXPECT_EQ(args.ir_out, "ir_out");
  EXPECT_EQ(args.namespaces_out, "namespaces_out");
  EXPECT_EQ(args.is_golden_test, true);
  EXPECT_EQ(args.kythe_annotations, false);
  EXPECT_EQ(args.check_importer_invariants, true);
  EXPECT_EQ(args.lazy_import_alien_decls, true);
  EXPECT_EQ(args.carcinize_mode, CarcinizeMode::kStrict);
}

TEST(CmdlineTest, InvalidBoolValue) {
  // `llvm::cl` reports the details of the invalid value on stderr.
  EXPECT_THAT(ParseArgv({"--is_golden_test=maybe"}),
              StatusIs(StatusCode::kInvalidArgument,
                       "Failed to parse command line arguments."));
}

TEST(CmdlineTest, ListFlags) {
  ASSERT_OK_AND_ASSIGN(
      CmdlineArgs args,
      ParseArgv({"--extra_rs_srcs=a.rs,b.rs",
                 "--extra_cpp_srcs=", "--unstable_rust_features=x,,y",
                 "--reexported_namespaces=ns1", "--reexported_namespaces=ns2",
                 "--do_not_bind_allowlist=allowed"}));
  EXPECT_THAT(args.extra_rs_srcs, ElementsAre("a.rs", "b.rs"));
  // Like Abseil, an empty value is an empty list ...
  EXPECT_THAT(args.extra_cpp_srcs, IsEmpty());
  // ... but empty elements are preserved ...
  EXPECT_THAT(args.unstable_rust_features, ElementsAre("x", "", "y"));
  // ... and the last occurrence wins.
  EXPECT_THAT(args.reexported_namespaces, ElementsAre("ns2"));
  EXPECT_THAT(args.do_not_bind_allowlist, Optional(ElementsAre("allowed")));
}

TEST(CmdlineTest, EmptyDoNotBindAllowlistIsPresent) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args,
                       ParseArgv({"--do_not_bind_allowlist="}));
  EXPECT_THAT(args.do_not_bind_allowlist, Optional(IsEmpty()));
}

TEST(CmdlineTest, ClangArgs) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args,
                       ParseArgv({"--", "-x", "c++", "--std=c++20"}));
  EXPECT_THAT(args.clang_args, ElementsAre("-x", "c++", "--std=c++20"));
}

TEST(CmdlineTest, TargetToArg) {
  ASSERT_OK_AND_ASSIGN(
      CmdlineArgs args,
      ParseArgv({R"(--target_to_arg={"t": "//:t1", "h": ["h1"]})",
                 R"(--target_to_arg={"t": "//:t2", "h": ["h2"]})"}));
  EXPECT_EQ(args.headers_to_targets.lookup(HeaderName("h1")),
            BazelLabel("//:t1"));
  EXPECT_EQ(args.headers_to_targets.lookup(HeaderName("h2")),
            BazelLabel("//:t2"));
}

TEST(CmdlineTest, StderrThreshold) {
  LogSeverity old_severity = StderrThreshold();
  ASSERT_OK(ParseArgv({"--stderrthreshold=0"}));
  EXPECT_EQ(StderrThreshold(), LogSeverity::kInfo);
  SetStderrThreshold(old_severity);
}

TEST(CmdlineTest, UnknownFlag) {
  EXPECT_THAT(ParseArgv({"--unknown_flag=123"}),
              StatusIs(StatusCode::kInvalidArgument,
                       HasSubstr("Unknown command line argument")));
}

TEST(CmdlineTest, MissingArgumentsAreReportedTogether) {
  const char* argv[] = {"rs_bindings_from_cc"};
  EXPECT_THAT(
      Cmdline::FromArgv(std::size(argv), argv),
      StatusIs(StatusCode::kInvalidArgument,
               AllOf(HasSubstr("please specify --target\n"),
                     HasSubstr("please specify --rs_out\n"),
                     HasSubstr("please specify --crubit_support_path_format\n"),
                     HasSubstr("please specify --target_args"))));
}

// Writes `contents` to a test-scoped temp file and returns its path.
std::string WriteTempFile(llvm::StringRef suffix, llvm::StringRef contents) {
  std::string path =
      testing::TempDir() + "/" +
      testing::UnitTest::GetInstance()->current_test_info()->name() +
      suffix.str();
  std::ofstream f(path);
  f.write(contents.data(), contents.size());
  return path;
}

// Writes `contents` to a paramfile and returns the `@path` argument for it.
std::string Paramfile(llvm::StringRef contents) {
  return "@" + WriteTempFile(".param", contents);
}

TEST(CmdlineTest, ParamfileExpand) {
  std::string paramfile = Paramfile("--ir_out=ir_out\n--is_golden_test\n");
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, ParseArgv({paramfile.c_str()}));
  EXPECT_EQ(args.ir_out, "ir_out");
  EXPECT_EQ(args.is_golden_test, true);
}

// Paramfiles are not expanded recursively; a nested `@file` is an ordinary
// (positional) argument.
TEST(CmdlineTest, ParamfileNested) {
  std::string paramfile = Paramfile("@unexpanded");
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, ParseArgv({paramfile.c_str()}));
  EXPECT_THAT(args.clang_args, ElementsAre("@unexpanded"));
}

// Bazel escapes special characters with a backslash.
TEST(CmdlineTest, ParamfileEscapes) {
  std::string unescaped_contents = "\"'\n\f\v\r ";
  std::string paramfile_contents = "--ir_out=";
  for (char c : unescaped_contents) {
    paramfile_contents += '\\';
    paramfile_contents += c;
  }
  std::string paramfile = Paramfile(paramfile_contents);
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, ParseArgv({paramfile.c_str()}));
  EXPECT_EQ(args.ir_out, unescaped_contents);
}

// Backslash escapes are read left to right: `\\'` is the two bytes `\'`.
TEST(CmdlineTest, ParamfileBackslashEscape) {
  std::string paramfile = Paramfile(R"(--ir_out=\\\')");
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, ParseArgv({paramfile.c_str()}));
  EXPECT_EQ(args.ir_out, R"(\')");
}

// Writes `contents` to a flagfile and returns its path.
std::string WriteFlagfile(llvm::StringRef suffix, llvm::StringRef contents) {
  return WriteTempFile(suffix.str() + ".flags", contents);
}

TEST(CmdlineTest, FlagfileExpand) {
  std::string path = WriteFlagfile(
      "",
      "# leading comment\n"
      "   # indented comment\n"
      "\n"
      "   \t  \n"
      "  --ir_out=ir_out\r\n"
      "--is_golden_test\n"
      "--crubit_support_path_format=\"support/{header}\"\n"
      "--template_blocklist_path_regex=foo\\bar#baz\n"
      "--target_args=[{\"t\": \"//:t1\", \"h\": [\"h1\", \"h2\"]}]\n");
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, ParseArgv({"--flagfile=" + path}));
  EXPECT_EQ(args.ir_out, "ir_out");
  EXPECT_EQ(args.is_golden_test, true);
  EXPECT_EQ(args.crubit_support_path_format,
            "\"support/{header}\"");
  EXPECT_EQ(args.template_blocklist_path_regex, R"(foo\bar#baz)");
  EXPECT_EQ(args.headers_to_targets.lookup(HeaderName("h2")),
            BazelLabel("//:t1"));
}

TEST(CmdlineTest, FlagfileSyntaxVariations) {
  std::string f1 = WriteFlagfile("_1", "--ir_out=from_f1\n");
  std::string f2 = WriteFlagfile("_2", "--namespaces_out=from_f2\n");
  std::string f3 = WriteFlagfile("_3", "--error_report_out=from_f3\n");
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args,
                       ParseArgv({"--flagfile=", "-flagfile=" + f1,
                                  "--flagfile", f2, "-flagfile", f3}));
  EXPECT_EQ(args.ir_out, "from_f1");
  EXPECT_EQ(args.namespaces_out, "from_f2");
  EXPECT_EQ(args.error_report_out, "from_f3");
}

TEST(CmdlineTest, FlagfileCommaSeparatedAndPrecedence) {
  std::string f1 =
      WriteFlagfile("_1", "--ir_out=from_f1\n--namespaces_out=from_f1\n");
  std::string f2 = WriteFlagfile("_2", "--ir_out=from_f2\n");
  std::string arg = "--flagfile=" + f1 + "," + f2;
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args,
                       ParseArgv({"--namespaces_out=before_cli", arg,
                                  "--error_report_out=after_cli"}));
  EXPECT_EQ(args.ir_out, "from_f2");
  EXPECT_EQ(args.namespaces_out, "from_f1");
  EXPECT_EQ(args.error_report_out, "after_cli");

  ASSERT_OK_AND_ASSIGN(CmdlineArgs overridden,
                       ParseArgv({arg, "--ir_out=after_cli"}));
  EXPECT_EQ(overridden.ir_out, "after_cli");
}

TEST(CmdlineTest, FlagfileNestedAndSequentialReuse) {
  std::string inner = WriteFlagfile("_inner", "--ir_out=from_inner\n");
  std::string outer =
      WriteFlagfile("_outer", "--ir_out=before\n--flagfile=" + inner +
                                  "\n--namespaces_out=from_outer\n"
                                  "--flagfile=" +
                                  inner + "\n");
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, ParseArgv({"--flagfile=" + outer}));
  EXPECT_EQ(args.ir_out, "from_inner");
  EXPECT_EQ(args.namespaces_out, "from_outer");
}

TEST(CmdlineTest, FlagfileRecursiveCycleErrors) {
  std::string f1 = WriteFlagfile("_1", "");
  std::string f2 = WriteFlagfile("_2", "--flagfile=" + f1 + "\n");
  WriteFlagfile("_1", "--flagfile=" + f2 + "\n");
  EXPECT_THAT(ParseArgv({"--flagfile=" + f1}),
              StatusIs(StatusCode::kInvalidArgument,
                       HasSubstr("Recursive --flagfile expansion")));
}

TEST(CmdlineTest, FlagfileInsideParamfile) {
  std::string flagfile = WriteFlagfile("", "--ir_out=from_flagfile\n");
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args,
                       ParseArgv({Paramfile("--flagfile=" + flagfile + "\n")}));
  EXPECT_EQ(args.ir_out, "from_flagfile");
}

TEST(CmdlineTest, FlagfileAfterDashDashIsPassedToClang) {
  ASSERT_OK_AND_ASSIGN(
      CmdlineArgs args,
      ParseArgv({"--", "--flagfile=nonexistent.flags", "-x", "c++"}));
  EXPECT_THAT(args.clang_args,
              ElementsAre("--flagfile=nonexistent.flags", "-x", "c++"));
}

TEST(CmdlineTest, FlagfileUnknownFlagErrors) {
  EXPECT_THAT(
      ParseArgv({"--flagfile=" + WriteFlagfile("", "--unknown_flag=123\n")}),
      StatusIs(StatusCode::kInvalidArgument,
               HasSubstr("Unknown command line argument")));
}

TEST(CmdlineTest, FlagfileInvalidInputs) {
  EXPECT_THAT(ParseArgv({"--flagfile"}),
              StatusIs(StatusCode::kInvalidArgument,
                       HasSubstr("Missing the value for the flag 'flagfile'")));

  EXPECT_THAT(
      ParseArgv({"--flagfile=" + testing::TempDir() + "/does_not_exist.flags"}),
      StatusIs(StatusCode::kInvalidArgument, HasSubstr("Can't open flagfile")));

  EXPECT_THAT(ParseArgv({"--flagfile=" +
                         WriteFlagfile("_positional", "# ok\nnot_a_flag\n")}),
              StatusIs(StatusCode::kInvalidArgument,
                       HasSubstr("Unexpected line 2 in the flagfile")));

  EXPECT_THAT(
      ParseArgv({"--flagfile=" + WriteFlagfile("_dashdash", "--\n-x\n")}),
      StatusIs(StatusCode::kInvalidArgument,
               HasSubstr("Flagfile can't contain positional arguments or --")));
}

TEST(CmdlineTest, TargetArgsEmpty) {
  CmdlineArgs args;
  EXPECT_THAT(internal::ParseTargetArgs("", args),
              StatusIs(crubit::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --target_args")));
}

TEST(CmdlineTest, TargetArgsInvalidJson) {
  CmdlineArgs args;
  EXPECT_THAT(internal::ParseTargetArgs("#!$%", args),
              StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfTopLevelArray) {
  CmdlineArgs args;
  EXPECT_THAT(internal::ParseTargetArgs("123", args),
              StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}

TEST(CmdlineTest, TargetArgsIntInTopLevelArray) {
  CmdlineArgs args;
  EXPECT_THAT(internal::ParseTargetArgs("[123, 456]", args),
              StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfHeadersArray) {
  CmdlineArgs args;
  EXPECT_THAT(internal::ParseTargetArgs(R"([{"t": "//:t1", "h": 123}])", args),
              StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}

TEST(CmdlineTest, TargetArgsMissingTarget) {
  CmdlineArgs args;
  EXPECT_THAT(internal::ParseTargetArgs(R"([{"h": ["h1", "h2"]}])", args),
              StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}

TEST(CmdlineTest, TargetArgsMissingHeader) {
  EXPECT_THAT(TestCmdlineArgs("//:t1", {"h1"}, R"([{"t": "//:t1"}])"),
              StatusIs(crubit::StatusCode::kInvalidArgument,
                       HasSubstr("Couldn't find header")));
}

TEST(CmdlineTest, TargetArgsEmptyHeader) {
  CmdlineArgs args;
  EXPECT_THAT(
      internal::ParseTargetArgs(R"([{"t": "t1", "h": ["", "h2"]}])", args),
      StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}
TEST(CmdlineTest, TargetArgsEmptyTarget) {
  CmdlineArgs args;
  EXPECT_THAT(
      internal::ParseTargetArgs(R"([{"t": "", "h": ["h1", "h2"]}])", args),
      StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfTarget) {
  CmdlineArgs args;
  EXPECT_THAT(
      internal::ParseTargetArgs(R"([{"t": 123, "h": ["h1", "h2"]}])", args),
      StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfHeader) {
  CmdlineArgs args;
  EXPECT_THAT(
      internal::ParseTargetArgs(R"([{"t": "//:t1", "h": [123, "h2"]}])", args),
      StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}

TEST(CmdlineTest, TargetArgsDuplicateHeader) {
  CmdlineArgs args;
  ASSERT_OK(internal::ParseTargetArgs(R"([
      {"t": "//:t1", "h": ["h1"]},
      {"t": "//:t2", "h": ["h1", "h2"]} ])",
                                      args));
  EXPECT_EQ(args.headers_to_targets.lookup(HeaderName("h1")),
            BazelLabel("//:t1"));
  EXPECT_EQ(args.headers_to_targets.lookup(HeaderName("h2")),
            BazelLabel("//:t2"));
}

TEST(CmdlineTest, PublicHeadersEmpty) {
  constexpr llvm::StringRef kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(TestCmdlineArgs({}, kTargetsAndHeaders),
              StatusIs(crubit::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --public_headers")));
}

TEST(CmdlineTest, PublicHeadersWhereFirstHeaderMissingInMap) {
  constexpr llvm::StringRef kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(TestCmdlineArgs({"missing-in-map.h"}, kTargetsAndHeaders),
              StatusIs(crubit::StatusCode::kInvalidArgument,
                       AllOf(HasSubstr("missing-in-map.h"),
                             HasSubstr("Couldn't find"))));
}

TEST(CmdlineTest, PublicHeadersWhereSecondHeaderMissingInMap) {
  constexpr llvm::StringRef kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]}
  ])";
  ASSERT_THAT(TestCmdlineArgs({"a.h", "missing.h"}, kTargetsAndHeaders),
              StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}

TEST(CmdlineTest, PublicHeadersCoveringMultipleTargets) {
  constexpr llvm::StringRef kTargetsAndHeaders = R"([
    {"t": "//:target1", "h": ["a.h", "b.h"]},
    {"t": "//:target2", "h": ["c.h", "d.h"]}
  ])";
  ASSERT_OK_AND_ASSIGN(
      CmdlineArgs args,
      TestCmdlineArgs("//:target1", {"a.h", "c.h"}, kTargetsAndHeaders));
  EXPECT_EQ(args.current_target.value(), "//:target1");
  EXPECT_EQ(args.headers_to_targets.lookup(HeaderName("a.h")),
            BazelLabel("//:target1"));
  EXPECT_EQ(args.headers_to_targets.lookup(HeaderName("b.h")),
            BazelLabel("//:target1"));
  EXPECT_EQ(args.headers_to_targets.lookup(HeaderName("c.h")),
            BazelLabel("//:target2"));
  EXPECT_EQ(args.headers_to_targets.lookup(HeaderName("d.h")),
            BazelLabel("//:target2"));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfFeaturesArray) {
  ASSERT_THAT(TestCmdlineArgs({"h1"}, R"([{"t": "t1", "f": 123}])"),
              StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}

TEST(CmdlineTest, TargetArgsEmptyFeature) {
  ASSERT_THAT(TestCmdlineArgs({"h1"}, R"([{"t": "t1", "f": ["", "h2"]}])"),
              StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}

TEST(CmdlineTest, TargetArgsIntInsteadOfFeature) {
  ASSERT_THAT(
      TestCmdlineArgs({"h1"}, R"([{"t": "t1", "f": [123, "supported"]}])"),
      StatusIs(crubit::StatusCode::kInvalidArgument, testing::_));
}

TEST(CmdlineTest, InstantiationsOutEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.srcs_to_scan_for_instantiations = {"lib.rs"};
  args.instantiations_out = "";
  EXPECT_THAT(
      Cmdline::Create(std::move(args)),
      StatusIs(
          crubit::StatusCode::kInvalidArgument,
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
          crubit::StatusCode::kInvalidArgument,
          HasSubstr(
              "please specify both --rust_sources and --instantiations_out "
              "when requesting a template instantiation mode")));
}

TEST(CmdlineTest, CcOutEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.cc_out = "";
  EXPECT_THAT(Cmdline::Create(std::move(args)),
              StatusIs(crubit::StatusCode::kInvalidArgument,
                       HasSubstr("please specify --cc_out")));
}

TEST(CmdlineTest, RsOutEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.rs_out = "";
  EXPECT_THAT(Cmdline::Create(std::move(args)),
              StatusIs(crubit::StatusCode::kInvalidArgument,
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
  EXPECT_OK(Cmdline::Create(std::move(args)));
}

TEST(CmdlineTest, RustfmtExePathEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.rustfmt_exe_path = "";
  EXPECT_OK(Cmdline::Create(std::move(args)));
}

TEST(CmdlineTest, SupportPathEmpty) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.crubit_support_path_format = "";
  EXPECT_THAT(
      Cmdline::Create(std::move(args)),
      StatusIs(crubit::StatusCode::kInvalidArgument,
               HasSubstr("please specify --crubit_support_path_format")));
}

TEST(CmdlineTest, SupportPathNoPlaceholder) {
  ASSERT_OK_AND_ASSIGN(CmdlineArgs args, TestCmdlineArgs());
  args.crubit_support_path_format = "<crubit/support/path>";
  EXPECT_THAT(Cmdline::Create(std::move(args)),
              StatusIs(crubit::StatusCode::kInvalidArgument,
                       HasSubstr("cannot find `{header}` placeholder in "
                                 "crubit_support_path_format")));
}

}  // namespace
}  // namespace crubit
