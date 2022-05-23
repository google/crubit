// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/generate_bindings_and_metadata.h"

#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "common/test_utils.h"
#include "rs_bindings_from_cc/cmdline.h"

namespace crubit {
namespace {

using ::testing::StrEq;

TEST(GenerateBindingsAndMetadataTest, GeneratingIR) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h"]}
  ])";

  WriteFileForCurrentTest("a.h", "//empty header");
  ASSERT_OK_AND_ASSIGN(
      Cmdline cmdline,
      Cmdline::CreateForTesting(
          "cc_out", "rs_out", "ir_out", "crubit_support_path",
          "nowhere/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */ {"a.h"}, std::string(kTargetsAndHeaders),
          /* rust_sources= */ {},
          /* instantiations_out= */ ""));

  ASSERT_OK_AND_ASSIGN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(cmdline, DefaultClangArgs()));

  ASSERT_EQ(result.ir.used_headers.size(), 1);
  ASSERT_EQ(result.ir.used_headers.front().IncludePath(), "a.h");
}

TEST(GenerateBindingsAndMetadataTest, InstantiationsAreEmptyInNormalMode) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h"]}
  ])";
  WriteFileForCurrentTest("a.h", "// empty header");
  ASSERT_OK_AND_ASSIGN(
      Cmdline cmdline,
      Cmdline::CreateForTesting(
          "cc_out", "rs_out", "ir_out", "crubit_support_path",
          "nowhere/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */ {"a.h"}, std::string(kTargetsAndHeaders),
          /* rust_sources= */ {},
          /* instantiations_out= */ ""));

  ASSERT_OK_AND_ASSIGN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(cmdline, DefaultClangArgs()));

  ASSERT_THAT(InstantiationsAsJson(result.ir), StrEq("{}"));
}

TEST(GenerateBindingsAndMetadataTest, InstantiationsJsonGenerated) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h"]}
  ])";
  WriteFileForCurrentTest("a.h", "// empty header");
  std::string a_rs_path =
      WriteFileForCurrentTest("a.rs", "cc_template!(MyTemplate<bool>);");
  ASSERT_OK_AND_ASSIGN(
      Cmdline cmdline,
      Cmdline::CreateForTesting(
          "cc_out", "rs_out", "ir_out", "crubit_support_path",
          "nowhere/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */ {"a.h"}, std::string(kTargetsAndHeaders),
          /* rust_sources= */ {a_rs_path}, "instantiations_out"));

  ASSERT_OK_AND_ASSIGN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(cmdline, DefaultClangArgs()));

  // TODO(b/440066049): Actually populate the instantiations map once
  // cl/430823388 is submitted.
  ASSERT_THAT(InstantiationsAsJson(result.ir), StrEq("{}"));
}

}  // namespace
}  // namespace crubit
