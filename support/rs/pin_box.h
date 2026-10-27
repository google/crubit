// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_PIN_BOX_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_PIN_BOX_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <memory>
#include <new>
#include <type_traits>
#include <utility>

#include "support/internal/check.h"
#include "support/internal/slot.h"
#include "support/movable.h"
#include "support/unsafe_relocate_tag.h"
#include "support/annotations_internal.h"

extern "C" {
void* crubit_nonnull crubit_alloc(::std::size_t size,
                                  ::std::size_t align) noexcept;
void crubit_dealloc(void* crubit_nonnull ptr, ::std::size_t size,
                    ::std::size_t align) noexcept;
}

namespace rs {

namespace pin_box_internal {
template <typename T>
inline constexpr bool is_rs_movable_v = false;
template <typename T>
inline constexpr bool is_rs_movable_v<::rs::Movable<T>> = true;
}  // namespace pin_box_internal

// `rs::PinBox<T>` represents a Rust `Pin<Box<T>>` in C++.
// It is non-C++-movable, but can be relocated via `crubit::UnsafeRelocateTag`
// and passed by-value via `rs::Movable`. Memory is managed by the Rust global
// allocator. This is the C++ spelling of `Pin<Box<T>>` and is used when `T`
// is not Rust-movable.
template <typename T>
class CRUBIT_INTERNAL_RUST_TYPE("::core::pin::Pin<::alloc::boxed::Box<{}>>", T)
    PinBox final {
  static_assert(!::std::is_reference_v<T>, "PinBox<T> cannot hold a reference");
  static_assert(!::std::is_array_v<T>, "PinBox<T> cannot hold an array");
  static_assert(!::std::is_const_v<T> && !::std::is_volatile_v<T>,
                "PinBox<T> cannot hold cv-qualified types");

 public:
  using element_type = T;

  template <typename... Args>
  explicit PinBox(::std::in_place_t, Args&&... args) {
    ptr_ = static_cast<T*>(crubit_alloc(sizeof(T), alignof(T)));
    new (ptr_) T(::std::forward<Args>(args)...);
  }

  // Convenience constructors that move or copy an existing `T` into the box.
  // Prefer `rs::make_pin_box` for types that are neither movable nor copyable
  // in C++, or when passing a newly created box by value.
  //
  // Constructing from an `rs::Movable<T>` aborts if `value` is valueless (i.e.
  // it has already been moved from or unwrapped).
  explicit PinBox(::rs::Movable<T> value) {
    CRUBIT_CHECK(!value.valueless_after_move())
        << "rs::PinBox: cannot box a valueless (moved-from) rs::Movable";
    ptr_ = static_cast<T*>(crubit_alloc(sizeof(T), alignof(T)));
    new (ptr_) T(::std::move(value).operator T());
  }

  explicit PinBox(T&& value)
    requires(::std::is_move_constructible_v<T>)
      : PinBox(::std::in_place, ::std::move(value)) {}

  explicit PinBox(const T& value)
    requires(::std::is_copy_constructible_v<T>)
      : PinBox(::std::in_place, value) {}

  // Takes ownership of a pointer allocated by `crubit_alloc(sizeof(T),
  // alignof(T))`. SAFETY: The pointer must be non-null and valid for the
  // lifetime of this `PinBox`.
  static PinBox unsafe_from_raw(T* crubit_nonnull ptr) noexcept {
    return PinBox(ptr);
  }

  PinBox(::std::nullptr_t) = delete;

  ~PinBox() {
    ptr_->~T();
    crubit_dealloc(ptr_, sizeof(T), alignof(T));
  }

  PinBox(const PinBox&) = delete;
  PinBox& operator=(const PinBox&) = delete;
  PinBox(PinBox&&) = delete;
  PinBox& operator=(PinBox&&) = delete;

  PinBox(::crubit::UnsafeRelocateTag, PinBox&& other) noexcept
      : ptr_(other.ptr_) {}

  T* crubit_nonnull get() noexcept { return ptr_; }
  const T* crubit_nonnull get() const noexcept { return ptr_; }

  T* crubit_nonnull operator->() noexcept { return ptr_; }
  const T* crubit_nonnull operator->() const noexcept { return ptr_; }

  T& operator*() noexcept { return *ptr_; }
  const T& operator*() const noexcept { return *ptr_; }

 private:
  explicit PinBox(T* crubit_nonnull ptr) : ptr_(ptr) {}

  T* crubit_nonnull ptr_;
};

// Partial specialization of `rs::Movable` for `rs::PinBox<T>`:
// - Forwards `operator->`, `operator*`, and `get()` directly to the pointee `T`
//   (like `std::unique_ptr<T>`), avoiding double-dereferencing on
//   `rs::make_pin_box<T>(...)` and Rust functions returning `Pin<Box<T>>`.
// - Provides `as_pin_box()` for accessing the underlying `rs::PinBox<T>`.
// - Represents the moved-from state with all-zero bytes in `storage_`, keeping
//   `sizeof(Movable<PinBox<T>>) == sizeof(PinBox<T>)`.
template <typename T>
class Movable<PinBox<T>> {
  static_assert(sizeof(PinBox<T>) == sizeof(::std::uintptr_t));

 public:
  using element_type = T;

  Movable() = delete;
  Movable(::std::nullptr_t) = delete;

  template <typename... Args>
  explicit Movable(::std::in_place_t, Args&&... args) {
    ::std::construct_at(&value_, ::std::forward<Args>(args)...);
  }

