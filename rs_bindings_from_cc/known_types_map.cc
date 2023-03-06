// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/known_types_map.h"

#include "absl/container/flat_hash_map.h"

namespace crubit {

// A mapping of C++ standard types to their equivalent Rust types.
// To produce more idiomatic results, these types receive special handling
// instead of using the generic type mapping mechanism.
std::optional<absl::string_view> MapKnownCcTypeToRsType(
    absl::string_view cc_type) {
  static const auto* const kWellKnownTypes =
      new absl::flat_hash_map<absl::string_view, absl::string_view>({
          // TODO(lukasza): Try to deduplicate the entries below - for example:
          // - Try to unify `std::int32_t` and `int32_t`
          // - Try to unify `class rs_std::rs_char` and `rs_std::rs_char`
          // One approach would be to desugar the types before calling
          // `MapKnownCcTypeToRsType`, but note that desugaring of type aliases
          // may be undesirable (i.e.  we may want the bindings to refer to
          // `TypeAlias` rather than directly to the type that it desugars to).
          // Note that b/254096006 tracks desire to preserve type aliases in
          // `cc_bindings_from_rs`.
          {"ptrdiff_t", "isize"},
          {"intptr_t", "isize"},
          {"size_t", "usize"},
          {"uintptr_t", "usize"},
          {"std::ptrdiff_t", "isize"},
          {"std::intptr_t", "isize"},
          {"std::size_t", "usize"},
          {"std::uintptr_t", "usize"},

          {"int8_t", "i8"},
          {"int16_t", "i16"},
          {"int32_t", "i32"},
          {"int64_t", "i64"},
          {"std::int8_t", "i8"},
          {"std::int16_t", "i16"},
          {"std::int32_t", "i32"},
          {"std::int64_t", "i64"},

          {"uint8_t", "u8"},
          {"uint16_t", "u16"},
          {"uint32_t", "u32"},

          {"uint64_t", "u64"},
          {"std::uint8_t", "u8"},
          {"std::uint16_t", "u16"},
          {"std::uint32_t", "u32"},
          {"std::uint64_t", "u64"},

          {"char16_t", "u16"},
          {"char32_t", "u32"},
          {"wchar_t", "i32"},

          // `class rs_std::rs_char` key covers direct usage of
          // `rs_std::rs_char`.  `rs_std::rs_char` key covers scenarios when
          // `using` has imported `rs_char` into another namespace.  See also
          // the deduplication TODO comment above.
          {"class rs_std::rs_char", "char"},
          {"rs_std::rs_char", "char"},
      });
  auto it = kWellKnownTypes->find(cc_type);
  if (it == kWellKnownTypes->end()) return std::nullopt;
  return it->second;
}

}  // namespace crubit
