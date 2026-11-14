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
#include <span>    // NOLINT(build/c++20); <internal link>
#include <string_view>
#include <type_traits>

#include "support/internal/check.h"
#include "support/internal/check_no_mutable_aliasing.h"
#include "support/internal/cpp_config.h"
#include "support/annotations.h"

namespace crubit::internal {
// See the `SliceRef(SliceRefDanglingTag, uintptr_t)` constructor.
struct SliceRefDanglingTag {};
}  // namespace crubit::internal

namespace rs {

namespace internal {
template <typename T>
concept ByteSliceTarget = ::std::is_const_v<T> && sizeof(T) == 1 &&
                          !::std::is_same_v<::std::remove_cv_t<T>, char>;
}  // namespace internal

// `::rs::SliceRef` is a C++ representation of a pointer or reference to a
// Rust slice. `SliceRef<int const>` is like a `&[c_int]` or `*const [c_int]`,
// while `SliceRef<int>` is like a `&mut [c_int]` or `*mut [c_int]`. `SliceRef`
// is trivially destructible, copyable, and moveable.
// `rust_builtin_type_abi_assumptions.md` documents the ABI compatibility of
// these types.
template <typename T>
class CRUBIT_INTERNAL_RUST_TYPE("&[]", T) CRUBIT_TRIVIAL_ABI CRUBIT_VIEW
    SliceRef final {
 public:
  using element_type = T;

  // Creates a default `SliceRef` - one that represents an empty slice.
  // To mirror slices in Rust, the data pointer is not null.
  constexpr SliceRef() noexcept : dangling_ptr_(alignof(T)), size_(0) {}

  // Only for generated bindings. Like `SliceRef()`, but takes `alignof(T)` as
  // `align` instead of computing it, so `T` may be incomplete. Generated
  // bindings use it to default-initialize struct fields whose element type is
  // defined later in the header, e.g. because it refers back to the struct.
  // They take `align` from rustc's layout, and check it against `alignof(T)`
  // once `T` is complete.
  constexpr SliceRef(::crubit::internal::SliceRefDanglingTag,
                     uintptr_t align) noexcept
      : dangling_ptr_(align), size_(0) {}

  // Primary constructor that initializes dangling_ptr_ for empty slices to
  // satisfy Rust's non-null slice invariant, or a ptr_ otherwise.
  constexpr SliceRef(T* crubit_nullable ptr, size_t size) noexcept
      : dangling_ptr_(alignof(T)), size_(ptr == nullptr ? 0 : size) {
    if (size_ > 0) {
      ptr_ = ptr;
    }
  }

  // Style waiver for implicit conversions granted in cl/662479273.
  //
  // This is a template, rather than taking `::std::span<T>`, so that copying
  // or moving a `SliceRef` doesn't consider converting it to `::std::span<T>`.
  // That would check whether `SliceRef<T>` is a range, which instantiates `T`
  // (see the converting constructor templates below).
  template <typename Span>
    requires(::std::same_as<Span, ::std::span<T>>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(Span span) noexcept : SliceRef(span.data(), span.size()) {}

  // Implicit conversion from mutable SliceRef to const SliceRef.
  //
  // Like the `::std::span<T>` constructor above, this is a template so that
  // copying or moving a `SliceRef<const T>` doesn't consider converting it to
  // `SliceRef<T>`, which would check `SliceRef<T>`'s converting constructors.
  template <typename U>
    requires(::std::is_const_v<T> &&
             ::std::same_as<U, ::std::remove_const_t<T>>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(const SliceRef<U>& other) noexcept
      : SliceRef(other.data(), other.size()) {}

  // Implicitly converts string-like types (literals, views) to SliceRef<const
  // T> in one conversion step.
  //
  // Each converting constructor template below rejects `SliceRef` itself
  // first. Constraints are checked in order, so copying or moving a `SliceRef`
  // never checks the other constraints, which need `T` to be complete (e.g. via
  // `sizeof(T)`, or ADL on `SliceRef<T>` in `std::ranges::begin`). Generated
  // bindings copy `SliceRef<T>` struct fields before `T` is defined.
  //
  // Style waiver for implicit conversions granted in cl/662479273.
  template <typename S>
    requires(!::std::same_as<::std::remove_cvref_t<S>, SliceRef> &&
             internal::ByteSliceTarget<T> &&
             ::std::convertible_to<S, ::std::string_view> &&
             ::std::is_trivially_copyable_v<::std::remove_cvref_t<S>>)
  // Non-owning views (pointers, string views, and literals) are trivially
  // copyable. These do not need lifetime annotation.
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(S str) noexcept
      : SliceRef(reinterpret_cast<T*>(::std::string_view(str).data()),
                 ::std::string_view(str).size()) {}

  template <typename S>
    requires(!::std::same_as<::std::remove_cvref_t<S>, SliceRef> &&
             internal::ByteSliceTarget<T> &&
             ::std::convertible_to<const S&, ::std::string_view> &&
             !::std::is_trivially_copyable_v<::std::remove_cvref_t<S>>)
  // Owning containers (like ::std::string) are not trivially copyable. These
  // are marked with CRUBIT_LIFETIME_BOUND to prevent temporaries from outliving
  // the SliceRef. NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(const S& str CRUBIT_LIFETIME_BOUND) noexcept
      : SliceRef(reinterpret_cast<T*>(::std::string_view(str).data()),
                 ::std::string_view(str).size()) {}

  // Implicit conversion from views.
  template <typename View>
  // NOLINTNEXTLINE(build/c++20)
    requires(!::std::same_as<::std::remove_cvref_t<View>, SliceRef> &&
             !::std::is_same_v<::std::decay_t<View>, ::std::span<T>> &&
             ::std::convertible_to<View &&, ::std::span<T>> &&
             ::std::ranges::view<::std::decay_t<View>>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(View&& view) noexcept
      : SliceRef(static_cast<::std::span<T>>(::std::forward<View>(view))) {}

  // Explicit conversion from views (if only explicitly convertible to
  // ::std::span).
  template <typename View>
  // NOLINTNEXTLINE(build/c++20)
    requires(!::std::same_as<::std::remove_cvref_t<View>, SliceRef> &&
             !::std::is_same_v<::std::decay_t<View>, ::std::span<T>> &&
             !::std::convertible_to<View &&, ::std::span<T>> &&
             ::std::constructible_from<::std::span<T>, View &&> &&
             ::std::ranges::view<::std::decay_t<View>>)
  constexpr explicit SliceRef(View&& view) noexcept
      : SliceRef(::std::span<T>(::std::forward<View>(view))) {}

  // Implicit conversion from non-view containers (only allowed if T is const).
  template <typename Container>
  // NOLINTNEXTLINE(build/c++20)
    requires(!::std::same_as<::std::remove_cvref_t<Container>, SliceRef> &&
             ::std::is_const_v<T> &&
             ::std::convertible_to<Container &&, ::std::span<T>> &&
             !::std::ranges::view<::std::decay_t<Container>>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(Container&& container CRUBIT_LIFETIME_BOUND) noexcept
      : SliceRef(
            static_cast<::std::span<T>>(::std::forward<Container>(container))) {
  }

  // Explicit conversion from non-view containers.
  //
  // Conversion must be explicit if T is mutable or if T is only explicitly
  // convertible to ::std::span.
  template <typename Container>
  // NOLINTNEXTLINE(build/c++20)
    requires(!::std::same_as<::std::remove_cvref_t<Container>, SliceRef> &&
             ((!::std::is_const_v<T> &&
               ::std::constructible_from<::std::span<T>, Container &&>) ||
              (::std::is_const_v<T> &&
               ::std::constructible_from<::std::span<T>, Container &&> &&
               !::std::convertible_to<Container &&, ::std::span<T>>)) &&
             !::std::ranges::view<::std::decay_t<Container>>)
  constexpr explicit SliceRef(
      Container&& container CRUBIT_LIFETIME_BOUND) noexcept
      : SliceRef(::std::span<T>(::std::forward<Container>(container))) {}

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr operator ::std::span<T>() const noexcept {
    return ::std::span<T>(size_ > 0 ? ptr_ : nullptr, size_);
  }

  constexpr SliceRef(const SliceRef&) = default;
  constexpr SliceRef& operator=(const SliceRef&) = default;
  constexpr SliceRef(SliceRef&&) noexcept = default;
  constexpr SliceRef& operator=(SliceRef&&) noexcept = default;
  ~SliceRef() = default;

  constexpr T* crubit_nullable data() const noexcept {
    return size_ > 0 ? ptr_ : nullptr;
  }
  constexpr size_t size() const noexcept { return size_; }

  using value_type = ::std::remove_cv_t<T>;
  using iterator = T* crubit_nullable;
  using const_iterator = const T* crubit_nullable;
  constexpr iterator begin() const noexcept { return data(); }
  constexpr iterator end() const noexcept { return data() + size(); }
  constexpr const_iterator cbegin() const noexcept { return data(); }
  constexpr const_iterator cend() const noexcept { return data() + size(); }
  constexpr bool empty() const noexcept { return size() == 0; }
  constexpr T& operator[](size_t i) const noexcept {
    CRUBIT_CHECK(i < size());
    return data()[i];
  }

  CRUBIT_DO_NOT_BIND constexpr ::std::span<T> to_span() const noexcept {
    return ::std::span<T>(data(), size());
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
  return ::std::equal(a.data(), a.data() + a.size(), b.data());
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
  requires(::std::convertible_to<U, SliceRef<const T>>)
constexpr bool operator==(const U& a, SliceRef<T> b) {
  return internal::EqualImpl(a, b);
}
template <typename T, typename U>
  requires(::std::convertible_to<U, SliceRef<const T>>)
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
  requires(::std::convertible_to<U, SliceRef<const T>>)
constexpr bool operator!=(const U& a, SliceRef<T> b) {
  return !(a == b);
}
template <typename T, typename U>
  requires(::std::convertible_to<U, SliceRef<const T>>)
constexpr bool operator!=(SliceRef<T> a, const U& b) {
  return !(a == b);
}

}  // namespace rs

namespace crubit::internal {

template <typename T>
struct PtrLike<::rs::SliceRef<T>> {
  static constexpr bool kIsConst = ::std::is_const_v<T>;
  static PtrData AsPtrData(::rs::SliceRef<T> t) {
    uintptr_t start = reinterpret_cast<uintptr_t>(t.data());
    return {
        .start = start,
        .end = start + t.size() * sizeof(T),
    };
  }
};

}  // namespace crubit::internal

namespace rs_std {
#ifdef CRUBIT_INTERNAL_RS_NAMESPACE
template <typename T>
using SliceRef [[deprecated("Use rs::SliceRef instead")]] = ::rs::SliceRef<T>;
#else
template <typename T>
using SliceRef = ::rs::SliceRef<T>;
#endif
}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_SLICEREF_H_
