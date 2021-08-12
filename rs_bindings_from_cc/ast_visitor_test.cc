// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <memory>
#include <string>
#include <vector>

#include "rs_bindings_from_cc/ast_consumer_factory.h"
#include "rs_bindings_from_cc/ir.h"
#include "testing/base/public/gmock.h"
#include "testing/base/public/gunit.h"
#include "third_party/absl/strings/string_view.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Tooling/Tooling.h"

namespace rs_bindings_from_cc {
namespace {

using ::testing::IsEmpty;
using ::testing::SizeIs;

IR ImportCode(const absl::string_view code,
              const std::vector<absl::string_view>& args) {
  IR ir;
  AstConsumerFactory ast_consumer_factory(ir);
  std::unique_ptr<clang::tooling::FrontendActionFactory> action_factory =
      clang::tooling::newFrontendActionFactory(&ast_consumer_factory);
  std::vector<std::string> args_as_strings(args.begin(), args.end());
  clang::tooling::runToolOnCodeWithArgs(action_factory->create(), code,
                                        args_as_strings);
  return ir;
}

TEST(AstVisitorTest, TestNoop) {
  IR ir = ImportCode("// nothing interesting there.", {});
  EXPECT_THAT(ir.Functions(), IsEmpty());
}

TEST(AstVisitorTest, TestImportFuncWithVoidReturnType) {
  IR ir = ImportCode("void Foo();", {});
  ASSERT_THAT(ir.Functions(), SizeIs(1));
  Func func = ir.Functions()[0];
  EXPECT_EQ(func.Ident().Ident(), "Foo");
  EXPECT_EQ(func.MangledName(), "_Z3Foov");
  EXPECT_TRUE(func.ReturnType().IsVoid());
  EXPECT_THAT(func.Params(), IsEmpty());
}

TEST(AstVisitorTest, TestImportTwoFuncs) {
  IR ir = ImportCode("void Foo(); void Bar();", {});
  ASSERT_THAT(ir.Functions(), SizeIs(2));

  Func foo = ir.Functions()[0];
  EXPECT_EQ(foo.Ident().Ident(), "Foo");
  EXPECT_EQ(foo.MangledName(), "_Z3Foov");
  EXPECT_TRUE(foo.ReturnType().IsVoid());
  EXPECT_THAT(foo.Params(), IsEmpty());

  Func bar = ir.Functions()[1];
  EXPECT_EQ(bar.Ident().Ident(), "Bar");
  EXPECT_EQ(bar.MangledName(), "_Z3Barv");
  EXPECT_TRUE(bar.ReturnType().IsVoid());
  EXPECT_THAT(bar.Params(), IsEmpty());
}

TEST(AstVisitorTest, TestImportFuncJustOnce) {
  IR ir = ImportCode(
      "void Foo();"
      "void Foo();",
      {});
  ASSERT_THAT(ir.Functions(), SizeIs(1));
  Func func = ir.Functions()[0];
  EXPECT_EQ(func.Ident().Ident(), "Foo");
}

TEST(AstVisitorTest, TestImportFuncParams) {
  IR ir = ImportCode("int Add(int a, int b);", {});
  EXPECT_THAT(ir.Functions(), SizeIs(1));

  Func func = ir.Functions()[0];
  EXPECT_EQ(func.Ident().Ident(), "Add");
  EXPECT_EQ(func.MangledName(), "_Z3Addii");
  EXPECT_EQ(func.ReturnType().RsName(), "i32");

  EXPECT_THAT(func.Params(), SizeIs(2));
  EXPECT_EQ(func.Params()[0].ParamType().RsName(), "i32");
  EXPECT_EQ(func.Params()[0].Ident().Ident(), "a");
  EXPECT_EQ(func.Params()[1].ParamType().RsName(), "i32");
  EXPECT_EQ(func.Params()[1].Ident().Ident(), "b");
}

}  // namespace
}  // namespace rs_bindings_from_cc
