// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/type_map.h"

#include <optional>
#include <string>

#include "absl/container/flat_hash_map.h"
#include "absl/strings/string_view.h"
#include "rs_bindings_from_cc/ir.h"
#include "clang/AST/Type.h"

namespace crubit {

namespace {
// Returns the Primitive enum if the type is a known primitive type.
std::optional<Primitive> LookupPrimitive(absl::string_view cpp_type) {
  static const auto* const kWellKnownTypes =
      new absl::flat_hash_map<absl::string_view, Primitive>({
          // TODO(lukasza): Try to deduplicate the entries below - for example:
          // - Try to unify `std::int32_t` and `int32_t`
          // One approach would be to desugar the types before calling
          // `LookupPrimitive`, but note that desugaring of type aliases
          // may be undesirable (i.e.  we may want the bindings to refer to
          // `TypeAlias` rather than directly to the type that it desugars to).
          // Note that b/254096006 tracks desire to preserve type aliases in
          // `cc_bindings_from_rs`.
          {"ptrdiff_t", Primitive::PTRDIFF_T},
          {"intptr_t", Primitive::INTPTR_T},
          {"size_t", Primitive::SIZE_T},
          {"uintptr_t", Primitive::UINTPTR_T},
          {"std::ptrdiff_t", Primitive::STD_PTRDIFF_T},
          {"std::intptr_t", Primitive::STD_INTPTR_T},
          {"std::size_t", Primitive::STD_SIZE_T},
          {"std::uintptr_t", Primitive::STD_UINTPTR_T},
          {"int8_t", Primitive::INT8_T},
          {"int16_t", Primitive::INT16_T},
          {"int32_t", Primitive::INT32_T},
          {"int64_t", Primitive::INT64_T},
          {"std::int8_t", Primitive::STD_INT8_T},
          {"std::int16_t", Primitive::STD_INT16_T},
          {"std::int32_t", Primitive::STD_INT32_T},
          {"std::int64_t", Primitive::STD_INT64_T},
          {"uint8_t", Primitive::UINT8_T},
          {"uint16_t", Primitive::UINT16_T},
          {"uint32_t", Primitive::UINT32_T},
          {"uint64_t", Primitive::UINT64_T},
          {"std::uint8_t", Primitive::STD_UINT8_T},
          {"std::uint16_t", Primitive::STD_UINT16_T},
          {"std::uint32_t", Primitive::STD_UINT32_T},
          {"std::uint64_t", Primitive::STD_UINT64_T},
      });
  auto it = kWellKnownTypes->find(cpp_type);
  if (it == kWellKnownTypes->end()) {
    return std::nullopt;
  }
  return it->second;
}

}  // namespace

std::optional<CcType> GetPrimitive(const clang::Type& cpp_type) {
  std::string type_string = clang::QualType(&cpp_type, 0).getAsString();
  if (std::optional<Primitive> p = LookupPrimitive(type_string)) {
    return CcType(*p);
  }
  return std::nullopt;
}

}  // namespace crubit
