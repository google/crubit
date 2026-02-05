// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/cxx_record.h"

#include <algorithm>
#include <array>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "clang/Basic/SourceLocation.h"
#include "absl/base/nullability.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "absl/log/die_if_null.h"
#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "common/annotation_reader.h"
#include "common/status_macros.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/ast_convert.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Attrs.inc"
#include "clang/AST/CXXInheritance.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/PrettyPrinter.h"
#include "clang/AST/RecordLayout.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/OperatorKinds.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Sema/Sema.h"
#include "llvm/Support/ErrorHandling.h"
namespace crubit {

namespace {

// Returns true if the given declaration is publicly accessible, i.e. if it and
// all its enclosing records and namespaces are not private.
bool IsDeclPubliclyAccessible(const clang::Decl* decl) {
  while (decl) {
    if (clang::isa<clang::TranslationUnitDecl>(decl)) {
      return true;
    }
    if (decl->isInAnonymousNamespace()) {
      return false;
    }
    switch (decl->getAccess()) {
      case clang::AS_private:
      case clang::AS_protected:
        return false;
      case clang::AS_public:
        break;
      case clang::AS_none:
        if (const auto* rd =
                clang::dyn_cast<clang::RecordDecl>(decl->getDeclContext())) {
          if (rd->isClass()) {
            return false;
          }
        }
        break;
    }
    decl = clang::dyn_cast<clang::Decl>(decl->getDeclContext());
  }
  return true;
}

// Checks that the given template argument is publicly accessible.
absl::Status CheckTemplateArgIsPublic(const clang::TemplateArgument& arg);

// Checks that the given type is publicly accessible.
absl::Status CheckQualTypeIsPublic(clang::QualType type) {
  if (type.isNull()) {
    return absl::OkStatus();
  }
  type = type.getCanonicalType();

  // Look through pointers and references.
  while (type->isPointerType() || type->isReferenceType()) {
    type = type->getPointeeType();
  }

  // Some function pointers have __attribute__((preserve_none)), which doesn't
  // get formatted in a way that clang likes to read back in. For now, such
  // pointers are conservatively disallowed.
  if (const clang::FunctionProtoType* fpt =
          type->getAs<clang::FunctionProtoType>()) {
    if (fpt->getCallConv() == clang::CallingConv::CC_PreserveNone) {
      return absl::FailedPreconditionError(
          absl::StrCat("Type '", type.getAsString(),
                       "' has unsupported calling convention preserve_none"));
    }
  }

  if (const clang::TagType* tag_type = type->getAs<clang::TagType>()) {
    if (!IsDeclPubliclyAccessible(tag_type->getDecl())) {
      return absl::FailedPreconditionError(absl::StrCat(
          "Type '", type.getAsString(), "' uses non-public declaration '",
          tag_type->getDecl()->getQualifiedNameAsString(),
          "', which cannot be spelled in C++ by generated code."));
    }
  }

  if (const auto* tst = type->getAs<clang::TemplateSpecializationType>()) {
    for (const auto& arg : tst->template_arguments()) {
      CRUBIT_RETURN_IF_ERROR(CheckTemplateArgIsPublic(arg));
    }
  } else if (const clang::RecordType* rt = type->getAs<clang::RecordType>()) {
    // If it's a specialization, it might be handled as TagType,
    // but if it's RecordType we can get template args too.
    if (auto* spec = clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
            rt->getDecl())) {
      for (const auto& arg : spec->getTemplateArgs().asArray()) {
        CRUBIT_RETURN_IF_ERROR(CheckTemplateArgIsPublic(arg));
      }
    }
  }