  static Movable TakeFromSlot(::crubit::Slot<PinBox<T>>&& slot) {
    return Movable(::std::move(slot));
  }

  void MoveToSlot(::crubit::Slot<PinBox<T>>& slot) && {
    CheckHasValue();
    ::std::construct_at(slot.Get(), ::crubit::UnsafeRelocateTag{},
                        ::std::move(value_));
    SetValueless();
  }

  ~Movable() {
    if (!valueless_after_move()) {
      ::std::destroy_at(&value_);
    }
  }

  Movable(Movable&& other) noexcept {
    if (!other.valueless_after_move()) {
      ::std::construct_at(&value_, ::crubit::UnsafeRelocateTag{},
                          ::std::move(other.value_));
      other.SetValueless();
    } else {
      SetValueless();
    }
  }

  Movable& operator=(Movable&& other) noexcept {
    if (this == &other) return *this;
    if (!valueless_after_move()) {
      ::std::destroy_at(&value_);
    }
    if (!other.valueless_after_move()) {
      ::std::construct_at(&value_, ::crubit::UnsafeRelocateTag{},
                          ::std::move(other.value_));
      other.SetValueless();
    } else {
      SetValueless();
    }
    return *this;
  }

  Movable(const Movable&) = delete;
  Movable& operator=(const Movable&) = delete;

  T* crubit_nonnull get() {
    CheckHasValue();
    return value_.get();
  }
  const T* crubit_nonnull get() const {
    CheckHasValue();
    return value_.get();
  }

  T* crubit_nonnull operator->() { return get(); }
  const T* crubit_nonnull operator->() const { return get(); }

  T& operator*() { return *get(); }
  const T& operator*() const { return *get(); }

  PinBox<T>& as_pin_box() & {
    CheckHasValue();
    return value_;
  }
  const PinBox<T>& as_pin_box() const& {
    CheckHasValue();
    return value_;
  }
  void as_pin_box() && = delete;
  void as_pin_box() const&& = delete;

  // Implicitly unwraps an rvalue `Movable<PinBox<T>>` into a `PinBox<T>`,
  // leaving this `Movable` valueless.
  // NOLINTNEXTLINE(google-explicit-constructor)
  operator PinBox<T>() && {
    CheckHasValue();
    struct SetValuelessOnDestruction {
      Movable* self;
      ~SetValuelessOnDestruction() { self->SetValueless(); }
    } set_valueless{this};
    return PinBox<T>(::crubit::UnsafeRelocateTag{}, ::std::move(value_));
  }

  bool valueless_after_move() const noexcept {
    // Inspect the object representation directly: an active `PinBox<T>` holds a
    // non-null `T*`, whereas `SetValueless()` activates `storage_` with
    // all-zero bytes.
    ::std::uintptr_t bits;
    ::std::memcpy(&bits, &storage_, sizeof(bits));
    return bits == 0;
  }
  explicit operator bool() const noexcept { return !valueless_after_move(); }

 private:
  explicit Movable(::crubit::Slot<PinBox<T>>&& slot) {
    ::std::construct_at(&value_, ::crubit::UnsafeRelocateTag{},
                        ::std::move(*slot.Get()));
  }

  void SetValueless() noexcept { ::std::construct_at(&storage_); }

  void CheckHasValue() const {
    CRUBIT_CHECK(!valueless_after_move())
        << "rs::Movable: cannot access a valueless (moved-from) PinBox";
  }

  union {
    PinBox<T> value_;
    ::std::array<unsigned char, sizeof(PinBox<T>)> storage_;
  };
};

// Allocates a `T` on the Rust heap in-place and returns it wrapped in
// `rs::Movable<rs::PinBox<T>>` so it can be passed by value or unwrapped into
// an `rs::PinBox<T>`.
template <typename T, typename... Args>
  requires(!(sizeof...(Args) == 1 &&
             (::std::is_same_v<::std::remove_cvref_t<Args>, ::rs::Movable<T>> &&
              ...)) &&
           ::std::is_constructible_v<T, Args && ...>)::rs::Movable<PinBox<T>>
make_pin_box(Args&&... args) {
  return ::rs::Movable<PinBox<T>>(::std::in_place, ::std::in_place,
                                  ::std::forward<Args>(args)...);
}

// Moves a `T` out of an `rs::Movable<T>` (such as a non-C++-movable Rust value
// returned by value) into a newly allocated `rs::PinBox<T>`.
template <typename T>
::rs::Movable<PinBox<T>> make_pin_box(::rs::Movable<T> value) {
  return ::rs::Movable<PinBox<T>>(::std::in_place, ::std::move(value));
}

// Deduces `T` when boxing a single C++-movable or copyable value.
template <int&... ExplicitArgumentBarrier, typename T>
  requires(!pin_box_internal::is_rs_movable_v<::std::remove_cvref_t<T>> &&
           !::std::is_same_v<::std::decay_t<T>, ::std::nullptr_t> &&
           ::std::is_constructible_v<
               ::std::decay_t<T>, T &&>)::rs::Movable<PinBox<::std::decay_t<T>>>
make_pin_box(T&& value) {
  return ::rs::Movable<PinBox<::std::decay_t<T>>>(
      ::std::in_place, ::std::in_place, ::std::forward<T>(value));
}

}  // namespace rs

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_PIN_BOX_H_
