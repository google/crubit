// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability/pragma.h"

#include <memory>
#include <optional>

#include "clang/AST/ASTConsumer.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Frontend/CompilerInstance.h"
#include "clang/Frontend/FrontendActions.h"
#include "clang/Testing/TestAST.h"
#include "llvm/ADT/StringMap.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googlemock/include/gmock/gmock.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang::tidy::nullability {
namespace {
using ::testing::ElementsAre;
using ::testing::IsEmpty;
using ::testing::Pair;
using ::testing::UnorderedElementsAre;

class PragmaTest : public ::testing::Test {
 protected:
  std::optional<TestAST> AST;
  TestInputs Inputs;
  NullabilityPragmas Pragmas;

  PragmaTest() {
    Inputs.FileName = "main.cc";
    Inputs.MakeAction = [&] {
      struct Action : public SyntaxOnlyAction {
        NullabilityPragmas &Pragmas;
        Action(NullabilityPragmas &Pragmas) : Pragmas(Pragmas) {}
        std::unique_ptr<ASTConsumer> CreateASTConsumer(
            CompilerInstance &CI, llvm::StringRef File) override {
          registerPragmaHandler(CI.getPreprocessor(), Pragmas);
          return SyntaxOnlyAction::CreateASTConsumer(CI, File);
        }
      };
      return std::make_unique<Action>(Pragmas);
    };
  }

  // Populates AST
  void parse() {
    Pragmas = NullabilityPragmas{};
    AST.emplace(Inputs);
  }

  FileID file(llvm::StringRef Path) {
    auto File = AST->fileManager().getFileRef(Path);
    if (!File) {
      ADD_FAILURE() << llvm::toString(File.takeError());
      return FileID();
    }

    FileID ID = AST->sourceManager().translateFile(*File);
    EXPECT_TRUE(ID.isValid()) << Path;
    return ID;
  }
};

TEST_F(PragmaTest, None) {
  Inputs.Code = "";
  parse();
  EXPECT_THAT(Pragmas, IsEmpty());
}

TEST_F(PragmaTest, Files) {
  Inputs.Code = R"cpp(
#include "header.h"
#pragma nullability file_default nonnull
  )cpp";
  Inputs.ExtraFiles["header.h"] = "#pragma nullability file_default nullable";
  parse();
  EXPECT_THAT(Pragmas, UnorderedElementsAre(
                           Pair(file("main.cc"), NullabilityKind::NonNull),
                           Pair(file("header.h"), NullabilityKind::Nullable)));
}

TEST_F(PragmaTest, Macro) {
  Inputs.Code = R"cpp(
#include "header.h"
    DEFAULT_NONNULL
  )cpp";
  Inputs.ExtraFiles["header.h"] = R"cpp(
#define DEFAULT_NONNULL _Pragma("nullability file_default nonnull")
  )cpp";
  parse();
  EXPECT_THAT(Pragmas, UnorderedElementsAre(
                           Pair(file("main.cc"), NullabilityKind::NonNull)));
}

MATCHER_P(message, M, "") { return arg.getMessage() == M; }

TEST_F(PragmaTest, Invalid) {
  Inputs.Code = R"cpp(
#pragma nullability file_default nonnull
#pragma nullability file_default nullable
#pragma nullability file_default bleh
  )cpp";
  parse();
  EXPECT_THAT(Pragmas, UnorderedElementsAre(
                           Pair(file("main.cc"), NullabilityKind::NonNull)));
  EXPECT_THAT(
      AST->diagnostics(),
      ElementsAre(
          message(
              "ignoring repeated #pragma nullability file_default directive"),
          message(
              "ignoring invalid #pragma nullability file_default directive")));
}

}  // namespace
}  // namespace clang::tidy::nullability
