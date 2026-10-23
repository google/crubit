// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/var.h"

#include <cstddef>
#include <cstdint>
#include <limits>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/types/span.h"
#include "common/status_macros.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/clang_compat_macros.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/APValue.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/DeclarationName.h"
#include "clang/AST/Expr.h"
#include "clang/AST/TypeBase.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Sema/DeclSpec.h"
#include "clang/Sema/Ownership.h"
#include "clang/Sema/Sema.h"
#if LLVM_DEV_DATE_GE(20261022)
#include "clang/Sema/SemaProxy.h"
#endif
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Casting.h"

namespace crubit {
namespace {

// The maximum number of explicitly initialized elements of a C-style array
// constant. Every such element is emitted individually in the generated
// bindings, so this bounds the size (and compile time) of the bindings, and of
// every crate that uses them. Elements that are not explicitly initialized are
// emitted as a single filler, and don't count towards this limit.
constexpr uint64_t kMaxInitializedArrayElements = 1024;

// Returns the number of explicitly initialized elements of `value`, an array
// with `num_dimensions` dimensions, counting the elements of nested arrays
// (including the filler, which is emitted once) rather than the arrays
// themselves.
uint64_t CountInitializedElements(const clang::APValue& value,
                                  size_t num_dimensions) {
  if (!value.isArray()) return 0;
  const unsigned num_initialized = value.getArrayInitializedElts();
  if (num_dimensions <= 1) return num_initialized;
  uint64_t count = 0;
  for (unsigned i = 0; i < num_initialized; ++i) {
    count += CountInitializedElements(value.getArrayInitializedElt(i),
                                      num_dimensions - 1);
  }
  if (value.hasArrayFiller()) {
    count +=
        CountInitializedElements(value.getArrayFiller(), num_dimensions - 1);
  }
  return count;
}

// Returns true if `type` is plain `char` (not `signed char` or
// `unsigned char`), which Crubit maps to `ffi_11::c_char`.
bool IsPlainChar(clang::QualType type) {
  const auto* builtin = type->getAs<clang::BuiltinType>();
  if (builtin == nullptr) {
    return false;
  }
  switch (builtin->getKind()) {
    // Plain `char`, on targets where it is signed or unsigned, respectively.
    case clang::BuiltinType::Char_S:
    case clang::BuiltinType::Char_U:
      return true;
    // `signed char` and `unsigned char`.
    case clang::BuiltinType::SChar:
    case clang::BuiltinType::UChar:
    default:
      return false;
  }
}

// If `init` initializes a one-dimensional `char` array of the given `size` by
// a narrow string literal which leaves room for its NULL terminator, returns
// the string literal. Otherwise, returns null.
const clang::StringLiteral* GetStringLiteralInit(const clang::Expr* init,
                                                 uint64_t size) {
  if (init == nullptr) {
    return nullptr;
  }
  const clang::Expr* expr = init->IgnoreImplicit()->IgnoreParens();
  // `char x[] = {"abc"};` is equivalent to `char x[] = "abc";`.
  if (const auto* init_list = llvm::dyn_cast<clang::InitListExpr>(expr);
      init_list != nullptr && init_list->isStringLiteralInit()) {
    expr = init_list->getInit(0)->IgnoreImplicit()->IgnoreParens();
  }
  const auto* string_literal = llvm::dyn_cast<clang::StringLiteral>(expr);
  // In C++, the string literal and its NULL terminator always fit, but in C the
  // NUL terminator may be dropped (`char x[2] = "ab";`). Such an array is
  // treated as individual elements instead.
  if (string_literal == nullptr || string_literal->getCharByteWidth() != 1 ||
      string_literal->getLength() >= size) {
    return nullptr;
  }
  return string_literal;
}

// Converts `value`, an integer or boolean array element, to `out`.
absl::Status ConvertIntegerElement(const clang::APValue& value,
                                   ir_proto::IntegerConstant& out) {
  if (!value.isInt()) {
    return absl::InvalidArgumentError(
        "array element did not evaluate to an integer");
  }
  CRUBIT_ASSIGN_OR_RETURN(ir_proto::IntegerConstant integer_constant,
                          IntegerConstantFromAPValue(value.getInt()));
  out = std::move(integer_constant);
  return absl::OkStatus();
}

// Converts `value`, a (possibly nested) array with the given `dimensions`
// (outermost first), to `out`.
//
// `char_array_init` is the initializer expression of a plain `char` array, or
// null for other arrays. It is only used to find rows initialized by string
// literals.
//
// Only the explicitly initialized elements are recorded individually. The
// remaining elements are value-initialized, and their value is recorded as the
// filler.
absl::Status ConvertArrayValue(const clang::APValue& value,
                               absl::Span<const uint64_t> dimensions,
                               const clang::Expr* char_array_init,
                               ir_proto::ArrayValue& out) {
  CHECK(!dimensions.empty()) << "must be called with at least one dimension";
  const uint64_t size = dimensions[0];
  CHECK(value.isArray()) << "`value` must be the evaluated value of an array";
  CHECK_EQ(value.getArraySize(), size)
      << "`dimensions` must come from the type `value` was evaluated from";
  out.set_length(static_cast<int32_t>(size));
  // "Filler" in Clang means the value of the trailing elements that were not
  // explicitly initialized, and are therefore value-initialized. For example,
  // `int x[4] = {1, 2};` has two initialized elements and a filler of `0`.
  const unsigned num_initialized = value.getArrayInitializedElts();
  const clang::APValue* filler =
      value.hasArrayFiller() ? &value.getArrayFiller() : nullptr;

  if (dimensions.size() > 1) {
    const clang::InitListExpr* char_array_init_list =
        char_array_init == nullptr
            ? nullptr
            : llvm::dyn_cast<clang::InitListExpr>(
                  char_array_init->IgnoreImplicit()->IgnoreParens());
    auto* elements = out.mutable_kind()->mutable_array_elements();
    for (unsigned i = 0; i < num_initialized; ++i) {
      const clang::Expr* element_char_array_init =
          char_array_init_list != nullptr &&
                  i < char_array_init_list->getNumInits()
              ? char_array_init_list->getInit(i)
              : nullptr;
      CRUBIT_RETURN_IF_ERROR(ConvertArrayValue(
          value.getArrayInitializedElt(i), dimensions.subspan(1),
          element_char_array_init, *elements->add_elements()));
    }
    if (filler != nullptr) {
      // The filler has no initializer expression of its own.
      CRUBIT_RETURN_IF_ERROR(ConvertArrayValue(*filler, dimensions.subspan(1),
                                               /*char_array_init=*/nullptr,
                                               *elements->mutable_filler()));
    }
    return absl::OkStatus();
  }

  if (const clang::StringLiteral* string_literal =
          GetStringLiteralInit(char_array_init, size)) {
    out.mutable_kind()->set_string_literal(string_literal->getBytes().str());
    return absl::OkStatus();
  }

  auto* elements = out.mutable_kind()->mutable_integer_elements();
  for (unsigned i = 0; i < num_initialized; ++i) {
    CRUBIT_RETURN_IF_ERROR(ConvertIntegerElement(
        value.getArrayInitializedElt(i), *elements->add_elements()));
  }
  if (filler != nullptr) {
    CRUBIT_RETURN_IF_ERROR(
        ConvertIntegerElement(*filler, *elements->mutable_filler()));
  }
  return absl::OkStatus();
}

// Returns true if `type` is `std::string_view`
// (`std::basic_string_view<char, std::char_traits<char>>`) or
// `absl::string_view` (including when `ABSL_USES_STD_STRING_VIEW` is 0).
bool IsStringViewType(const clang::ASTContext& ast_context,
                      clang::QualType type) {
  const clang::CXXRecordDecl* record_decl = type->getAsCXXRecordDecl();
  if (record_decl == nullptr) {
    return false;
  }
  std::optional<llvm::StringRef> top_level_namespace =
      AsTopLevelNamespace(record_decl->getDeclContext());
  if (const auto* specialization =
          llvm::dyn_cast<clang::ClassTemplateSpecializationDecl>(record_decl)) {
    if (top_level_namespace != "std" ||
        specialization->getName() != "basic_string_view") {
      return false;
    }
    absl::StatusOr<clang::QualType> char_type = ParameterizedByTAndStdTraitT(
        ast_context, specialization, "char_traits");
    // Unlike `TemplateSpecialization::StdStringView`, only plain `char` (not
    // `signed char` or `unsigned char`) is treated as text.
    return char_type.ok() && IsPlainChar(*char_type);
  }
  return top_level_namespace == "absl" &&
         record_decl->getName() == "string_view";
}

// Returns the contents of `var_decl`, a constant `std::string_view` or
// `absl::string_view`, by constant-evaluating `var_decl.data()` and
// `var_decl.size()`.
//
// This is how Clang evaluates user-generated `static_assert` messages (see
// `Sema::EvaluateAsString`), except that failures are returned as errors
// instead of being diagnosed. Using the public `data()` and `size()` API keeps
// this independent of the standard library's private layout, and the constant
// evaluator accepts any constant characters (string literals, `constexpr`
// arrays, substrings, ...) and rejects anything else.
absl::StatusOr<std::string> EvaluateStringViewContents(
    ImportContext& ictx, clang::VarDecl& var_decl) {
  clang::Sema& sema = ictx.sema_;
  clang::ASTContext& ast_context = ictx.ctx_;
  const clang::SourceLocation loc = var_decl.getLocation();
  FakeTUScope tu_scope(ictx);
  // Errors while building the calls below (e.g. a missing or inaccessible
  // member) are trapped here instead of being reported to the user.
  clang::Sema::SFINAETrap trap(sema);

  clang::Expr* string_view_ref =
      sema.BuildDeclRefExpr(&var_decl, var_decl.getType().getNonReferenceType(),
                            clang::VK_LValue, loc);
  auto build_member_call = [&](llvm::StringRef name) -> clang::Expr* {
    clang::CXXScopeSpec scope_spec;
    clang::ExprResult member = sema.BuildMemberReferenceExpr(
        string_view_ref, string_view_ref->getType(), loc, /*IsArrow=*/false,
        scope_spec, /*TemplateKWLoc=*/clang::SourceLocation(),
        /*FirstQualifierInScope=*/nullptr,
        clang::DeclarationNameInfo(&ast_context.Idents.get(name), loc),
        /*TemplateArgs=*/nullptr, /*S=*/nullptr);
    if (member.isInvalid()) {
      return nullptr;
    }
    clang::ExprResult call =
        sema.BuildCallExpr(/*S=*/nullptr, member.get(), loc, {}, loc);
    return call.isInvalid() ? nullptr : call.get();
  };
  clang::Expr* data_call = build_member_call("data");
  clang::Expr* size_call = build_member_call("size");
  if (trap.hasErrorOccurred() || data_call == nullptr || size_call == nullptr ||
      !data_call->getType()->isPointerType() ||
      !IsPlainChar(data_call->getType()->getPointeeType()) ||
      !size_call->getType()->isIntegerType()) {
    return absl::InvalidArgumentError(
        "string_view constant has no usable `data()` and `size()`");
  }

  std::string contents;
  clang::Expr::EvalResult eval_result;
#if LLVM_DEV_DATE_GE(20261022)
  clang::sema::EvalProxy sema_proxy(sema);
  if (!string_view_ref->EvaluateCharRangeAsString(contents, size_call,
                                                  data_call, ast_context,
                                                  sema_proxy, eval_result)) {
#else
  if (!string_view_ref->EvaluateCharRangeAsString(
          contents, size_call, data_call, ast_context, eval_result)) {
#endif
    return absl::InvalidArgumentError(
        "string_view constant does not refer to constant characters");
  }
  return contents;
}

}  // namespace

std::unique_ptr<ir_proto::Item> VarDeclImporter::Import(
    clang::VarDecl* var_decl) {
  // Most vars are not globals – fail fast if this is the case.
  clang::DeclContext* decl_context = var_decl->getDeclContext();
  if (!decl_context) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        {FormattedError::Static("DeclContext was unexpectedly null")});
  }
  if (!decl_context->isTranslationUnit() && !decl_context->isExternCContext() &&
      !decl_context->isExternCXXContext() && !decl_context->isNamespace() &&
      !var_decl->isStaticDataMember()) {
    return nullptr;
  }

