// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_VALUE_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_VALUE_H_

#include <memory>
#include <utility>

#include "support/internal/slot.h"

namespace rs {

namespace internal {
template <typename T>
void Relocate(T* dest, T* src) {
  if constexpr (requires(T x) {
                  T(crubit::UnsafeRelocateTag{}, std::move(x));
                }) {
    new (dest) T(crubit::UnsafeRelocateTag{}, std::move(*src));
  } else {
    new (dest) T(std::move(*src));
    std::destroy_at(src);
  }
}
}  // namespace internal

// A wrapper that makes Rust-movable (but potentially C++ non-movable) types
// C++ movable by introducing a "moved-from" state.
template <typename T>
class RelocatableValue {
 public:
  // Default constructor: empty.
  RelocatableValue() : has_value_(false) {}

  // Construct from T by moving it in.
  // Only allowed if T is C++ move constructible, to ensure temporary
  // destruction is safe.
  template <typename U = T>
    requires(std::is_move_constructible_v<U>)
  explicit RelocatableValue(U&& x) {
    new (&value_) T(std::forward<U>(x));
    has_value_ = true;
  }

  // Construct T in-place.
  template <typename... Args>
  explicit RelocatableValue(std::in_place_t, Args&&... args) {
    new (&value_) T(std::forward<Args>(args)...);
    has_value_ = true;
  }

  // Construct from Slot<T>&&, taking the value.
  explicit RelocatableValue(crubit::Slot<T>&& slot) {
    internal::Relocate(&value_, slot.Get());
    has_value_ = true;
  }

  ~RelocatableValue() {
    if (has_value_) {
      value_.~T();
    }
  }

  RelocatableValue(RelocatableValue&& other) noexcept {
    if (other.has_value_) {
      internal::Relocate(&value_, &other.value_);
      has_value_ = true;
      other.has_value_ = false;
    } else {
      has_value_ = false;
    }
  }

  RelocatableValue& operator=(RelocatableValue&& other) noexcept {
    if (this == &other) return *this;
    if (has_value_) {
      value_.~T();
      has_value_ = false;
    }
    if (other.has_value_) {
      internal::Relocate(&value_, &other.value_);
      has_value_ = true;
      other.has_value_ = false;
    }
    return *this;
  }

  // Disable copy
  RelocatableValue(const RelocatableValue&) = delete;
  RelocatableValue& operator=(const RelocatableValue&) = delete;

  T& operator*() & { return value_; }
  const T& operator*() const& { return value_; }
  T&& operator*() && { return std::move(value_); }
  const T&& operator*() const&& { return std::move(value_); }

  T* operator->() { return &value_; }
  const T* operator->() const { return &value_; }

  bool has_value() const { return has_value_; }
  explicit operator bool() const { return has_value_; }

 private:
  union {
    T value_;
  };
  bool has_value_ = false;
};

}  // namespace rs

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_VALUE_H_