  return absl::OkStatus();
}

absl::Status CheckTemplateArgIsPublic(const clang::TemplateArgument& arg) {
  switch (arg.getKind()) {
    case clang::TemplateArgument::ArgKind::Type:
      return CheckQualTypeIsPublic(arg.getAsType());
    case clang::TemplateArgument::ArgKind::Declaration: {
      clang::NamedDecl* decl = arg.getAsDecl();
      if (!IsDeclPubliclyAccessible(decl)) {
        return absl::FailedPreconditionError(absl::StrCat(
            "Declaration template argument '", decl->getQualifiedNameAsString(),
            "' is not publicly accessible"));
      }
      if (clang::ValueDecl* vd = clang::dyn_cast<clang::ValueDecl>(decl)) {
        CRUBIT_RETURN_IF_ERROR(CheckQualTypeIsPublic(vd->getType()));
      }
      return absl::OkStatus();
    }
    case clang::TemplateArgument::ArgKind::Template: {
      clang::TemplateDecl* decl = arg.getAsTemplate().getAsTemplateDecl();
      if (!IsDeclPubliclyAccessible(decl)) {
        return absl::FailedPreconditionError(absl::StrCat(
            "Template template argument '", decl->getQualifiedNameAsString(),
            "' is not publicly accessible"));
      }
      return absl::OkStatus();
    }
    case clang::TemplateArgument::ArgKind::Pack:
      for (const auto& pack_arg : arg.pack_elements()) {
        CRUBIT_RETURN_IF_ERROR(CheckTemplateArgIsPublic(pack_arg));
      }
      return absl::OkStatus();
    case clang::TemplateArgument::ArgKind::Expression:
    case clang::TemplateArgument::ArgKind::Integral:
    case clang::TemplateArgument::ArgKind::NullPtr:
    case clang::TemplateArgument::ArgKind::TemplateExpansion:
    case clang::TemplateArgument::ArgKind::StructuralValue:
    case clang::TemplateArgument::ArgKind::Null:
      return absl::OkStatus();
  }
}

absl::StatusOr<std::string> CcName(
    const clang::ASTContext& ast_context,
    const clang::ClassTemplateSpecializationDecl* specialization_decl) {
  if (!IsDeclPubliclyAccessible(
          specialization_decl->getSpecializedTemplate()->getTemplatedDecl())) {
    return absl::FailedPreconditionError(
        absl::StrCat("Template '",
                     specialization_decl->getSpecializedTemplate()
                         ->getTemplatedDecl()
                         ->getQualifiedNameAsString(),
                     "' is not publicly accessible"));
  }
  for (const auto& arg : specialization_decl->getTemplateArgs().asArray()) {
    CRUBIT_RETURN_IF_ERROR(CheckTemplateArgIsPublic(arg));
  }

  clang::PrintingPolicy policy(ast_context.getLangOpts());
  policy.IncludeTagDefinition = false;
  // Canonicalize types -- in particular, the template parameter types must be
  // desugared out of an `ElaboratedType` so that their namespaces are written
  // down.
  policy.PrintAsCanonical = true;
  policy.UsePreferredNames = false;
  // Use type suffix (e.g. `123u` rather than just `123`) to avoid the
  // `-Wimplicitly-unsigned-literal` warning.  See also b/244616557.
  policy.AlwaysIncludeTypeForTemplateArgument = true;

  return clang::QualType(ast_context.getCanonicalTagType(specialization_decl))
      .getAsString(policy);
}

AccessSpecifier TranslateAccessSpecifier(clang::AccessSpecifier access) {
  switch (access) {
    case clang::AS_public:
      return kPublic;
    case clang::AS_protected:
      return kProtected;
    case clang::AS_private:
      return kPrivate;
    case clang::AS_none:
      LOG(FATAL)
          << "We should never be encoding a 'none' access specifier in IR.";
      // We have to return something. Conservatively return private so we don't
      // inadvertently make a private member variable accessible in Rust.
      return kPrivate;
  }
}

absl::StatusOr<RecordType> TranslateRecordType(
    const clang::RecordDecl& record_decl) {
  switch (record_decl.getTagKind()) {
    case clang::TagTypeKind::Struct:
      return RecordType::kStruct;
    case clang::TagTypeKind::Union:
      return RecordType::kUnion;
    case clang::TagTypeKind::Class:
      return RecordType::kClass;
    case clang::TagTypeKind::Enum:
      llvm::report_fatal_error(
          "clang::RecordDecl::getTagKind shouldn't return TagTypeKind::Enum");
    case clang::TagTypeKind::Interface:
      // Some docs about `__interface` can be found here:
      // https://docs.microsoft.com/en-us/cpp/cpp/interface?view=msvc-170
      return absl::UnimplementedError(
          "`__interface` / clang::TagTypeKind::Interface is not supported");
  }

  llvm::report_fatal_error("Unrecognized clang::TagKind");
}

// Returns the values of the given keys in `record_decl` if they all exist and
// are strings. Otherwise, returns `std::nullopt`.
template <size_t N>
std::optional<std::array<std::string, N>> GetKeyValues(
    const clang::RecordDecl& record_decl,
    std::array<absl::string_view, N> keys) {
  std::array<std::string, N> values;
  for (int i = 0; i < N; ++i) {
    absl::StatusOr<std::optional<std::string>> value =
        GetAnnotationWithStringArg(record_decl, keys[i]);
    CHECK_OK(value);
    if (!value->has_value()) {
      CHECK_EQ(i, 0) << "Missing value for key: '" << keys[i]
                     << "'; use the provided annotation to ensure that all "
                        "keys are present";
      return std::nullopt;
    }
    values[i] = **std::move(value);
  }
  return values;
}

// Returns the bridge type annotation for the given `record_decl` if it exists.
std::optional<BridgeType> GetBridgeTypeAnnotation(
    ImportContext& ictx, const clang::RecordDecl& record_decl) {
  auto void_converter_values = GetKeyValues<3>(
      record_decl,
      {"crubit_bridge_type", "crubit_bridge_type_rust_to_cpp_converter",
       "crubit_bridge_type_cpp_to_rust_converter"});
  auto crubit_abi_values = GetKeyValues<3>(
      record_decl, {"crubit_bridge_rust_name", "crubit_bridge_abi_rust",
                    "crubit_bridge_abi_cpp"});
  CHECK(1 >= void_converter_values.has_value() + crubit_abi_values.has_value())
      << "CRUBIT_BRIDGE_VOID_CONVERTERS, CRUBIT_BRIDGE, and are mutually "
         "exclusive, and cannot be used on the same type.";

  if (crubit::IsProto2Message(record_decl)) {
    return BridgeType{BridgeType::ProtoMessageBridge{
        .rust_name = record_decl.getNameAsString()}};
  }

  if (void_converter_values.has_value()) {
    auto [rust_name, rust_to_cpp_converter, cpp_to_rust_converter] =
        *void_converter_values;
    return BridgeType{BridgeType::BridgeVoidConverters{
        .rust_name = std::move(rust_name),
        .rust_to_cpp_converter = std::move(rust_to_cpp_converter),
        .cpp_to_rust_converter = std::move(cpp_to_rust_converter),
    }};
  }
  if (crubit_abi_values.has_value()) {
    std::vector<TemplateArg> template_args;
    // If this is a template specialization, need to iterate through the
    // template args
    if (const clang::ClassTemplateSpecializationDecl* specialization_decl =
            clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
                &record_decl)) {
      for (const clang::TemplateArgument& template_arg :
           specialization_decl->getTemplateArgs().asArray()) {
        if (template_arg.getKind() == clang::TemplateArgument::ArgKind::Type) {
          // TODO(b/454627672): is record_decl the right decl to check for
          // assumed_lifetimes?
          template_args.emplace_back(TemplateArg{
              ictx.ConvertQualType(template_arg.getAsType(),
                                   /*lifetimes=*/nullptr, /*nullable=*/true,
                                   ictx.AreAssumedLifetimesEnabledForTarget(
                                       ictx.GetOwningTarget(&record_decl)))});
        }
      }
    }
    auto [rust_name, abi_rust, abi_cpp] = *crubit_abi_values;
    return BridgeType{BridgeType::Bridge{
        .rust_name = std::move(rust_name),
        .abi_rust = std::move(abi_rust),
        .abi_cpp = std::move(abi_cpp),
        .template_args = std::move(template_args),
    }};
  }
  return std::nullopt;
}

// Helper function for `GetTraitDerives` to populate the derived traits in
// `TraitDerives`.
absl::Status AddTraitDerives(const clang::Decl& decl, TraitDerives& result) {
  CRUBIT_ASSIGN_OR_RETURN(
      std::optional<AnnotateArgs> args,
      GetAnnotateAttrArgs(decl, "crubit_internal_trait_derive"));

  if (!args.has_value()) return absl::OkStatus();
  clang::ASTContext& ast_context = decl.getASTContext();

  absl::flat_hash_set<absl::string_view> custom_traits;
  for (const clang::Expr* arg : *args) {
    CRUBIT_ASSIGN_OR_RETURN(const absl::string_view derived_trait,
                            GetExprAsStringLiteral(*arg, ast_context));
    absl::string_view trait;
    TraitImplPolarity polarity;
    if (derived_trait.starts_with("!")) {
      trait = derived_trait.substr(1);
      polarity = TraitImplPolarity::kNegative;
    } else {
      trait = derived_trait;
      polarity = TraitImplPolarity::kPositive;
    }

    if (trait == "Send" || trait == "Sync") {
      if (polarity == TraitImplPolarity::kNegative) {
        return absl::InvalidArgumentError(absl::StrCat(
            "Trait '", trait, "' is negatively derived by default."));
      }
      return absl::InvalidArgumentError(
          absl::StrCat("Trait '", trait,
                       "' is an unsafe trait, and must be implemented with the "
                       "`CRUBIT_UNSAFE_IMPL(\"",
                       trait, "\")` macro."));
    }

    TraitImplPolarity* absl_nullable selected = result.Polarity(trait);
    if (selected == nullptr) {
      // Custom (unrecognized) trait.
      if (polarity == TraitImplPolarity::kNegative) {
        return absl::InvalidArgumentError(absl::StrCat(
            "Custom trait '", trait, "' cannot be negatively derived."));
      }
      auto [it, did_insert] = custom_traits.insert(trait);
      if (did_insert) continue;

      return absl::InvalidArgumentError(absl::StrCat(
          "Custom trait '", trait, "' is derived multiple times."));
    }

    if (*selected == TraitImplPolarity::kNone) {
      // Trait is not yet derived, happy path :)
      *selected = polarity;
      continue;
    }

    if (*selected != polarity) {
      return absl::InvalidArgumentError(absl::StrCat(
          "Trait '", trait, "' is derived both positively and negatively."));
    }
    return absl::InvalidArgumentError(
        absl::StrCat("Trait '", trait, "' is derived multiple times."));
  }
  result.custom.reserve(custom_traits.size());
  for (const absl::string_view trait : custom_traits) {
    result.custom.emplace_back(trait);
  }
  return absl::OkStatus();
}

