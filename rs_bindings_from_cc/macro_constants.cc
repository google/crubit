// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/macro_constants.h"

#include <cstddef>
#include <optional>

#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/Expr.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/Type.h"
#include "clang/AST/TypeBase.h"
#include "clang/Basic/Diagnostic.h"
#include "clang/Basic/IdentifierTable.h"
#include "clang/Basic/OperatorPrecedence.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Basic/TokenKinds.h"
#include "clang/Lex/ExternalPreprocessorSource.h"
#include "clang/Lex/LiteralSupport.h"
#include "clang/Lex/MacroInfo.h"
#include "clang/Lex/Preprocessor.h"
#include "clang/Lex/Token.h"
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
// Returns false if the macro can't be expanded, e.g. because it uses token
// pasting or exceeds the expansion limits.
bool ExpandObjectLikeMacro(
    clang::Preprocessor& preprocessor, const clang::IdentifierInfo& name,
    const clang::MacroInfo& macro_info,
    llvm::SmallVectorImpl<const clang::IdentifierInfo*>& expanding,
    size_t& expansion_steps, llvm::SmallVectorImpl<clang::Token>& out) {
  if (macro_info.isFunctionLike() || macro_info.isBuiltinMacro()) {
    return false;
  }
  if (expanding.size() >= kMaxExpansionDepth ||
      ++expansion_steps > kMaxExpansionSteps) {
    return false;
  }
  expanding.push_back(&name);
  for (const clang::Token& token : macro_info.tokens()) {
    // Token pasting (`##`) is also performed in object-like macros. (`#` is
    // not an operator in object-like macros, and isn't part of any supported
    // expression either.)
    if (token.isOneOf(clang::tok::hash, clang::tok::hashhash,
                      clang::tok::hashat)) {
      return false;
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
            if (!ExpandObjectLikeMacro(preprocessor, *identifier,
                                       *nested_macro_info, expanding,
                                       expansion_steps, out)) {
              return false;
            }
            continue;
          }
        }
      }
    }
    if (out.size() >= kMaxExpandedTokens) return false;
    out.push_back(token);
  }
  expanding.pop_back();
  return true;
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

// Returns the builtin type named by `tokens`, a sequence of type specifier
// keywords such as `unsigned long long`, or `std::nullopt` if they don't name
// a supported builtin type.
std::optional<clang::QualType> GetBuiltinType(
    const clang::ASTContext& ast_context, llvm::ArrayRef<clang::Token> tokens) {
  int num_signed = 0;
  int num_unsigned = 0;
  int num_short = 0;
  int num_long = 0;
  int num_int = 0;
  int num_char = 0;
  // Types that are named by a single keyword, which can't be combined with
  // any other keyword (e.g. `bool`).
  std::optional<clang::QualType> standalone_type;
  int num_standalone = 0;
  for (const clang::Token& token : tokens) {
    switch (token.getKind()) {
      case clang::tok::kw_signed:
        ++num_signed;
        break;
      case clang::tok::kw_unsigned:
        ++num_unsigned;
        break;
      case clang::tok::kw_short:
        ++num_short;
        break;
      case clang::tok::kw_long:
        ++num_long;
        break;
      case clang::tok::kw_int:
        ++num_int;
        break;
      case clang::tok::kw_char:
        ++num_char;
        break;
      case clang::tok::kw_bool:
        standalone_type = ast_context.BoolTy;
        ++num_standalone;
        break;
      case clang::tok::kw_char16_t:
        standalone_type = ast_context.Char16Ty;
        ++num_standalone;
        break;
      case clang::tok::kw_char32_t:
        standalone_type = ast_context.Char32Ty;
        ++num_standalone;
        break;
      default:
        return std::nullopt;
    }
  }
  const int num_integer_keywords =
      num_signed + num_unsigned + num_short + num_long + num_int + num_char;
  if (num_standalone > 0) {
    if (num_standalone > 1 || num_integer_keywords > 0) return std::nullopt;
    return standalone_type;
  }
  if (num_signed + num_unsigned > 1 || num_int > 1) return std::nullopt;
  const bool is_unsigned = num_unsigned > 0;
  if (num_char > 0) {
    if (num_char > 1 || num_short > 0 || num_long > 0 || num_int > 0) {
      return std::nullopt;
    }
    if (num_signed > 0) return ast_context.SignedCharTy;
    if (num_unsigned > 0) return ast_context.UnsignedCharTy;
    return ast_context.CharTy;
  }
  if (num_short > 0) {
    if (num_short > 1 || num_long > 0) return std::nullopt;
    return is_unsigned ? ast_context.UnsignedShortTy : ast_context.ShortTy;
  }
  switch (num_long) {
    case 0:
      if (num_integer_keywords == 0) return std::nullopt;
      return is_unsigned ? ast_context.UnsignedIntTy : ast_context.IntTy;
    case 1:
      return is_unsigned ? ast_context.UnsignedLongTy : ast_context.LongTy;
    case 2:
      return is_unsigned ? ast_context.UnsignedLongLongTy
                         : ast_context.LongLongTy;
    default:
      return std::nullopt;
  }
}

