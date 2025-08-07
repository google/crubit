// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_annotations.h"

#include <cassert>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/lifetime_error.h"
#include "lifetime_annotations/lifetime_symbol_table.h"
#include "lifetime_annotations/pointee_type.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "clang/AST/APValue.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclarationName.h"
#include "clang/AST/PrettyPrinter.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeLoc.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/LangOptions.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Lex/Lexer.h"
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
      [&symbol_table,
       &next_lifetime](const clang::Expr*) -> llvm::Expected<Lifetime> {
        llvm::StringRef next = next_lifetime();
        if (next.empty()) {
          return llvm::make_error<LifetimeError>(
              LifetimeError::Type::Other,
              "Invalid lifetime annotation: too few lifetimes");
        }
        return symbol_table.LookupNameAndMaybeDeclare(next);
      });

  auto ret = FunctionLifetimes::CreateForDecl(func, factory);

  if (!next_lifetime().empty()) {
    return llvm::make_error<LifetimeError>(
        LifetimeError::Type::Other,
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
    return llvm::make_error<LifetimeError>(LifetimeError::Type::Other, msg);
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
          return llvm::make_error<LifetimeError>(
              LifetimeError::Type::Other,
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
        : elision_enabled_(elision_enabled),
          func_(func),
          symbol_table_(symbol_table) {}

   private:
    llvm::Expected<Lifetime> LifetimeFromName(const clang::Expr* name) const {
      llvm::StringRef name_str;
      if (llvm::Error err =
              EvaluateAsStringLiteral(name, func_->getASTContext())
                  .moveInto(name_str)) {
        return std::move(err);
      }
      return symbol_table_.LookupNameAndMaybeDeclare(name_str);
    }

    LifetimeFactory ParamLifetimeFactory() const {
      return [this](const clang::Expr* name) -> llvm::Expected<Lifetime> {
        if (name) {
          Lifetime lifetime;
          if (llvm::Error err = LifetimeFromName(name).moveInto(lifetime)) {
            return std::move(err);
          }
          return lifetime;
        }

        // As a special-case, lifetime is always inferred for the `this`
        // parameter for destructors. The obvious lifetime is definitionally
        // correct in this case: the object must be valid for the duration
        // of the call, or else the behavior is undefined. So we can infer
        // safely even if elision is disabled.
        if (!elision_enabled_ &&
            func_->getDeclName().getNameKind() !=
                clang::DeclarationName::CXXDestructorName) {
          return llvm::make_error<LifetimeError>(
              LifetimeError::Type::ElisionNotEnabled,
              absl::StrCat("Lifetime elision not enabled for '",
                           func_->getNameAsString(), "'"));
        }

        Lifetime lifetime = Lifetime::CreateVariable();
        symbol_table_.LookupLifetimeAndMaybeDeclare(lifetime);
        return lifetime;
      };
    }

    llvm::Expected<ValueLifetimes> CreateThisLifetimes(
        clang::QualType type, const clang::Expr* lifetime_name) const override {
      LifetimeFactory lifetime_factory = ParamLifetimeFactory();

      clang::QualType pointee_type = PointeeType(type);
      assert(!pointee_type.isNull());

      ValueLifetimes value_lifetimes;
      if (llvm::Error err =
              ValueLifetimes::Create(pointee_type, clang::TypeLoc(),
                                     lifetime_factory)
                  .moveInto(value_lifetimes)) {
        return std::move(err);
      }

      Lifetime object_lifetime;
      if (llvm::Error err =
              lifetime_factory(lifetime_name).moveInto(object_lifetime)) {
        return std::move(err);
      }

      return ValueLifetimes::ForPointerLikeType(
          type, ObjectLifetimes(object_lifetime, value_lifetimes));
    }

    llvm::Expected<ValueLifetimes> CreateParamLifetimes(
        clang::QualType param_type,
        clang::TypeLoc param_type_loc) const override {
      return ValueLifetimes::Create(param_type, param_type_loc,
                                    ParamLifetimeFactory());
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
        // Function pointers and function references have an implied static
        // lifetime, but this shouldn't count as an input lifetime for the
        // purposes of lifetime elision.
        const ValueLifetimes& lifetimes_to_traverse =
            v.Type()->isFunctionPointerType() ||
                    v.Type()->isFunctionReferenceType()
                ? v.GetPointeeLifetimes().GetValueLifetimes()
                : v;
        lifetimes_to_traverse.Traverse(
            [&all_input_lifetimes](Lifetime l, Variance) {
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
        clang::QualType return_type, clang::TypeLoc return_type_loc,
        const llvm::SmallVector<ValueLifetimes>& param_lifetimes,
        const std::optional<ValueLifetimes>& this_lifetimes) const override {
      // TODO(veluca): adapt to lifetime elision for function pointers.

      std::optional<Lifetime> input_lifetime =
          GetSingleInputLifetime(param_lifetimes, this_lifetimes);

      return ValueLifetimes::Create(
          return_type, return_type_loc,
          [&input_lifetime,
           this](const clang::Expr* name) -> llvm::Expected<Lifetime> {
            if (name) {
              Lifetime lifetime;
              if (llvm::Error err = LifetimeFromName(name).moveInto(lifetime)) {
                return std::move(err);
              }
              return lifetime;
            }

            if (!elision_enabled_) {
              return llvm::make_error<LifetimeError>(
                  LifetimeError::Type::ElisionNotEnabled,
                  absl::StrCat("Lifetime elision not enabled for '",
                               func_->getNameAsString(), "'"));
            }

            // If we have a single input lifetime, its lifetime is assigned to
            // all output lifetimes.
            if (input_lifetime.has_value()) {
              return *input_lifetime;
            } else {
              // Otherwise, we don't know how to elide the output lifetime.
              return llvm::make_error<LifetimeError>(
                  LifetimeError::Type::CannotElideOutputLifetimes,
                  absl::StrCat("Cannot elide output lifetimes for '",
                               func_->getNameAsString(),
                               "' because it is a non-member function that "
                               "does not have "
                               "exactly one input lifetime"));
            }
          });
    }

    bool elision_enabled_;
    const clang::FunctionDecl* func_;
    LifetimeSymbolTable& symbol_table_;
  };

  Factory factory(elision_enabled, func, symbol_table);
  return FunctionLifetimes::CreateForDecl(func, factory);
}
}  // namespace

char LifetimeError::ID;

llvm::Expected<FunctionLifetimes> GetLifetimeAnnotations(
    const clang::FunctionDecl* func, const LifetimeAnnotationContext& context,
    LifetimeSymbolTable* symbol_table) {
  // TODO(mboehme): if we have multiple declarations of a function, make sure
  // they are all annotated with the same lifetimes.

  clang::SourceManager& source_manager =
      func->getASTContext().getSourceManager();
  clang::FileID file_id = source_manager.getFileID(
      source_manager.getExpansionLoc(func->getSourceRange().getBegin()));
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

}  // namespace lifetimes
}  // namespace tidy
}  // namespace clang
