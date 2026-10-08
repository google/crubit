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
// functions in `namespace rs` below: `rs::get_if`, `rs::get_unchecked`,
// `rs::holds_alternative`, and `rs::discriminant`.
//
//   // Given the generated binding `Shape` for a Rust
//   // `enum Shape { Point, Circle { radius: i32 }, Rect { w: i32, h: i32 } }`:
//
//   // Construct by naming the variant and passing its fields. `Make` returns
//   // the enum, the variant is built directly in the enum's storage, and
//   // construction is a constant expression.
//   constexpr Shape kCircle = Shape::Circle::Make(7);
//
//   // `Make` is shorthand for the enum's in-place constructor, which is also
//   // public:
//   constexpr Shape kRect(std::in_place_type<Shape::Rect>, 3, 4);
//
//   // Variants cannot be constructed on their own: `Shape::Circle(7)` does not
//   // compile, because it would produce a variant rather than a `Shape` while
//   // reading exactly like the Rust expression that produces one.
//
//   // Ask what is inside.
//   if (const Shape::Circle* circle = rs::get_if<Shape::Circle>(&kCircle)) {
//     Use(circle->radius);  // fields are read directly, with no proxy between
//   }
//
//   // When the active variant is already known, project without the check.
//   Use(rs::get_unchecked<Shape::Rect>(&kRect)->w);
//
// `rs::holds_alternative<Shape::Circle>(s)` answers the yes/no form of
// `get_if`, and `rs::discriminant(s)` returns an opaque, comparable
// `rs::Discriminant<Shape>` (like Rust's `std::mem::discriminant`).
// These decode the tag at runtime, and are not usable in constant
// expressions: that needs C++26 `std::is_within_lifetime`, which not every
// supported compiler provides. Construction and `rs::get_unchecked` are
// constexpr.

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

// Returns a discriminant corresponding to the active variant of `e`.
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

// Returns a pointer to the active variant, similar to `rs::get_if`.
//
// However, unlike `rs::get_if`, `rs::get_unchecked` does not check whether `V`
// is the active variant of the enum. Callers must ensure that `V` is the active
// variant of the enum, or else the behavior is undefined.
template <typename V, typename Enum>
  requires ::crubit::internal::VariantOfEnum<V, Enum>
constexpr auto* crubit_nonnull get_unchecked(Enum* crubit_nonnull e) noexcept {
  if (!std::is_constant_evaluated()) {
    CRUBIT_DCHECK(holds_alternative<V>(*e))
        << "rs::get_unchecked: V is not the active variant";
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
