// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/macro_constants.h"

#include <cstddef>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>

#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/recording_diagnostic_consumer.h"
#include "clang/AST/APValue.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Expr.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeBase.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Basic/DiagnosticIDs.h"
#include "clang/Basic/IdentifierTable.h"
#include "clang/Basic/PartialDiagnostic.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Basic/TokenKinds.h"
#include "clang/Lex/ExternalPreprocessorSource.h"
#include "clang/Lex/LiteralSupport.h"
#include "clang/Lex/MacroInfo.h"
#include "clang/Lex/Preprocessor.h"
#include "clang/Lex/Token.h"
#include "clang/Parse/Parser.h"
#include "clang/Sema/Ownership.h"
#include "clang/Sema/Sema.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/ADT/SmallString.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Casting.h"

namespace crubit {
namespace {

// Bounds on the work done for a single macro, so that pathological macros
// can't make the bindings generator slow or overflow its stack.
//
// The maximum number of tokens that a macro may expand to.
constexpr size_t kMaxExpandedTokens = 1024;
// The maximum depth of nested macro expansions.
constexpr size_t kMaxExpansionDepth = 64;
// The maximum total number of nested macro expansions performed for a single
// macro, bounding exponential expansion of empty or small macro DAGs.
constexpr size_t kMaxExpansionSteps = 4096;

// Returns true if `type` is an integral type that Crubit can bind (e.g. not
// `wchar_t` or `char8_t`, which have no Rust equivalent in Crubit).
bool IsSupportedIntegralType(clang::QualType type) {
  const auto* builtin = type->getAs<clang::BuiltinType>();
  if (builtin == nullptr) return false;
  switch (builtin->getKind()) {
    case clang::BuiltinType::Bool:
    case clang::BuiltinType::Char_S:
    case clang::BuiltinType::Char_U:
    case clang::BuiltinType::SChar:
    case clang::BuiltinType::UChar:
    case clang::BuiltinType::Char16:
    case clang::BuiltinType::Char32:
    case clang::BuiltinType::Short:
    case clang::BuiltinType::UShort:
    case clang::BuiltinType::Int:
    case clang::BuiltinType::UInt:
    case clang::BuiltinType::Long:
    case clang::BuiltinType::ULong:
    case clang::BuiltinType::LongLong:
    case clang::BuiltinType::ULongLong:
    case clang::BuiltinType::Int128:
    case clang::BuiltinType::UInt128:
      return true;
    default:
      return false;
  }
}

// Appends the tokens that the object-like macro `name` (defined by
// `macro_info`) expands to to `out`, recursively expanding other object-like
// macros.
//
// `expanding` holds the macros that are currently being expanded. As in the C
// preprocessor, they are not expanded again (but they will make parsing fail,
// because they are left as identifiers).
//
// Returns an error if the macro can't be expanded, e.g. because it uses token
// pasting or exceeds the expansion limits.
std::optional<FormattedError> ExpandObjectLikeMacro(
    clang::Preprocessor& preprocessor, const clang::IdentifierInfo& name,
    const clang::MacroInfo& macro_info,
    llvm::SmallVectorImpl<const clang::IdentifierInfo*>& expanding,
    size_t& expansion_steps, llvm::SmallVectorImpl<clang::Token>& out) {
  if (macro_info.isFunctionLike()) {
    if (expanding.empty()) {
      return FormattedError::Static("function-like macros are not supported");
    }
    return FormattedError::Substitute(
        "`{0}` is a function-like macro, which is not supported",
        name.getName().str());
  }
  if (macro_info.isBuiltinMacro()) {
    return FormattedError::Substitute(
        "`{0}` is a builtin macro, which is not supported",
        name.getName().str());
  }
  if (expanding.size() >= kMaxExpansionDepth) {
    return FormattedError::Static("the macro's expansion is nested too deeply");
  }
  if (++expansion_steps > kMaxExpansionSteps) {
    return FormattedError::Static(
        "the macro's expansion expands too many other macros");
  }
  expanding.push_back(&name);
  for (const clang::Token& token : macro_info.tokens()) {
    // Token pasting (`##`) is also performed in object-like macros. (`#` is
    // not an operator in object-like macros, and isn't part of any supported
    // expression either.)
    if (token.isOneOf(clang::tok::hash, clang::tok::hashhash,
                      clang::tok::hashat)) {
      return FormattedError::Static(
          "the `#` and `##` preprocessor operators are not supported");
    }
    if (const clang::IdentifierInfo* identifier = token.getIdentifierInfo()) {
      // Tokens deserialized from a precompiled header module may not have
      // loaded their macro definition yet.
      if (identifier->isOutOfDate()) {
        if (clang::ExternalPreprocessorSource* external =
                preprocessor.getExternalSource()) {
          external->updateOutOfDateIdentifier(*identifier);
        }
      }
      if (identifier->hasMacroDefinition() &&
          !llvm::is_contained(expanding, identifier)) {
        clang::MacroDefinition nested_def =
            preprocessor.getMacroDefinition(identifier);
        if (nested_def && !nested_def.isAmbiguous()) {
          if (const clang::MacroInfo* nested_macro_info =
                  nested_def.getMacroInfo()) {
            // Function-like macros are only expanded if they are followed by
            // `(`, which can't be part of a supported expression. Since they
            // are never supported, `ExpandObjectLikeMacro` fails for them in
            // either case.
            if (std::optional<FormattedError> error = ExpandObjectLikeMacro(
                    preprocessor, *identifier, *nested_macro_info, expanding,
                    expansion_steps, out)) {
              return error;
            }
            continue;
          }
        }
      }
    }
    if (out.size() >= kMaxExpandedTokens) {
      return FormattedError::Static("the macro expands to too many tokens");
    }
    out.push_back(token);
  }
  expanding.pop_back();
  return std::nullopt;
}

// Returns true if the numeric literal `token` has a suffix (e.g. `42u`), or
// nullopt if it is invalid.
std::optional<bool> HasSuffix(clang::Sema& sema, const clang::Token& token) {
  llvm::SmallString<32> buffer;
  bool invalid = false;
  llvm::StringRef spelling =
      sema.getPreprocessor().getSpelling(token, buffer, &invalid);
  if (invalid) return std::nullopt;
  clang::NumericLiteralParser literal(
      spelling, token.getLocation(), sema.getSourceManager(),
      sema.getLangOpts(), sema.getASTContext().getTargetInfo(),
      sema.getDiagnostics());
  if (literal.hadError) return std::nullopt;
  return literal.isUnsigned || literal.isLong || literal.isLongLong ||
         literal.isSizeT || literal.isBitInt || literal.MicrosoftInteger != 0 ||
         literal.isFloat;
}

// Returns true if the macro's (expanded) `tokens` contain a numeric literal
// with a suffix (e.g. `42u`) or a `char16_t` or `char32_t` literal (e.g.
// `u'x'`), or if `expr`, the expression they were parsed as, contains an
// explicit cast (e.g. `(uint32_t)1`). See `MacroConstant::is_typed`.
bool IsTyped(clang::Sema& sema, llvm::ArrayRef<clang::Token> tokens,
             const clang::Expr& expr) {
  for (const clang::Token& token : tokens) {
    if (token.isOneOf(clang::tok::utf16_char_constant,
                      clang::tok::utf32_char_constant)) {
      return true;
    }
    if (token.is(clang::tok::numeric_constant) &&
        HasSuffix(sema, token).value_or(false)) {
      return true;
    }
  }
  llvm::SmallVector<const clang::Stmt*, 16> worklist = {&expr};
  while (!worklist.empty()) {
    const clang::Stmt* stmt = worklist.pop_back_val();
    if (stmt == nullptr) continue;
    if (llvm::isa<clang::ExplicitCastExpr>(stmt)) return true;
    llvm::append_range(worklist, stmt->children());
  }
  return false;
}

// Parses `tokens` as a C++ constant expression with `parser`, the parser of the
// translation unit, after the end of the translation unit. Returns null if they
// aren't a single valid expression.
clang::Expr* _Nullable ParseConstantExpression(
    clang::Sema& sema, clang::Parser& parser,
    llvm::ArrayRef<clang::Token> tokens, clang::SourceLocation end_loc) {
  // Ends the token stream with an `eof` token that identifies it, so that the
  // parser stops at the end of the macro (as Clang does for late-parsed default
  // arguments). The parser never consumes an `eof` token, but it may look ahead
  // past it: the translation unit is processed incrementally (see
  // `FrontendAction::ExecuteAction`), so the preprocessor then lexes
  // the `eof` of the main file again.
  int eof_marker = 0;
  // `Preprocessor::EnterTokenStream` takes the number of tokens as `unsigned`.
  const unsigned num_tokens = static_cast<unsigned>(tokens.size()) + 1;
  auto stream = std::make_unique<clang::Token[]>(num_tokens);
  llvm::copy(tokens, stream.get());
  clang::Token& eof = stream[tokens.size()];
  eof.startToken();
  eof.setKind(clang::tok::eof);
  eof.setLocation(end_loc);
  eof.setEofData(&eof_marker);
  auto is_at_end = [&] {
    const clang::Token& token = parser.getCurToken();
    return token.is(clang::tok::eof) && token.getEofData() == &eof_marker;
  };

  // The preprocessor takes ownership of the tokens, which stay on its stack
  // until it is destroyed. Macros in them were already expanded by
  // `ExpandObjectLikeMacro`.
  sema.getPreprocessor().EnterTokenStream(std::move(stream), num_tokens,
                                          /*DisableMacroExpansion=*/true,
                                          /*IsReinject=*/true);
  parser.ConsumeAnyToken();  // Reads the first token of the macro.
  // Anything that parses reaches `Sema`, even if the macro is rejected later
  // (e.g. because of its type). This can have side effects on the AST: e.g. a
  // cast to a class template specialization instantiates it, and
  // `sizeof(struct S)` declares `S` in the translation unit (which
  // `Importer::ImportMacroConstants` excludes from the import).
  clang::ExprResult result = parser.ParseConstantExpression();
  clang::Expr* expr = result.isUsable() && is_at_end() ? result.get() : nullptr;
  // Skips the rest of the macro, e.g. after a parse error. This stops at any
  // `eof`, since the parser never consumes one.
  while (parser.getCurToken().isNot(clang::tok::eof)) {
    parser.ConsumeAnyToken();
  }
  // The parser never consumes an `eof`, so it can't get past the end of the
  // macro, but reject the macro if it did anyway.
  if (!is_at_end()) return nullptr;
  return expr;
}

// Returns Clang's explanation of why `expr` is not a C++ constant expression
// (the first note it produces when evaluating `expr`), e.g. "division by
// zero".
std::string ExplainNonConstantExpression(clang::Sema& sema,
                                         const clang::Expr& expr) {
  llvm::SmallVector<clang::PartialDiagnosticAt, 4> notes;
  clang::Expr::EvalResult eval_result;
  eval_result.Diag = &notes;
  (void)expr.EvaluateAsRValue(eval_result, sema.getASTContext(),
                              /*InConstantContext=*/true);
  if (notes.empty()) return "it can't be evaluated";
  llvm::SmallString<64> explanation;
  notes.front().second.EmitToString(sema.getDiagnostics(), explanation);
  return explanation.str().str();
}

}  // namespace

std::variant<MacroConstant, FormattedError> SynthesizeMacroConstant(
    clang::Sema& sema, clang::Parser& parser, const clang::IdentifierInfo& name,
    const clang::MacroInfo& macro_info) {
  clang::SourceLocation name_loc = macro_info.getDefinitionLoc();
  clang::SourceLocation end_loc = macro_info.getDefinitionEndLoc();
  if (name_loc.isInvalid() || !name_loc.isFileID() || end_loc.isInvalid() ||
      !end_loc.isFileID()) {
    return FormattedError::Static("the macro isn't defined in a source file");
  }

  llvm::SmallVector<clang::Token, 16> tokens;
  llvm::SmallVector<const clang::IdentifierInfo*, 4> expanding;
  size_t expansion_steps = 0;
  if (std::optional<FormattedError> error =
          ExpandObjectLikeMacro(sema.getPreprocessor(), name, macro_info,
                                expanding, expansion_steps, tokens)) {
    return *std::move(error);
  }
  if (tokens.empty()) return FormattedError::Static("the macro is empty");

  clang::ASTContext& ast_context = sema.getASTContext();
  // Records the diagnostics (instead of reporting them), so that they can
  // explain why the macro isn't supported.
  clang::Expr* expr = nullptr;
  RecordingDiagnosticConsumer diagnostics =
      RecordDiagnostics(sema.getDiagnostics(), [&] {
        expr = ParseConstantExpression(sema, parser, tokens, end_loc);
        if (expr == nullptr || expr->containsErrors()) return;
        // E.g. a reference to a `constexpr` variable is an lvalue, but the
        // variable is initialized with its value. (String literals stay
        // lvalues, since lvalue-to-rvalue conversions don't apply to arrays.)
        // This also reports placeholder expressions, e.g. a builtin function
        // that isn't called.
        clang::ExprResult rvalue = sema.DefaultLvalueConversion(expr);
        expr = rvalue.isUsable() ? rvalue.get() : nullptr;
      });
  // Only actual errors count, not warnings that are promoted to errors (e.g. by
  // `-Werror`): otherwise, whether a macro is supported would depend on the
  // warning flags of the target.
  const clang::DiagnosticIDs& diagnostic_ids =
      *sema.getDiagnostics().getDiagnosticIDs();
  for (const RecordingDiagnosticConsumer::Diagnostic& diagnostic :
       diagnostics.GetDiagnostics()) {
    if (diagnostic.diagnostic_level >= clang::DiagnosticsEngine::Error &&
        !diagnostic_ids.isWarningOrExtension(diagnostic.diagnostic_id)) {
      return FormattedError::Substitute(
          "the macro is not a valid C++ expression: {0}",
          diagnostic.diagnostic.str().str());
    }
  }
  if (expr == nullptr || expr->containsErrors()) {
    return FormattedError::Static("the macro is not a single C++ expression");
  }

  // Only integral (including boolean and character) and (narrow or UTF-8)
  // string constants are supported.
  clang::QualType type = expr->getType();
  if (const auto* string_literal =
          llvm::dyn_cast<clang::StringLiteral>(expr->IgnoreParens())) {
    if (!string_literal->isOrdinary() && !string_literal->isUTF8()) {
      return FormattedError::Static(
          "only ordinary and UTF-8 string literals are supported in macro "
          "constants");
    }
  } else if (IsSupportedIntegralType(type)) {
    if (!expr->getIntegerConstantExpr(ast_context).has_value()) {
      return FormattedError::Substitute(
          "the macro is not a C++ constant expression: {0}",
          ExplainNonConstantExpression(sema, *expr));
    }
  } else {
    return FormattedError::Substitute(
        "macro constants of type `{0}` are not supported", type.getAsString());
  }

  // Make the initializer span the whole macro definition (rather than, say,
  // the definition of a nested macro), so that the source range of the
  // variable is the macro definition.
  clang::ExprResult wrapped_expr = sema.ActOnParenExpr(name_loc, end_loc, expr);
  if (!wrapped_expr.isUsable()) {
    return FormattedError::Static("the macro is not a valid C++ expression");
  }
  expr = wrapped_expr.get();

  clang::QualType var_type = ast_context.getConstType(type);
  clang::TranslationUnitDecl* translation_unit =
      ast_context.getTranslationUnitDecl();
  clang::VarDecl* var_decl = clang::VarDecl::Create(
      ast_context, translation_unit, /*StartLoc=*/name_loc, /*IdLoc=*/name_loc,
      &const_cast<clang::IdentifierInfo&>(name), var_type,
      ast_context.getTrivialTypeSourceInfo(var_type, name_loc), clang::SC_None);
  var_decl->setConstexpr(true);
  var_decl->setInit(expr);
  return MacroConstant{.var_decl = var_decl,
                       .is_typed = IsTyped(sema, tokens, *expr)};
}

clang::VarDecl* _Nonnull CreateUnsupportedMacroPlaceholder(
    clang::ASTContext& ast_context, const clang::IdentifierInfo& name,
    const clang::MacroInfo& macro_info) {
  clang::SourceLocation name_loc = macro_info.getDefinitionLoc();
  return clang::VarDecl::Create(
      ast_context, ast_context.getTranslationUnitDecl(), /*StartLoc=*/name_loc,
      /*IdLoc=*/name_loc, &const_cast<clang::IdentifierInfo&>(name),
      ast_context.IntTy,
      ast_context.getTrivialTypeSourceInfo(ast_context.IntTy, name_loc),
      clang::SC_None);
}

}  // namespace crubit
