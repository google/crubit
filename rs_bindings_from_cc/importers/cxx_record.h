// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_

#include <optional>
#include <string>
#include <vector>

#include "absl/functional/any_invocable.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "rs_bindings_from_cc/decl_importer.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Decl.h"
#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"

namespace crubit {

namespace internal {
// Determines the Rust bridge path for a given Proto2 message declaration.
// Resolves nesting using `is_parent_proto` which checks whether a given prefix
// corresponds to an enclosing protobuf message.
std::string GetProto2MessageRustNameImpl(
    absl::string_view message_name,
    absl::AnyInvocable<bool(absl::string_view)> is_parent_proto);
}  // namespace internal

// A `DeclImporter` for `CXXRecordDecl`s.
class CXXRecordDeclImporter : public DeclImporterBase<clang::CXXRecordDecl> {
 public:
  explicit CXXRecordDeclImporter(ImportContext& context)
      : DeclImporterBase(context) {}
  std::optional<IR::Item> Import(clang::CXXRecordDecl*) override;

 private:
  std::vector<Field> ImportFields(clang::CXXRecordDecl*);
  std::vector<BaseClass> GetUnambiguousPublicBases(
      const clang::CXXRecordDecl& record_decl) const;
  std::optional<Identifier> GetTranslatedFieldName(
      const clang::FieldDecl* field);

  // Returns the bridge type of the given `decl` if it is a builtin bridge type
  // (e.g., `std::optional`). Otherwise, returns `std::nullopt`.
  absl::StatusOr<std::optional<BridgeType>> GetBuiltinBridgeType(
      const clang::ClassTemplateSpecializationDecl* decl);

  absl::StatusOr<SafetyAnnotation> GetSafetyAnnotation(const clang::Decl& decl);
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_