// Helper function for `GetTraitDerives` to populate the unsafe implementation
// fields in `TraitDerives`.
absl::Status AddUnsafeImpls(const clang::Decl& decl, TraitDerives& result) {
  CRUBIT_ASSIGN_OR_RETURN(
      std::optional<AnnotateArgs> args,
      GetAnnotateAttrArgs(decl, "crubit_internal_unsafe_impl"));

  if (!args.has_value()) return absl::OkStatus();
  clang::ASTContext& ast_context = decl.getASTContext();

  for (const clang::Expr* arg : *args) {
    CRUBIT_ASSIGN_OR_RETURN(const absl::string_view unsafe_impl,
                            GetExprAsStringLiteral(*arg, ast_context));
    if (unsafe_impl == "Send") {
      if (result.send) {
        return absl::InvalidArgumentError(
            "Unsafe implementation 'Send' is derived multiple times.");
      }
      result.send = true;
    } else if (unsafe_impl == "Sync") {
      if (result.sync) {
        return absl::InvalidArgumentError(
            "Unsafe implementation 'Sync' is derived multiple times.");
      }
      result.sync = true;
    } else {
      return absl::InvalidArgumentError(absl::StrCat(
          "Unsafe implementation '", unsafe_impl, "' is not supported."));
    }
  }
  return absl::OkStatus();
}

// Returns the set of traits to derive on the Rust type.
absl::StatusOr<TraitDerives> GetTraitDerives(const clang::Decl& decl) {
  TraitDerives result;
  CRUBIT_RETURN_IF_ERROR(AddTraitDerives(decl, result));
  CRUBIT_RETURN_IF_ERROR(AddUnsafeImpls(decl, result));
  return result;
}

absl::StatusOr<SafetyAnnotation> GetSafetyAnnotation(const clang::Decl& decl) {
  CRUBIT_ASSIGN_OR_RETURN(std::optional<AnnotateArgs> args,
                          GetAnnotateAttrArgs(decl, "crubit_override_unsafe"));
  if (!args.has_value()) return SafetyAnnotation::kUnannotated;
  if (args->size() != 1) {
    return absl::InvalidArgumentError(
        "`crubit_override_unsafe` annotation must have exactly one argument");
  }

  absl::StatusOr<bool> is_unsafe =
      GetExprAsBool(*args->front(), decl.getASTContext());
  if (!is_unsafe.ok()) {
    return absl::InvalidArgumentError(
        "`crubit_override_unsafe` annotation must have a bool argument");
  }
  if (*is_unsafe) {
    return SafetyAnnotation::kUnsafe;
  } else {
    return SafetyAnnotation::kDisableUnsafe;
  }
}

std::optional<Identifier> StringRefToOptionalIdentifier(llvm::StringRef name) {
  if (name.empty()) {
    return std::nullopt;
  }
  return Identifier(std::string(name));
}

// Returns true if the given record (or any of its bases) has an overloaded
// operator delete.
bool OverloadsOperatorDelete(clang::CXXRecordDecl& record_decl) {
  for (const clang::CXXMethodDecl* method : record_decl.methods()) {
    if (method->getOverloadedOperator() == clang::OO_Delete) {
      return true;
    }
  }
  for (const clang::CXXBaseSpecifier& base : record_decl.bases()) {
    if (OverloadsOperatorDelete(*base.getType()->getAsCXXRecordDecl())) {
      return true;
    }
  }
  return false;
}

// Returns true if the given record is, or could have, an overloaded operator
// delete.
//
// Note: we implement this here, instead of as a query, so that this works even
// in the face of ambiguous bases, bases that don't get bindings, methods
// that don't get bindings, etc.
bool MayOverloadOperatorDelete(clang::CXXRecordDecl& record_decl) {
  // If it has a virtual destructor, then operator delete may be
  // overloaded by a derived class, regardless of whether it is defined
  // in the base.
  // NOTE: we may want to relax this in the case of e.g. std::vector<T> for
  // non-final T.
  if (record_decl.getDestructor()->isVirtual() &&
      !record_decl.isEffectivelyFinal()) {
    return true;
  }
  return OverloadsOperatorDelete(record_decl);
}

// Returns the name of this DeclContext if it is a top-level namespace,
// otherwise std::nullopt.
std::optional<llvm::StringRef> AsTopLevelNamespace(
    const clang::DeclContext* context) {
  if (!context->isNamespace()) {
    return std::nullopt;
  }

  const auto* namespace_decl = clang::cast<clang::NamespaceDecl>(context);
  if (namespace_decl->isInline()) {
    return AsTopLevelNamespace(namespace_decl->getParent());
  }

  if (!context->getParent()->getRedeclContext()->isTranslationUnit()) {
    return std::nullopt;
  }

  const clang::IdentifierInfo* identifier_info =
      namespace_decl->getIdentifier();
  if (!identifier_info) {
    return std::nullopt;
  }
  return identifier_info->getName();
}

// Checks that a ClassTemplateSpecializationDecl has template arguments of the
// form `<T, std::std_trait_name<T>>`. If so, returns `T`.
//
// Examples:
//
// ```
// ParameterizedByTAndStdTraitT(std::vector<T, std::allocator<T>>,
//                              "allocator") -> T
// ParameterizedByTAndStdTraitT(std::unique_ptr<T, std::default_delete<T>>,
//                              "default_delete") -> T
//
// ParameterizedByTAndStdTraitT(std::basic_string_view<T, something_else>,
//                              "char_traits") -> error
// ```
absl::StatusOr<clang::QualType> ParameterizedByTAndStdTraitT(
    ImportContext& ictx,
    const clang::ClassTemplateSpecializationDecl* spec_decl,
    llvm::StringRef std_trait_name) {
  const clang::TemplateArgumentList& args = spec_decl->getTemplateArgs();
  if (args.size() != 2) {
    return absl::InvalidArgumentError(
        absl::StrCat("Template specialization '", spec_decl->getNameAsString(),
                     "' should have two template arguments"));
  }

  clang::QualType t = args[0].getAsType();
  clang::QualType std_trait_t = args[1].getAsType();

  const auto* std_trait_spec_decl =
      clang::dyn_cast_or_null<clang::ClassTemplateSpecializationDecl>(
          std_trait_t->getAsCXXRecordDecl());
  if (std_trait_spec_decl == nullptr ||
      std_trait_spec_decl->getTemplateArgs().size() != 1) {
    return absl::InvalidArgumentError(
        absl::StrCat("Template specialization '", std_trait_t.getAsString(),
                     "' should have one template argument"));
  }

  const clang::CXXRecordDecl* std_trait_templated_decl =
      std_trait_spec_decl->getSpecializedTemplate()->getTemplatedDecl();
  if (std_trait_templated_decl == nullptr ||
      std_trait_templated_decl->getName() != std_trait_name ||
      !std_trait_templated_decl->getDeclContext()->isStdNamespace()) {
    return absl::InvalidArgumentError(
        absl::StrCat("Template specialization '", std_trait_t.getAsString(),
                     "' is not a specialization of std::",
                     std::string_view(std_trait_name)));
  }

  const clang::TemplateArgument& arg =
      std_trait_spec_decl->getTemplateArgs()[0];
  LOG_IF(FATAL, arg.getKind() != clang::TemplateArgument::Type)
      << "Expected type template argument on type trait '"
      << std::string_view(std_trait_name) << "'";
  if (!ictx.ctx_.hasSameType(arg.getAsType(), t)) {
    return absl::InvalidArgumentError(
        absl::StrCat("Template argument '", t.getAsString(),
                     "' does not match template argument '",
                     arg.getAsType().getAsString(), "'"));
  }

  return t;
}

