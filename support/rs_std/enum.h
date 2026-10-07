// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_ENUM_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_ENUM_H_

#include <type_traits>

#include "support/annotations.h"
#include "support/internal/check.h"
#include "support/rs_std/internal/enum.h"  // IWYU pragma: export

// C++ API for Rust enums bound by Crubit.
//
// Build an enum with its variant's `Make`, and inspect it with the free
// functions in `namespace rs` below: `rs::get_if`, `rs::get`,
// `rs::holds_alternative`, and `rs::discriminant`.
//
//   // Given the generated binding `Option` for a Rust
//   // `enum Option { None, Some(i32) }`:
//
//   // Construct by naming the variant and passing its fields. `Make` returns
//   // the enum, the variant is built directly in the enum's storage, and
//   // construction is a constant expression.
//   constexpr Option kSeven = Option::Some::Make(7);
//
//   // `Make` is shorthand for the enum's in-place constructor, which is also
//   // public:
//   constexpr Option kEight(std::in_place_type<Option::Some>, 8);
//
//   // Variants cannot be constructed on their own: `Option::Some(7)` does not
//   // compile, because it would produce a variant rather than an `Option`
//   // while reading exactly like the Rust expression that produces one. See
//   // `crubit::internal::VariantKey`.
//
//   // Ask what is inside.
//   if (const Option::Some* some = rs::get_if<Option::Some>(&kSeven)) {
//     Use(some->value);   // fields are read directly, with no proxy between
//   }
//
//   // When the active variant is already known, project without the check.
//   Use(rs::get<Option::Some>(&kSeven)->value);
//
// `rs::holds_alternative<Option::Some>(o)` answers the yes/no form of
// `get_if`, and `rs::discriminant(o)` returns an opaque, comparable
// `rs::Discriminant<Option>` (like Rust's `std::mem::discriminant`).
// These decode the tag at runtime, and are not usable in constant
// expressions: that needs C++26 `std::is_within_lifetime`, which not every
// supported compiler provides. Construction and `rs::get` are constexpr.
//
// The generated bindings are built from the machinery in
// support/rs_std/internal/enum.h, which is not part of the
// API.

// =============================================================================
// Public API Operations (rs)
// =============================================================================

namespace rs {

// --- Discriminant Access -----------------------------------------------------

// An opaque value identifying which variant of the enum `E` is active, like
// Rust's `std::mem::Discriminant<E>`.
//
// It can be copied and compared for equality, and nothing else. In particular
// it does not expose the integer underneath: that type is chosen by rustc's
// layout, and callers must not come to depend on it. Get one from an enum with
// `rs::discriminant(e)`.
template <typename E>
  requires ::crubit::internal::RustEnum<E>
class Discriminant {
 public:
  friend constexpr bool operator==(const Discriminant&,
                                   const Discriminant&) = default;

 private:
  friend struct ::crubit::internal::EnumAccess;

  constexpr explicit Discriminant(typename E::Discriminant value) noexcept
      : value_(value) {}

  typename E::Discriminant value_;
};

// Recovers the active variant's discriminant. Runtime-only (see
// `VariadicUnionStorage::discriminant`).
template <typename E>
  requires ::crubit::internal::RustEnum<E>
Discriminant<E> discriminant(const E& e) noexcept {
  return ::crubit::internal::EnumAccess::wrap_discriminant<Discriminant<E>>(
      ::crubit::internal::EnumAccess::storage(e).discriminant());
}

// --- Inspection & Projection -------------------------------------------------

// Runtime-only, like `discriminant`.
template <typename V, typename Enum>
  requires ::crubit::internal::VariantOfEnum<V, Enum>
bool holds_alternative(const Enum& e) noexcept {
  return ::crubit::internal::EnumAccess::storage(e).discriminant() ==
         ::crubit::internal::EnumAccess::variant_discriminant<V>();
}

// Projects to `V` without consulting the discriminant (except in debug builds,
// see below). `Enum` may be const-qualified, in which case so is the result.
//
// Precondition: `V` is the active variant. Unlike `std::get` on a variant this
// is only verified in debug builds (by `CRUBIT_DCHECK`); in optimized builds
// violating it is undefined behavior -- use `get_if` when the active variant is
// not already known. (In a constant expression the compiler catches a wrong
// `V`, since it reads an inactive union member.)
template <typename V, typename Enum>
  requires ::crubit::internal::VariantOfEnum<V, Enum>
constexpr auto* crubit_nonnull get(Enum* crubit_nonnull e) noexcept {
  if (!std::is_constant_evaluated()) {
    CRUBIT_DCHECK(holds_alternative<V>(*e))
        << "rs::get: V is not the active variant";
  }
  return ::crubit::internal::EnumAccess::storage(*e).template get<V>();
}

// Returns a pointer to the `V` held by `*e`, or null if `e` is null or holds a
// different variant. `Enum` may be const-qualified, in which case so is the
// result. Runtime-only, like `discriminant`.
template <typename V, typename Enum>
  requires ::crubit::internal::VariantOfEnum<V, Enum>
auto* crubit_nullable get_if(Enum* crubit_nullable e) noexcept {
  return e != nullptr
             ? ::crubit::internal::EnumAccess::storage(*e).template get_if<V>()
             : nullptr;
}

}  // namespace rs

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_ENUM_H_
