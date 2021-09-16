// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "devtools/cymbal/common/clang_tool.h"
#include "rs_bindings_from_cc/frontend_action.h"
#include "rs_bindings_from_cc/ir.h"
#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "third_party/absl/container/flat_hash_map.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/absl/strings/substitute.h"
#include "third_party/absl/types/span.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Frontend/FrontendAction.h"

namespace rs_bindings_from_cc {
namespace {

using ::testing::IsEmpty;
using ::testing::SizeIs;

constexpr absl::string_view kVirtualInputPath =
    "ast_visitor_test_virtual_input.cc";

IR ImportCode(absl::Span<const absl::string_view> header_files_contents,
              const std::vector<absl::string_view>& args) {
  std::vector<std::string> headers;
  absl::flat_hash_map<std::string, std::string> file_contents;
  std::string virtual_input_file_content;

  int counter = 0;
  for (const absl::string_view header_content : header_files_contents) {
    std::string filename(
        absl::Substitute("test/testing_header_$0.h", counter++));
    file_contents.insert({filename, std::string(header_content)});
    absl::SubstituteAndAppend(&virtual_input_file_content, "#include \"$0\"\n",
                              filename);
    headers.emplace_back(std::move(filename));
  }

  file_contents.insert(
      {std::string(kVirtualInputPath), virtual_input_file_content});

  std::vector<std::string> args_as_strings(args.begin(), args.end());
  args_as_strings.emplace_back(std::string("--syntax-only"));
  args_as_strings.emplace_back(std::string(kVirtualInputPath));

  IR ir;
  devtools::cymbal::RunToolWithClangFlagsOnCode(
      args_as_strings, file_contents,
      std::make_unique<rs_bindings_from_cc::FrontendAction>(
          std::vector<absl::string_view>(headers.begin(), headers.end()), ir));
  return ir;
}

TEST(AstVisitorTest, Noop) {
  IR ir = ImportCode({"// nothing interesting there."}, {});
  EXPECT_THAT(ir.Functions(), IsEmpty());
  EXPECT_THAT(ir.UsedHeaders(), SizeIs(1));
  EXPECT_EQ(ir.UsedHeaders()[0].IncludePath(), "test/testing_header_0.h");
}

TEST(AstVisitorTest, IREmptyOnInvalidInput) {
  IR ir = ImportCode({"int foo(); But this is not C++"}, {});
  EXPECT_THAT(ir.Functions(), IsEmpty());
}

TEST(AstVisitorTest, FuncWithVoidReturnType) {
  IR ir = ImportCode({"void Foo();"}, {});
  ASSERT_THAT(ir.Functions(), SizeIs(1));
  Func func = ir.Functions()[0];
  EXPECT_EQ(func.identifier.Ident(), "Foo");
  EXPECT_EQ(func.mangled_name, "_Z3Foov");
  EXPECT_TRUE(func.return_type.IsVoid());
  EXPECT_THAT(func.params, IsEmpty());
}

TEST(AstVisitorTest, TwoFuncs) {
  IR ir = ImportCode({"void Foo(); void Bar();"}, {});
  ASSERT_THAT(ir.Functions(), SizeIs(2));

  Func foo = ir.Functions()[0];
  EXPECT_EQ(foo.identifier.Ident(), "Foo");
  EXPECT_EQ(foo.mangled_name, "_Z3Foov");
  EXPECT_TRUE(foo.return_type.IsVoid());
  EXPECT_THAT(foo.params, IsEmpty());

  Func bar = ir.Functions()[1];
  EXPECT_EQ(bar.identifier.Ident(), "Bar");
  EXPECT_EQ(bar.mangled_name, "_Z3Barv");
  EXPECT_TRUE(bar.return_type.IsVoid());
  EXPECT_THAT(bar.params, IsEmpty());
}

TEST(AstVisitorTest, TwoFuncsFromTwoHeaders) {
  IR ir = ImportCode({"void Foo();", "void Bar();"}, {});
  ASSERT_THAT(ir.Functions(), SizeIs(2));
  Func foo = ir.Functions()[0];
  EXPECT_EQ(foo.identifier.Ident(), "Foo");
  Func bar = ir.Functions()[1];
  EXPECT_EQ(bar.identifier.Ident(), "Bar");
}

TEST(AstVisitorTest, NonInlineFunc) {
  IR ir = ImportCode({"void Foo() {}"}, {});
  ASSERT_THAT(ir.Functions(), SizeIs(1));
  Func func = ir.Functions()[0];
  EXPECT_EQ(func.identifier.Ident(), "Foo");
  EXPECT_FALSE(func.is_inline);
}

TEST(AstVisitorTest, InlineFunc) {
  IR ir = ImportCode({"inline void Foo() {}"}, {});
  ASSERT_THAT(ir.Functions(), SizeIs(1));
  Func func = ir.Functions()[0];
  EXPECT_EQ(func.identifier.Ident(), "Foo");
  EXPECT_TRUE(func.is_inline);
}

TEST(AstVisitorTest, FuncJustOnce) {
  IR ir = ImportCode({"void Foo(); void Foo();"}, {});
  ASSERT_THAT(ir.Functions(), SizeIs(1));
  Func func = ir.Functions()[0];
  EXPECT_EQ(func.identifier.Ident(), "Foo");
}

TEST(AstVisitorTest, FuncParams) {
  IR ir = ImportCode({"int Add(int a, int b);"}, {});
  EXPECT_THAT(ir.Functions(), SizeIs(1));

  Func func = ir.Functions()[0];
  EXPECT_EQ(func.identifier.Ident(), "Add");
  EXPECT_EQ(func.mangled_name, "_Z3Addii");
  EXPECT_EQ(func.return_type.rs_name, "i32");
  EXPECT_THAT(func.return_type.type_params, IsEmpty());

  EXPECT_THAT(func.params, SizeIs(2));
  EXPECT_EQ(func.params[0].type.rs_name, "i32");
  EXPECT_THAT(func.params[0].type.type_params, IsEmpty());
  EXPECT_EQ(func.params[0].identifier.Ident(), "a");
  EXPECT_EQ(func.params[1].type.rs_name, "i32");
  EXPECT_THAT(func.params[1].type.type_params, IsEmpty());
  EXPECT_EQ(func.params[1].identifier.Ident(), "b");
}

TEST(AstVisitorTest, TestImportPointerFunc) {
  IR ir = ImportCode({"int* Foo(int* a);"}, {});
  ASSERT_THAT(ir.Functions(), SizeIs(1));

  Func func = ir.Functions()[0];

  ASSERT_THAT(func.params, SizeIs(1));
  Type return_type = func.return_type;
  Type param_type = func.params[0].type;

  for (Type type : {return_type, param_type}) {
    EXPECT_EQ(type.rs_name, "*mut");
    EXPECT_EQ(type.cc_name, "*");
    ASSERT_THAT(type.type_params, SizeIs(1));
    const Type& pointee = type.type_params[0];
    EXPECT_EQ(pointee.rs_name, "i32");
    EXPECT_EQ(pointee.cc_name, "int");
    EXPECT_THAT(pointee.type_params, IsEmpty());
  }
}

TEST(AstVisitorTest, Struct) {
  IR ir = ImportCode(
      {"struct SomeStruct { int first_field; int second_field; };"}, {});
  EXPECT_THAT(ir.Functions(), SizeIs(0));
  EXPECT_THAT(ir.Records(), SizeIs(1));

  Record some_struct = ir.Records()[0];
  EXPECT_THAT(some_struct.Fields(), SizeIs(2));
  EXPECT_EQ(some_struct.Ident().Ident(), "SomeStruct");
  Field first = some_struct.Fields()[0];
  EXPECT_EQ(first.identifier.Ident(), "first_field");
  EXPECT_EQ(first.type.cc_name, "int");
  EXPECT_EQ(first.type.rs_name, "i32");
  Field second = some_struct.Fields()[1];
  EXPECT_EQ(second.identifier.Ident(), "second_field");
  EXPECT_EQ(second.type.cc_name, "int");
  EXPECT_EQ(second.type.rs_name, "i32");
}

}  // namespace
}  // namespace rs_bindings_from_cc