absl::StatusOr<TemplateSpecialization::Kind> GetTemplateSpecializationKind(
    ImportContext& ictx,
    const clang::ClassTemplateSpecializationDecl* specialization_decl) {
  const clang::CXXRecordDecl* templated_decl =
      specialization_decl->getSpecializedTemplate()->getTemplatedDecl();

  std::optional<llvm::StringRef> top_level_namespace =
      AsTopLevelNamespace(templated_decl->getDeclContext());
  if (top_level_namespace == "std") {
    if (templated_decl->getName() == "basic_string_view") {
      CRUBIT_ASSIGN_OR_RETURN(clang::QualType t,
                              ParameterizedByTAndStdTraitT(
                                  ictx, specialization_decl, "char_traits"));
      if (t->isCharType()) {
        return TemplateSpecialization::StdStringView();
      } else if (t->isWideCharType()) {
        return TemplateSpecialization::StdWStringView();
      }
      // Other character types do not get special support.
    } else if (templated_decl->getName() == "unique_ptr") {
      CRUBIT_ASSIGN_OR_RETURN(clang::QualType t,
                              ParameterizedByTAndStdTraitT(
                                  ictx, specialization_decl, "default_delete"));
      return TemplateSpecialization::StdUniquePtr(
          // TODO(b/454627672): is specialization_decl the right decl to check
          // for assumed_lifetimes?
          TemplateArg(ictx.ConvertQualType(
              t, /*lifetimes=*/nullptr, /*nullable=*/true,
              ictx.AreAssumedLifetimesEnabledForTarget(
                  ictx.GetOwningTarget(specialization_decl)))));
    } else if (templated_decl->getName() == "vector") {
      CRUBIT_ASSIGN_OR_RETURN(
          clang::QualType t,
          ParameterizedByTAndStdTraitT(ictx, specialization_decl, "allocator"));
      // TODO(b/454627672): is specialization_decl the right decl to check for
      // assumed_lifetimes?
      return TemplateSpecialization::StdVector(TemplateArg(ictx.ConvertQualType(
          t, /*lifetimes=*/nullptr, /*nullable=*/true,
          ictx.AreAssumedLifetimesEnabledForTarget(
              ictx.GetOwningTarget(specialization_decl)))));
    }
  } else if (top_level_namespace == "absl") {
    if (templated_decl->getName() == "Span") {
      LOG_IF(FATAL, specialization_decl->getTemplateArgs().size() != 1)
          << "absl::Span should have one template arg";
      clang::QualType t = specialization_decl->getTemplateArgs()[0].getAsType();
      // TODO(b/454627672): is specialization_decl the right decl to check for
      // assumed_lifetimes?
      return TemplateSpecialization::AbslSpan(TemplateArg(ictx.ConvertQualType(
          t,
          /*lifetimes=*/nullptr, /*nullable=*/true,
          ictx.AreAssumedLifetimesEnabledForTarget(
              ictx.GetOwningTarget(specialization_decl)))));
    }
  } else if (top_level_namespace == "c9") {
    if (templated_decl->getName() == "Co") {
      if (specialization_decl->getTemplateArgs().size() != 1) {
        return absl::InvalidArgumentError(
            "c9::Co should have one template arg");
      }
      // TODO(b/454627672): is specialization_decl the right decl to check for
      // assumed_lifetimes?
      clang::QualType t = specialization_decl->getTemplateArgs()[0].getAsType();
      // Check that t is completable, or void (which is always incomplete).
      if (!t->isVoidType() &&
          !ictx.sema_.isCompleteType(specialization_decl->getLocation(), t)) {
        return absl::InvalidArgumentError(absl::StrCat(
            "c9::Co return type is incomplete: ", t.getAsString()));
      }
      return TemplateSpecialization::C9Co(TemplateArg(ictx.ConvertQualType(
          t,
          /*lifetimes=*/nullptr, /*nullable=*/true,
          ictx.AreAssumedLifetimesEnabledForTarget(
              ictx.GetOwningTarget(specialization_decl)))));
    }
  }

  return TemplateSpecialization::NonSpecial();
}

