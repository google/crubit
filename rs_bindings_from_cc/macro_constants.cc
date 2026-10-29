// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/macro_constants.h"

#include <cstddef>
#include <optional>

#include "absl/base/nullability.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Expr.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/Type.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Basic/IdentifierTable.h"
#include "clang/Basic/OperatorPrecedence.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Basic/TokenKinds.h"
#include "clang/Lex/MacroInfo.h"
#include "clang/Lex/Token.h"
#include "clang/Sema/Ownership.h"
#include "clang/Sema/Sema.h"
#include "llvm/ADT/ArrayRef.h"

namespace crubit {
namespace {

// The maximum number of tokens in a macro, so that pathological macros can't
// make the bindings generator slow or overflow its stack.
constexpr size_t kMaxTokens = 1024;

// Silences all diagnostics while it is alive (unsupported macros are skipped
// silently), but records whether any errors occurred.
class ScopedDiagnosticSuppression {
 public:
  explicit ScopedDiagnosticSuppression(clang::DiagnosticsEngine& diags)
      : diags_(diags),
        old_suppress_all_(diags.getSuppressAllDiagnostics()),
        // `DiagnosticErrorTrap` counts errors even if they are suppressed.
        error_trap_(diags) {
    diags_.setSuppressAllDiagnostics(true);
  }
  ~ScopedDiagnosticSuppression() {
    diags_.setSuppressAllDiagnostics(old_suppress_all_);
  }

  ScopedDiagnosticSuppression(const ScopedDiagnosticSuppression&) = delete;
  ScopedDiagnosticSuppression& operator=(const ScopedDiagnosticSuppression&) =
      delete;

  // Returns true if an actual error (as opposed to a warning that was promoted
  // to an error, e.g. by `-Werror`) occurred. Otherwise, whether a macro is
  // supported would depend on the warning flags of the target.
  bool HasErrorOccurred() const {
    return error_trap_.hasUnrecoverableErrorOccurred();
  }

 private:
  clang::DiagnosticsEngine& diags_;
  bool old_suppress_all_;
  clang::DiagnosticErrorTrap error_trap_;
};

// Returns true if `type` is an integral type that Crubit can bind (e.g. not
// `wchar_t`, `char8_t`, or `__int128`, which have no Rust equivalent in
// Crubit).
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
      return true;
    default:
      return false;
  }
}

std::optional<clang::BinaryOperatorKind> GetBinaryOperatorKind(
    clang::tok::TokenKind kind) {
  switch (kind) {
    case clang::tok::star:
      return clang::BO_Mul;
    case clang::tok::slash:
      return clang::BO_Div;
    case clang::tok::percent:
      return clang::BO_Rem;
    case clang::tok::plus:
      return clang::BO_Add;
    case clang::tok::minus:
      return clang::BO_Sub;
    case clang::tok::lessless:
      return clang::BO_Shl;
    case clang::tok::greatergreater:
      return clang::BO_Shr;
    case clang::tok::less:
      return clang::BO_LT;
    case clang::tok::greater:
      return clang::BO_GT;
    case clang::tok::lessequal:
      return clang::BO_LE;
    case clang::tok::greaterequal:
      return clang::BO_GE;
    case clang::tok::equalequal:
      return clang::BO_EQ;
    case clang::tok::exclaimequal:
      return clang::BO_NE;
    case clang::tok::amp:
      return clang::BO_And;
    case clang::tok::caret:
      return clang::BO_Xor;
    case clang::tok::pipe:
      return clang::BO_Or;
    case clang::tok::ampamp:
      return clang::BO_LAnd;
    case clang::tok::pipepipe:
      return clang::BO_LOr;
    default:
      return std::nullopt;
  }
}

std::optional<clang::UnaryOperatorKind> GetUnaryOperatorKind(
    clang::tok::TokenKind kind) {
  switch (kind) {
    case clang::tok::plus:
      return clang::UO_Plus;
    case clang::tok::minus:
      return clang::UO_Minus;
    case clang::tok::tilde:
      return clang::UO_Not;
    case clang::tok::exclaim:
      return clang::UO_LNot;
    default:
      return std::nullopt;
  }
}

// A recursive descent parser for the subset of C++ constant expressions that
// is supported in macro constants: integer, character, and boolean literals,
// parentheses, and unary and binary operators. The expressions are built using
// `Sema`, so that they have the same types and semantics as in C++.
class MacroExpressionParser {
 public:
  MacroExpressionParser(clang::Sema& sema, llvm::ArrayRef<clang::Token> tokens)
      : sema_(sema), tokens_(tokens) {}

  // Parses all the tokens as a single expression. Returns null on failure.
  clang::Expr* absl_nullable Parse() {
    clang::Expr* expr = ParseBinary(clang::prec::LogicalOr);
    if (expr == nullptr || position_ != tokens_.size()) return nullptr;
    return expr;
  }

 private:
  static clang::Expr* absl_nullable Get(clang::ExprResult result) {
    return result.isUsable() ? result.get() : nullptr;
  }

  const clang::Token* absl_nullable Peek() const {
    return position_ < tokens_.size() ? &tokens_[position_] : nullptr;
  }

  bool PeekIs(clang::tok::TokenKind kind) const {
    const clang::Token* token = Peek();
    return token != nullptr && token->is(kind);
  }

