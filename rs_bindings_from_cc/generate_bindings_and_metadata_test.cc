// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/generate_bindings_and_metadata.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "common/test_utils.h"
#include "rs_bindings_from_cc/cmdline.h"
#include "rs_bindings_from_cc/ir.h"

namespace crubit {
namespace {

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
          "cc_out", "rs_out", "ir_out", "crubit_support_path",
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
          "cc_out", "rs_out", "ir_out", "crubit_support_path",
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

  ASSERT_THAT(InstantiationsAsJson(result.ir), StrEq("{}"));
}

TEST(GenerateBindingsAndMetadataTest, InstantiationsJsonGenerated) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h"]}
  ])";
  std::string a_rs_path =
      WriteFileForCurrentTest("a.rs", "cc_template!(MyTemplate<bool>);");
  ASSERT_OK_AND_ASSIGN(
      Cmdline cmdline,
      Cmdline::CreateForTesting(
          "cc_out", "rs_out", "ir_out", "crubit_support_path",
          std::string(kDefaultRustfmtExePath), "nowhere/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */ {"a.h"}, std::string(kTargetsAndHeaders),
          /* rust_sources= */ {a_rs_path}, "instantiations_out"));

  ASSERT_OK_AND_ASSIGN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(cmdline, DefaultClangArgs(),
                                  /* virtual_headers_contents= */
                                  {{HeaderName("a.h"), "// empty header"}}));

  // TODO(b/440066049): Actually populate the instantiations map once
  // cl/430823388 is submitted.
  ASSERT_THAT(InstantiationsAsJson(result.ir), StrEq("{}"));
}

}  // namespace
}  // namespace crubit
