// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_ENUM_TEST_OPTION_I32_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_ENUM_TEST_OPTION_I32_H_

#include <cstddef>
#include <cstdint>
#include <utility>

#include "support/rs_std/enum.h"

// Hand-written stand-in for what Crubit generates for a Rust
// `enum Option { None, Some(i32) }`, used by enum_test.cc. It lives in a test
// namespace, not `rs`, so that it cannot collide with a real binding.
namespace crubit::enum_test {

// Only the `int32_t` binding exists.
template <typename T>
class Option;

template <>
class Option<int32_t> {
 public:
  using Discriminant = uint32_t;

  // Every variant constructor takes a `VariantKey` first, which only the
  // enum's storage can supply. That makes the in-place constructor below the
  // only way to build a variant, so `Option::Some(7)` -- which would produce a
  // variant while reading like the Rust expression that produces an
  // `Option` -- does not compile. `Option::Some::Make(7)`, inherited from
  // `::crubit::internal::Variant`, is the spelling that does.
  //
  // Copy and move are private, so a variant cannot be copied or moved out of
  // its enum either; `Option` itself is still trivially copyable.
  struct None : public ::crubit::internal::Variant<None, Option<int32_t>, 0> {
    constexpr explicit None(::crubit::internal::VariantKey) {}

   private:
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    None(const None&) = default;
    None(None&&) = default;
    None& operator=(const None&) = default;
    None& operator=(None&&) = default;

    // NOLINTNEXTLINE(clang-diagnostic-unused-private-field)
    uint32_t tag = 0;
  };

  struct Some : public ::crubit::internal::Variant<Some, Option<int32_t>, 1> {
    constexpr explicit Some(::crubit::internal::VariantKey, int32_t val)
        : value(val) {}

   private:
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Some(const Some&) = default;
    Some(Some&&) = default;
    Some& operator=(const Some&) = default;
    Some& operator=(Some&&) = default;

    // NOLINTNEXTLINE(clang-diagnostic-unused-private-field)
    uint32_t tag = 1;

   public:
    int32_t value;
  };

  // The only constructor, and there is exactly one no matter how many variants
  // an enum has: the variant is named by the `std::in_place_type_t<V>` tag
  // rather than by an overload, and `args` are forwarded to `V`'s constructor
  // running directly on the storage bytes.
  template <typename V, typename... Args>
    requires ::crubit::internal::ConstructibleVariantOf<V, Option, Args...>
  constexpr explicit Option(std::in_place_type_t<V> v, Args&&... args)
      : storage_(v, std::forward<Args>(args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;

  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<uint32_t, /*TagOffset=*/0>, None,
      Some>
      storage_;
};

// Layout taken from rustc, not derived by hand (`rustc -Zprint-type-sizes`):
//
//   type `Option<i32>`: 8 bytes, alignment: 4 bytes
//       discriminant: 4 bytes
//       variant `Some`: 4 bytes
//           field `.0`: 4 bytes
//       variant `None`: 0 bytes
//
// The field offset is asserted, not just the size: a variant that agreed on
// size while putting its payload somewhere else would read the wrong bytes
// with nothing to catch it. Real generated code would spell this
// `CRUBIT_OFFSET_OF` (crubit/support/internal/offsetof.h), which tolerates
// commas in the type name; plain `offsetof` is enough for a name this simple.
static_assert(sizeof(Option<int32_t>) == 8);
static_assert(alignof(Option<int32_t>) == 4);
static_assert(offsetof(Option<int32_t>::Some, value) == 4);

}  // namespace crubit::enum_test

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_ENUM_TEST_OPTION_I32_H_
