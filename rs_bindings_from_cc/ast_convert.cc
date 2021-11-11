// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_convert.h"

#include <assert.h>

#include "rs_bindings_from_cc/ir.h"
#include "third_party/absl/functional/function_ref.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/Decl.h"
#include "third_party/llvm/llvm-project/clang/include/clang/AST/DeclCXX.h"
#include "third_party/llvm/llvm-project/clang/include/clang/Basic/Specifiers.h"

namespace rs_bindings_from_cc {
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
// special member implicitly or via =default.
//
// Args:
//   record: the class/struct to check.
//   getter: a function which returns the special member function in question.
//       returns null if the special member function is implicitly defined.
bool HasNoUserProvidedSpecialMember(
    const clang::CXXRecordDecl* record,
    absl::FunctionRef<const clang::FunctionDecl*(const clang::CXXRecordDecl*)>
        getter) {
  auto nonrecursive_has_only_defaulted =
      [&getter](const clang::CXXRecordDecl* record) {
        const clang::FunctionDecl* decl = getter(record);
        return decl == nullptr || !decl->isUserProvided();
      };

  if (!nonrecursive_has_only_defaulted(record)) {
    return false;
  }
  return record->forallBases(nonrecursive_has_only_defaulted);
}

SpecialMemberFunc GetSpecialMemberFunc(
    const clang::RecordDecl& record_decl,
    absl::FunctionRef<const clang::FunctionDecl*(const clang::CXXRecordDecl*)>
        getter) {
  SpecialMemberFunc smf = {
      .definition = SpecialMemberFunc::Definition::kTrivial,
      .access = kPublic,
  };

  const auto* cxx_record_decl =
      clang::dyn_cast<clang::CXXRecordDecl>(&record_decl);
  if (cxx_record_decl == nullptr) {
    return smf;
  }

  const clang::FunctionDecl* decl = getter(cxx_record_decl);
  if (decl == nullptr) {
    smf.definition = SpecialMemberFunc::Definition::kDeleted;
    return smf;
  }

  smf.access = TranslateAccessSpecifier(decl->getAccess());
  if (decl->isDeleted()) {
    smf.definition = SpecialMemberFunc::Definition::kDeleted;
  } else if (decl->isTrivial()) {
    smf.definition = SpecialMemberFunc::Definition::kTrivial;
  } else if (HasNoUserProvidedSpecialMember(cxx_record_decl, getter)) {
    smf.definition = SpecialMemberFunc::Definition::kNontrivialMembers;
  } else {
    smf.definition = SpecialMemberFunc::Definition::kNontrivialSelf;
  }
  return smf;
}
}  // namespace

AccessSpecifier TranslateAccessSpecifier(clang::AccessSpecifier access) {
  switch (access) {
    case clang::AS_public:
      return kPublic;
    case clang::AS_protected:
      return kProtected;
    case clang::AS_private:
      return kPrivate;
    case clang::AS_none:
      // We should never be encoding a "none" access specifier in IR.
      assert(false);
      // We have to return something. Conservatively return private so we don't
      // inadvertently make a private member variable accessible in Rust.
      return kPrivate;
  }
}

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

}  // namespace rs_bindings_from_cc
