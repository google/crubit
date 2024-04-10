// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/inference/replace_macros.h"

#include <memory>
#include <string>
#include <string_view>

#include "nullability/inference/ctn_replacement_macros.h"
#include "nullability/macro_arg_capture.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Expr.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/ASTMatchers/ASTMatchers.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/StringRef.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"  // IWYU pragma: keep
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {

using ::clang::CallExpr;
using ::clang::ast_matchers::anyOf;
using ::clang::ast_matchers::callExpr;
using ::clang::ast_matchers::functionDecl;
using ::clang::ast_matchers::hasDeclaration;
using ::clang::ast_matchers::hasName;
using ::clang::ast_matchers::match;
using ::clang::ast_matchers::selectFirst;
using ::testing::IsEmpty;
using ::testing::IsNull;
using ::testing::NotNull;

clang::TestInputs getInputs(llvm::StringRef Source) {
  clang::TestInputs Inputs = Source;
  for (const auto& Entry : llvm::ArrayRef(ctn_replacement_macros_create(),
                                          ctn_replacement_macros_size()))
    Inputs.ExtraFiles.try_emplace(Entry.name, Entry.data);
  Inputs.ExtraArgs.push_back("-include");
  Inputs.ExtraArgs.push_back(std::string(ReplacementMacrosHeaderFileName));

  Inputs.MakeAction = []() { return std::make_unique<ReplaceMacrosAction>(); };
  return Inputs;
}

TEST(ReplaceMacrosAction, ReplacesCHECK) {
  static constexpr std::string_view Source = R"cc(
#define CHECK(x) \
      if (!x) __builtin_abort();

    void foo() { CHECK(nullptr); }
  )cc";
  clang::TestAST AST(getInputs(Source));

  const CallExpr* ArgumentCapture = selectFirst<CallExpr>(
      "call", match(callExpr(hasDeclaration(
                                 functionDecl(hasName(ArgCaptureAbortIfFalse))))
                        .bind("call"),
                    AST.context()));
  ASSERT_THAT(ArgumentCapture, NotNull());
  ASSERT_THAT(ArgumentCapture->getArg(0), NotNull());
  EXPECT_TRUE(ArgumentCapture->getArg(0)->isNullPointerConstant(
      AST.context(), Expr::NPC_ValueDependentIsNotNull));
}

TEST(ReplaceMacrosAction, ReplacesCHECK_NE) {
  static constexpr std::string_view Source = R"cc(
#define CHECK_NE(x, y) \
      if (x == y) __builtin_abort();

    void foo() {
      CHECK_NE(nullptr, nullptr);
    }
  )cc";
  clang::TestAST AST(getInputs(Source));

  const CallExpr* ArgumentCapture = selectFirst<CallExpr>(
      "call", match(callExpr(hasDeclaration(
                                 functionDecl(hasName(ArgCaptureAbortIfEqual))))
                        .bind("call"),
                    AST.context()));
  ASSERT_THAT(ArgumentCapture, NotNull());
  ASSERT_THAT(ArgumentCapture->getArg(0), NotNull());
  EXPECT_TRUE(ArgumentCapture->getArg(0)->isNullPointerConstant(
      AST.context(), Expr::NPC_ValueDependentIsNotNull));
  ASSERT_THAT(ArgumentCapture->getArg(1), NotNull());
  EXPECT_TRUE(ArgumentCapture->getArg(1)->isNullPointerConstant(
      AST.context(), Expr::NPC_ValueDependentIsNotNull));
}

TEST(ReplaceMacrosAction, DoesNotReplaceMacroNotInReplacementFile) {
  static constexpr std::string_view Source = R"cc(
#define TOTALLY_MADE_UP_CHECK_THAT_IS_NOT_IN_REPLACEMENT_FILE(x) \
      if (!x) __builtin_abort();

    void foo() {
      TOTALLY_MADE_UP_CHECK_THAT_IS_NOT_IN_REPLACEMENT_FILE(nullptr);
    }
  )cc";
  clang::TestAST AST(getInputs(Source));

  const CallExpr* ArgumentCaptureFunctionCall = selectFirst<CallExpr>(
      "call",
      match(callExpr(hasDeclaration(
                         anyOf(functionDecl(hasName(ArgCaptureAbortIfFalse)),
                               functionDecl(hasName(ArgCaptureAbortIfEqual)))))
                .bind("call"),
            AST.context()));
  ASSERT_THAT(ArgumentCaptureFunctionCall, IsNull());
}

TEST(ReplaceMacrosAction, ArgumentCaptureCompilesWithVariousTypes) {
  static constexpr std::string_view Source = R"cc(
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wmacro-redefined"
#define CHECK(x) \
      if (!x) __builtin_abort();
#pragma clang diagnostic pop

    struct S {
      S();
      S(const S &);
      S(S &&);

      operator bool() &;
      operator bool() const &;
    };

    void foo() {
      S s;
      CHECK(static_cast<S *>(&s));
      CHECK(static_cast<const S *>(&s));
      CHECK(static_cast<S &>(s));
      CHECK(static_cast<const S &>(s));
      CHECK(static_cast<S &&>(s));
    }
  )cc";

  clang::TestAST AST(getInputs(Source));

  EXPECT_THAT(AST.diagnostics(), IsEmpty());
}
}  // namespace
}  // namespace clang::tidy::nullability