  // Parses a sequence of binary operators with a precedence of at least
  // `min_precedence` (precedence climbing). All supported binary operators are
  // left-associative.
  clang::Expr* absl_nullable ParseBinary(clang::prec::Level min_precedence) {
    clang::Expr* lhs = ParseUnary();
    while (lhs != nullptr) {
      const clang::Token* token = Peek();
      if (token == nullptr) break;
      std::optional<clang::BinaryOperatorKind> opcode =
          GetBinaryOperatorKind(token->getKind());
      if (!opcode.has_value()) break;
      clang::prec::Level precedence =
          clang::getBinOpPrecedence(token->getKind(),
                                    /*GreaterThanIsOperator=*/true,
                                    /*CPlusPlus11=*/true);
      if (precedence < min_precedence) break;
      clang::SourceLocation op_loc = token->getLocation();
      ++position_;
      clang::Expr* rhs =
          ParseBinary(static_cast<clang::prec::Level>(precedence + 1));
      if (rhs == nullptr) return nullptr;
      lhs = Get(sema_.CreateBuiltinBinOp(op_loc, *opcode, lhs, rhs));
    }
    return lhs;
  }

  // unary-expression:
  //     primary-expression
  //     (`+` | `-` | `~` | `!`) unary-expression
  clang::Expr* absl_nullable ParseUnary() {
    const clang::Token* token = Peek();
    if (token == nullptr) return nullptr;
    std::optional<clang::UnaryOperatorKind> opcode =
        GetUnaryOperatorKind(token->getKind());
    if (!opcode.has_value()) return ParsePrimary();
    clang::SourceLocation op_loc = token->getLocation();
    ++position_;
    clang::Expr* operand = ParseUnary();
    if (operand == nullptr) return nullptr;
    return Get(sema_.CreateBuiltinUnaryOp(op_loc, *opcode, operand));
  }

  // primary-expression:
  //     integer-literal
  //     character-literal
  //     boolean-literal
  //     `(` logical-or-expression `)`
  clang::Expr* absl_nullable ParsePrimary() {
    const clang::Token* token = Peek();
    if (token == nullptr) return nullptr;
    ++position_;
    switch (token->getKind()) {
      case clang::tok::numeric_constant:
        // User-defined literals are rejected, since there is no scope to look
        // up the literal operator in.
        return Get(sema_.ActOnNumericConstant(*token, /*UDLScope=*/nullptr));
      case clang::tok::char_constant:
      case clang::tok::wide_char_constant:
      case clang::tok::utf8_char_constant:
      case clang::tok::utf16_char_constant:
      case clang::tok::utf32_char_constant:
        return Get(sema_.ActOnCharacterConstant(*token, /*UDLScope=*/nullptr));
      case clang::tok::kw_true:
      case clang::tok::kw_false:
        return Get(
            sema_.ActOnCXXBoolLiteral(token->getLocation(), token->getKind()));
      case clang::tok::l_paren: {
        clang::SourceLocation l_paren_loc = token->getLocation();
        clang::Expr* inner = ParseBinary(clang::prec::LogicalOr);
        if (inner == nullptr || !PeekIs(clang::tok::r_paren)) return nullptr;
        clang::SourceLocation r_paren_loc = tokens_[position_++].getLocation();
        return Get(sema_.ActOnParenExpr(l_paren_loc, r_paren_loc, inner));
      }
      default:
        return nullptr;
    }
  }

  clang::Sema& sema_;
  llvm::ArrayRef<clang::Token> tokens_;
  size_t position_ = 0;
};

}  // namespace

clang::VarDecl* absl_nullable SynthesizeMacroConstant(
    clang::Sema& sema, const clang::IdentifierInfo& name,
    const clang::MacroInfo& macro_info) {
  clang::SourceLocation name_loc = macro_info.getDefinitionLoc();
  clang::SourceLocation end_loc = macro_info.getDefinitionEndLoc();
  if (name_loc.isInvalid() || !name_loc.isFileID() || end_loc.isInvalid() ||
      !end_loc.isFileID()) {
    return nullptr;
  }
  if (macro_info.isFunctionLike() || macro_info.isBuiltinMacro() ||
      macro_info.getNumTokens() > kMaxTokens) {
    return nullptr;
  }

  clang::ASTContext& ast_context = sema.getASTContext();
  ScopedDiagnosticSuppression diagnostic_suppression(sema.getDiagnostics());
  clang::Expr* expr = MacroExpressionParser(sema, macro_info.tokens()).Parse();
  if (expr == nullptr || diagnostic_suppression.HasErrorOccurred() ||
      expr->containsErrors()) {
    return nullptr;
  }

  // Only integral (including boolean and character) constants are supported.
  clang::QualType type = expr->getType();
  if (!IsSupportedIntegralType(type)) return nullptr;
  if (!expr->getIntegerConstantExpr(ast_context).has_value()) return nullptr;

  // Make the initializer span the whole macro definition, so that the source
  // range of the variable is the macro definition.
  clang::ExprResult wrapped_expr = sema.ActOnParenExpr(name_loc, end_loc, expr);
  if (!wrapped_expr.isUsable()) return nullptr;
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
  return var_decl;
}

}  // namespace crubit
