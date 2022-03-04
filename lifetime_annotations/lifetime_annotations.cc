// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "lifetime_annotations/lifetime_annotations.h"

#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "lifetime_annotations/function_lifetimes.h"
#include "lifetime_annotations/pointee_type.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "third_party/absl/strings/str_cat.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/APValue.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/ASTContext.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Attr.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Attrs.inc"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclCXX.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/PrettyPrinter.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/LangOptions.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Lex/Pragma.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Lex/Preprocessor.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/SmallVector.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/StringRef.h"

namespace devtools_rust {

namespace {

llvm::Expected<llvm::SmallVector<Lifetime>> GetAnnotatedOrElidedLifetimes(
    llvm::ArrayRef<const clang::Attr*> /*attrs*/, size_t num_expected,
    LifetimeSymbolTable& /*symbol_table*/,
    const std::function<llvm::Expected<Lifetime>()>& elided_lifetime_factory,
    const clang::ASTContext& /*ast_context*/) {
  assert(num_expected > 0);

  llvm::SmallVector<Lifetime> lifetimes;
  lifetimes.reserve(num_expected);

  // TODO(mboehme): Extract lifetime annotations from `attrs` if present.

  // No lifetime annotations: Use elided lifetimes.
  for (size_t i = 0; i < num_expected; ++i) {
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
    clang::QualType type, LifetimeSymbolTable& symbol_table,
    const std::function<llvm::Expected<Lifetime>()>& elided_lifetime_factory,
    const clang::ASTContext& ast_context, TypeLifetimes& lifetimes) {
  assert(!type.isNull());
  assert(elided_lifetime_factory);

  llvm::SmallVector<const clang::Attr*> attrs;

  size_t num_lifetime_params = GetLifetimeParameters(type).size();
  if (num_lifetime_params > 0) {
    llvm::Expected<llvm::SmallVector<Lifetime>> maybe_lifetime_args =
        GetAnnotatedOrElidedLifetimes(attrs, num_lifetime_params, symbol_table,
                                      elided_lifetime_factory, ast_context);
    if (maybe_lifetime_args) {
      lifetimes.append(maybe_lifetime_args.get());
    } else {
      return maybe_lifetime_args.takeError();
    }
  }

  const llvm::ArrayRef<clang::TemplateArgument> template_args =
      GetTemplateArgs(type);
  if (!template_args.empty()) {
    for (const clang::TemplateArgument& template_arg : template_args) {
      if (llvm::Error err = AddLifetimeAnnotationsForType(
              template_arg.getAsType(), symbol_table, elided_lifetime_factory,
              ast_context, lifetimes)) {
        return err;
      }
    }
    return llvm::Error::success();
  }

  if (clang::QualType pointee_type = PointeeType(type);
      !pointee_type.isNull()) {
    if (llvm::Error err = AddLifetimeAnnotationsForType(
            pointee_type, symbol_table, elided_lifetime_factory, ast_context,
            lifetimes)) {
      return err;
    }

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
  std::string lifetimes_str = lifetimes.str();
  clang::LangOptions lang_opts;
  clang::Lexer lexer(attr->getLoc(), lang_opts, lifetimes_str.data(),
                     lifetimes_str.data(),
                     lifetimes_str.data() + lifetimes_str.size());

  const char* end = lifetimes_str.data() + lifetimes_str.size();

  auto tok = [&lexer, &lifetimes_str, end, attr]() -> llvm::StringRef {
    clang::Token token;
    if (lexer.getBufferLocation() != end) {
      lexer.LexFromRawLexer(token);
      return llvm::StringRef(lifetimes_str.data() +
                                 token.getLocation().getRawEncoding() -
                                 attr->getLoc().getRawEncoding(),
                             token.getLength());
    }
    return "";
  };

  llvm::SmallVector<TypeLifetimes> fn_lifetimes;
  bool has_this_lifetimes = false;
  bool has_return_lifetimes = false;

  for (llvm::StringRef token; !(token = tok()).empty();) {
    if (token == ",") {
      continue;
    }
    if (has_return_lifetimes) {
      return error();
    }
    if (token == ":") {
      if (has_this_lifetimes || fn_lifetimes.size() != 1) {
        return error();
      }
      has_this_lifetimes = true;
      continue;
    }
    // Skip the -> and parse return lifetimes. No more lifetimes should be
    // parsed afterwards.
    if (token == "->") {
      has_return_lifetimes = true;
      token = tok();
    }
    fn_lifetimes.emplace_back();
    if (token == "(") {
      for (; (token = tok()) != ")" && !token.empty();) {
        if (token == ",") continue;
        fn_lifetimes.back().push_back(
            symbol_table.LookupNameAndMaybeDeclare(token));
      }
    } else {
      fn_lifetimes.back().push_back(
          symbol_table.LookupNameAndMaybeDeclare(token));
    }
  }

  FunctionLifetimes function_lifetimes;
  size_t param_start = 0;
  if (has_this_lifetimes) {
    function_lifetimes.this_lifetimes = fn_lifetimes[0];
    param_start = 1;
  }
  size_t param_end = fn_lifetimes.size();
  if (has_return_lifetimes) {
    function_lifetimes.return_lifetimes = fn_lifetimes.back();
    param_end -= 1;
  }
  function_lifetimes.param_lifetimes.assign(fn_lifetimes.begin() + param_start,
                                            fn_lifetimes.begin() + param_end);

  if (function_lifetimes.Validate(func)) {
    return function_lifetimes;
  }
  return error();
}

llvm::Expected<FunctionLifetimes> GetLifetimeAnnotationsInternal(
    const clang::FunctionDecl* func, LifetimeSymbolTable& symbol_table,
    bool elision_enabled) {
  FunctionLifetimes result;

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
  for (unsigned i = 0; i < func->getNumParams(); i++) {
    const clang::ParmVarDecl* param = func->getParamDecl(i);
    if (llvm::Error err = AddLifetimeAnnotationsForType(
            param->getType(), symbol_table, elided_lifetime_factory,
            func->getASTContext(), result.param_lifetimes[i])) {
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
          func->getReturnType(), symbol_table, elided_return_lifetime_factory,
          func->getASTContext(), result.return_lifetimes)) {
    return std::move(err);
  }

  return result;
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

}  // namespace devtools_rust