// Returns the `DynCallable` information for the given `specialization_decl`.
//
// If the given `specialization_decl` is not a `rs_std::DynCallable`, returns
// `std::nullopt`. If it is a `rs_std::DynCallable` but has other errors,
// returns an error.
std::optional<absl::StatusOr<BridgeType>> ExtractCallable(
    ImportContext& ictx,
    const clang::ClassTemplateSpecializationDecl& specialization_decl) {
  const clang::CXXRecordDecl* templated_decl =
      specialization_decl.getSpecializedTemplate()->getTemplatedDecl();

  auto top_level_namespace =
      AsTopLevelNamespace(templated_decl->getDeclContext());
  BridgeType::Callable::BackingType backing_type;
  if (top_level_namespace == "rs_std" &&
      templated_decl->getName() == "DynCallable") {
    backing_type = BridgeType::Callable::BackingType::kDynCallable;
  } else if (top_level_namespace == "absl" &&
             templated_decl->getName() == "AnyInvocable") {
    backing_type = BridgeType::Callable::BackingType::kAnyInvocable;
  } else {
    return std::nullopt;
  }

  if (specialization_decl.getTemplateArgs().size() != 1) {
    return absl::InvalidArgumentError(
        "Callable template specialization must have exactly one template "
        "argument");
  }
  const clang::FunctionProtoType* sig_fn_type =
      specialization_decl.getTemplateArgs()
          .get(0)
          .getAsType()
          .getTypePtr()
          ->getAs<clang::FunctionProtoType>();

  if (sig_fn_type == nullptr) {
    return absl::InvalidArgumentError(
        "Failed to get function signature for DynCallable");
  }

  // Extract the function kind based on the qualifiers.
  BridgeType::Callable::FnTrait fn_trait;
  if (sig_fn_type->getRefQualifier() == clang::RQ_RValue) {
    // Regardless of whether it's && or const &&, it's a FnOnce.
    fn_trait = BridgeType::Callable::FnTrait::kFnOnce;
  } else if (sig_fn_type->getMethodQuals().hasConst()) {
    fn_trait = BridgeType::Callable::FnTrait::kFn;
  } else {
    fn_trait = BridgeType::Callable::FnTrait::kFnMut;
  }

  // Convert the return type, ensuring that it is complete first.
  if (sig_fn_type->getReturnType()->isIncompleteType()) {
    // void is always considered incomplete, but is valid.
    bool ok = sig_fn_type->getReturnType()->isVoidType() ||
              ictx.sema_.isCompleteType(specialization_decl.getLocation(),
                                        sig_fn_type->getReturnType());
    if (!ok) {
      return absl::InvalidArgumentError(
          absl::StrCat("Return type of callable is incomplete: ",
                       sig_fn_type->getReturnType().getAsString()));
    }
  }
  // TODO(b/454627672): is templated_decl the right decl to check for
  // assumed_lifetimes?
  CcType return_type =
      ictx.ConvertQualType(sig_fn_type->getReturnType(),
                           /*lifetimes=*/nullptr,
                           /*nullable=*/true,
                           ictx.AreAssumedLifetimesEnabledForTarget(
                               ictx.GetOwningTarget(templated_decl)));

  std::vector<CcType> param_types;
  // Convert the parameter types, ensuring that they are complete first.
  param_types.reserve(sig_fn_type->getNumParams());
  for (clang::QualType param_type : sig_fn_type->getParamTypes()) {
    if (param_type->isIncompleteType()) {
      bool ok = ictx.sema_.isCompleteType(specialization_decl.getLocation(),
                                          param_type);
      if (!ok) {
        return absl::InvalidArgumentError(
            absl::StrCat("Parameter type of callable is incomplete: ",
                         param_type.getAsString()));
      }
    }
    // TODO(b/454627672): is specialization_decl the right decl to check for
    // assumed_lifetimes?
    CcType param_cc_type =
        ictx.ConvertQualType(param_type, /*lifetimes=*/nullptr,
                             /*nullable=*/true,
                             ictx.AreAssumedLifetimesEnabledForTarget(
                                 ictx.GetOwningTarget(&specialization_decl)));
    param_types.push_back(std::move(param_cc_type));
  }

  return BridgeType(BridgeType::Callable{
      .backing_type = backing_type,
      .fn_trait = fn_trait,
      .return_type = std::make_shared<CcType>(std::move(return_type)),
      .param_types = std::move(param_types),
  });
}

}  // namespace

std::optional<Identifier> CXXRecordDeclImporter::GetTranslatedFieldName(
    const clang::FieldDecl* field_decl) {
  if (field_decl->getName().empty()) {
    CHECK(!field_decl->hasAttr<clang::NoUniqueAddressAttr>() &&
          "Unnamed fields can't be annotated with [[no_unique_address]]");
    // We don't just conjure an artificial name for an unnamed field, because
    // in the future such fields may be elided entirely - see unnamed members
    // in:
    // - https://en.cppreference.com/w/c/language/struct
    // - https://rust-lang.github.io/rfcs/2102-unnamed-fields.html
    return std::nullopt;
  }

  absl::StatusOr<TranslatedIdentifier> name =
      ictx_.GetTranslatedIdentifier(field_decl);
  if (!name.ok()) {
    unsigned field_pos = field_decl->getFieldIndex();
    return {Identifier(absl::StrCat("__field_", field_pos))};
  }
  return (*name).rs_identifier();
}

bool IsKnownAttr(const clang::Attr& attr) {
  return clang::isa<clang::AlignedAttr>(attr) ||
         clang::isa<clang::CoroAwaitElidableAttr>(attr) ||
         clang::isa<clang::CoroLifetimeBoundAttr>(attr) ||
         clang::isa<clang::CoroOnlyDestroyWhenCompleteAttr>(attr) ||
         clang::isa<clang::CoroReturnTypeAttr>(attr) ||
         clang::isa<clang::FinalAttr>(attr) ||
         clang::isa<clang::OwnerAttr>(attr) ||
         clang::isa<clang::PointerAttr>(attr) ||
         clang::isa<clang::PreferredNameAttr>(attr) ||
         clang::isa<clang::TrivialABIAttr>(attr) ||
         clang::isa<clang::WarnUnusedResultAttr>(attr) ||
         clang::isa<clang::TypeNullableAttr>(attr) ||
         clang::isa<clang::ScopedLockableAttr>(attr) ||
         clang::isa<clang::CapabilityAttr>(attr) ||
         clang::isa<clang::ReentrantCapabilityAttr>(attr);
}

