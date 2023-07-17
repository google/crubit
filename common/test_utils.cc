// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/test_utils.h"

#include <string>
#include <vector>

#include "gtest/gtest.h"
#include "absl/log/check.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/file_io.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/FileSystem.h"
#include "llvm/Support/Path.h"

namespace crubit {

static std::string MakeTmpdirForCurrentTest() {
  absl::string_view current_test_name =
      testing::UnitTest::GetInstance()->current_test_info()->name();
  std::string current_test_tmpdir_path =
      absl::StrCat(testing::TempDir(), "/", current_test_name, "/");
  llvm::StringRef parent_dir =
      llvm::sys::path::parent_path(current_test_tmpdir_path);
  CHECK(!llvm::sys::fs::create_directories(parent_dir));
  return current_test_tmpdir_path;
}

std::string WriteFileForCurrentTest(absl::string_view filename,
                                    absl::string_view content) {
  std::string path = absl::StrCat(MakeTmpdirForCurrentTest(), "/", filename);
  CHECK_OK(SetFileContents(path, content));
  return path;
}

std::vector<std::string> DefaultClangArgs() {
  return {"-I", MakeTmpdirForCurrentTest()};
}

}  // namespace crubit
