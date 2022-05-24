// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "migrator/rs_from_cc/rs_from_cc_lib.h"

#include <variant>

#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "clang/AST/ASTContext.h"

namespace crubit_rs_from_cc {
namespace {

using ::testing::Eq;
using ::testing::IsEmpty;
using ::testing::status::StatusIs;

TEST(RsFromCcTest, Noop) {
  // Nothing interesting there, but also not empty, so that the header gets
  // generated.
  ASSERT_OK_AND_ASSIGN(std::string rs_code, RsFromCc(" "));

  EXPECT_THAT(rs_code, IsEmpty());
}

TEST(RsFromCcTest, Comment) {
  ASSERT_OK_AND_ASSIGN(std::string rs_code, RsFromCc("// This is a comment"));

  EXPECT_THAT(rs_code, IsEmpty());
}

TEST(RsFromCcTest, ErrorOnInvalidInput) {
  ASSERT_THAT(RsFromCc("int foo(); But this is not C++"),
              StatusIs(absl::StatusCode::kInvalidArgument));
}

TEST(RsFromCcTest, FunctionDeclaration) {
  ASSERT_OK_AND_ASSIGN(std::string rs_code, RsFromCc("void f() {}"));

  EXPECT_THAT(rs_code, Eq(R"end_of_string(
// Unsupported decl:
//
// FunctionDecl <testing/file_name.cc:1:1, col:11> col:6 f 'void ()'
// `-CompoundStmt <col:10, col:11>
)end_of_string"));
}

}  // namespace
}  // namespace crubit_rs_from_cc
