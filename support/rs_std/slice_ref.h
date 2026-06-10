// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/slice_ref.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_SLICEREF_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_SLICEREF_H_

#include <concepts>
#include <cstddef>
#include <cstdint>
#include <ranges>  // NOLINT(build/c++20); <internal link>
#include <span>  // NOLINT(build/c++20); <internal link>
#include <string_view>
#include <type_traits>

#include "support/annotations.h"
#include "support/internal/check_no_mutable_aliasing.h"

namespace rs_std {

// `rs_std::SliceRef` is a C++ representation of a pointer or reference to a
// Rust slice. `SliceRef<int const>` is like a `&[c_int]` or `*const [c_int]`,
// while `SliceRef<int>` is like a `&mut [c_int]` or `*mut [c_int]`. `SliceRef`
// is trivially destructible, copyable, and moveable.
// `rust_builtin_type_abi_assumptions.md` documents the ABI compatibility of
// these types.
template <typename T>
class CRUBIT_INTERNAL_RUST_TYPE("&[]", T) CRUBIT_TRIVIAL_ABI CRUBIT_VIEW
    SliceRef final {
 public:
  // Creates a default `SliceRef` - one that represents an empty slice.
  // To mirror slices in Rust, the data pointer is not null.
  constexpr SliceRef() noexcept : dangling_ptr_(alignof(T)), size_(0) {}

  // Style waiver for implicit conversions granted in cl/662479273.
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(std::span<T> span) noexcept
      // Store a dangling pointer assuming `span` is empty-- we have to
      // initialize the union to something.
      : dangling_ptr_(alignof(T)), size_(span.size()) {
    // Store a valid pointer when `span` is not empty.
    if (!span.empty()) {
      ptr_ = span.data();
    }
  }

  // Implicit conversion from mutable SliceRef to const SliceRef.
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(const SliceRef<std::remove_const_t<T>>& other) noexcept
    requires(std::is_const_v<T>)
      : SliceRef(std::span<T>(other.data(), other.size())) {}

  // Explicit conversion from `std::string_view` in order to avoid
  // marking this case as `CRUBIT_LIFETIME_BOUND`.
  //
  // Note that `std::span` solves this using an `EnableIfIsView` typeclass.
  //
  // Style waiver for implicit conversions granted in cl/662479273.
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(std::string_view str) noexcept
      : dangling_ptr_(alignof(T)), size_(str.size()) {
    if (!str.empty()) {
      ptr_ = str.data();
    }
  }

  // Implicit conversion from views.
  template <typename View>
  // NOLINTNEXTLINE(build/c++20)
    requires(std::ranges::view<std::decay_t<View>> &&
             std::convertible_to<View &&, std::span<T>> &&
             !std::is_same_v<std::decay_t<View>, std::span<T>>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(View&& view) noexcept
      : SliceRef(static_cast<std::span<T>>(std::forward<View>(view))) {}

  // Explicit conversion from views (if only explicitly convertible to
  // std::span).
  template <typename View>
  // NOLINTNEXTLINE(build/c++20)
    requires(std::ranges::view<std::decay_t<View>> &&
             std::constructible_from<std::span<T>, View &&> &&
             !std::convertible_to<View &&, std::span<T>> &&
             !std::is_same_v<std::decay_t<View>, std::span<T>>)
  constexpr explicit SliceRef(View&& view) noexcept
      : SliceRef(std::span<T>(std::forward<View>(view))) {}

  // Implicit conversion from non-view containers (only allowed if T is const).
  template <typename Container>
  // NOLINTNEXTLINE(build/c++20)
    requires(!std::ranges::view<std::decay_t<Container>> &&
             std::is_const_v<T> &&
             std::convertible_to<Container &&, std::span<T>>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(Container&& container CRUBIT_LIFETIME_BOUND) noexcept
      : SliceRef(
            static_cast<std::span<T>>(std::forward<Container>(container))) {}

  // Explicit conversion from non-view containers.
  //
  // Conversion must be explicit if T is mutable or if T is only explicitly
  // convertible to std::span.
  template <typename Container>
  // NOLINTNEXTLINE(build/c++20)
    requires(!std::ranges::view<std::decay_t<Container>> &&
             ((!std::is_const_v<T> &&
               std::constructible_from<std::span<T>, Container &&>) ||
              (std::is_const_v<T> &&
               std::constructible_from<std::span<T>, Container &&> &&
               !std::convertible_to<Container &&, std::span<T>>)))
  constexpr explicit SliceRef(
      Container&& container CRUBIT_LIFETIME_BOUND) noexcept
      : SliceRef(std::span<T>(std::forward<Container>(container))) {}

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr operator std::span<T>() const noexcept {
    return std::span<T>(size_ > 0 ? ptr_ : nullptr, size_);
  }

  constexpr SliceRef(const SliceRef&) = default;
  constexpr SliceRef& operator=(const SliceRef&) = default;
  constexpr SliceRef(SliceRef&&) noexcept = default;
  constexpr SliceRef& operator=(SliceRef&&) noexcept = default;
  ~SliceRef() = default;

  constexpr T* crubit_nullability_unknown data() const noexcept {
    return size_ > 0 ? ptr_ : nullptr;
  }
  constexpr size_t size() const noexcept { return size_; }

  CRUBIT_DO_NOT_BIND constexpr std::span<T> to_span() const noexcept {
    return std::span<T>(data(), size());
  }

 private:
  // Stick to the following invariants when changing the data member values:
  // (1) `ptr_` and `dangling_ptr_` must never be 0 (to mirror slices in Rust).
  // (2) if `size_ > 0` then `ptr_` is a valid pointer, otherwise
  //     `dangling_ptr_` is a dangling pointer.
  //
  // `dangling_ptr_` is never read from in C++, and `ptr_` must only ever be
  // read from when `size_ > 0`.
  union {
    T* crubit_nonnull ptr_;
    uintptr_t dangling_ptr_;
  };
  size_t size_;
};

namespace internal {
template <typename T>
constexpr bool EqualImpl(SliceRef<const T> a, SliceRef<const T> b) {
  if (a.size() != b.size()) return false;
  return std::equal(a.data(), a.data() + a.size(), b.data());
}
}  // namespace internal

template <typename T>
constexpr bool operator==(SliceRef<T> a, SliceRef<T> b) {
  return internal::EqualImpl(a, b);
}
template <typename T>
constexpr bool operator==(SliceRef<const T> a, SliceRef<T> b) {
  return internal::EqualImpl(a, b);
}
template <typename T>
constexpr bool operator==(SliceRef<T> a, SliceRef<const T> b) {
  return internal::EqualImpl(a, b);
}
template <typename T, typename U>
  requires(std::convertible_to<U, SliceRef<const T>>)
constexpr bool operator==(const U& a, SliceRef<T> b) {
  return internal::EqualImpl(a, b);
}
template <typename T, typename U>
  requires(std::convertible_to<U, SliceRef<const T>>)
constexpr bool operator==(SliceRef<T> a, const U& b) {
  return internal::EqualImpl(a, b);
}

template <typename T>
constexpr bool operator!=(SliceRef<T> a, SliceRef<T> b) {
  return !(a == b);
}
template <typename T>
constexpr bool operator!=(SliceRef<const T> a, SliceRef<T> b) {
  return !(a == b);
}
template <typename T>
constexpr bool operator!=(SliceRef<T> a, SliceRef<const T> b) {
  return !(a == b);
}
template <typename T, typename U>
  requires(std::convertible_to<U, SliceRef<const T>>)
constexpr bool operator!=(const U& a, SliceRef<T> b) {
  return !(a == b);
}
template <typename T, typename U>
  requires(std::convertible_to<U, SliceRef<const T>>)
constexpr bool operator!=(SliceRef<T> a, const U& b) {
  return !(a == b);
}

}  // namespace rs_std

namespace crubit::internal {

template <typename T>
struct PtrLike<rs_std::SliceRef<T>> {
  static constexpr bool kIsConst = std::is_const_v<T>;
  static PtrData AsPtrData(rs_std::SliceRef<T> t) {
    uintptr_t start = reinterpret_cast<uintptr_t>(t.data());
    return {
        .start = start,
        .end = start + t.size() * sizeof(T),
    };
  }
};

}  // namespace crubit::internal

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_SLICEREF_H_
