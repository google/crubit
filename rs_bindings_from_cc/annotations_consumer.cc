// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/annotations_consumer.h"

#include <optional>
#include <string>
#include <utility>

#include "absl/base/no_destructor.h"
#include "absl/container/flat_hash_set.h"
#include "absl/log/check.h"
#include "absl/strings/string_view.h"
#include "common/annotation_reader.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/DeclBase.h"

namespace crubit {

static constexpr absl::string_view kCrubitRustNameTag = "crubit_rust_name";

const absl::flat_hash_set<absl::string_view>& RustOperatorSymbols() {
  static const absl::NoDestructor<absl::flat_hash_set<absl::string_view>>
      kRustOperatorSymbols({
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
  absl::StatusOr<std::optional<std::string>> crubit_rust_name =
      GetAnnotationWithStringArg(decl, kCrubitRustNameTag);
  CHECK_OK(crubit_rust_name);
  if (!crubit_rust_name->has_value()) {
    return std::nullopt;
  }

  if (RustOperatorSymbols().contains(**crubit_rust_name)) {
    return Operator(**std::move(crubit_rust_name));
  }

  return Identifier(**std::move(crubit_rust_name));
}

}  // namespace crubit
