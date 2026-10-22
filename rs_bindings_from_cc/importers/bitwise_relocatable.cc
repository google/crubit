// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/importers/bitwise_relocatable.h"

#include "absl/log/check.h"
#include "absl/strings/str_cat.h"
#include "common/annotation_reader.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "clang/AST/RecordLayout.h"
#include "clang/Basic/LangOptions.h"
#include "clang/Lex/Preprocessor.h"
#include "clang/Sema/Overload.h"
#include "clang/Sema/Sema.h"
#include "llvm/Support/Casting.h"

namespace crubit {
namespace {

clang::CXXMethodDecl* LookupSpecialMemberFromXValue(
    clang::Sema& SemaRef, const clang::CXXRecordDecl* RD, bool Assign) {
  RD = RD->getDefinition();
  if (!RD) return nullptr;
  clang::SourceLocation LookupLoc = RD->getLocation();
  clang::CanQualType CanTy = SemaRef.getASTContext().getCanonicalTagType(RD);
  clang::DeclarationName Name;
  if (Assign)
    Name = SemaRef.getASTContext().DeclarationNames.getCXXOperatorName(
        clang::OO_Equal);
  else
    Name =
        SemaRef.getASTContext().DeclarationNames.getCXXConstructorName(CanTy);

  clang::OpaqueValueExpr FakeArg(LookupLoc, CanTy, clang::VK_XValue);
  clang::Expr* Arg = &FakeArg;
  clang::QualType ThisTy = CanTy;
  clang::Expr::Classification Classification =
      clang::OpaqueValueExpr(LookupLoc, ThisTy, clang::VK_LValue)
          .Classify(SemaRef.getASTContext());

  clang::OverloadCandidateSet OCS(LookupLoc,
                                  clang::OverloadCandidateSet::CSK_Normal);
  clang::DeclContext::lookup_result R = RD->lookup(Name);
  if (R.empty()) return nullptr;

  llvm::SmallVector<clang::NamedDecl*, 8> Candidates(R.begin(), R.end());
  for (clang::NamedDecl* CandDecl : Candidates) {
    if (CandDecl->isInvalidDecl()) continue;
    clang::DeclAccessPair Cand =
        clang::DeclAccessPair::make(CandDecl, clang::AS_none);
    auto CtorInfo = clang::getConstructorInfo(Cand);
    if (clang::CXXMethodDecl* M =
            llvm::dyn_cast<clang::CXXMethodDecl>(Cand->getUnderlyingDecl())) {
      if (Assign)
        SemaRef.AddMethodCandidate(
            M, Cand, const_cast<clang::CXXRecordDecl*>(RD), ThisTy,
            Classification, llvm::ArrayRef(&Arg, 1), OCS, true);
      else {
        assert(CtorInfo);
        SemaRef.AddOverloadCandidate(CtorInfo.Constructor, CtorInfo.FoundDecl,
                                     llvm::ArrayRef(&Arg, 1), OCS, true);
      }
    } else if (clang::FunctionTemplateDecl* Tmpl =
                   llvm::dyn_cast<clang::FunctionTemplateDecl>(
                       Cand->getUnderlyingDecl())) {
      if (Assign)
        SemaRef.AddMethodTemplateCandidate(
            Tmpl, Cand, const_cast<clang::CXXRecordDecl*>(RD), nullptr, ThisTy,
            Classification, llvm::ArrayRef(&Arg, 1), OCS, true);
      else {
        assert(CtorInfo);
        SemaRef.AddTemplateOverloadCandidate(
            CtorInfo.ConstructorTmpl, CtorInfo.FoundDecl, nullptr,
            llvm::ArrayRef(&Arg, 1), OCS, true);
      }
    }
  }
  clang::OverloadCandidateSet::iterator Best;
  switch (OCS.BestViableFunction(SemaRef, LookupLoc, Best)) {
    case clang::OR_Success:
    case clang::OR_Deleted:
      return llvm::cast<clang::CXXMethodDecl>(Best->Function)
          ->getCanonicalDecl();
    default:
      return nullptr;
  }
}

bool HasAnnotation(const clang::Decl* decl, absl::string_view name) {
  auto args = GetAnnotateAttrArgs(*decl, name);
  return args.ok() && args.value().has_value();
}

enum class AnnotationState { kNone, kIfEligible, kAsserted };

AnnotationState GetAnnotationState(const clang::CXXRecordDecl* decl) {
  if (HasAnnotation(decl, "crubit_unsafe_bitwise_relocatable"))
    return AnnotationState::kAsserted;
  if (HasAnnotation(decl, "crubit_unsafe_bitwise_relocatable_if_eligible"))
    return AnnotationState::kIfEligible;
  if (decl->hasAttr<clang::TrivialABIAttr>())
    return AnnotationState::kIfEligible;
  return AnnotationState::kNone;
}

enum class NeverBRKind {
  kNotNever,
  kPolymorphic,
  kVirtualBases,
  kAddressDependent,
  kStdBasicString
};

NeverBRKind IsNeverBR(const clang::CXXRecordDecl* decl) {
  if (decl->isPolymorphic()) return NeverBRKind::kPolymorphic;
  if (decl->getNumVBases() > 0) return NeverBRKind::kVirtualBases;
  if (decl->getArgPassingRestrictions() ==
      clang::RecordArgPassingKind::CanNeverPassInRegs)
    return NeverBRKind::kAddressDependent;
  if (auto* spec =
          clang::dyn_cast<clang::ClassTemplateSpecializationDecl>(decl)) {
    if (auto* tmpl = spec->getSpecializedTemplate()) {
      if (tmpl->isInStdNamespace() && tmpl->getNameAsString() == "basic_string")
        return NeverBRKind::kStdBasicString;
    }
  }
  return NeverBRKind::kNotNever;
}

std::string NeverBRReason(NeverBRKind kind) {
  switch (kind) {
    case NeverBRKind::kPolymorphic:
      return "type is polymorphic";
    case NeverBRKind::kVirtualBases:
      return "type has virtual bases";
    case NeverBRKind::kAddressDependent:
      return "type can never pass in registers (__weak or __ptrauth)";
    case NeverBRKind::kStdBasicString:
      return "std::basic_string is never bitwise-relocatable";
    default:
      return "";
  }
}

bool CheckOwnSpecialMembers(clang::Sema& SemaRef,
                            const clang::CXXRecordDecl* D) {
  // Per P2786 [class.prop], a union with no user-declared special member
  // functions is trivially relocatable whenever all of its variant members are
  // (even if a variant member's non-trivial move constructor or destructor
  // causes the union's implicit special members to be deleted, as in an
  // anonymous `union { char none_; T value_; };` inside an optional wrapper).
  if (D->isUnion() && !D->hasUserDeclaredCopyConstructor() &&
      !D->hasUserDeclaredCopyAssignment() &&
      !D->hasUserDeclaredMoveOperation() && !D->hasUserDeclaredDestructor()) {
    return true;
  }

  SemaRef.ForceDeclarationOfImplicitMembers(
      const_cast<clang::CXXRecordDecl*>(D));

  auto* dtor = D->getDestructor();
  if (!dtor) return false;
  dtor = llvm::cast<clang::CXXDestructorDecl>(dtor->getCanonicalDecl());
  if (dtor->isDeleted() || (dtor->isUserProvided() && !dtor->isDefaulted())) {
    return false;
  }

  // Under C++11 DR1402 (`Sema::AddOverloadCandidate`), a defaulted move
  // constructor that Clang implicitly deleted is ignored by overload
  // resolution. When all subobjects of `D` are bitwise-relocatable, such a move
  // constructor (`D(D&&) = default` or implicit `D(D&&)`) was only deleted
  // because a subobject deletes its C++ move constructor (e.g. `rs::Box<T>`),
  // and would otherwise be an exact match for an xvalue of `D`.
  for (const clang::CXXConstructorDecl* move_ctor : D->ctors()) {
    move_ctor = move_ctor->getCanonicalDecl();
    if (!move_ctor->isMoveConstructor() ||
        move_ctor->isIneligibleOrNotSelected() || move_ctor->isUserProvided() ||
        move_ctor->isDeletedAsWritten()) {
      continue;
    }
    if (move_ctor->getNumParams() == 1 && !move_ctor->getParamDecl(0)
                                               ->getType()
                                               ->getPointeeType()
                                               .hasQualifiers()) {
      return true;
    }
  }

  // Otherwise, check overload resolution from an xvalue of `D`. A defaulted
  // copy constructor that Clang implicitly deleted (`!isDeletedAsWritten()`)
  // also does not disqualify `D` unless it was deleted because `D` itself
  // declared a move operation.
  auto* ctor = llvm::dyn_cast_or_null<clang::CXXConstructorDecl>(
      LookupSpecialMemberFromXValue(SemaRef, D, /*Assign=*/false));
  if (!ctor || !ctor->isCopyOrMoveConstructor() || ctor->isUserProvided() ||
      ctor->isDeletedAsWritten()) {
    return false;
  }
  if (ctor->isImplicit() && ctor->isCopyConstructor() &&
      D->hasUserDeclaredMoveOperation()) {
    return false;
  }
  return true;
}

}  // namespace

BitwiseRelocatableResult RelocatabilityAnalysis::GetBitwiseRelocatability(
    ImportContext& ictx, const clang::CXXRecordDecl* record_decl) {
  record_decl = record_decl->getDefinition();
  if (!record_decl)
    return BitwiseRelocatableResult{
        BitwiseRelocatableResult::Kind::kNo, "type is incomplete", {}};

  auto it = cache_.find(record_decl);
  if (it != cache_.end()) {
    return it->second;
  }
  cache_[record_decl] = BitwiseRelocatableResult{
      BitwiseRelocatableResult::Kind::kNo, "recursive evaluation", {}};
  BitwiseRelocatableResult result =
      ComputeBitwiseRelocatability(ictx, record_decl);
  cache_[record_decl] = result;
  return result;
}

BitwiseRelocatableResult RelocatabilityAnalysis::ComputeBitwiseRelocatability(
    ImportContext& ictx, const clang::CXXRecordDecl* record_decl) {
  NeverBRKind never_kind = IsNeverBR(record_decl);
  if (never_kind != NeverBRKind::kNotNever) {
    AnnotationState state = GetAnnotationState(record_decl);
    if (state == AnnotationState::kAsserted) {
      return {
          BitwiseRelocatableResult::Kind::kError,
          absl::StrCat(
              NeverBRReason(never_kind),
              " (CRUBIT_UNSAFE_BITWISE_RELOCATABLE applied to never-set type)"),
          {}};
    }
    return {BitwiseRelocatableResult::Kind::kNo, NeverBRReason(never_kind), {}};
  }

  bool members_ok = true;
  bool hard_error = false;
  std::string members_reason;
  std::vector<std::string> libcxx_only_fields;

  auto check_member = [&](const clang::QualType& qt,
                          const clang::Decl* decl_for_attr, std::string name) {
    if (decl_for_attr) {
      if (HasAnnotation(
              decl_for_attr,
              "crubit_unsafe_bitwise_relocatable_field_libcxx_only")) {
        if (!ictx.sema_.getPreprocessor().isMacroDefined("_LIBCPP_VERSION")) {
          hard_error = true;
          members_ok = false;
          members_reason =
              absl::StrCat("member `", name,
                           "` is annotated with "
                           "CRUBIT_UNSAFE_BITWISE_RELOCATABLE_FIELD_LIBCXX_"
                           "ONLY but TU is not compiled against libc++");
          return;
        }
        libcxx_only_fields.push_back(name);

        // Recursively check array element type.
        const clang::Type* t = qt->getBaseElementTypeUnsafe();
        if (auto* rec = t->getAsCXXRecordDecl()) {
          NeverBRKind k = IsNeverBR(rec);
          if (k != NeverBRKind::kNotNever) {
            if (k != NeverBRKind::kStdBasicString) {
              hard_error = true;
              members_ok = false;
              members_reason = absl::StrCat(
                  "member `", name, "` is ", NeverBRReason(k),
                  ", which cannot be laundered by field overrides");
            }
          }
        }
        return;
      }
      if (HasAnnotation(decl_for_attr,
                        "crubit_unsafe_bitwise_relocatable_field")) {
        const clang::Type* t = qt->getBaseElementTypeUnsafe();
        if (auto* rec = t->getAsCXXRecordDecl()) {
          NeverBRKind k = IsNeverBR(rec);
          if (k != NeverBRKind::kNotNever) {
            hard_error = true;
            members_ok = false;
            members_reason =
                absl::StrCat("member `", name, "` is ", NeverBRReason(k),
                             ", which cannot be laundered by "
                             "CRUBIT_UNSAFE_BITWISE_RELOCATABLE_FIELD");
          }
        }
        return;
      }
    }

    const clang::Type* type = qt.getTypePtr();
    if (type->isReferenceType() || type->isScalarType() ||
        type->isPointerType() || type->isMemberPointerType())
      return;
    type = qt->getBaseElementTypeUnsafe();
    if (auto* rec = type->getAsCXXRecordDecl()) {
      auto res = GetBitwiseRelocatability(ictx, rec);
      if (res.kind != BitwiseRelocatableResult::Kind::kYes) {
        members_ok = false;
        members_reason = absl::StrCat(
            "member `", name, "` is not bitwise-relocatable: ", res.reason);
      }
      if (!res.libcxx_only_fields.empty()) {
        libcxx_only_fields.insert(libcxx_only_fields.end(),
                                  res.libcxx_only_fields.begin(),
                                  res.libcxx_only_fields.end());
      }
    }
  };

  // decl_for_attr is nullptr for base classes because fields overrides only
  // apply to fields.
  for (const clang::CXXBaseSpecifier& base : record_decl->bases()) {
    check_member(
        base.getType(), nullptr,
        absl::StrCat("base class `", base.getType().getAsString(), "`"));
    if (hard_error) break;
  }
  if (!hard_error) {
    for (const clang::FieldDecl* field : record_decl->fields()) {
      check_member(field->getType(), field, field->getNameAsString());
      if (hard_error) break;
    }
  }

  if (hard_error) {
    return {BitwiseRelocatableResult::Kind::kError, members_reason,
            libcxx_only_fields};
  }

  AnnotationState state = GetAnnotationState(record_decl);

  if (!members_ok) {
    if (state == AnnotationState::kAsserted) {
      return {BitwiseRelocatableResult::Kind::kError, members_reason,
              libcxx_only_fields};
    }
    return {BitwiseRelocatableResult::Kind::kNo, members_reason,
            libcxx_only_fields};
  }

  if (state == AnnotationState::kAsserted ||
      state == AnnotationState::kIfEligible) {
    return {BitwiseRelocatableResult::Kind::kYes, "", libcxx_only_fields};
  }

  if (!CheckOwnSpecialMembers(ictx.sema_, record_decl)) {
    return {BitwiseRelocatableResult::Kind::kNo,
            "destructor or move constructor is user-provided",
            libcxx_only_fields};
  }

  return {BitwiseRelocatableResult::Kind::kYes, "", libcxx_only_fields};
}

}  // namespace crubit
