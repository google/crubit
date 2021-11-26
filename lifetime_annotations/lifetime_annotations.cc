// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_annotations.h"

#include <optional>
#include <utility>

#include "third_party/absl/strings/str_cat.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclCXX.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Lex/Pragma.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Lex/Preprocessor.h"

namespace devtools_rust {

static std::optional<FunctionLifetimes> ElidedLifetimes(
    const clang::FunctionDecl* func) {
  FunctionLifetimes result;

  // Every input lifetime is assigned a distinct lifetime.
  result.param_lifetimes.resize(func->getNumParams());
  llvm::SmallVector<Lifetime> all_input_lifetimes;
  for (unsigned i = 0; i < func->getNumParams(); ++i) {
    const clang::ParmVarDecl* param = func->getParamDecl(i);

    result.param_lifetimes[i] =
        CreateLifetimesForType(param->getType(), Lifetime::CreateVariable);
    all_input_lifetimes.append(result.param_lifetimes[i]);
  }

  if (clang::isa<clang::CXXMethodDecl>(func)) {
    Lifetime this_lifetime = Lifetime::CreateVariable();
    result.this_lifetimes.push_back(this_lifetime);

    // If we have an implicit `this` parameter, its lifetime is assigned to all
    // output lifetimes.
    result.return_lifetimes = CreateLifetimesForType(
        func->getReturnType(), [this_lifetime]() { return this_lifetime; });
    return result;
  }

  // If we have no output lifetimes, there's nothing left to do.
  if (CreateLifetimesForType(func->getReturnType(), Lifetime::Static).empty()) {
    return result;
  }

  // If we have a single input lifetime, its lifetime is assigned to all output
  // lifetimes.
  if (all_input_lifetimes.size() == 1) {
    result.return_lifetimes = CreateLifetimesForType(
        func->getReturnType(),
        [&all_input_lifetimes]() { return all_input_lifetimes[0]; });
    return result;
  }

  return std::nullopt;
}

llvm::Expected<FunctionLifetimes> GetLifetimeAnnotations(
    const clang::FunctionDecl* func, const LifetimeAnnotationContext& context) {
  // TODO(mboehme):
  // - Add support for retrieving actual lifetime annotations (not just
  //   lifetimes implied by elision).
  // - If we have multiple declarations of a function, make sure they are all
  //   annotated with the same lifetimes.

  // For the time being, we only return elided lifetimes.
  std::optional<FunctionLifetimes> elided_lifetimes = ElidedLifetimes(func);

  if (!elided_lifetimes.has_value()) {
    return llvm::createStringError(
        llvm::inconvertibleErrorCode(),
        absl::StrCat("Cannot determine output lifetimes for '",
                     func->getNameAsString(),
                     "' because it does not have exactly one input lifetime"));
  }

  // If the function has any elided lifetimes, we need to check if lifetime
  // elision is enabled.
  if (elided_lifetimes->ContainsLifetimes()) {
    clang::SourceManager& source_manager =
        func->getASTContext().getSourceManager();
    clang::FileID file_id =
        source_manager.getFileID(func->getSourceRange().getBegin());
    if (!context.lifetime_elision_files.contains(file_id)) {
      return llvm::createStringError(
          llvm::inconvertibleErrorCode(),
          absl::StrCat("Lifetime elision not enabled for '",
                       func->getNameAsString(), "'"));
    }
  }

  return *std::move(elided_lifetimes);
}

namespace {

class LifetimeElisionPragmaHandler : public clang::PragmaHandler {
 public:
  explicit LifetimeElisionPragmaHandler(
      std::shared_ptr<LifetimeAnnotationContext> context)
      : clang::PragmaHandler("lifetime_elision"), context_(context) {}

  void HandlePragma(clang::Preprocessor& preprocessor,
                    clang::PragmaIntroducer introducer,
                    clang::Token&) override {
    clang::SourceManager& source_manager = preprocessor.getSourceManager();
    clang::FileID file_id = source_manager.getFileID(introducer.Loc);
    context_->lifetime_elision_files.insert(file_id);
  }

 private:
  std::shared_ptr<LifetimeAnnotationContext> context_;
};

}  // namespace

void AddLifetimeAnnotationHandlers(
    clang::CompilerInstance& compiler,
    std::shared_ptr<LifetimeAnnotationContext> context) {
  // Preprocessor takes ownership of the handler.
  compiler.getPreprocessor().AddPragmaHandler(
      new LifetimeElisionPragmaHandler(context));
}

}  // namespace devtools_rust
