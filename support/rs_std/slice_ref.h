// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_SLICEREF_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_SLICEREF_H_

#include <concepts>
#include <cstddef>
#include <cstdint>
#include <span>  // NOLINT(build/c++20); <internal link>
#include <type_traits>

#include "crubit/support/annotations_internal.h"
#include "absl/base/attributes.h"
#include "absl/types/span.h"
#include "support/internal/check_no_mutable_aliasing.h"

namespace rs_std {

// `rs_std::SliceRef` is a C++ representation of a pointer or reference to a
// Rust slice. `SliceRef<int const>` is like a `&[c_int]` or `*const [c_int]`,
// while `SliceRef<int>` is like a `&mut [c_int]` or `*mut [c_int]`. `SliceRef`
// is trivially destructible, copyable, and moveable.
// `rust_builtin_type_abi_assumptions.md` documents the ABI compatibility of
// these types.
template <typename T>
class CRUBIT_INTERNAL_RUST_TYPE("&[]")
    ABSL_ATTRIBUTE_TRIVIAL_ABI SliceRef final {
 public:
  // Creates a default `SliceRef` - one that represents an empty slice.
  // To mirror slices in Rust, the data pointer is not null.
  constexpr SliceRef() noexcept : dangling_ptr_(alignof(T)), size_(0) {}

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(absl::Span<T> span) noexcept
      // Store a dangling pointer assuming `span` is empty-- we have to
      // initialize the union to something.
      : dangling_ptr_(alignof(T)), size_(span.size()) {
    // Store a valid pointer when `span` is not empty.
    if (!span.empty()) {
      ptr_ = span.data();
    }
  }

  // Re-use implicit conversions to `absl::Span`. Prevent a delegation circle
  // by excluding `absl::Span<T>` as the converted type.
  template <typename Container>
    requires(std::convertible_to<Container &&, absl::Span<T>> &&
             !std::is_same_v<Container, absl::Span<T>>)
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr SliceRef(Container&& container) noexcept
      : SliceRef(
            // This is using `static_cast` instead of `absl::implicit_cast` to
            // avoid a dependency on `absl/base/casts.h`, which has a lot of
            // transitive dependencies. Doing so is safe, because the extra
            // guarantees are already checked by std::convertible_to.
            static_cast<absl::Span<T>>(std::forward<Container>(container))) {}

  // Also mirror explicit conversions from `absl::Span`.
  template <typename Container>
    requires(std::constructible_from<absl::Span<T>, Container &&> &&
             !std::convertible_to<Container &&, absl::Span<T>> &&
             !std::is_same_v<Container, absl::Span<T>>)
  constexpr explicit SliceRef(Container&& container) noexcept
      : SliceRef(absl::Span<T>(std::forward<Container>(container))) {}

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr operator std::span<T>() const noexcept {
    return std::span<T>(size_ > 0 ? ptr_ : nullptr, size_);
  }

  constexpr SliceRef(const SliceRef&) = default;
  constexpr SliceRef& operator=(const SliceRef&) = default;
  constexpr SliceRef(SliceRef&&) noexcept = default;
  constexpr SliceRef& operator=(SliceRef&&) noexcept = default;
  ~SliceRef() = default;

  constexpr T* data() const noexcept { return size_ > 0 ? ptr_ : nullptr; }
  constexpr size_t size() const noexcept { return size_; }

  constexpr absl::Span<T> to_span() const noexcept {
    return absl::Span<T>(data(), size());
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
    T* ptr_;
    uintptr_t dangling_ptr_;
  };
  size_t size_;
};

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
