// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_

#include <memory>
#include <optional>
#include <string>
#include <vector>

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
    llvm::StringRef message_name,
    llvm::unique_function<bool(llvm::StringRef)> is_parent_proto);
}  // namespace internal

// A `DeclImporter` for `CXXRecordDecl`s.
class CXXRecordDeclImporter : public DeclImporterBase<clang::CXXRecordDecl> {
 public:
  explicit CXXRecordDeclImporter(ImportContext& context)
      : DeclImporterBase(context) {}
  std::unique_ptr<crubit::ffi::Item> Import(clang::CXXRecordDecl*) override;

 private:
  std::vector<crubit::ffi::Field> ImportFields(clang::CXXRecordDecl*);
  std::vector<crubit::ffi::BaseClass> GetUnambiguousPublicBases(
      const clang::CXXRecordDecl& record_decl) const;
  std::optional<Identifier> GetTranslatedFieldName(
      const clang::FieldDecl* field);

  // Returns the bridge type of the given `decl` if it is a builtin bridge type
  // (e.g., `std::optional`). Otherwise, returns `std::nullopt`.
  llvm::Expected<std::optional<BridgeType>> GetBuiltinBridgeType(
      const clang::ClassTemplateSpecializationDecl* decl);

  llvm::Expected<SafetyAnnotation> GetSafetyAnnotation(const clang::Decl& decl);
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_CXX_RECORD_H_
