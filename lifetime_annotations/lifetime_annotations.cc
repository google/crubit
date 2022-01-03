// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_annotations.h"

#include <functional>
#include <optional>
#include <utility>

#include "third_party/absl/strings/str_cat.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclCXX.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Lex/Pragma.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Lex/Preprocessor.h"

namespace devtools_rust {

static std::optional<FunctionLifetimes> ElidedLifetimes(
    const clang::FunctionDecl* func,
    std::function<Lifetime()> lifetime_factory) {
  FunctionLifetimes result;

  // Every input lifetime is assigned a distinct lifetime.
  result.param_lifetimes.resize(func->getNumParams());
  llvm::SmallVector<Lifetime> all_input_lifetimes;
  for (unsigned i = 0; i < func->getNumParams(); ++i) {
    const clang::ParmVarDecl* param = func->getParamDecl(i);

    result.param_lifetimes[i] =
        CreateLifetimesForType(param->getType(), lifetime_factory);
    all_input_lifetimes.append(result.param_lifetimes[i]);
  }

  if (clang::isa<clang::CXXMethodDecl>(func)) {
    Lifetime this_lifetime = lifetime_factory();
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
    const clang::FunctionDecl* func, const LifetimeAnnotationContext& context,
    LifetimeSymbolTable* symbol_table) {
  // TODO(mboehme):
  // - Add support for retrieving actual lifetime annotations (not just
  //   lifetimes implied by elision).
  // - If we have multiple declarations of a function, make sure they are all
  //   annotated with the same lifetimes.

  // For the time being, we only return elided lifetimes.

  // See whether there are any lifetimes to be elided at all.
  std::optional<FunctionLifetimes> elided_lifetimes =
      ElidedLifetimes(func, Lifetime::Static);

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

  // We know we're allowed to elide lifetimes, so produce the elided lifetimes
  // again, but this time create new variables for the elided lifetimes.
  if (symbol_table) {
    elided_lifetimes = ElidedLifetimes(func, [symbol_table]() {
      Lifetime lifetime = Lifetime::CreateVariable();
      symbol_table->LookupLifetimeAndMaybeDeclare(lifetime);
      return lifetime;
    });
  } else {
    elided_lifetimes = ElidedLifetimes(func, Lifetime::CreateVariable);
  }

  assert(elided_lifetimes.has_value());
  return std::move(elided_lifetimes).value();
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
      "clang", new LifetimeElisionPragmaHandler(context));
}

llvm::SmallVector<clang::TypeLoc> GetTemplateArgs(clang::TypeLoc type_loc) {
  llvm::SmallVector<clang::TypeLoc> args;
  if (auto template_specialization_type_loc =
          type_loc.getAs<clang::TemplateSpecializationTypeLoc>()) {
    for (unsigned i = 0; i < template_specialization_type_loc.getNumArgs();
         ++i) {
      args.push_back(template_specialization_type_loc.getArgLoc(i)
                         .getTypeSourceInfo()
                         ->getTypeLoc());
    }
  } else if (auto dependent_template_specialization_type_loc =
                 type_loc
                     .getAs<clang::DependentTemplateSpecializationTypeLoc>()) {
    // TODO(mboehme): Where does this occur exactly? Do we need to be handling
    // it?
    // AFAICT, this happens if we're looking at a dependent template name
    // (https://en.cppreference.com/w/cpp/language/dependent_name), which
    // probably means that this can only happen in template definitions (as
    // opposed to template instantiations), and we aren't analyzing those for
    // now. At the least, I haven't been able to trigger this case from a test.
    // Putting an assertion in here so that we notice this case if it does come
    // up.
    assert(false);
    for (unsigned i = 0;
         i < dependent_template_specialization_type_loc.getNumArgs(); ++i) {
      args.push_back(dependent_template_specialization_type_loc.getArgLoc(i)
                         .getTypeSourceInfo()
                         ->getTypeLoc());
    }
  }
  return args;
}

}  // namespace devtools_rust
