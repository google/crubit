// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_AST_UTIL_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_AST_UTIL_H_

#include <optional>
#include <string>

#include "absl/functional/function_ref.h"
#include "absl/status/statusor.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "clang/AST/Attr.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Type.h"
#include "clang/Basic/AttrKinds.h"

namespace crubit {

// Returns true if `decl` is either 1) a ClassTemplateSpecializationDecl (but
// not ClassTemplatePartialSpecializationDecl) or 2) a decl (e.g. a member
// function decl) nested inside a ClassTemplateSpecializationDecl.
bool IsFullClassTemplateSpecializationOrChild(const clang::Decl* decl);

// Returns true if `attr` is clang::lifetimebound or clang::lifetime_capture_by.
bool IsClangLifetimeAnnotation(const clang::Attr& attr);

// Returns a human-readable string containing the list of unknown attrs.
//
// is_known is called exactly once on every attribute, and returns true if the
// attribute is understood.
//
// Annotate attributes are ignored and CRUBIT_UNSAFE_IGNORE_ATTR is considered.
absl::StatusOr<std::optional<std::string>> CollectUnknownAttrs(
    const clang::Decl& decl,
    absl::FunctionRef<bool(const clang::Attr&)> is_known =
        [](const clang::Attr& attr) { return false; });

// Returns a human-readable string containing the list of unknown attrs.
//
// is_known is called exactly once on every attribute, and returns true if the
// attribute is understood.
std::optional<std::string> CollectUnknownTypeAttrs(
    const clang::Type& t, absl::FunctionRef<bool(clang::attr::Kind)> is_known =
                              [](clang::attr::Kind attr) { return false; });

struct ClangLifetimeAnnotations {
  bool lifetimebound = false;
  std::vector<int> lifetime_capture_by;
};

// Collects the Clang lifetimebound and lifetime_capture_by attributes from
// `t`, assuming that `t` is the type of a member function.
absl::StatusOr<ClangLifetimeAnnotations>
CollectClangLifetimeAnnotationsForMemberFunctionType(
    const clang::ASTContext& ast_context, const clang::Type& t);

// Collects all lifetime names annotated on `t` under `ast_context`, returning
// an error if any were invalid. Any returned string_views belong to
// `ast_context`.
absl::StatusOr<std::vector<absl::string_view>> CollectExplicitLifetimes(
    const clang::ASTContext& ast_context, const clang::Type& t);

// Returns true if `decl` is non-null and refers to a (code-generated) proto2
// message.
bool IsProto2Message(const clang::Decl& decl);

// An RAII guard that sets a fake TU scope for the duration of its lifetime
// and restores the previous TU scope when it goes out of scope.
class FakeTUScope {
 public:
  // Sets `ctx`'s `Sema`'s `TUScope` to a fake scope that points to the
  // translation unit declaration.
  explicit FakeTUScope(ImportContext& ctx)
      : ctx_(ctx),
        scope_(std::make_unique<clang::Scope>(nullptr, clang::Scope::DeclScope,
                                              ctx_.sema_.getDiagnostics())),
        old_tu_scope_(ctx_.sema_.TUScope) {
    ctx_.sema_.TUScope = scope_.get();
    ctx_.sema_.TUScope->setEntity(ctx_.ctx_.getTranslationUnitDecl());
  }

  ~FakeTUScope() {
    ctx_.sema_.TUScope->setEntity(nullptr);
    ctx_.sema_.TUScope = old_tu_scope_;
  }

 private:
  ImportContext& ctx_;
  std::unique_ptr<clang::Scope> scope_;
  clang::Scope* old_tu_scope_;
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_AST_UTIL_H_
