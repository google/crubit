// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_convert.h"

#include "absl/functional/function_ref.h"
#include "common/check.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/Basic/Specifiers.h"

namespace crubit {
namespace {

// Returns a copy constructor for `record`, or `nullptr` if none is declared.
//
// Does not traverse to the base classes.
static const clang::CXXConstructorDecl* GetCopyCtor(
    const clang::CXXRecordDecl* record) {
  for (clang::CXXConstructorDecl* ctor_decl : record->ctors()) {
    if (ctor_decl->isCopyConstructor()) {
      return ctor_decl;
    }
  }
  return nullptr;
}

// Returns a move constructor for `record`, or `nullptr` if none is declared.
//
// Does not traverse to the base classes.
static const clang::CXXConstructorDecl* GetMoveCtor(
    const clang::CXXRecordDecl* record) {
  for (clang::CXXConstructorDecl* ctor_decl : record->ctors()) {
    if (ctor_decl->isMoveConstructor()) {
      return ctor_decl;
    }
  }
  return nullptr;
}

// Returns true if this class, and all base classes, only define the specified
// special member implicitly or via =default, and only do so non-virtually.
//
// Args:
//   record: the class/struct to check.
//   getter: a function which returns the special member function in question.
//       returns null if the special member function is implicitly defined.
bool HasNoUserProvidedSpecialMember(
    const clang::CXXRecordDecl* record,
    absl::FunctionRef<const clang::CXXMethodDecl*(const clang::CXXRecordDecl*)>
        getter) {
  auto nonrecursive_has_only_defaulted =
      [&getter](const clang::CXXRecordDecl* record) {
        const clang::CXXMethodDecl* decl = getter(record);
        return decl == nullptr ||
               (!decl->isUserProvided() && !decl->isVirtual());
      };

  if (!nonrecursive_has_only_defaulted(record)) {
    return false;
  }
  return record->forallBases(nonrecursive_has_only_defaulted);
}

SpecialMemberFunc GetSpecialMemberFunc(
    const clang::RecordDecl& record_decl,
    absl::FunctionRef<const clang::CXXMethodDecl*(const clang::CXXRecordDecl*)>
        getter) {
  const auto* cxx_record_decl =
      clang::dyn_cast<clang::CXXRecordDecl>(&record_decl);
  if (cxx_record_decl == nullptr) {
    return SpecialMemberFunc::kTrivial;
  }

  const clang::CXXMethodDecl* decl = getter(cxx_record_decl);
  if (decl == nullptr) {
    return SpecialMemberFunc::kUnavailable;
  }

  switch (decl->getAccess()) {
    case clang::AS_public:
      break;
    case clang::AS_protected:
    case clang::AS_private:
      return SpecialMemberFunc::kUnavailable;
    case clang::AS_none:
      CRUBIT_CHECK(
          false &&
          "We should never be encoding a 'none' access specifier in IR.");
      // We have to return something. kDeleted seems like a safe fallback.
      return SpecialMemberFunc::kUnavailable;
  }

  if (decl->isDeleted()) {
    return SpecialMemberFunc::kUnavailable;
  } else if (decl->isTrivial()) {
    return SpecialMemberFunc::kTrivial;
  } else if (HasNoUserProvidedSpecialMember(cxx_record_decl, getter)) {
    return SpecialMemberFunc::kNontrivialMembers;
  } else {
    return SpecialMemberFunc::kNontrivialUserDefined;
  }
}

}  // namespace

SpecialMemberFunc GetCopyCtorSpecialMemberFunc(
    const clang::RecordDecl& record_decl) {
  return GetSpecialMemberFunc(record_decl, &GetCopyCtor);
}

SpecialMemberFunc GetMoveCtorSpecialMemberFunc(
    const clang::RecordDecl& record_decl) {
  return GetSpecialMemberFunc(record_decl, &GetMoveCtor);
}

SpecialMemberFunc GetDestructorSpecialMemberFunc(
    const clang::RecordDecl& record_decl) {
  return GetSpecialMemberFunc(record_decl,
                              [](auto c) { return c->getDestructor(); });
}

}  // namespace crubit
