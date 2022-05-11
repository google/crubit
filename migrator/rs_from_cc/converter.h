// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_MIGRATOR_RS_FROM_CC_CONVERTER_H_
#define CRUBIT_MIGRATOR_RS_FROM_CC_CONVERTER_H_

#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/container/flat_hash_set.h"
#include "absl/status/statusor.h"
#include "absl/types/span.h"
#include "lifetime_annotations/lifetime_annotations.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/Decl.h"
#include "clang/AST/Mangle.h"
#include "clang/AST/RawCommentList.h"
#include "clang/AST/Type.h"
#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/Specifiers.h"
#include "clang/Sema/Sema.h"

namespace crubit_rs_from_cc {

// Visits the C++ AST and generates the corresponding Rust code in the
// Invocation object.
class Converter {
 public:
  // Top-level parameters as well as return value of a migrator invocation.
  class Invocation {
   public:
    std::string rs_code_;
  };

  explicit Converter(Invocation& invocation, clang::ASTContext& ctx)
      : result_(invocation.rs_code_), ctx_(ctx) {}

  // Import all visible declarations from a translation unit.
  void Convert(const clang::TranslationUnitDecl* decl);

  // "converts" a not-yet-supported declaration to Rust by dumping the C++ AST
  // into a comment.
  void ConvertUnsupported(const clang::Decl* decl);

 private:
  // The main output of the conversion process (Rust code).
  std::string& result_;

  clang::ASTContext& ctx_;
};  // class Converter

}  // namespace crubit_rs_from_cc

#endif  // CRUBIT_MIGRATOR_RS_FROM_CC_CONVERTER_H_