  // Static data members are imported like namespace-scoped variables: the
  // bindings generator places them in the snake-cased module that holds the
  // record's other nested items (e.g. `Foo::kBar` becomes `foo::kBar`).
  if (var_decl->isStaticDataMember()) {
    if (decl_context->isDependentContext()) {
      // Members of uninstantiated templates have no concrete type or value.
      return nullptr;
    }
    if (clang::isa<clang::ClassTemplateSpecializationDecl>(decl_context)) {
      // Static data members of class template specializations are instantiated
      // lazily: their initializers may not have been evaluated and their
      // definitions may never be emitted by C++ unless ODR-used, which would
      // lead to link errors from the Rust side.
      return ictx_.ImportUnsupportedItem(
          *var_decl, std::nullopt,
          {FormattedError::Static(
              "static data members of class template specializations are not "
              "supported")});
    }
  }

  if (var_decl->getTLSKind() != clang::VarDecl::TLS_None) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        {FormattedError::Static("thread_local variables are not supported")});
  }

  // Note that `[const|inline] T x = /* constant initializer */;` acts like
  // constexpr and does not create an external symbol in Clang. This was
  // apparently done to support constexpr-like patterns before it existed in the
  // language.
  bool is_const_or_inline =
      var_decl->getType().isConstQualified() || var_decl->isInline();
  bool has_const_init =
      var_decl->isConstexpr() ||
      (is_const_or_inline && var_decl->hasConstantInitialization());

  if (!has_const_init && !var_decl->hasExternalFormalLinkage()) {
    return nullptr;
  }

  if (llvm::isa<clang::VarTemplateSpecializationDecl>(var_decl)) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        {FormattedError::Static("templated variables are not supported")});
  }

  absl::StatusOr<TranslatedIdentifier> var_name =
      ictx_.GetTranslatedIdentifier(*var_decl);
  if (!var_name.ok()) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        {FormattedError::PrefixedStrCat("variable name is not supported",
                                        var_name.status().message())});
  }

  auto enclosing_item_id = ictx_.GetEnclosingItemId(var_decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        {FormattedError::FromStatus(std::move(enclosing_item_id.status()))});
  }

  std::optional<std::string> deprecated;
  absl::StatusOr<std::optional<std::string>> unknown_attr =
      CollectUnknownAttrs(*var_decl, [&](const clang::Attr& attr) {
        if (auto* deprecated_attr =
                clang::dyn_cast<clang::DeprecatedAttr>(&attr)) {
          deprecated.emplace(deprecated_attr->getMessage());
          return true;
        } else if (clang::isa<clang::VisibilityAttr>(attr)) {
          // Visibility attributes on global/static variables do not affect Rust
          // FFI bindings.
          return true;
        }
        return false;
      });
  if (!unknown_attr.ok()) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        {FormattedError::FromStatus(std::move(unknown_attr.status()))});
  }

  if (has_const_init) {
    return ImportConstant(var_decl, *var_name, *enclosing_item_id,
                          std::move(*unknown_attr), std::move(deprecated));
  }

  CcType type =
      ictx_.ConvertQualType(var_decl->getType(), nullptr, /*nullable=*/true,
                            ictx_.AreAssumedLifetimesEnabledForTarget(
                                ictx_.GetOwningTarget(*var_decl)));

  // Global variables without extern "C" have different linkage, but in practice
  // all this means is that the name is mangled, not that the ABI is different.
  // So we can support directly binding to all variables as long as we use the
  // correct name.
  std::optional<std::string> mangled_name = std::nullopt;
  if (!var_decl->isExternC()) {
    mangled_name = ictx_.GetMangledName(*var_decl);
  }
  if (mangled_name == var_name->rs_identifier().Ident()) {
    mangled_name = std::nullopt;
  }

  ictx_.MarkAsSuccessfullyImported(*var_decl);
  auto item = std::make_unique<ir_proto::Item>();
  auto* global_var = item->mutable_global_var();
  global_var->mutable_cc_name()->set_identifier(
      var_name->cc_identifier.Ident());
  global_var->mutable_rs_name()->set_identifier(
      var_name->rs_identifier().Ident());
  global_var->set_unique_name(ictx_.GetUniqueName(*var_decl));
  global_var->set_id(ictx_.GenerateItemId(*var_decl).value());
  global_var->set_owning_target(ictx_.GetOwningTarget(*var_decl).value());
  global_var->set_source_loc(
      ictx_.ConvertSourceLocation(var_decl->getBeginLoc(), nullptr));
  if (mangled_name.has_value()) {
    global_var->set_mangled_name(std::move(*mangled_name));
  }
  *global_var->mutable_type() = std::move(type);
  if (unknown_attr->has_value()) {
    global_var->set_unknown_attr(std::move(**unknown_attr));
  }
  if (enclosing_item_id->has_value()) {
    global_var->set_enclosing_item_id((*enclosing_item_id)->value());
  }
  if (deprecated.has_value()) {
    global_var->set_deprecated(std::move(*deprecated));
  }
  if (auto comment = ictx_.GetComment(*var_decl); comment.has_value()) {
    global_var->set_doc_comment(std::move(*comment));
  }
  return item;
}