std::optional<IR::Item> CXXRecordDeclImporter::Import(
    clang::CXXRecordDecl* record_decl) {
  const clang::DeclContext* decl_context = record_decl->getDeclContext();
  if (decl_context->isFunctionOrMethod()) {
    return std::nullopt;
  }
  if (ictx_.HasBeenAlreadySuccessfullyImported(record_decl)) {
    LOG(FATAL)
        << ("THIS IS A BUG: the type was marked as imported, so we "
            "short-circuited evaluation here. However, instead of the fully "
            "imported type being used, apparently this empty stub was used "
            "instead. Report this upstream.");
  }
  if (record_decl->isInjectedClassName()) {
    return std::nullopt;
  }
  if (record_decl->isImplicit()) {
    return std::nullopt;
  }
  if (clang::isa<clang::ClassTemplatePartialSpecializationDecl>(record_decl)) {
    return ictx_.ImportUnsupportedItem(
        *record_decl, std::nullopt,
        FormattedError::Static(
            "Partially-specialized class templates are not supported"));
  }

  if (record_decl->isInvalidDecl()) {
    return std::nullopt;
  }

  std::optional<IR::Item> attr_error_item;
  absl::StatusOr<std::optional<std::string>> unknown_attr =
      CollectUnknownAttrs(*record_decl, [&](const clang::Attr& attr) {
        if (IsKnownAttr(attr)) {
          return true;
        } else if (auto* visibility =
                       clang::dyn_cast<clang::VisibilityAttr>(&attr);
                   visibility && record_decl->isInStdNamespace()) {
          if (visibility->getVisibility() ==
              clang::VisibilityAttr::VisibilityType::Hidden) {
            attr_error_item = ictx_.ImportUnsupportedItem(
                *record_decl, std::nullopt,
                FormattedError::Static("Records from the standard library with "
                                       "hidden visibility are not supported"));
          }
          return true;
        }
        return false;
      });
  if (!unknown_attr.ok()) {
    return ictx_.ImportUnsupportedItem(
        *record_decl, std::nullopt,
        FormattedError::FromStatus(std::move(unknown_attr).status()));
  }
  if (attr_error_item.has_value()) {
    return attr_error_item;
  }

  std::string rs_name, cc_name;
  clang::SourceLocation source_loc;
  std::optional<std::string> doc_comment;
  bool is_explicit_class_template_instantiation_definition = false;
  std::optional<TemplateSpecialization> template_specialization;
  std::optional<BridgeType> bridge_type =
      GetBridgeTypeAnnotation(ictx_, *record_decl);

  absl::StatusOr<std::optional<std::string>> owned_ptr_type =
      GetAnnotationWithStringArg(*record_decl, "crubit_owned_pointee");
  if (!owned_ptr_type.ok()) {
    return ictx_.ImportUnsupportedItem(
        *record_decl, std::nullopt,
        FormattedError::FromStatus(std::move(owned_ptr_type).status()));
  }

  BazelLabel owning_target = ictx_.GetOwningTarget(record_decl);
  if (auto* specialization_decl =
          clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(
              record_decl)) {
    is_explicit_class_template_instantiation_definition =
        specialization_decl->getSpecializationKind() ==
        clang::TSK_ExplicitInstantiationDefinition;
    rs_name = ictx_.GetMangledName(specialization_decl);
    absl::StatusOr<std::string> status_or_cc_name =
        CcName(ictx_.ctx_, specialization_decl);
    if (!status_or_cc_name.ok()) {
      return ictx_.ImportUnsupportedItem(
          *record_decl, std::nullopt,
          FormattedError::FromStatus(std::move(status_or_cc_name).status()));
    }
    cc_name = *std::move(status_or_cc_name);

    TemplateSpecialization& ts = template_specialization.emplace();

    absl::StatusOr<TemplateSpecialization::Kind> status_or_ts_kind =
        GetTemplateSpecializationKind(ictx_, specialization_decl);
    if (!status_or_ts_kind.ok()) {
      return ictx_.ImportUnsupportedItem(
          *record_decl, std::nullopt,
          FormattedError::FromStatus(std::move(status_or_ts_kind).status()));
    }
    ts.kind = *std::move(status_or_ts_kind);

    doc_comment = ictx_.GetComment(specialization_decl);
    if (!doc_comment.has_value()) {
      doc_comment =
          ictx_.GetComment(specialization_decl->getSpecializedTemplate());
    }
    source_loc = specialization_decl->getBeginLoc();
    // Specify defining_target if it's a template instantiation.
    auto instantiation_source =
        specialization_decl->getSpecializedTemplateOrPartial();
    clang::NamedDecl* decl;
    if (auto* template_decl =
            instantiation_source.dyn_cast<clang::ClassTemplateDecl*>()) {
      // `getSpecializedTemplateOrPartial()` can return a ClassTemplateDecl
      // corresponding to a forward declaration, even if a definition is
      // available elsewhere. If we have a forward declaration, we need to
      // navigate to the definition's ClassTemplateDecl to ensure we generate
      // bindings against the full definition of the template instead of just
      // the forward declaration.
      if (clang::CXXRecordDecl* definition =
              template_decl->getTemplatedDecl()->getDefinition()) {
        if (auto* definition_template_decl =
                definition->getDescribedClassTemplate()) {
          template_decl = definition_template_decl;
        }
      }
      decl = template_decl;
    } else {
      decl = instantiation_source
                 .dyn_cast<clang::ClassTemplatePartialSpecializationDecl*>();
    }
    ts.defining_target = ictx_.GetOwningTarget(decl);
    // TODO(okabayashi): File a bug for generalizing "canonical insts".
    // When a template like `std::string_view` is instantiated, it will be
    // owned by whatever target it was instantiated in. The C++ compiler is
    // then responsible for unifying identical instantiations. However, this
    // is a pain for Crubit because we aren't able to generally unify these.
    // In the case of `std::string_view`, however, we know that there's an
    // instantiation in the `cc_std` target, so I've chosen that as the
    // canonical instantiation, and am mapping all other instantiations to
    // that instantiation.
    // A problem with this is it's not _actually_ the same ItemId, and it
    // really should be. This ensures that when we refer to this Item, it's
    // spelled `cc_std::__CcTemplateInst...`. But a major downside is that we
    // still generate this template inst struct...
    if (std::holds_alternative<TemplateSpecialization::StdStringView>(
            ts.kind)) {
      owning_target = ts.defining_target;
    }

    if (std::optional<absl::StatusOr<BridgeType>> extracted_callable =
            ExtractCallable(ictx_, *specialization_decl)) {
      if (!extracted_callable->ok()) {
        return ictx_.ImportUnsupportedItem(
            *record_decl, std::nullopt,
            FormattedError::FromStatus(
                std::move(extracted_callable)->status()));
      }
      bridge_type = **std::move(extracted_callable);
    }

    if (!bridge_type.has_value()) {
      absl::StatusOr<std::optional<BridgeType>> builtin_bridge_type =
          GetBuiltinBridgeType(specialization_decl);
      if (!builtin_bridge_type.ok()) {
        return ictx_.ImportUnsupportedItem(
            *record_decl, std::nullopt,
            FormattedError::FromStatus(
                std::move(builtin_bridge_type).status()));
      }
      bridge_type = *std::move(builtin_bridge_type);
    }
  } else {
    const clang::NamedDecl* named_decl = record_decl;
    if (record_decl->getName().empty()) {
      if (auto* typedef_decl = record_decl->getTypedefNameForAnonDecl()) {
        named_decl = typedef_decl;
      } else {
        // Skip anonymous structs that don't get a name via typedecl.
        return std::nullopt;
      }
    }
    CHECK(!named_decl->getName().empty());

    absl::StatusOr<TranslatedIdentifier> record_name =
        ictx_.GetTranslatedIdentifier(named_decl);
    if (!record_name.ok()) {
      return ictx_.ImportUnsupportedItem(
          *record_decl, std::nullopt,
          FormattedError::PrefixedStrCat("Record name is not supported",
                                         record_name.status().message()));
    }
    rs_name = record_name->rs_identifier().Ident();
    cc_name = record_name->cc_identifier.Ident();
    doc_comment = ictx_.GetComment(record_decl);
    source_loc = record_decl->getBeginLoc();
  }

  auto enclosing_item_id = ictx_.GetEnclosingItemId(record_decl);
  if (!enclosing_item_id.ok()) {
    return ictx_.ImportUnsupportedItem(
        *record_decl, std::nullopt,
        FormattedError::FromStatus(std::move(enclosing_item_id).status()));
  }

  // Reports an unsupported type with the given error.
  //
  // This is preferred to invoking `ImportUnsupportedItem` directly because it
  // ensures that the path is set correctly. Note that this cannot be used above
  // because the enclosing item ID and translated name are not yet available.
  auto unsupported = [this, &record_decl, &rs_name,
                      &enclosing_item_id](FormattedError error) {
    return ictx_.ImportUnsupportedItem(
        *record_decl,
        UnsupportedItem::Path{.ident = Identifier(rs_name),
                              .enclosing_item_id = *enclosing_item_id},
        error);
  };

  if (record_decl->isDependentContext()) {
    // We can't pass this to getASTRecordLayout() or it'll segfault.
    // TODO(jeanpierreda): investigate what we can do to support dependent records?
    // All I know is that I saw other code calling getASTRecordLayout() do the
    // same check. But getASTRecordLayout() itself doesn't actually document
    // this.
    return unsupported(
        FormattedError::Static("Dependent records are not supported"));
  }

  absl::StatusOr<RecordType> record_type = TranslateRecordType(*record_decl);
  if (!record_type.ok()) {
    return unsupported(
        FormattedError::FromStatus(std::move(record_type).status()));
  }

  if (record_decl->hasAttr<clang::PackedAttr>() ||
      std::any_of(record_decl->field_begin(), record_decl->field_end(),
                  [](const clang::FieldDecl* field_decl) {
                    return field_decl->hasAttr<clang::PackedAttr>();
                  })) {
    return unsupported(
        FormattedError::Static("Records with packed layout are not supported"));
  }

  ictx_.MarkAsSuccessfullyImported(record_decl);
  if (!record_decl->isCompleteDefinition()) {
    return IncompleteRecord{.cc_name = Identifier(cc_name),
                            .rs_name = Identifier(rs_name),
                            .unique_name = ictx_.GetUniqueName(*record_decl),
                            .id = ictx_.GenerateItemId(record_decl),
                            .owning_target = std::move(owning_target),
                            .unknown_attr = *std::move(unknown_attr),
                            .record_type = *std::move(record_type),
                            .enclosing_item_id = *std::move(enclosing_item_id)};
  }

  ictx_.sema_.ForceDeclarationOfImplicitMembers(record_decl);

  const clang::ASTRecordLayout& layout =
      ictx_.ctx_.getASTRecordLayout(record_decl);

  bool is_derived_class = record_decl->getNumBases() != 0;
  bool override_alignment = record_decl->hasAttr<clang::AlignedAttr>() ||
                            is_derived_class || layout.hasOwnVFPtr();

  bool is_effectively_final =
      record_decl->isEffectivelyFinal() || record_decl->isUnion();

  std::optional<std::string> nodiscard;
  if (const auto* attr = record_decl->getAttr<clang::WarnUnusedResultAttr>();
      attr != nullptr) {
    nodiscard.emplace(attr->getMessage());
  }

  auto item_ids = ictx_.GetItemIdsInSourceOrder(record_decl);
  const clang::TypedefNameDecl* anon_typedef =
      record_decl->getTypedefNameForAnonDecl();

  absl::StatusOr<TraitDerives> trait_derives = GetTraitDerives(*record_decl);
  if (!trait_derives.ok()) {
    return unsupported(
        FormattedError::FromStatus(std::move(trait_derives).status()));
  }

  absl::StatusOr<SafetyAnnotation> safety_annotation =
      GetSafetyAnnotation(*record_decl);
  if (!safety_annotation.ok()) {
    return unsupported(
        FormattedError::FromStatus(std::move(safety_annotation).status()));
  }

  auto record = Record{
      .rs_name = Identifier(rs_name),
      .cc_name = Identifier(cc_name),
      .unique_name = ictx_.GetUniqueName(*record_decl),
      .mangled_cc_name = ictx_.GetMangledName(record_decl),
      .id = ictx_.GenerateItemId(record_decl),
      .owning_target = std::move(owning_target),
      .template_specialization = std::move(template_specialization),
      .unknown_attr = std::move(*unknown_attr),
      .doc_comment = std::move(doc_comment),
      .bridge_type = std::move(bridge_type),
      .owned_ptr_type = *std::move(owned_ptr_type),
      .source_loc = ictx_.ConvertSourceLocation(source_loc),
      .unambiguous_public_bases = GetUnambiguousPublicBases(*record_decl),
      .fields = ImportFields(record_decl),
      .size_align =
          {
              .size = layout.getSize().getQuantity(),
              .alignment = layout.getAlignment().getQuantity(),
          },
      .trait_derives = *std::move(trait_derives),
      .is_derived_class = is_derived_class,
      .override_alignment = override_alignment,
      .safety_annotation = *safety_annotation,
      .copy_constructor = GetCopyCtorSpecialMemberFunc(ictx_, *record_decl),
      .move_constructor = GetMoveCtorSpecialMemberFunc(ictx_, *record_decl),
      .destructor = GetDestructorSpecialMemberFunc(*record_decl),
      .is_trivial_abi = record_decl->canPassInRegisters(),
      .is_inheritable = !is_effectively_final,
      .is_abstract = record_decl->isAbstract(),
      .nodiscard = std::move(nodiscard),
      .record_type = *record_type,
      .is_aggregate = record_decl->isAggregate(),
      .is_anon_record_with_typedef = anon_typedef != nullptr,
      .is_explicit_class_template_instantiation_definition =
          is_explicit_class_template_instantiation_definition,
      .child_item_ids = std::move(item_ids),
      .enclosing_item_id = *std::move(enclosing_item_id),
      .overloads_operator_delete = MayOverloadOperatorDelete(*record_decl),
  };

  // If the align attribute was attached to the typedef decl, we should
  // apply it to the generated record.
  //
  // TODO(jeanpierreda): We also need this logic for non-anonymous structs, where we
  // instead copy the struct into a new decl with this typedef's decl id.
  // So this part probably needs to be factored out somewhere that
  // typedef_name.cc can get at it.
  if (anon_typedef != nullptr) {
    auto* aligned = anon_typedef->getAttr<clang::AlignedAttr>();
    if (aligned) {
      int64_t& size = record.size_align.size;
      int64_t& alignment = record.size_align.alignment;
      alignment =
          ictx_.ctx_.toCharUnitsFromBits(aligned->getAlignment(ictx_.ctx_))
              .getQuantity();
      record.override_alignment = true;

      // If it has alignment, update the `record->size` to the aligned
      // one, because that size is going to be used as this record's
      // canonical size in IR and in the binding code.

      // Make sure that `alignment` is a power of 2.
      CHECK(!(alignment & (alignment - 1)));

      // Given that `alignment` is a power of 2, we can round it up by
      // a bit arithmetic: `alignment - 1` clears the single bit of it
      // while turning all the zeros in the right to 1s. Adding
      // `alignment - 1` and doing &~ with it effectively rounds it up
      // to the next multiple of the alignment.
      size = (size + alignment - 1) & ~(alignment - 1);
    }
  }
  return record;
}

