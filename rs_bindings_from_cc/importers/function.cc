// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/function.h"

#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "absl/log/check.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/substitute.h"
#include "lifetime_annotations/lifetime.h"
#include "lifetime_annotations/lifetime_annotations.h"
#include "lifetime_annotations/lifetime_error.h"
#include "lifetime_annotations/lifetime_symbol_table.h"
#include "lifetime_annotations/type_lifetimes.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/bazel_types.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/DeclarationName.h"
#include "clang/AST/Type.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Sema/Sema.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/Support/Error.h"

namespace crubit {

static bool IsInStdNamespace(const clang::FunctionDecl* decl) {
  const clang::DeclContext* context = decl->getDeclContext();
  while (context) {
    if (context->isStdNamespace()) {
      return true;
    }
    context = context->getParent();
  }
  return false;
}

Identifier FunctionDeclImporter::GetTranslatedParamName(
    const clang::ParmVarDecl* param_decl) {
  int param_pos = param_decl->getFunctionScopeIndex();
  absl::StatusOr<Identifier> name = ictx_.GetTranslatedIdentifier(param_decl);
  if (!name.ok()) {
    return {Identifier(absl::StrCat("__param_", param_pos))};
  }
  if (auto* sttpt =
          param_decl->getType()->getAs<clang::SubstTemplateTypeParmType>();
      sttpt && sttpt->getReplacedParameter()->isParameterPack()) {
    // Avoid giving the same name to all parameters expanded from a pack.
    return {Identifier(absl::StrCat("__", name->Ident(), "_", param_pos))};
  }
  return *name;
}

std::optional<IR::Item> FunctionDeclImporter::Import(
    clang::FunctionDecl* function_decl) {
  if (!ictx_.IsFromCurrentTarget(function_decl)) return std::nullopt;
  if (function_decl->isDeleted()) return std::nullopt;

  if (IsInStdNamespace(function_decl)) {
    if (clang::IdentifierInfo* id = function_decl->getIdentifier();
        id != nullptr && id->getName().find("__") != llvm::StringRef::npos) {
      return ictx_.ImportUnsupportedItem(
          function_decl,
          "Internal functions from the standard library are not supported");
    }
    // Disable all member functions except the destructor (which cannot have
    // special requirements) until we can conditionally import them, or disable
    // them on a more fine-grained basis.
    if (clang::FunctionDecl* templated_function_decl =
            function_decl->getInstantiatedFromMemberFunction();
        templated_function_decl != nullptr &&
        !ictx_.IsFromCurrentTarget(templated_function_decl) &&
        templated_function_decl->getDeclName().getNameKind() !=
            clang::DeclarationName::NameKind::CXXDestructorName) {
      return ictx_.ImportUnsupportedItem(
          function_decl,
          "TODO(b/248542210,b/248577708): as a temporary workaround for "
          "un-instantiable function templates, template functions from the STL "
          "cannot be instantiated in user crates");
    }
  }
  // Method is private, we don't need to import it.
  if (auto* method_decl =
          clang::dyn_cast<clang::CXXMethodDecl>(function_decl)) {
    switch (method_decl->getAccess()) {
      case clang::AS_public:
        break;
      case clang::AS_protected:
      case clang::AS_private:
      case clang::AS_none:
        // No need for IR to include Func representing private methods.
        // TODO(lukasza): Revisit this for protected methods.
        return std::nullopt;
    }
  }

  clang::tidy::lifetimes::LifetimeSymbolTable lifetime_symbol_table;
  std::optional<clang::tidy::lifetimes::FunctionLifetimes> lifetimes;
  llvm::Expected<clang::tidy::lifetimes::FunctionLifetimes> lifetimes_or_err =
      clang::tidy::lifetimes::GetLifetimeAnnotations(
          function_decl, *ictx_.invocation_.lifetime_context_,
          &lifetime_symbol_table);
  if (lifetimes_or_err) {
    lifetimes = std::move(*lifetimes_or_err);
  } else {
    using clang::tidy::lifetimes::LifetimeError;
    llvm::Error remaining_err = llvm::handleErrors(
        lifetimes_or_err.takeError(),
        [](std::unique_ptr<LifetimeError> lifetime_err) -> llvm::Error {
          switch (lifetime_err->type()) {
            case LifetimeError::Type::ElisionNotEnabled:
            case LifetimeError::Type::CannotElideOutputLifetimes:
              // If elision is not enabled or output lifetimes cannot be elided,
              // we want to import the function with raw lifetime-less pointers.
              // Just return success here; this will leave the `lifetimes`
              // optional empty, and we will then handle this accordingly below.
              return llvm::Error::success();
              break;
            default:
              return llvm::Error(std::move(lifetime_err));
              break;
          }
        });
    if (remaining_err) {
      return ictx_.ImportUnsupportedItem(
          function_decl, llvm::toString(std::move(remaining_err)));
    }
  }

  absl::StatusOr<UnqualifiedIdentifier> translated_name =
      ictx_.GetTranslatedName(function_decl);
  if (!translated_name.ok()) {
    return ictx_.ImportUnsupportedItem(
        function_decl, absl::StrCat("Function name is not supported: ",
                                    translated_name.status().message()));
  }

  std::vector<FuncParam> params;
  std::set<std::string> errors;
  auto add_error = [&errors](std::string msg) {
    auto result = errors.insert(std::move(msg));
    CHECK(result.second) << "Duplicated error message";
  };
  if (auto* method_decl =
          clang::dyn_cast<clang::CXXMethodDecl>(function_decl)) {
    if (!ictx_.HasBeenAlreadySuccessfullyImported(method_decl->getParent())) {
      return ictx_.ImportUnsupportedItem(function_decl,
                                         "Couldn't import the parent");
    }

    // non-static member functions receive an implicit `this` parameter.
    if (method_decl->isInstance()) {
      const clang::tidy::lifetimes::ValueLifetimes* this_lifetimes = nullptr;
      if (lifetimes) {
        this_lifetimes = &lifetimes->GetThisLifetimes();
      }
      auto param_type =
          ictx_.ConvertQualType(method_decl->getThisType(), this_lifetimes,
                                std::optional<clang::RefQualifierKind>(
                                    method_decl->getRefQualifier()),
                                /*nullable=*/false);
      if (!param_type.ok()) {
        add_error(absl::StrCat("`this` parameter is not supported: ",
                               param_type.status().message()));
      } else {
        params.push_back({*std::move(param_type), Identifier("__this")});
      }
    }
  }

  if (lifetimes) {
    CHECK(lifetimes->IsValidForDecl(function_decl));
  }

  for (unsigned i = 0; i < function_decl->getNumParams(); ++i) {
    const clang::ParmVarDecl* param = function_decl->getParamDecl(i);
    const clang::tidy::lifetimes::ValueLifetimes* param_lifetimes = nullptr;
    if (lifetimes) {
      param_lifetimes = &lifetimes->GetParamLifetimes(i);
    }
    auto param_type =
        ictx_.ConvertQualType(param->getType(), param_lifetimes, std::nullopt);
    if (!param_type.ok()) {
      add_error(absl::Substitute("Parameter #$0 is not supported: $1", i,
                                 param_type.status().message()));
      continue;
    }

    std::optional<Identifier> param_name = GetTranslatedParamName(param);
    CHECK(param_name.has_value());  // No known failure cases.
    params.push_back({*param_type, *std::move(param_name)});
  }

  if (function_decl->getReturnType()->isUndeducedType()) {
    bool still_undeduced = ictx_.sema_.DeduceReturnType(
        function_decl, function_decl->getLocation());
    if (still_undeduced) {
      add_error("Couldn't deduce the return type");
    }
  }

  const clang::tidy::lifetimes::ValueLifetimes* return_lifetimes = nullptr;
  if (lifetimes) {
    return_lifetimes = &lifetimes->GetReturnLifetimes();
  }

  auto return_type = ictx_.ConvertQualType(function_decl->getReturnType(),
                                           return_lifetimes, std::nullopt);
  if (!return_type.ok()) {
    add_error(absl::StrCat("Return type is not supported: ",
                           return_type.status().message()));
  }

  llvm::DenseSet<clang::tidy::lifetimes::Lifetime> all_free_lifetimes;
  if (lifetimes) {
    all_free_lifetimes = lifetimes->AllFreeLifetimes();
  }

  std::vector<LifetimeName> lifetime_params;
  for (clang::tidy::lifetimes::Lifetime lifetime : all_free_lifetimes) {
    std::optional<llvm::StringRef> name =
        lifetime_symbol_table.LookupLifetime(lifetime);
    CHECK(name.has_value());
    lifetime_params.push_back(
        {.name = name->str(), .id = LifetimeId(lifetime.Id())});
  }
  llvm::sort(lifetime_params,
             [](const LifetimeName& l1, const LifetimeName& l2) {
               return l1.name < l2.name;
             });

  std::optional<MemberFuncMetadata> member_func_metadata;
  if (auto* method_decl =
          clang::dyn_cast<clang::CXXMethodDecl>(function_decl)) {
    std::optional<MemberFuncMetadata::InstanceMethodMetadata> instance_metadata;
    if (method_decl->isInstance()) {
      MemberFuncMetadata::ReferenceQualification reference;
      switch (method_decl->getRefQualifier()) {
        case clang::RQ_LValue:
          reference = MemberFuncMetadata::kLValue;
          break;
        case clang::RQ_RValue:
          reference = MemberFuncMetadata::kRValue;
          break;
        case clang::RQ_None:
          reference = MemberFuncMetadata::kUnqualified;
          break;
      }
      instance_metadata = MemberFuncMetadata::InstanceMethodMetadata{
          .reference = reference,
          .is_const = method_decl->isConst(),
          .is_virtual = method_decl->isVirtual(),
      };
    }

    member_func_metadata = MemberFuncMetadata{
        .record_id = GenerateItemId(method_decl->getParent()),
        .instance_method_metadata = instance_metadata};
  }

  if (!errors.empty()) {
    return ictx_.ImportUnsupportedItem(function_decl, errors);
  }

  bool has_c_calling_convention =
      function_decl->getType()->getAs<clang::FunctionType>()->getCallConv() ==
      clang::CC_C;
  bool is_member_or_descendant_of_class_template =
      IsFullClassTemplateSpecializationOrChild(function_decl);

  std::optional<std::string> doc_comment = ictx_.GetComment(function_decl);
  if (!doc_comment.has_value() && is_member_or_descendant_of_class_template) {
    // Despite `is_member_or_descendant_of_class_template` check above, we are
    // not guaranteed that a `func_pattern` exists below.  For example, it may
    // be missing when `function_decl` is an implicitly defined constructor of a
    // class template -- such decls are generated, not instantiated.
    if (clang::FunctionDecl* func_pattern =
            function_decl->getTemplateInstantiationPattern()) {
      doc_comment = ictx_.GetComment(func_pattern);
    }
  }

  std::string mangled_name = ictx_.GetMangledName(function_decl);
  if (is_member_or_descendant_of_class_template) {
    // `thunks_for_class_template_member_functions.md` explains in more detail
    // why the `mangled_name` has to include the target name when working with
    // members or descendants of a class template.
    mangled_name += '_';
    mangled_name += ConvertToCcIdentifier(ictx_.GetOwningTarget(function_decl));
  }

  // Silence ClangTidy, checked above: calling `add_error` if
  // `!return_type.ok()` and returning early if `!errors.empty()`.
  CHECK_OK(return_type);

  return Func{
      .name = *translated_name,
      .owning_target = ictx_.GetOwningTarget(function_decl),
      .doc_comment = std::move(doc_comment),
      .mangled_name = std::move(mangled_name),
      .return_type = *return_type,
      .params = std::move(params),
      .lifetime_params = std::move(lifetime_params),
      .is_inline = function_decl->isInlined(),
      .member_func_metadata = std::move(member_func_metadata),
      .has_c_calling_convention = has_c_calling_convention,
      .is_member_or_descendant_of_class_template =
          is_member_or_descendant_of_class_template,
      .source_loc = ictx_.ConvertSourceLocation(function_decl->getBeginLoc()),
      .id = GenerateItemId(function_decl),
      .enclosing_namespace_id = GetEnclosingNamespaceId(function_decl),
  };
}

}  // namespace crubit