std::unique_ptr<ir_proto::Item> VarDeclImporter::ImportConstant(
    clang::VarDecl* var_decl, TranslatedIdentifier& var_name,
    std::optional<ItemId> enclosing_item_id,
    std::optional<std::string> unknown_attr,
    std::optional<std::string> deprecated) {
  // For (possibly nested) C-style arrays, the type of the constant is the
  // innermost element type.
  std::vector<uint64_t> dimensions;
  clang::QualType value_type = var_decl->getType();
  while (const clang::ConstantArrayType* array_type =
             ictx_.ctx_.getAsConstantArrayType(value_type)) {
    uint64_t dimension = array_type->getZExtSize();
    if (dimension == 0) {
      return ictx_.ImportUnsupportedItem(
          *var_decl, std::nullopt,
          {FormattedError::Static(
              "zero-length C-style arrays are not supported")});
    }
    if (dimension > std::numeric_limits<int32_t>::max()) {
      return ictx_.ImportUnsupportedItem(
          *var_decl, std::nullopt,
          {FormattedError::Static("C-style array dimension is too large")});
    }
    dimensions.push_back(dimension);
    value_type = array_type->getElementType();
  }
  bool is_array = !dimensions.empty();
  bool is_string_view = IsStringViewType(ictx_.ctx_, var_decl->getType());
  if (!value_type->isBooleanType() && !value_type->isIntegerType() &&
      !is_string_view) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        {is_array ? FormattedError::Static(
                        "only C-style arrays of boolean and integer elements "
                        "are supported")
                  : FormattedError::Static(
                        "only boolean, integer, and string_view constexpr "
                        "variables are supported")});
  }

  const clang::APValue* value = var_decl->evaluateValue();
  if (value == nullptr) {
    return ictx_.ImportUnsupportedItem(
        *var_decl, std::nullopt,
        {FormattedError::Static("unable to evaluate constexpr value")});
  }
  if (is_array) {
    uint64_t num_initialized =
        CountInitializedElements(*value, dimensions.size());
    if (num_initialized > kMaxInitializedArrayElements) {
      return ictx_.ImportUnsupportedItem(
          *var_decl, std::nullopt,
          {FormattedError::Substitute(
              "C-style array constants can have at most $0 explicitly "
              "initialized elements, but this one has $1. Each one is emitted "
              "in the generated bindings, which would make them slow to "
              "compile. To use this array from Rust, expose it through a "
              "function instead, e.g. one that returns an `absl::Span` or a "
              "pointer to its first element.",
              kMaxInitializedArrayElements, num_initialized)});
    }
  }

  auto item = std::make_unique<ir_proto::Item>();
  auto* constant = item->mutable_constant();
  if (is_string_view) {
    absl::StatusOr<std::string> string_view_value =
        EvaluateStringViewContents(ictx_, *var_decl);
    if (!string_view_value.ok()) {
      return ictx_.ImportUnsupportedItem(
          *var_decl, std::nullopt,
          {FormattedError::FromStatus(std::move(string_view_value.status()))});
    }
    constant->mutable_kind()->set_string_view_value(
        *std::move(string_view_value));
  } else if (!is_array) {
    if (!value->isInt()) {
      return ictx_.ImportUnsupportedItem(
          *var_decl, std::nullopt,
          {FormattedError::Static(
              "constexpr value did not evaluate to an integer")});
    }
    absl::StatusOr<ir_proto::IntegerConstant> integer_constant =
        IntegerConstantFromAPValue(value->getInt());
    if (!integer_constant.ok()) {
      return ictx_.ImportUnsupportedItem(
          *var_decl, std::nullopt,
          {FormattedError::FromStatus(std::move(integer_constant.status()))});
    }
    *constant->mutable_kind()->mutable_value() = *std::move(integer_constant);
  } else {
    // String-literal rows are only recorded for plain `char` arrays, which
    // Crubit maps to `ffi_11::c_char`.
    const clang::Expr* char_array_init =
        IsPlainChar(value_type) ? var_decl->getInit() : nullptr;
    if (absl::Status status =
            ConvertArrayValue(*value, dimensions, char_array_init,
                              *constant->mutable_kind()->mutable_array_value());
        !status.ok()) {
      return ictx_.ImportUnsupportedItem(
          *var_decl, std::nullopt,
          {FormattedError::FromStatus(std::move(status))});
    }
  }

  if (!is_string_view) {
    CcType type =
        ictx_.ConvertQualType(value_type, nullptr, /*nullable=*/true,
                              ictx_.AreAssumedLifetimesEnabledForTarget(
                                  ictx_.GetOwningTarget(*var_decl)));
    *constant->mutable_type() = std::move(type);
  }

  ictx_.MarkAsSuccessfullyImported(*var_decl);
  constant->mutable_cc_name()->set_identifier(var_name.cc_identifier.Ident());
  constant->mutable_rs_name()->set_identifier(var_name.rs_identifier().Ident());
  constant->set_unique_name(ictx_.GetUniqueName(*var_decl));
  constant->set_id(ictx_.GenerateItemId(*var_decl).value());
  constant->set_owning_target(ictx_.GetOwningTarget(*var_decl).value());
  constant->set_source_loc(
      ictx_.ConvertSourceLocation(var_decl->getBeginLoc(), nullptr));
  if (unknown_attr.has_value()) {
    constant->set_unknown_attr(std::move(*unknown_attr));
  }
  if (enclosing_item_id.has_value()) {
    constant->set_enclosing_item_id(enclosing_item_id->value());
  }
  if (deprecated.has_value()) {
    constant->set_deprecated(std::move(*deprecated));
  }
  if (auto comment = ictx_.GetComment(*var_decl); comment.has_value()) {
    constant->set_doc_comment(std::move(*comment));
  }
  return item;
}

}  // namespace crubit