std::vector<Field> CXXRecordDeclImporter::ImportFields(
    clang::CXXRecordDecl* record_decl) {
  clang::AccessSpecifier default_access =
      record_decl->isClass() ? clang::AS_private : clang::AS_public;
  std::vector<Field> fields;
  const clang::ASTRecordLayout& layout =
      ictx_.ctx_.getASTRecordLayout(record_decl);
  for (const clang::FieldDecl* field_decl : record_decl->fields()) {
    clang::AccessSpecifier access = field_decl->getAccess();
    if (access == clang::AS_none) {
      access = default_access;
    }

    const clang::tidy::lifetimes::ValueLifetimes* no_lifetimes = nullptr;
    CcType type = [&]() {
      switch (access) {
        case clang::AS_public:
          // TODO(mboehme): Once lifetime_annotations supports retrieving
          // lifetimes in field types, pass these to ConvertQualType().
          // TODO(b/454627672): is record_decl the right decl to check for
          // assumed_lifetimes?
          return ictx_.ConvertQualType(
              field_decl->getType(), no_lifetimes,
              /*nullable=*/true,
              ictx_.AreAssumedLifetimesEnabledForTarget(
                  ictx_.GetOwningTarget(record_decl)));
        case clang::AS_protected:
        case clang::AS_private:
        case clang::AS_none:
          // As a performance optimization (i.e. to keep the generated code
          // small) we can emit private fields as opaque blobs of bytes.  This
          // may avoid the need to include supporting types in the generated
          // code (e.g. avoiding extra template instantiations).  See also
          // b/226580208 and <internal link>.
          return CcType(FormattedError::Static(
              "Types of non-public C++ fields can be elided away"));
      }
    }();

    bool is_inheritable = false;
    auto* field_record = field_decl->getType()->getAsCXXRecordDecl();
    if (field_record) {
      // If it is a record as a direct member, its item must be already
      // imported.
      auto item = ictx_.GetImportedItem(field_record);
      if (item.has_value()) {
        if (const auto* record = std::get_if<Record>(&item.value())) {
          is_inheritable = record->is_inheritable;
        }
      }
    }

    uint64_t size;
    if (field_decl->isZeroSize(ictx_.ctx_)) {
      size = 0;
    } else if (field_decl->isBitField()) {
      size = field_decl->getBitWidthValue();
    } else {
      size = ictx_.ctx_.getTypeSize(field_decl->getType());
    }

    fields.push_back(
        {.rust_identifier = GetTranslatedFieldName(field_decl),
         .cpp_identifier = StringRefToOptionalIdentifier(field_decl->getName()),
         .doc_comment = ictx_.GetComment(field_decl),
         .type = std::move(type),
         .access = TranslateAccessSpecifier(access),
         .offset = layout.getFieldOffset(field_decl->getFieldIndex()),
         .size = size,
         .unknown_attr = CollectUnknownAttrs(*field_decl),
         .is_no_unique_address =
             field_decl->hasAttr<clang::NoUniqueAddressAttr>(),
         .is_bitfield = field_decl->isBitField(),
         .is_inheritable = is_inheritable});
  }
  return fields;
}

