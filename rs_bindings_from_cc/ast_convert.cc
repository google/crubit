// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_convert.h"

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
//       Signature: `const clang::FunctionDecl* (const clang::CXXRecordDecl*)`.
template <typename F>
bool HasOnlyDefaultedSpecialMember(const clang::CXXRecordDecl* record,
                                   const F& getter) {
  auto nonrecursive_has_only_defaulted =
      [&getter](const clang::CXXRecordDecl* record) {
        const clang::FunctionDecl* decl = getter(record);
        return decl == nullptr || decl->isDefaulted();
      };

  if (!nonrecursive_has_only_defaulted(record)) {
    return false;
  }
  return record->forallBases(nonrecursive_has_only_defaulted);
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
  SpecialMemberFunc copy_ctor = {
      .definition = SpecialMemberFunc::Definition::kTrivial,
      .access = kPublic,
  };
  const auto* cxx_record_decl =
      clang::dyn_cast<clang::CXXRecordDecl>(&record_decl);
  if (cxx_record_decl == nullptr) {
    return copy_ctor;
  }

  if (cxx_record_decl->hasTrivialCopyConstructor()) {
    copy_ctor.definition = SpecialMemberFunc::Definition::kTrivial;
  } else if (cxx_record_decl->hasNonTrivialCopyConstructor()) {
    if (HasOnlyDefaultedSpecialMember(cxx_record_decl, &GetCopyCtor)) {
      copy_ctor.definition = SpecialMemberFunc::Definition::kNontrivialMembers;
    } else {
      copy_ctor.definition = SpecialMemberFunc::Definition::kNontrivialSelf;
    }
  } else {
    // The copy constructor can be **implicitly deleted**, e.g. by the
    // presence of a move ctor.
    copy_ctor.definition = SpecialMemberFunc::Definition::kDeleted;
  }

  const clang::CXXConstructorDecl* copy_ctor_decl =
      GetCopyCtor(cxx_record_decl);
  if (copy_ctor_decl != nullptr) {
    copy_ctor.access = TranslateAccessSpecifier(copy_ctor_decl->getAccess());
    if (copy_ctor_decl->isDeleted()) {
      copy_ctor.definition = SpecialMemberFunc::Definition::kDeleted;
    }
  }

  return copy_ctor;
}

SpecialMemberFunc GetMoveCtorSpecialMemberFunc(
    const clang::RecordDecl& record_decl) {
  SpecialMemberFunc move_ctor = {
      .definition = SpecialMemberFunc::Definition::kTrivial,
      .access = kPublic,
  };
  const auto* cxx_record_decl =
      clang::dyn_cast<clang::CXXRecordDecl>(&record_decl);
  if (cxx_record_decl == nullptr) {
    return move_ctor;
  }

  if (cxx_record_decl->hasTrivialMoveConstructor()) {
    move_ctor.definition = SpecialMemberFunc::Definition::kTrivial;
  } else if (cxx_record_decl->hasNonTrivialMoveConstructor()) {
    if (HasOnlyDefaultedSpecialMember(cxx_record_decl, &GetMoveCtor)) {
      move_ctor.definition = SpecialMemberFunc::Definition::kNontrivialMembers;
    } else {
      move_ctor.definition = SpecialMemberFunc::Definition::kNontrivialSelf;
    }
  } else {
    // The move constructor can be **implicitly deleted**, e.g. by the
    // presence of a copy ctor.
    move_ctor.definition = SpecialMemberFunc::Definition::kDeleted;
  }

  const clang::CXXConstructorDecl* move_ctor_decl =
      GetMoveCtor(cxx_record_decl);
  if (move_ctor_decl != nullptr) {
    move_ctor.access = TranslateAccessSpecifier(move_ctor_decl->getAccess());
    if (move_ctor_decl->isDeleted()) {
      move_ctor.definition = SpecialMemberFunc::Definition::kDeleted;
    }
  }

  return move_ctor;
}

SpecialMemberFunc GetDestructorSpecialMemberFunc(
    const clang::RecordDecl& record_decl) {
  SpecialMemberFunc dtor = {
      .definition = SpecialMemberFunc::Definition::kTrivial,
      .access = kPublic,
  };
  const auto* cxx_record_decl =
      clang::dyn_cast<clang::CXXRecordDecl>(&record_decl);
  if (cxx_record_decl == nullptr) {
    return dtor;
  }

  if (cxx_record_decl->hasTrivialDestructor()) {
    dtor.definition = SpecialMemberFunc::Definition::kTrivial;
  } else {
    const bool has_only_defaulted_destructors = HasOnlyDefaultedSpecialMember(
        cxx_record_decl, [](auto c) { return c->getDestructor(); });
    if (has_only_defaulted_destructors) {
      dtor.definition = SpecialMemberFunc::Definition::kNontrivialMembers;
    } else {
      dtor.definition = SpecialMemberFunc::Definition::kNontrivialSelf;
    }
  }

  const clang::CXXDestructorDecl* dtor_decl = cxx_record_decl->getDestructor();
  if (dtor_decl != nullptr) {
    dtor.access = TranslateAccessSpecifier(dtor_decl->getAccess());
    if (dtor_decl->isDeleted()) {
      dtor.definition = SpecialMemberFunc::Definition::kDeleted;
    }
  }
  return dtor;
}

}  // namespace rs_bindings_from_cc
