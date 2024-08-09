// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_SLICEREF_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_SLICEREF_H_

#include <cstddef>
#include <cstdint>
#include <span>  // NOLINT(build/c++20); <internal link>

#include "absl/base/attributes.h"
#include "absl/types/span.h"
#include "support/internal/attribute_macros.h"

namespace rs_std {

// `rs_std::SliceRef` is a C++ representation of a pointer or reference to a
// Rust slice. `SliceRef<int const>` is like a `&[c_int]` or `*const [c_int]`,
// while `SliceRef<int>` is like a `&[c_int]` or `*mut [c_int]`. `SliceRef` is
// trivially destructible, copyable, and moveable.
// `rust_builtin_type_abi_assumptions.md` documents the ABI compatibility of
// these types.
template <typename T>
class CRUBIT_INTERNAL_RUST_TYPE("&[]")
    ABSL_ATTRIBUTE_TRIVIAL_ABI SliceRef final {
 public:
  // Creates a default `SliceRef` - one that represents an empty slice.
  // To mirror slices in Rust, the data pointer is not null.
  constexpr SliceRef() noexcept : ptr_(alignof(T)), size_(0) {}

  // This constructor cannot be constexpr because it calls `reinterpret_cast`.
  // The `reinterpret_cast`, in turn, is needed if we want to keep `ptr_` an
  // `uintptr_t` instead of a `T*`, which we do to avoid storing dangling
  // pointers. The `reinterpret_cast` preserves both invariants required for
  // `ptr_`, because the resulting value comes from a valid, non-null pointer.
  // NOLINTNEXTLINE(google-explicit-constructor)
  SliceRef(absl::Span<T> span) noexcept
      : ptr_(span.empty() ? alignof(T)
                          : reinterpret_cast<uintptr_t>(span.data())),
        size_(span.size()) {}

  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr operator std::span<T>() const {
    return std::span<T>(size_ > 0 ? static_cast<T*>(ptr_) : nullptr, size_);
  }

  constexpr SliceRef(const SliceRef&) = default;
  constexpr SliceRef& operator=(const SliceRef&) = default;
  constexpr SliceRef(SliceRef&&) = default;
  constexpr SliceRef& operator=(SliceRef&&) = default;
  ~SliceRef() = default;

  // The `reinterpret_cast` is safe thanks to invariant (2) (see the definition
  // of `ptr_`).
  T* data() const { return size_ > 0 ? reinterpret_cast<T*>(ptr_) : nullptr; }
  size_t size() const { return size_; }

  absl::Span<T> to_span() const { return absl::Span<T>(data(), size()); }

 private:
  // Stick to the following invariants when changing the data member values:
  // (1) `ptr_` is never 0 (to mirror slices in Rust).
  // (2) if `size_ > 0` then `reinterpret_cast<T*>(ptr_)` is a valid pointer.
  uintptr_t ptr_;
  size_t size_;
};

}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_SLICEREF_H_
