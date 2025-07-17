// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/annotations_consumer.h"

#include <optional>
#include <string>
#include <utility>

#include "absl/log/check.h"
#include "absl/strings/string_view.h"
#include "common/annotation_reader.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/DeclBase.h"

namespace crubit {

static constexpr absl::string_view kCrubitRustNameTag = "crubit_rust_name";

std::optional<Identifier> CrubitRustName(const clang::Decl& decl) {
  absl::StatusOr<std::optional<std::string>> crubit_rust_name =
      GetAnnotationWithStringArg(decl, kCrubitRustNameTag);
  CHECK_OK(crubit_rust_name);
  if (!crubit_rust_name->has_value()) {
    return std::nullopt;
  }
  return Identifier(**std::move(crubit_rust_name));
}

}  // namespace crubit
