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

namespace {

llvm::Expected<llvm::SmallVector<Lifetime>> GetAnnotatedOrElidedLifetimes(
    llvm::ArrayRef<const clang::Attr*> /*attrs*/, int num_expected,
    LifetimeSymbolTable& /*symbol_table*/,
    const std::function<llvm::Expected<Lifetime>()>& elided_lifetime_factory,
    const clang::ASTContext& /*ast_context*/) {
  assert(num_expected > 0);

  llvm::SmallVector<Lifetime> lifetimes;
  lifetimes.reserve(num_expected);

  // TODO(mboehme): Extract lifetime annotations from `attrs` if present.

  // No lifetime annoations: Use elided lifetimes.
  for (int i = 0; i < num_expected; ++i) {
    llvm::Expected<Lifetime> maybe_lifetime = elided_lifetime_factory();
    if (maybe_lifetime) {
      lifetimes.push_back(maybe_lifetime.get());
    } else {
      return maybe_lifetime.takeError();
    }
  }
  return lifetimes;
}

llvm::Error AddLifetimeAnnotationsForType(
    clang::TypeLoc type_loc, LifetimeSymbolTable& symbol_table,
    const std::function<llvm::Expected<Lifetime>()>& elided_lifetime_factory,
    const clang::ASTContext& ast_context, TypeLifetimes& lifetimes) {
  assert(type_loc);
  assert(elided_lifetime_factory);

  llvm::SmallVector<const clang::Attr*> attrs;

  clang::TypeLoc next_type_loc = type_loc.getNextTypeLoc();
  if (next_type_loc) {
    if (llvm::Error err = AddLifetimeAnnotationsForType(
            next_type_loc, symbol_table, elided_lifetime_factory, ast_context,
            lifetimes)) {
      return err;
    }
  }

  llvm::SmallVector<clang::TypeLoc> template_args = GetTemplateArgs(type_loc);
  if (!template_args.empty()) {
    for (size_t i = 0; i < template_args.size(); ++i) {
      if (llvm::Error err = AddLifetimeAnnotationsForType(
              template_args[i], symbol_table, elided_lifetime_factory,
              ast_context, lifetimes)) {
        return err;
      }
    }
    return llvm::Error::success();
  }

  if (type_loc.getAs<clang::PointerTypeLoc>() ||
      type_loc.getAs<clang::ReferenceTypeLoc>()) {
    llvm::Expected<llvm::SmallVector<Lifetime>> maybe_pointee_lifetime =
        GetAnnotatedOrElidedLifetimes(attrs, 1, symbol_table,
                                      elided_lifetime_factory, ast_context);
    if (maybe_pointee_lifetime) {
      lifetimes.append(maybe_pointee_lifetime.get());
    } else {
      return maybe_pointee_lifetime.takeError();
    }
  }

  return llvm::Error::success();
}

llvm::Expected<llvm::SmallVector<Lifetime>> GetThisLifetimes(
    const clang::CXXMethodDecl* method, LifetimeSymbolTable& symbol_table,
    const std::function<llvm::Expected<Lifetime>()>& elided_lifetime_factory) {
  llvm::ArrayRef<const clang::Attr*> attrs;
  if (method->hasAttrs()) {
    attrs = method->getAttrs();
  }
  return GetAnnotatedOrElidedLifetimes(
      attrs, 1, symbol_table, elided_lifetime_factory, method->getASTContext());
}

llvm::Expected<FunctionLifetimes> GetLifetimeAnnotationsInternal(
    const clang::FunctionDecl* func, LifetimeSymbolTable& symbol_table,
    bool elision_enabled) {
  FunctionLifetimes result;

  if (!func->getTypeSourceInfo()) {
    // TODO(mboehme): At least try to do lifetime elision.
    return llvm::createStringError(
        llvm::inconvertibleErrorCode(),
        absl::StrCat("Can't extract lifetimes as '", func->getNameAsString(),
                     "' appears to be a generated function"));
  }

  std::function<llvm::Expected<Lifetime>()> elided_lifetime_factory;
  if (elision_enabled) {
    elided_lifetime_factory = [&symbol_table]() -> llvm::Expected<Lifetime> {
      Lifetime lifetime = Lifetime::CreateVariable();
      symbol_table.LookupLifetimeAndMaybeDeclare(lifetime);
      return lifetime;
    };
  } else {
    elided_lifetime_factory = [func]() -> llvm::Expected<Lifetime> {
      return llvm::createStringError(
          llvm::inconvertibleErrorCode(),
          absl::StrCat("Lifetime elision not enabled for '",
                       func->getNameAsString(), "'"));
    };
  }

  if (const auto* method = clang::dyn_cast<clang::CXXMethodDecl>(func)) {
    if (llvm::Error err =
            GetThisLifetimes(method, symbol_table, elided_lifetime_factory)
                .moveInto(result.this_lifetimes)) {
      return std::move(err);
    }
  }

  llvm::SmallVector<Lifetime> all_input_lifetimes;
  result.param_lifetimes.resize(func->getNumParams());
  for (unsigned i = 0; i < func->getNumParams(); ++i) {
    const clang::ParmVarDecl* param = func->getParamDecl(i);

    if (llvm::Error err = AddLifetimeAnnotationsForType(
            param->getTypeSourceInfo()->getTypeLoc(), symbol_table,
            elided_lifetime_factory, func->getASTContext(),
            result.param_lifetimes[i])) {
      return std::move(err);
    }

    all_input_lifetimes.append(result.param_lifetimes[i]);
  }

  std::function<llvm::Expected<Lifetime>()> elided_return_lifetime_factory;
  if (!elision_enabled) {
    elided_return_lifetime_factory = [func]() -> llvm::Expected<Lifetime> {
      return llvm::createStringError(
          llvm::inconvertibleErrorCode(),
          absl::StrCat("Lifetime elision not enabled for '",
                       func->getNameAsString(), "'"));
    };
  } else if (!result.this_lifetimes.empty()) {
    // If we have an implicit `this` parameter, its lifetime is assigned to all
    // output lifetimes.
    elided_return_lifetime_factory =
        [this_lifetime =
             result.this_lifetimes.back()]() -> llvm::Expected<Lifetime> {
      return this_lifetime;
    };
  } else if (all_input_lifetimes.size() == 1) {
    // If we have a single input lifetime, its lifetime is assigned to all
    // output lifetimes.
    // Otherwise, elided_return_lifetime_factory remains empty, and we get an
    // error if there are any elided lifetimes in the return type.
    elided_return_lifetime_factory =
        [return_lifetime =
             all_input_lifetimes[0]]() -> llvm::Expected<Lifetime> {
      return return_lifetime;
    };
  } else {
    elided_return_lifetime_factory = [func]() -> llvm::Expected<Lifetime> {
      return llvm::createStringError(
          llvm::inconvertibleErrorCode(),
          absl::StrCat(
              "Cannot elide output lifetimes for '", func->getNameAsString(),
              "' because it is a non-member function that does not have "
              "exactly one input lifetime"));
    };
  }

  if (llvm::Error err = AddLifetimeAnnotationsForType(
          func->getTypeSourceInfo()
              ->getTypeLoc()
              .getAsAdjusted<clang::FunctionTypeLoc>()
              .getReturnLoc(),
          symbol_table, elided_return_lifetime_factory, func->getASTContext(),
          result.return_lifetimes)) {
    return err;
  }

  return result;
}

}  // namespace

llvm::Expected<FunctionLifetimes> GetLifetimeAnnotations(
    const clang::FunctionDecl* func, const LifetimeAnnotationContext& context,
    LifetimeSymbolTable* symbol_table) {
  // TODO(mboehme):
  // - Add support for retrieving actual lifetime annotations (not just
  //   lifetimes implied by elision).
  // - If we have multiple declarations of a function, make sure they are all
  //   annotated with the same lifetimes.

  clang::SourceManager& source_manager =
      func->getASTContext().getSourceManager();
  clang::FileID file_id =
      source_manager.getFileID(func->getSourceRange().getBegin());
  bool elision_enabled = context.lifetime_elision_files.contains(file_id);

  LifetimeSymbolTable throw_away_symbol_table;
  if (!symbol_table) {
    symbol_table = &throw_away_symbol_table;
  }
  return GetLifetimeAnnotationsInternal(func, *symbol_table, elision_enabled);
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
