// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_annotations.h"

#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/strings/str_cat.h"
#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/lifetime_symbol_table.h"
#include "lifetime_annotations/pointee_type.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "clang/AST/APValue.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/PrettyPrinter.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LangOptions.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Lex/Pragma.h"
#include "clang/Lex/Preprocessor.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"

namespace clang {
namespace tidy {
namespace lifetimes {
namespace {

llvm::Expected<FunctionLifetimes> ParseLifetimeAnnotations(
    const clang::FunctionDecl* func, LifetimeSymbolTable& symbol_table,
    const std::string& lifetimes_str) {
  clang::LangOptions lang_opts;
  clang::Lexer lexer(clang::SourceLocation(), lang_opts, lifetimes_str.data(),
                     lifetimes_str.data(),
                     lifetimes_str.data() + lifetimes_str.size());

  const char* end = lifetimes_str.data() + lifetimes_str.size();

  auto tok = [&lexer, &lifetimes_str, end]() -> llvm::StringRef {
    clang::Token token;
    if (lexer.getBufferLocation() != end) {
      lexer.LexFromRawLexer(token);
      return llvm::StringRef(
          lifetimes_str.data() + token.getLocation().getRawEncoding(),
          token.getLength());
    }
    return "";
  };

  // TODO(veluca): this is too permissive.
  auto next_lifetime = [&]() {
    llvm::StringRef next = tok();
    while (next == "(" || next == ")" || next == "," || next == "->" ||
           next == ":" || next == "[" || next == "]" || next == ">" ||
           next == "<") {
      next = tok();
    }
    return next;
  };

  FunctionLifetimeFactorySingleCallback factory(
      [&symbol_table, &next_lifetime]() -> llvm::Expected<Lifetime> {
        llvm::StringRef next = next_lifetime();
        if (next.empty()) {
          return llvm::createStringError(
              llvm::inconvertibleErrorCode(),
              "Invalid lifetime annotation: too few lifetimes");
        }
        return symbol_table.LookupNameAndMaybeDeclare(next);
      });

  auto ret = FunctionLifetimes::CreateForDecl(func, factory);

  if (!next_lifetime().empty()) {
    return llvm::createStringError(
        llvm::inconvertibleErrorCode(),
        "Invalid lifetime annotation: too many lifetimes");
  }
  return ret;
}

// Parse a "(a, b): (a, b), (), a -> b"-style annotation into a
// FunctionLifetimes.
// TODO(veluca): this is a temporary solution.
llvm::Expected<FunctionLifetimes> ParseLifetimeAnnotations(
    const clang::FunctionDecl* func, LifetimeSymbolTable& symbol_table,
    const clang::AnnotateAttr* attr) {
  auto error = [func](absl::string_view detail = absl::string_view()) {
    std::string msg = absl::StrCat("Invalid lifetime annotation for function ",
                                   func->getNameAsString());
    if (!detail.empty()) {
      absl::StrAppend(&msg, ": ", detail);
    }
    return llvm::createStringError(llvm::inconvertibleErrorCode(), msg);
  };

  if (attr->args_size() != 1) {
    return error("`lifetimes` attribute must have exactly one argument");
  }

  // The lexer requires a null character at the end of the string, so copy it to
  // a std::string to guarantee this.
  llvm::StringRef lifetimes;
  if (llvm::Error err =
          EvaluateAsStringLiteral(*attr->args_begin(), func->getASTContext())
              .moveInto(lifetimes)) {
    return error(toString(std::move(err)));
  }
  return ParseLifetimeAnnotations(func, symbol_table, lifetimes.str());
}

llvm::Expected<FunctionLifetimes> GetLifetimeAnnotationsInternal(
    const clang::FunctionDecl* func, LifetimeSymbolTable& symbol_table,
    bool elision_enabled) {
  const clang::AnnotateAttr* lifetime_annotation = nullptr;
  for (const clang::Attr* attr : func->attrs()) {
    if (auto annotate = clang::dyn_cast<clang::AnnotateAttr>(attr)) {
      if (annotate->getAnnotation() == "lifetimes") {
        if (lifetime_annotation != nullptr) {
          return llvm::createStringError(
              llvm::inconvertibleErrorCode(),
              absl::StrCat("Can't extract lifetimes as '",
                           func->getNameAsString(),
                           "' has multiple lifetime annotations"));
        }
        lifetime_annotation = annotate;
      }
    }
  }
  if (lifetime_annotation) {
    return ParseLifetimeAnnotations(func, symbol_table, lifetime_annotation);
  }

  class Factory : public FunctionLifetimeFactory {
   public:
    Factory(bool elision_enabled, const clang::FunctionDecl* func,
            LifetimeSymbolTable& symbol_table)
        : elision_enabled(elision_enabled),
          func(func),
          symbol_table(symbol_table) {}

   private:
    llvm::Expected<ValueLifetimes> CreateParamLifetimes(
        clang::QualType param_type) const override {
      // TODO(mboehme): parse lifetime annotations from `type` if present.
      return ValueLifetimes::Create(
          param_type, [this]() -> llvm::Expected<Lifetime> {
            // As a special-case, lifetime is always inferred for the `this`
            // parameter for destructors. The obvious lifetime is definitionally
            // correct in this case: the object must be valid for the duration
            // of the call, or else the behavior is undefined. So we can infer
            // safely even if elision is disabled.
            if (!elision_enabled &&
                func->getDeclName().getNameKind() !=
                    clang::DeclarationName::CXXDestructorName) {
              return llvm::createStringError(
                  llvm::inconvertibleErrorCode(),
                  absl::StrCat("Lifetime elision not enabled for '",
                               func->getNameAsString(), "'"));
            }

            Lifetime lifetime = Lifetime::CreateVariable();
            symbol_table.LookupLifetimeAndMaybeDeclare(lifetime);
            return lifetime;
          });
    }

    static std::optional<Lifetime> GetSingleInputLifetime(
        const llvm::SmallVector<ValueLifetimes>& param_lifetimes,
        const std::optional<ValueLifetimes>& this_lifetimes) {
      // If we have an implicit `this` parameter, its lifetime is assigned to
      // all lifetimes in the return type.
      if (this_lifetimes.has_value()) {
        return this_lifetimes->GetPointeeLifetimes().GetLifetime();
      }

      llvm::DenseSet<Lifetime> all_input_lifetimes;
      for (const ValueLifetimes& v : param_lifetimes) {
        v.Traverse([&all_input_lifetimes](Lifetime l, Variance) {
          all_input_lifetimes.insert(l);
        });
      }

      if (all_input_lifetimes.size() == 1) {
        // If we have a single input lifetime, its lifetime is assigned to all
        // output lifetimes.
        return *all_input_lifetimes.begin();
      } else {
        // Otherwise, we don't know how to elide the output lifetime.
        return std::nullopt;
      }
    }

    llvm::Expected<ValueLifetimes> CreateReturnLifetimes(
        clang::QualType return_type,
        const llvm::SmallVector<ValueLifetimes>& param_lifetimes,
        const std::optional<ValueLifetimes>& this_lifetimes) const override {
      // TODO(mboehme): parse lifetime annotations from `type` if present.

      // TODO(veluca): adapt to lifetime elision for function pointers.

      std::optional<Lifetime> input_lifetime =
          GetSingleInputLifetime(param_lifetimes, this_lifetimes);

      return ValueLifetimes::Create(
          return_type, [&input_lifetime, this]() -> llvm::Expected<Lifetime> {
            if (!elision_enabled) {
              return llvm::createStringError(
                  llvm::inconvertibleErrorCode(),
                  absl::StrCat("Lifetime elision not enabled for '",
                               func->getNameAsString(), "'"));
            }

            // If we have a single input lifetime, its lifetime is assigned to
            // all output lifetimes.
            if (input_lifetime.has_value()) {
              return *input_lifetime;
            } else {
              // Otherwise, we don't know how to elide the output lifetime.
              return llvm::createStringError(
                  llvm::inconvertibleErrorCode(),
                  absl::StrCat("Cannot elide output lifetimes for '",
                               func->getNameAsString(),
                               "' because it is a non-member function that "
                               "does not have "
                               "exactly one input lifetime"));
            }
          });
    }

    bool elision_enabled;
    const clang::FunctionDecl* func;
    LifetimeSymbolTable& symbol_table;
  };

  Factory factory(elision_enabled, func, symbol_table);
  return FunctionLifetimes::CreateForDecl(func, factory);
}
}  // namespace

llvm::Expected<FunctionLifetimes> GetLifetimeAnnotations(
    const clang::FunctionDecl* func, const LifetimeAnnotationContext& context,
    LifetimeSymbolTable* symbol_table) {
  // TODO(mboehme): if we have multiple declarations of a function, make sure
  // they are all annotated with the same lifetimes.
  // TODO(veluca): the syntax we are using for lifetime annotations here is just
  // a placeholder. Adapt this to the actual syntax once the clang-side support
  // is there.

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

llvm::Expected<FunctionLifetimes> ParseLifetimeAnnotations(
    const clang::FunctionDecl* func, const std::string& lifetimes_str,
    LifetimeSymbolTable* symbol_table) {
  LifetimeSymbolTable throw_away_symbol_table;
  if (!symbol_table) {
    symbol_table = &throw_away_symbol_table;
  }
  return ParseLifetimeAnnotations(func, *symbol_table, lifetimes_str);
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
    clang::Preprocessor& preprocessor,
    std::shared_ptr<LifetimeAnnotationContext> context) {
  // Preprocessor takes ownership of the handler.
  preprocessor.AddPragmaHandler("clang",
                                new LifetimeElisionPragmaHandler(context));
}

llvm::SmallVector<llvm::SmallVector<clang::TypeLoc>> GetTemplateArgs(
    clang::TypeLoc type_loc) {
  llvm::SmallVector<llvm::SmallVector<clang::TypeLoc>> args;

  if (auto elaborated_type_loc = type_loc.getAs<clang::ElaboratedTypeLoc>()) {
    if (clang::NestedNameSpecifierLoc qualifier =
            elaborated_type_loc.getQualifierLoc()) {
      args = GetTemplateArgs(qualifier.getTypeLoc());
    }
    args.append(GetTemplateArgs(elaborated_type_loc.getNamedTypeLoc()));
  } else if (auto template_specialization_type_loc =
                 type_loc.getAs<clang::TemplateSpecializationTypeLoc>()) {
    args.push_back({});
    for (unsigned i = 0; i < template_specialization_type_loc.getNumArgs();
         ++i) {
      args.back().push_back(template_specialization_type_loc.getArgLoc(i)
                                .getTypeSourceInfo()
                                ->getTypeLoc());
    }
  } else if (auto dependent_template_specialization_type_loc =
                 type_loc
                     .getAs<clang::DependentTemplateSpecializationTypeLoc>()) {
    args.push_back({});
    // TODO(mboehme): Where does this occur exactly? Do we need to be handling
    // it?
    // AFAICT, this happens if we're looking at a dependent template name
    // (https://en.cppreference.com/w/cpp/language/dependent_name), which
    // probably means that this can only happen in template definitions (as
    // opposed to template instantiations), and we aren't analyzing those for
    // now. At the least, I haven't been able to trigger this case from a test.
    // Triggering a fatal error here so that we notice this case if it does come
    // up.
    llvm::report_fatal_error(
        "Unexpectedly got a DependentSpecializationTypeLoc");
    for (unsigned i = 0;
         i < dependent_template_specialization_type_loc.getNumArgs(); ++i) {
      args.back().push_back(
          dependent_template_specialization_type_loc.getArgLoc(i)
              .getTypeSourceInfo()
              ->getTypeLoc());
    }
  }

  return args;
}

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
