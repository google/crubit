// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "common/test_utils.h"

#include <string>
#include <vector>

#include "gtest/gtest.h"
#include "common/check.h"
#include "common/file_io.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/Twine.h"
#include "llvm/Support/FileSystem.h"
#include "llvm/Support/Path.h"

namespace crubit {

static std::string MakeTmpdirForCurrentTest() {
  llvm::StringRef current_test_name =
      testing::UnitTest::GetInstance()->current_test_info()->name();
  std::string current_test_tmpdir_path =
      (llvm::Twine(testing::TempDir()) + "/" + current_test_name + "/").str();
  llvm::StringRef parent_dir =
      llvm::sys::path::parent_path(current_test_tmpdir_path);
  CRUBIT_CHECK(!llvm::sys::fs::create_directories(parent_dir));
  return current_test_tmpdir_path;
}

std::string WriteFileForCurrentTest(llvm::StringRef filename,
                                    llvm::StringRef content) {
  std::string path =
      (llvm::Twine(MakeTmpdirForCurrentTest()) + "/" + filename).str();
  llvm::Error status = SetFileContents(path, content);
  CRUBIT_CHECK(!status) << llvm::toString(std::move(status));
  return path;
}

std::vector<std::string> DefaultClangArgs() {
  return {"-I", MakeTmpdirForCurrentTest()};
}

}  // namespace crubit
