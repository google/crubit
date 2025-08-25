// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/type_map.h"

#include <optional>
#include <string>

#include "absl/container/flat_hash_set.h"
#include "absl/strings/string_view.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Type.h"

namespace crubit {

namespace {
// Returns true if the type is a known Rust type.
bool IsKnownRustType(absl::string_view cpp_type) {
  static const auto* const kWellKnownTypes =
      new absl::flat_hash_set<absl::string_view>({
          // TODO(lukasza): Try to deduplicate the entries below - for example:
          // - Try to unify `std::int32_t` and `int32_t`
          // One approach would be to desugar the types before calling
          // `IsKnownRustType`, but note that desugaring of type aliases
          // may be undesirable (i.e.  we may want the bindings to refer to
          // `TypeAlias` rather than directly to the type that it desugars to).
          // Note that b/254096006 tracks desire to preserve type aliases in
          // `cc_bindings_from_rs`.
          "ptrdiff_t",      "intptr_t",      "size_t",        "uintptr_t",
          "std::ptrdiff_t", "std::intptr_t", "std::size_t",   "std::uintptr_t",
          "int8_t",         "int16_t",       "int32_t",       "int64_t",
          "std::int8_t",    "std::int16_t",  "std::int32_t",  "std::int64_t",
          "uint8_t",        "uint16_t",      "uint32_t",      "uint64_t",
          "std::uint8_t",   "std::uint16_t", "std::uint32_t", "std::uint64_t",
      });
  return kWellKnownTypes->contains(cpp_type);
}

}  // namespace

std::optional<CcType> GetExistingRustType(const clang::Type& cpp_type) {
  std::string type_string = clang::QualType(&cpp_type, 0).getAsString();
  if (IsKnownRustType(type_string)) {
    return CcType(CcType::Primitive{type_string});
  }
  return std::nullopt;
}

}  // namespace crubit
