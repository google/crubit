// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/test/run_on_code.h"

#include <functional>
#include <memory>
#include <string>

#include "lifetime_annotations/lifetime_annotations.h"
#include "clang/include/clang/AST/ASTConsumer.h"
#include "clang/include/clang/Frontend/FrontendAction.h"
#include "clang/include/clang/Serialization/PCHContainerOperations.h"
#include "clang/include/clang/Tooling/Tooling.h"
#include "llvm/include/llvm/ADT/ArrayRef.h"
#include "llvm/include/llvm/ADT/StringRef.h"

namespace clang {
namespace tidy {
namespace lifetimes {

namespace {

class RunOnCodeASTConsumer : public clang::ASTConsumer {
 public:
  explicit RunOnCodeASTConsumer(
      const std::function<void(clang::ASTContext&,
                               const LifetimeAnnotationContext&)>& operation,
      std::shared_ptr<LifetimeAnnotationContext> lifetime_context)
      : operation_(operation), lifetime_context_(lifetime_context) {}

  void HandleTranslationUnit(clang::ASTContext& ast_context) override {
    operation_(ast_context, *lifetime_context_);
  }

 private:
  const std::function<void(clang::ASTContext&,
                           const LifetimeAnnotationContext&)>& operation_;
  std::shared_ptr<LifetimeAnnotationContext> lifetime_context_;
};

class RunOnCodeAction : public clang::ASTFrontendAction {
 public:
  explicit RunOnCodeAction(
      const std::function<void(clang::ASTContext&,
                               const LifetimeAnnotationContext&)>& operation,
      std::shared_ptr<LifetimeAnnotationContext> lifetime_context)
      : operation_(operation), lifetime_context_(lifetime_context) {}

  std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
      clang::CompilerInstance& compiler, llvm::StringRef) override {
    AddLifetimeAnnotationHandlers(compiler.getPreprocessor(),
                                  lifetime_context_);
    return std::make_unique<RunOnCodeASTConsumer>(operation_,
                                                  lifetime_context_);
  }

 private:
  const std::function<void(clang::ASTContext&,
                           const LifetimeAnnotationContext&)>& operation_;
  std::shared_ptr<LifetimeAnnotationContext> lifetime_context_;
};

}  // namespace

bool runOnCodeWithLifetimeHandlers(
    llvm::StringRef code,
    const std::function<void(clang::ASTContext&,
                             const LifetimeAnnotationContext&)>& operation,
    llvm::ArrayRef<std::string> args,
    const clang::tooling::FileContentMappings& file_contents) {
  auto context = std::make_shared<LifetimeAnnotationContext>();
  return clang::tooling::runToolOnCodeWithArgs(
      std::make_unique<RunOnCodeAction>(operation, context), code, args,
      "input.cc", "run-on-code-with-lifetime-handlers",
      std::make_shared<clang::PCHContainerOperations>(), file_contents);
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
