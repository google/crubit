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
using ::testing::status::StatusIs;

TEST(RsFromCcTest, Noop) {
  // Nothing interesting there, but also not empty, so that the header gets
  // generated.
  ASSERT_OK_AND_ASSIGN(std::string rs_code, RsFromCc(" "));

  EXPECT_THAT(rs_code, Eq(R"end_of_string(
// Unsupported decl:
//
// TranslationUnitDecl <<invalid sloc>> <invalid sloc>
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __int128_t '__int128'
// | `-BuiltinType '__int128'
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __uint128_t 'unsigned __int128'
// | `-BuiltinType 'unsigned __int128'
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __NSConstantString '__NSConstantString_tag'
// | `-RecordType '__NSConstantString_tag'
// |   `-CXXRecord '__NSConstantString_tag'
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __builtin_ms_va_list 'char *'
// | `-PointerType 'char *'
// |   `-BuiltinType 'char'
// `-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __builtin_va_list '__va_list_tag[1]'
//   `-ConstantArrayType '__va_list_tag[1]' 1
//     `-RecordType '__va_list_tag'
//       `-CXXRecord '__va_list_tag'
)end_of_string"));
}

TEST(RsFromCcTest, Comment) {
  ASSERT_OK_AND_ASSIGN(std::string rs_code, RsFromCc("// This is a comment"));

  EXPECT_THAT(rs_code, Eq(R"end_of_string(
// Unsupported decl:
//
// TranslationUnitDecl <<invalid sloc>> <invalid sloc>
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __int128_t '__int128'
// | `-BuiltinType '__int128'
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __uint128_t 'unsigned __int128'
// | `-BuiltinType 'unsigned __int128'
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __NSConstantString '__NSConstantString_tag'
// | `-RecordType '__NSConstantString_tag'
// |   `-CXXRecord '__NSConstantString_tag'
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __builtin_ms_va_list 'char *'
// | `-PointerType 'char *'
// |   `-BuiltinType 'char'
// `-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __builtin_va_list '__va_list_tag[1]'
//   `-ConstantArrayType '__va_list_tag[1]' 1
//     `-RecordType '__va_list_tag'
//       `-CXXRecord '__va_list_tag'
)end_of_string"));
}

TEST(RsFromCcTest, ErrorOnInvalidInput) {
  ASSERT_THAT(RsFromCc("int foo(); But this is not C++"),
              StatusIs(absl::StatusCode::kInvalidArgument));
}

TEST(RsFromCcTest, FunctionDeclaration) {
  ASSERT_OK_AND_ASSIGN(std::string rs_code, RsFromCc("void f();"));

  EXPECT_THAT(rs_code, Eq(R"end_of_string(
// Unsupported decl:
//
// TranslationUnitDecl <<invalid sloc>> <invalid sloc>
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __int128_t '__int128'
// | `-BuiltinType '__int128'
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __uint128_t 'unsigned __int128'
// | `-BuiltinType 'unsigned __int128'
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __NSConstantString '__NSConstantString_tag'
// | `-RecordType '__NSConstantString_tag'
// |   `-CXXRecord '__NSConstantString_tag'
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __builtin_ms_va_list 'char *'
// | `-PointerType 'char *'
// |   `-BuiltinType 'char'
// |-TypedefDecl <<invalid sloc>> <invalid sloc> implicit __builtin_va_list '__va_list_tag[1]'
// | `-ConstantArrayType '__va_list_tag[1]' 1
// |   `-RecordType '__va_list_tag'
// |     `-CXXRecord '__va_list_tag'
// `-FunctionDecl <testing/file_name.cc:1:1, col:8> col:6 f 'void ()'
)end_of_string"));
}

}  // namespace
}  // namespace crubit_rs_from_cc