// Returns the type declared by the (possibly qualified) name in `tokens`, such
// as `uint32_t` or `::std::uint32_t`, or null if there is none.
//
// Names are looked up starting from the global namespace, because macros can
// be expanded anywhere.
const clang::TypeDecl* _Nullable LookUpTypeDecl(
    const clang::ASTContext& ast_context, llvm::ArrayRef<clang::Token> tokens) {
  const clang::DeclContext* context = ast_context.getTranslationUnitDecl();
  size_t i = 0;
  if (i < tokens.size() && tokens[i].is(clang::tok::coloncolon)) ++i;
  while (true) {
    if (i >= tokens.size() || !tokens[i].is(clang::tok::identifier)) {
      return nullptr;
    }
    clang::IdentifierInfo* name = tokens[i].getIdentifierInfo();
    ++i;
    const bool is_last = i == tokens.size();
    if (!is_last && !tokens[i].is(clang::tok::coloncolon)) return nullptr;

    const clang::NamedDecl* found = nullptr;
    for (const clang::NamedDecl* decl : context->lookup(name)) {
      // E.g. `std::uint32_t` is declared by `using ::uint32_t;`.
      if (const auto* shadow = llvm::dyn_cast<clang::UsingShadowDecl>(decl)) {
        decl = shadow->getTargetDecl();
      }
      if (is_last ? llvm::isa<clang::TypeDecl>(decl)
                  : llvm::isa<clang::NamespaceDecl, clang::NamespaceAliasDecl>(
                        decl)) {
        found = decl;
        break;
      }
    }
    if (found == nullptr) return nullptr;
    if (is_last) return llvm::cast<clang::TypeDecl>(found);
    if (const auto* alias = llvm::dyn_cast<clang::NamespaceAliasDecl>(found)) {
      context = alias->getNamespace();
    } else {
      context = llvm::cast<clang::NamespaceDecl>(found);
    }
    // Skip the `::`.
    ++i;
  }
}

// Returns the type named by `tokens`, or `std::nullopt` if they don't name a
// supported type.
//
// Supported types are builtin integral types (e.g. `unsigned char`) and
// (possibly qualified) names of such types (e.g. `uint32_t`), optionally with
// `const` or `volatile` qualifiers, which are ignored since they don't affect
// the result of a cast.
//
// If `simple` is true, only a simple type specifier is accepted, i.e. a single
// keyword or a name without cv-qualifiers, as required in a functional cast
// (e.g. `int(x)` or `std::uint32_t(x)`, but not `unsigned int(x)`).
std::optional<clang::QualType> ParseTypeName(
    const clang::ASTContext& ast_context, llvm::ArrayRef<clang::Token> tokens,
    bool simple) {
  // As in C++, `const` and `volatile` may each appear at most once. They may
  // appear anywhere among type specifier keywords (e.g. `unsigned const int`),
  // but not inside a (qualified) name (e.g. `std:: const int64_t`).
  llvm::SmallVector<clang::Token, 4> unqualified_tokens;
  bool has_const = false;
  bool has_volatile = false;
  // Whether a cv-qualifier follows a part of a name.
  bool cv_qualifier_after_name = false;
  for (const clang::Token& token : tokens) {
    const bool is_name_part =
        token.isOneOf(clang::tok::identifier, clang::tok::coloncolon);
    if (token.isOneOf(clang::tok::kw_const, clang::tok::kw_volatile)) {
      bool& has_qualifier =
          token.is(clang::tok::kw_const) ? has_const : has_volatile;
      if (simple || has_qualifier) return std::nullopt;
      has_qualifier = true;
      if (!unqualified_tokens.empty() &&
          unqualified_tokens.back().isOneOf(clang::tok::identifier,
                                            clang::tok::coloncolon)) {
        cv_qualifier_after_name = true;
      }
      continue;
    }
    if (is_name_part && cv_qualifier_after_name) return std::nullopt;
    unqualified_tokens.push_back(token);
  }
  if (unqualified_tokens.empty()) return std::nullopt;

  std::optional<clang::QualType> type;
  if (unqualified_tokens[0].isOneOf(clang::tok::identifier,
                                    clang::tok::coloncolon)) {
    const clang::TypeDecl* type_decl =
        LookUpTypeDecl(ast_context, unqualified_tokens);
    if (type_decl == nullptr) return std::nullopt;
    type = ast_context.getTypeDeclType(type_decl);
  } else {
    if (simple && unqualified_tokens.size() != 1) return std::nullopt;
    type = GetBuiltinType(ast_context, unqualified_tokens);
  }
  // Only casts to the types that macro constants can have are supported. In
  // particular, casts to class types must be rejected before `Sema` sees them:
  // building them could e.g. instantiate templates as a side effect, even if
  // the macro is skipped later.
  if (!type.has_value() || !IsSupportedIntegralType(*type)) return std::nullopt;
  return type;
}

