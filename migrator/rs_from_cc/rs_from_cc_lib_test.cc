// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "migrator/rs_from_cc/rs_from_cc_lib.h"

#include <variant>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "clang/AST/ASTContext.h"

namespace crubit_rs_from_cc {
namespace {

using ::testing::Eq;
using ::testing::IsEmpty;

TEST(RsFromCcTest, Noop) {
  // Nothing interesting there, but also not empty, so that the header gets
  // generated.
  auto rs_code_or = RsFromCc(" ");
  ASSERT_TRUE(static_cast<bool>(rs_code_or))
      << llvm::toString(rs_code_or.takeError());
  std::string rs_code = std::move(rs_code_or.get());

  EXPECT_THAT(rs_code, IsEmpty());
}

TEST(RsFromCcTest, Comment) {
  auto rs_code_or = RsFromCc("// This is a comment");
  ASSERT_TRUE(static_cast<bool>(rs_code_or))
      << llvm::toString(rs_code_or.takeError());
  std::string rs_code = std::move(rs_code_or.get());

  EXPECT_THAT(rs_code, IsEmpty());
}

TEST(RsFromCcTest, ErrorOnInvalidInput) {
  auto err = RsFromCc("int foo(); But this is not C++");
  ASSERT_FALSE(static_cast<bool>(err));
  EXPECT_THAT(llvm::toString(err.takeError()),
              ::testing::HasSubstr("Could not compile source file contents"));
}

TEST(RsFromCcTest, FunctionDeclaration) {
  auto rs_code_or = RsFromCc("void f() {}");
  ASSERT_TRUE(static_cast<bool>(rs_code_or))
      << llvm::toString(rs_code_or.takeError());
  std::string rs_code = std::move(rs_code_or.get());

  EXPECT_THAT(rs_code, Eq(R"end_of_string(
// Unsupported decl:
//
// FunctionDecl <testing/file_name.cc:1:1, col:11> col:6 f 'void ()' external-linkage
// `-CompoundStmt <col:10, col:11>
)end_of_string"));
}

}  // namespace
}  // namespace crubit_rs_from_cc
