// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/annotations_consumer.h"

#include <optional>
#include <string>
#include <utility>

#include "common/annotation_reader.h"
#include "common/check.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/DeclBase.h"
#include "llvm/ADT/DenseSet.h"
#include "llvm/ADT/StringRef.h"

namespace crubit {

static constexpr llvm::StringRef kCrubitRustNameTag = "crubit_rust_name";

const llvm::DenseSet<llvm::StringRef>& RustOperatorSymbols() {
  static const auto* const kRustOperatorSymbols =
      new llvm::DenseSet<llvm::StringRef>({
          // Some of the symbols are not supported by Crubit, but let's include
          // them here for completeness so that we don't have to keep them in
          // sync.
          //
          // Note: Should we name traits in `CRUBIT_RUST_NAME` instead? What
          // about `Index`, `Fn` etc.?
          "!",  "!=", "%",  "%=",  "&",   "&=", "*",  "*=",  "+",  "+=",
          "-",  "-=", "..", "..=", "/",   "/=", "<<", "<<=", "<",  "<=",
          "==", ">",  ">=", ">>",  ">>=", "^",  "^=", "|",   "|=",
      });
  return *kRustOperatorSymbols;
}

std::optional<UnqualifiedIdentifier> CrubitRustName(const clang::Decl& decl) {
  auto crubit_rust_name = GetAnnotationWithStringArg(decl, kCrubitRustNameTag);
  CRUBIT_CHECK(crubit_rust_name)
      << llvm::toString(crubit_rust_name.takeError());
  if (!crubit_rust_name->has_value()) {
    return std::nullopt;
  }

  if (RustOperatorSymbols().contains(**crubit_rust_name)) {
    return UnqualifiedIdentifier(Operator(**crubit_rust_name));
  }

  return UnqualifiedIdentifier(Identifier(**std::move(crubit_rust_name)));
}

}  // namespace crubit
