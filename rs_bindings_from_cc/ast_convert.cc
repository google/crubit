// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/ast_convert.h"

#include "absl/base/nullability.h"
#include "absl/functional/function_ref.h"
#include "absl/log/check.h"
#include "rs_bindings_from_cc/ast_util.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "rs_bindings_from_cc/recording_diagnostic_consumer.h"
#include "clang/AST/ASTMutationListener.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Sema/Sema.h"

namespace crubit {
namespace {

// Returns a copy constructor for `record`, or `nullptr` if none is declared.
//
// Does not traverse to the base classes.
static clang::CXXConstructorDecl* GetCopyCtor(clang::CXXRecordDecl* record) {
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
static clang::CXXConstructorDecl* GetMoveCtor(clang::CXXRecordDecl* record) {
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
    clang::CXXRecordDecl* record,
    absl::FunctionRef<clang::CXXMethodDecl*(clang::CXXRecordDecl*)> getter) {
  auto nonrecursive_has_only_defaulted =
      [&getter](const clang::CXXRecordDecl* record) {
        const clang::CXXMethodDecl* decl =
            getter(const_cast<clang::CXXRecordDecl*>(record));
        return decl == nullptr ||
               (!decl->isUserProvided() && !decl->isVirtual());
      };

  if (!nonrecursive_has_only_defaulted(record)) {
    return false;
  }
  return record->forallBases(nonrecursive_has_only_defaulted);
}

SpecialMemberFunc GetSpecialMemberFunc(
    ImportContext* absl_nullable ictx, clang::RecordDecl& record_decl,
    absl::FunctionRef<clang::CXXMethodDecl*(clang::CXXRecordDecl*)> getter) {
  auto* cxx_record_decl = clang::dyn_cast<clang::CXXRecordDecl>(&record_decl);
  if (cxx_record_decl == nullptr) {
    return SpecialMemberFunc::kTrivial;
  }

  clang::CXXMethodDecl* decl = getter(cxx_record_decl);
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
      CHECK(false &&
            "We should never be encoding a 'none' access specifier in IR.");
      // We have to return something. kDeleted seems like a safe fallback.
      return SpecialMemberFunc::kUnavailable;
  }

  if (auto* ctor = clang::dyn_cast<clang::CXXConstructorDecl>(decl);
      ctor != nullptr && ctor->isDefaulted() && !ctor->isDefaultConstructor() &&
      !ctor->doesThisDeclarationHaveABody() && !ctor->isDeleted() &&
      ictx != nullptr) {
    // Alternate options that don't seem to work include
    // Sema::ShouldDeleteSpecialMember and
    // Sema::ForceDeclarationOfImplicitMembers. These might be cheaper, but it
    // appears that we need to fully synthesize the special member functions
    // and the templates they use to catch any possible errors.
    crubit::RecordingDiagnosticConsumer diagnostic_recorder =
        crubit::RecordDiagnostics(ictx->sema_.getDiagnostics(), [&] {
          auto* mutable_ctor = const_cast<clang::CXXConstructorDecl*>(ctor);
          FakeTUScope fake_tu_scope(*ictx);
          clang::Sema::SynthesizedFunctionScope synthesized_function_scope(
              ictx->sema_, mutable_ctor);
          // We can't use DefineImplicitDefaultConstructor directly because
          // mutable_ctor is *not* a default ctor (it's *defaulted*).
          // DefineImplicitDefaultConstructor eventually calls to
          // SetCtorInitializers, which in turn will produce diagnostics if
          // the defaulted ctor is impossible.
          ictx->sema_.SetCtorInitializers(mutable_ctor, false);
        });
    if (diagnostic_recorder.getNumErrors() != 0) {
      return SpecialMemberFunc::kUnavailable;
    }
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

SpecialMemberFunc GetCopyCtorSpecialMemberFunc(ImportContext& ictx,
                                               clang::RecordDecl& record_decl) {
  return GetSpecialMemberFunc(&ictx, record_decl, &GetCopyCtor);
}

SpecialMemberFunc GetMoveCtorSpecialMemberFunc(ImportContext& ictx,
                                               clang::RecordDecl& record_decl) {
  return GetSpecialMemberFunc(&ictx, record_decl, &GetMoveCtor);
}

SpecialMemberFunc GetDestructorSpecialMemberFunc(
    clang::RecordDecl& record_decl) {
  return GetSpecialMemberFunc(nullptr, record_decl,
                              [](auto c) { return c->getDestructor(); });
}

}  // namespace crubit