std::vector<BaseClass> CXXRecordDeclImporter::GetUnambiguousPublicBases(
    const clang::CXXRecordDecl& record_decl) const {
  // This function is unfortunate: the only way to correctly get information
  // about the bases is lookupInBases. It runs a complex O(N^3) algorithm for
  // e.g. correctly determining virtual base paths, etc.
  //
  // However, lookupInBases does not recurse into a class once it's found.
  // So we need to call lookupInBases once per class, making this O(N^4).

  llvm::SmallPtrSet<const clang::CXXRecordDecl*, 4> seen;
  std::vector<BaseClass> bases;
  clang::CXXBasePaths paths;
  // the const cast is a common pattern, apparently, see e.g.
  // https://clang.llvm.org/doxygen/CXXInheritance_8cpp_source.html#l00074
  paths.setOrigin(const_cast<clang::CXXRecordDecl*>(&record_decl));

  auto next_class = [&]() {
    const clang::CXXRecordDecl* found = nullptr;

    // Matches the first new class it encounters (and adds it to `seen`, so
    // that future runs don't rediscover it.)
    auto is_new_class = [&](const clang::CXXBaseSpecifier* base_specifier,
                            clang::CXXBasePath&) {
      const auto* record_decl = base_specifier->getType()->getAsCXXRecordDecl();
      if (found) {
        return record_decl == found;
      }

      if (record_decl && seen.insert(record_decl).second) {
        found = record_decl;
        return true;
      }
      return false;
    };
    return record_decl.lookupInBases(is_new_class, paths);
  };

  for (; next_class(); paths.clear()) {
    for (const clang::CXXBasePath& path : paths) {
      if (path.Access != clang::AS_public) {
        continue;
      }
      const clang::CXXBaseSpecifier& base_specifier =
          *path[path.size() - 1].Base;
      const clang::QualType& base = base_specifier.getType();
      if (paths.isAmbiguous(ictx_.ctx_.getCanonicalType(base))) {
        continue;
      }

      clang::CXXRecordDecl* base_record_decl =
          ABSL_DIE_IF_NULL(base_specifier.getType()->getAsCXXRecordDecl());
      if (!ictx_.HasBeenAlreadySuccessfullyImported(base_record_decl)) {
        continue;
      }

      std::optional<int64_t> offset = {0};
      for (const clang::CXXBasePathElement& base_path_element : path) {
        if (base_path_element.Base->isVirtual()) {
          offset.reset();
          break;
        }
        *offset +=
            ictx_.ctx_.getASTRecordLayout(base_path_element.Class)
                .getBaseClassOffset(ABSL_DIE_IF_NULL(
                    base_path_element.Base->getType()->getAsCXXRecordDecl()))
                .getQuantity();
      }
      CHECK((!offset.has_value() || *offset >= 0) &&
            "Concrete base classes should have non-negative offsets.");
      bases.push_back(
          BaseClass{.base_record_id = ictx_.GenerateItemId(base_record_decl),
                    .offset = offset});
      break;
    }
  }
  return bases;
}

absl::StatusOr<std::optional<BridgeType>>
CXXRecordDeclImporter::GetBuiltinBridgeType(
    const clang::ClassTemplateSpecializationDecl* decl) {
  const clang::CXXRecordDecl* cxx_record_decl =
      decl->getSpecializedTemplate()->getTemplatedDecl();
  const clang::DeclContext* context = cxx_record_decl->getDeclContext();
  bool is_std_namespace = false;
  while (context) {
    if (context->isStdNamespace()) {
      is_std_namespace = true;
      break;
    }
    context = context->getParent();
  }

  if (!is_std_namespace) {
    return std::nullopt;
  }

  clang::StringRef name = cxx_record_decl->getName();
  // TODO(b/454627672): is cxx_record_decl the right decl to check for
  // assumed_lifetimes?
  auto cc_type_of_arg = [&](int index) {
    return ictx_.ConvertQualType(
        /*qual_type=*/decl->getTemplateArgs()[index].getAsType(),
        /*lifetimes=*/nullptr, /*nullable=*/true,
        ictx_.AreAssumedLifetimesEnabledForTarget(
            ictx_.GetOwningTarget(cxx_record_decl)));
  };

  if (name == "optional") {
    CcType inner = cc_type_of_arg(0);
    return BridgeType{BridgeType::StdOptional{
        .inner_type = std::make_shared<CcType>(std::move(inner)),
    }};
  }

  if (name == "pair") {
    CcType first = cc_type_of_arg(0);
    CcType second = cc_type_of_arg(1);
    return BridgeType{BridgeType::StdPair{
        .first_type = std::make_shared<CcType>(std::move(first)),
        .second_type = std::make_shared<CcType>(std::move(second)),
    }};
  }

  if (name == "basic_string") {
    CcType char_type = cc_type_of_arg(0);
    if (const auto* primitive =
            std::get_if<CcType::Primitive>(&char_type.variant);
        primitive != nullptr && primitive->spelling == "char") {
      return BridgeType{BridgeType::StdString{}};
    }
    // HACK: restoring old behavior that hid a bug in our logic for std::wstring
    // TODO(b/468093766): Fail in the ordinary Record handling logic, not here.
    if (auto* error = std::get_if<FormattedError>(&char_type.variant)) {
      return absl::InternalError(error->message());
    }
  }
  // Add builtin bridge types here as needed.

  return std::nullopt;
}

}  // namespace crubit