// A recursive descent parser for the subset of C++ constant expressions that
// is supported in macro constants: integer, character, and boolean literals,
// (concatenated) ordinary and UTF-8 string literals, parentheses, unary and
// binary operators, `?:`, and casts (C-style, functional, and `static_cast`)
// to builtin integral types or names of such types (e.g. `uint32_t`).
// References to other object-like macros are expanded beforehand. The
// expressions are built using `Sema`, so that they have the same types and
// semantics as in C++.
class MacroExpressionParser {
 public:
  MacroExpressionParser(clang::Sema& sema, llvm::ArrayRef<clang::Token> tokens)
      : sema_(sema), tokens_(tokens) {}

  // Parses all the tokens as a single expression. Returns null on failure.
  clang::Expr* _Nullable Parse() {
    clang::Expr* expr = ParseConditional();
    if (expr == nullptr || position_ != tokens_.size()) return nullptr;
    return expr;
  }

  // Returns true if the parsed tokens contain a numeric literal with a suffix
  // (e.g. `42u`), a `char16_t` or `char32_t` literal (e.g. `u'x'`), or a cast.
  bool is_typed() const { return is_typed_; }

 private:
  static clang::Expr* _Nullable Get(clang::ExprResult result) {
    return result.isUsable() ? result.get() : nullptr;
  }

  // Returns true if the numeric literal `token` has a suffix, or nullopt if it
  // is invalid.
  std::optional<bool> HasSuffix(const clang::Token& token) const {
    llvm::SmallString<32> buffer;
    bool invalid = false;
    llvm::StringRef spelling =
        sema_.getPreprocessor().getSpelling(token, buffer, &invalid);
    if (invalid) return std::nullopt;
    clang::NumericLiteralParser literal(
        spelling, token.getLocation(), sema_.getSourceManager(),
        sema_.getLangOpts(), sema_.getASTContext().getTargetInfo(),
        sema_.getDiagnostics());
    if (literal.hadError) return std::nullopt;
    return literal.isUnsigned || literal.isLong || literal.isLongLong ||
           literal.isSizeT || literal.isBitInt || literal.MicrosoftInteger != 0;
  }

  const clang::Token* _Nullable Peek() const {
    return position_ < tokens_.size() ? &tokens_[position_] : nullptr;
  }

  bool PeekIs(clang::tok::TokenKind kind) const {
    const clang::Token* token = Peek();
    return token != nullptr && token->is(kind);
  }

  // If the tokens from the current position up to (but excluding) the next
  // `terminator` name a type (see `ParseTypeName`), consumes them and returns
  // the type. Otherwise, consumes nothing and returns `std::nullopt`.
  std::optional<clang::QualType> TryParseTypeNameUntil(
      clang::tok::TokenKind terminator, bool simple) {
    size_t end = position_;
    while (end < tokens_.size() && !tokens_[end].is(terminator)) ++end;
    if (end == position_ || end == tokens_.size()) return std::nullopt;
    std::optional<clang::QualType> type =
        ParseTypeName(sema_.getASTContext(),
                      tokens_.slice(position_, end - position_), simple);
    if (type.has_value()) position_ = end;
    return type;
  }

