// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_AST_UTIL_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_AST_UTIL_H_

#include <optional>
#include <string>

#include "absl/container/flat_hash_set.h"
#include "absl/functional/function_ref.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "clang/AST/Attr.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclBase.h"
#include "clang/AST/Type.h"
#include "clang/Basic/AttrKinds.h"
#include "clang/Basic/Specifiers.h"

namespace crubit {

// Returns true if `decl` is either 1) a ClassTemplateSpecializationDecl (but
// not ClassTemplatePartialSpecializationDecl) or 2) a decl (e.g. a member
// function decl) nested inside a ClassTemplateSpecializationDecl.
bool IsFullClassTemplateSpecializationOrChild(const clang::Decl* decl);

// Returns true if `attr` is clang::lifetimebound or clang::lifetime_capture_by.
bool IsClangLifetimeAnnotation(const clang::Attr& attr);

// Returns true if `attr` is a Clang coroutine attribute.
bool IsClangCoroAnnotation(const clang::Attr& attr);

// Returns the set of ignored attribute names.
absl::flat_hash_set<absl::string_view> GetIgnoredAttrs(const clang::Decl* decl);

// Wrappers that respect CRUBIT_UNSAFE_IGNORE_ATTR
template <typename T>
bool HasAttr(const clang::Decl* decl) {
  if (!decl->hasAttr<T>()) return false;

  // Fast path: if there are no annotations at all, nothing can be ignored.
  if (!decl->hasAttr<clang::AnnotateAttr>()) return true;

  absl::flat_hash_set<absl::string_view> ignored = GetIgnoredAttrs(decl);
  for (const auto* attr : decl->specific_attrs<T>()) {
    if (attr->getAttrName()) {
      if (!ignored.contains(attr->getNormalizedFullName())) {
        return true;
      }
    } else {
      if (!ignored.contains(attr->getSpelling())) {
        return true;
      }
    }
  }
  return false;
}
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

// Collects explicitly-defined lifetime input parameters from `decl` under
// `ast_context`.
absl::StatusOr<std::vector<absl::string_view>> CollectLifetimeInputs(
    const clang::ASTContext& ast_context, const clang::Decl* decl);

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

// Reduces a clang::CallingConv into a crubit::CallingConv, which is a subset.
// If the variant isn't in the subset, returns an error.
absl::StatusOr<CallingConv> ConvertCcCallConvToSupportedCallingConv(
    clang::CallingConv cc_call_conv);

// Returns true if `decl` is non-null and refers to a (code-generated) proto2
// message.
bool IsProto2Message(const clang::Decl& decl);

// If `alias_decl` is a C-style name-introducing typedef (e.g.,
// `typedef struct Foo Foo;` or `typedef struct { ... } Foo;`), returns the
// underlying `TagDecl` representing the C type. Otherwise, returns `nullptr`.
const clang::TagDecl* StripCStyleNameIntroducingTypedef(
    const clang::TypedefNameDecl* alias_decl);

// Attempts to define the implicit/default function `function_decl`. Has no
// effect if `function_decl` is not an interesting special member or is not
// implicit. Returns `false` if there were errors defining the function
// (e.g., an implicit copy constructor could rely on a deleted copy constructor
// for a member variable).
bool ForceDefineImplicitFunction(ImportContext& ictx,
                                 clang::FunctionDecl* function_decl);

// Checks to see if `function_decl` can reach an invalid template instantiation
// or an invalid default/implicit member (including if `function_decl` itself
// is invalid). Returns the fully-qualified name of the first invalid decl
// reached (suitable for including in a diagnostic message). Returns the empty
// string if no such decl was found.
//
// Clang won't diagnose invalid template declarations multiple times, but we
// can crawl over the syntax tree to determine if a template instantiation will
// be invalid because it transitively reaches an invalid declaration.
//
// Note that we only need to recurse on template instantiations. Concrete
// declaration instances can't hide like templates can.
std::string GetInvalidCallTarget(ImportContext& ictx,
                                 clang::FunctionDecl* function_decl);

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
