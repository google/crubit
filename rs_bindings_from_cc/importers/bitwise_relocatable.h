// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_BITWISE_RELOCATABLE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_BITWISE_RELOCATABLE_H_

#include <optional>
#include <string>
#include <vector>

#include "clang/AST/DeclCXX.h"
#include "clang/AST/DeclTemplate.h"
#include "llvm/ADT/DenseMap.h"

namespace crubit {

class ImportContext;

struct BitwiseRelocatableResult {
  enum class Kind {
    kYes,
    kNo,
    kError,
  };
  Kind kind = Kind::kNo;
  std::string reason;
  std::vector<std::string> libcxx_only_fields;
};

class RelocatabilityAnalysis {
 public:
  RelocatabilityAnalysis() = default;

  BitwiseRelocatableResult GetBitwiseRelocatability(
      ImportContext& ictx, const clang::CXXRecordDecl* record_decl);

 private:
  llvm::DenseMap<const clang::CXXRecordDecl*, BitwiseRelocatableResult> cache_;

  BitwiseRelocatableResult ComputeBitwiseRelocatability(
      ImportContext& ictx, const clang::CXXRecordDecl* record_decl);

  std::optional<BitwiseRelocatableResult> CheckStdTable(
      ImportContext& ictx, const clang::ClassTemplateSpecializationDecl* spec);
};

}  // namespace crubit

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IMPORTERS_BITWISE_RELOCATABLE_H_
