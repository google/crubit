// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/generate_bindings_and_metadata.h"

#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "common/file_io.h"
#include "rs_bindings_from_cc/cmdline.h"

namespace crubit {
namespace {

TEST(GenerateBindingsAndMetadataTest, GeneratingIR) {
  constexpr absl::string_view kTargetsAndHeaders = R"([
    {"t": "target1", "h": ["a.h"]}
  ])";

  std::string tmpdir = absl::GetFlag(FLAGS_test_tmpdir);
  ASSERT_OK(SetFileContents(absl::StrCat(tmpdir, "/a.h"), "// empty header"));
  ASSERT_OK_AND_ASSIGN(
      Cmdline cmdline,
      Cmdline::CreateForTesting(
          "cc_out", "rs_out", "ir_out", "crubit_support_path",
          "external/rustfmt/rustfmt.toml",
          /* do_nothing= */ false,
          /* public_headers= */ {"a.h"}, std::string(kTargetsAndHeaders),
          /* rust_sources= */ {},
          /* instantiations_out= */ ""));

  ASSERT_OK_AND_ASSIGN(
      BindingsAndMetadata result,
      GenerateBindingsAndMetadata(
          cmdline, /* clang_args= */ {std::string("-I"), std::move(tmpdir)}));

  ASSERT_EQ(result.ir.used_headers.size(), 1);
  ASSERT_EQ(result.ir.used_headers.front().IncludePath(), "a.h");
}

}  // namespace
}  // namespace crubit
