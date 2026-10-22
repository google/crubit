// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_MOVABLE_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_MOVABLE_H_

#include <memory>
#include <type_traits>
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
//
// NOTE: `rs::Movable<T>` is NOT layout-compatible with `T` due to the extra
// boolean tracking whether the value has been moved. Do not add
// `CRUBIT_INTERNAL_RUST_TYPE` here.
template <typename T>
class Movable {
 public:
  // Default constructor: constructs contained T if T is default constructible.
  Movable()
    requires(std::is_default_constructible_v<T>)
  {
    new (&value_) T();
    valueless_after_move_ = false;
  }

  // Construct from T by moving it in.
  // Only allowed if T is C++ move constructible, to ensure temporary
  // destruction is safe.
  template <typename U = T>
    requires(!std::is_same_v<std::remove_cvref_t<U>, Movable> &&
             std::is_move_constructible_v<T> &&
             std::is_constructible_v<T, U &&>)
  explicit Movable(U&& x) {
    new (&value_) T(std::forward<U>(x));
    valueless_after_move_ = false;
  }

  // Construct T in-place.
  template <typename... Args>
  explicit Movable(std::in_place_t, Args&&... args) {
    new (&value_) T(std::forward<Args>(args)...);
    valueless_after_move_ = false;
  }

  // Take ownership of a value stored in a Slot<T>.
  static Movable TakeFromSlot(crubit::Slot<T>&& slot) {
    return Movable(std::move(slot));
  }

  // Moves the contained value into a Slot and leaves this object valueless.
  void MoveToSlot(crubit::Slot<T>& slot) && {
    if (!valueless_after_move_) {
      internal::Relocate(slot.Get(), &value_);
      valueless_after_move_ = true;
    }
  }

  ~Movable() {
    if (!valueless_after_move_) {
      value_.~T();
    }
  }

  Movable(Movable&& other) noexcept {
    if (!other.valueless_after_move_) {
      internal::Relocate(&value_, &other.value_);
      valueless_after_move_ = false;
      other.valueless_after_move_ = true;
    } else {
      valueless_after_move_ = true;
    }
  }

  Movable& operator=(Movable&& other) noexcept {
    if (this == &other) return *this;
    if (!valueless_after_move_) {
      value_.~T();
      valueless_after_move_ = true;
    }
    if (!other.valueless_after_move_) {
      internal::Relocate(&value_, &other.value_);
      valueless_after_move_ = false;
      other.valueless_after_move_ = true;
    }
    return *this;
  }

  // Copy constructor (only available if T is copy constructible)
  Movable(const Movable& other)
    requires(std::is_copy_constructible_v<T>)
  {
    if (!other.valueless_after_move_) {
      new (&value_) T(other.value_);
      valueless_after_move_ = false;
    } else {
      valueless_after_move_ = true;
    }
  }

  Movable(const Movable&)
    requires(!std::is_copy_constructible_v<T>)
  = delete;

  // Copy assignment (only available if T is copy constructible and
  // copy assignable)
  Movable& operator=(const Movable& other)
    requires(std::is_copy_constructible_v<T> && std::is_copy_assignable_v<T>)
  {
    if (this == &other) return *this;
    if (!valueless_after_move_ && !other.valueless_after_move_) {
      value_ = other.value_;
    } else if (!valueless_after_move_ && other.valueless_after_move_) {
      value_.~T();
      valueless_after_move_ = true;
    } else if (valueless_after_move_ && !other.valueless_after_move_) {
      new (&value_) T(other.value_);
      valueless_after_move_ = false;
    }
    return *this;
  }

  Movable& operator=(const Movable&)
    requires(!std::is_copy_constructible_v<T> || !std::is_copy_assignable_v<T>)
  = delete;

  T& operator*() & { return value_; }
  const T& operator*() const& { return value_; }
  T&& operator*() && { return std::move(value_); }
  const T&& operator*() const&& { return std::move(value_); }

  T* operator->() { return &value_; }
  const T* operator->() const { return &value_; }

  bool valueless_after_move() const { return valueless_after_move_; }
  explicit operator bool() const { return !valueless_after_move_; }

 private:
  explicit Movable(crubit::Slot<T>&& slot) {
    internal::Relocate(&value_, slot.Get());
    valueless_after_move_ = false;
  }

  union {
    T value_;
  };
  bool valueless_after_move_ = false;
};

}  // namespace rs

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_MOVABLE_H_