  clang::TypeSourceInfo* TypeSourceInfo(clang::QualType type,
                                        clang::SourceLocation loc) {
    return sema_.getASTContext().getTrivialTypeSourceInfo(type, loc);
  }

  // conditional-expression:
  //     logical-or-expression
  //     logical-or-expression `?` conditional-expression `:`
  //         conditional-expression
  clang::Expr* _Nullable ParseConditional() {
    clang::Expr* condition = ParseBinary(clang::prec::LogicalOr);
    if (condition == nullptr || !PeekIs(clang::tok::question)) {
      return condition;
    }
    clang::SourceLocation question_loc = tokens_[position_++].getLocation();
    clang::Expr* true_expr = ParseConditional();
    if (true_expr == nullptr || !PeekIs(clang::tok::colon)) return nullptr;
    clang::SourceLocation colon_loc = tokens_[position_++].getLocation();
    clang::Expr* false_expr = ParseConditional();
    if (false_expr == nullptr) return nullptr;
    return Get(sema_.ActOnConditionalOp(question_loc, colon_loc, condition,
                                        true_expr, false_expr));
  }

  // Parses a sequence of binary operators with a precedence of at least
  // `min_precedence` (precedence climbing). All supported binary operators are
  // left-associative.
  clang::Expr* _Nullable ParseBinary(clang::prec::Level min_precedence) {
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
  //     `(` type-name `)` unary-expression
  clang::Expr* _Nullable ParseUnary() {
    const clang::Token* token = Peek();
    if (token == nullptr) return nullptr;

    // A C-style cast, as opposed to a parenthesized expression.
    if (token->is(clang::tok::l_paren)) {
      const size_t l_paren_position = position_;
      clang::SourceLocation l_paren_loc = token->getLocation();
      ++position_;
      if (std::optional<clang::QualType> type =
              TryParseTypeNameUntil(clang::tok::r_paren, /*simple=*/false)) {
        clang::SourceLocation r_paren_loc = tokens_[position_++].getLocation();
        clang::Expr* operand = ParseUnary();
        if (operand == nullptr) return nullptr;
        is_typed_ = true;
        return Get(sema_.BuildCStyleCastExpr(l_paren_loc,
                                             TypeSourceInfo(*type, l_paren_loc),
                                             r_paren_loc, operand));
      }
      position_ = l_paren_position;
      return ParsePrimary();
    }

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
  //     string-literal+
  //     boolean-literal
  //     `(` conditional-expression `)`
  //     `static_cast` `<` type-name `>` `(` conditional-expression `)`
  //     simple-type-name `(` conditional-expression `)`
  clang::Expr* _Nullable ParsePrimary() {
    const clang::Token* token = Peek();
    if (token == nullptr) return nullptr;

    // A functional cast, e.g. `uint32_t(1)`.
    if (std::optional<clang::QualType> type =
            TryParseTypeNameUntil(clang::tok::l_paren, /*simple=*/true)) {
      clang::SourceLocation type_loc = token->getLocation();
      clang::SourceLocation l_paren_loc = tokens_[position_++].getLocation();
      clang::Expr* operand = ParseConditional();
      if (operand == nullptr || !PeekIs(clang::tok::r_paren)) return nullptr;
      clang::SourceLocation r_paren_loc = tokens_[position_++].getLocation();
      is_typed_ = true;
      return Get(sema_.BuildCXXFunctionalCastExpr(
          TypeSourceInfo(*type, type_loc), *type, l_paren_loc, operand,
          r_paren_loc));
    }

    ++position_;
    switch (token->getKind()) {
      case clang::tok::kw_static_cast: {
        clang::SourceLocation op_loc = token->getLocation();
        if (!PeekIs(clang::tok::less)) return nullptr;
        clang::SourceLocation l_angle_loc = tokens_[position_++].getLocation();
        std::optional<clang::QualType> type =
            TryParseTypeNameUntil(clang::tok::greater, /*simple=*/false);
        if (!type.has_value()) return nullptr;
        clang::SourceLocation r_angle_loc = tokens_[position_++].getLocation();
        if (!PeekIs(clang::tok::l_paren)) return nullptr;
        clang::SourceLocation l_paren_loc = tokens_[position_++].getLocation();
        clang::Expr* operand = ParseConditional();
        if (operand == nullptr || !PeekIs(clang::tok::r_paren)) return nullptr;
        clang::SourceLocation r_paren_loc = tokens_[position_++].getLocation();
        is_typed_ = true;
        return Get(sema_.BuildCXXNamedCast(
            op_loc, clang::tok::kw_static_cast, TypeSourceInfo(*type, op_loc),
            operand, clang::SourceRange(l_angle_loc, r_angle_loc),
            clang::SourceRange(l_paren_loc, r_paren_loc)));
      }
      case clang::tok::numeric_constant: {
        std::optional<bool> has_suffix = HasSuffix(*token);
        if (!has_suffix.has_value()) return nullptr;
        is_typed_ |= *has_suffix;
        // User-defined literals are rejected, since there is no scope to look
        // up the literal operator in.
        return Get(sema_.ActOnNumericConstant(*token, /*UDLScope=*/nullptr));
      }
      case clang::tok::utf16_char_constant:
      case clang::tok::utf32_char_constant:
        is_typed_ = true;
        [[fallthrough]];
      case clang::tok::char_constant:
      case clang::tok::wide_char_constant:
      case clang::tok::utf8_char_constant:
        return Get(sema_.ActOnCharacterConstant(*token, /*UDLScope=*/nullptr));
      case clang::tok::string_literal:
      case clang::tok::wide_string_literal:
      case clang::tok::utf8_string_literal:
      case clang::tok::utf16_string_literal:
      case clang::tok::utf32_string_literal: {
        // Adjacent string literals are concatenated.
        const size_t begin = position_ - 1;
        while (position_ < tokens_.size() &&
               clang::tok::isStringLiteral(tokens_[position_].getKind())) {
          ++position_;
        }
        return Get(sema_.ActOnStringLiteral(
            tokens_.slice(begin, position_ - begin), /*UDLScope=*/nullptr));
      }
      case clang::tok::kw_true:
      case clang::tok::kw_false:
        return Get(
            sema_.ActOnCXXBoolLiteral(token->getLocation(), token->getKind()));
      case clang::tok::l_paren: {
        clang::SourceLocation l_paren_loc = token->getLocation();
        clang::Expr* inner = ParseConditional();
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
  bool is_typed_ = false;
};

}  // namespace

std::optional<MacroConstant> SynthesizeMacroConstant(
    clang::Sema& sema, const clang::IdentifierInfo& name,
    const clang::MacroInfo& macro_info) {
  clang::SourceLocation name_loc = macro_info.getDefinitionLoc();
  clang::SourceLocation end_loc = macro_info.getDefinitionEndLoc();
  if (name_loc.isInvalid() || !name_loc.isFileID() || end_loc.isInvalid() ||
      !end_loc.isFileID()) {
    return std::nullopt;
  }

  llvm::SmallVector<clang::Token, 16> tokens;
  llvm::SmallVector<const clang::IdentifierInfo*, 4> expanding;
  size_t expansion_steps = 0;
  if (!ExpandObjectLikeMacro(sema.getPreprocessor(), name, macro_info,
                             expanding, expansion_steps, tokens)) {
    return std::nullopt;
  }

  clang::ASTContext& ast_context = sema.getASTContext();
  ScopedDiagnosticSuppression diagnostic_suppression(sema.getDiagnostics());
  MacroExpressionParser parser(sema, tokens);
  clang::Expr* expr = parser.Parse();
  if (expr == nullptr || diagnostic_suppression.HasErrorOccurred() ||
      expr->containsErrors()) {
    return std::nullopt;
  }

  // Only integral (including boolean and character) and (narrow or UTF-8)
  // string constants are supported.
  clang::QualType type = expr->getType();
  if (const auto* string_literal =
          llvm::dyn_cast<clang::StringLiteral>(expr->IgnoreParens())) {
    if (!string_literal->isOrdinary() && !string_literal->isUTF8()) {
      return std::nullopt;
    }
  } else if (!IsSupportedIntegralType(type) ||
             !expr->getIntegerConstantExpr(ast_context).has_value()) {
    return std::nullopt;
  }

  // Make the initializer span the whole macro definition (rather than, say,
  // the definition of a nested macro), so that the source range of the
  // variable is the macro definition.
  clang::ExprResult wrapped_expr = sema.ActOnParenExpr(name_loc, end_loc, expr);
  if (!wrapped_expr.isUsable()) return std::nullopt;
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
  return MacroConstant{.var_decl = var_decl, .is_typed = parser.is_typed()};
}

}  // namespace crubit
