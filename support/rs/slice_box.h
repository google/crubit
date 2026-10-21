// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_SLICE_BOX_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_SLICE_BOX_H_

#include <cstddef>
#include <cstdint>
#include <initializer_list>
#include <memory>
#include <new>
#include <span>
#include <type_traits>

#include "support/internal/check.h"
#include "support/unsafe_relocate_tag.h"
#include "support/annotations_internal.h"
#include "support/rs_std/slice_ref.h"

extern "C" {
void* crubit_nonnull crubit_alloc(::std::size_t size,
                                  ::std::size_t align) noexcept;
void crubit_dealloc(void* crubit_nonnull ptr, ::std::size_t size,
                    ::std::size_t align) noexcept;
}

namespace rs {

// `rs::SliceBox<T>` represents a Rust `Box<[T]>` in C++.
// It is non-C++-movable, but can be relocated via `crubit::UnsafeRelocateTag`
// and passed by-value via `rs::Movable`. Memory is managed by the Rust global
// allocator.
template <typename T>
class CRUBIT_INTERNAL_RUST_TYPE("::alloc::boxed::Box<[{}]>", T) SliceBox final {
  static_assert(!::std::is_reference_v<T>,
                "SliceBox<T> cannot hold a reference");
  static_assert(!::std::is_array_v<T>, "SliceBox<T> cannot hold an array");
  static_assert(!::std::is_const_v<T> && !::std::is_volatile_v<T>,
                "SliceBox<T> cannot hold cv-qualified types");

 public:
  using element_type = T;

  SliceBox() noexcept : dangling_ptr_(alignof(T)), size_(0) {}

  explicit SliceBox(::std::span<const T> span)
    requires(::std::is_copy_constructible_v<T>)
      : dangling_ptr_(alignof(T)), size_(span.size()) {
    if (size_ > 0) {
      ptr_ = static_cast<T*>(crubit_alloc(size_ * sizeof(T), alignof(T)));
      ::std::uninitialized_copy_n(span.data(), size_, ptr_);
    }
  }

  SliceBox(::std::initializer_list<T> ilist)
    requires(::std::is_copy_constructible_v<T>)
      : SliceBox(::std::span<const T>(ilist.begin(), ilist.size())) {}

  SliceBox(size_t n, const T& value)
    requires(::std::is_copy_constructible_v<T>)
      : dangling_ptr_(alignof(T)), size_(n) {
    if (size_ > 0) {
      ptr_ = static_cast<T*>(crubit_alloc(size_ * sizeof(T), alignof(T)));
      ::std::uninitialized_fill_n(ptr_, size_, value);
    }
  }

  // Takes ownership of a pointer allocated by `crubit_alloc(size * sizeof(T),
  // alignof(T))`. SAFETY: The pointer must be non-null if size > 0, and valid
  // for the lifetime of this `SliceBox`.
  static SliceBox unsafe_from_raw(T* crubit_nullable ptr,
                                  size_t size) noexcept {
    return SliceBox(ptr, size);
  }

  ~SliceBox() {
    if (size_ > 0) {
      ::std::destroy_n(ptr_, size_);
      crubit_dealloc(ptr_, size_ * sizeof(T), alignof(T));
    }
  }

  SliceBox(const SliceBox&) = delete;
  SliceBox& operator=(const SliceBox&) = delete;
  SliceBox(SliceBox&&) = delete;
  SliceBox& operator=(SliceBox&&) = delete;

  SliceBox(::crubit::UnsafeRelocateTag, SliceBox&& other) noexcept
      : dangling_ptr_(alignof(T)), size_(other.size_) {
    if (size_ > 0) {
      ptr_ = other.ptr_;
    }
  }

  T* crubit_nullable data() noexcept { return size_ > 0 ? ptr_ : nullptr; }
  const T* crubit_nullable data() const noexcept {
    return size_ > 0 ? ptr_ : nullptr;
  }

  size_t size() const noexcept { return size_; }
  bool empty() const noexcept { return size_ == 0; }

  T& operator[](size_t index) noexcept {
    CRUBIT_CHECK(index < size_);
    return ptr_[index];
  }
  const T& operator[](size_t index) const noexcept {
    CRUBIT_CHECK(index < size_);
    return ptr_[index];
  }

  T* crubit_nullable begin() noexcept { return data(); }
  T* crubit_nullable end() noexcept { return data() + size(); }
  const T* crubit_nullable begin() const noexcept { return data(); }
  const T* crubit_nullable end() const noexcept { return data() + size(); }

  SliceRef<T> as_slice() noexcept { return SliceRef<T>(data(), size()); }
  SliceRef<const T> as_slice() const noexcept {
    return SliceRef<const T>(data(), size());
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  operator SliceRef<const T>() const noexcept { return as_slice(); }

  // NOLINTNEXTLINE(google-explicit-constructor)
  operator ::std::span<T>() noexcept { return ::std::span<T>(data(), size()); }

  // NOLINTNEXTLINE(google-explicit-constructor)
  operator ::std::span<const T>() const noexcept {
    return ::std::span<const T>(data(), size());
  }

 private:
  explicit SliceBox(T* crubit_nullable ptr, size_t size) noexcept
      : dangling_ptr_(alignof(T)), size_(size) {
    if (size_ > 0) {
      CRUBIT_CHECK(ptr != nullptr);
      if (ptr != nullptr) {
        ptr_ = ptr;
      }
    }
  }

  union {
    T* crubit_nonnull ptr_;
    uintptr_t dangling_ptr_;
  };
  size_t size_;
};

}  // namespace rs

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_SLICE_BOX_H_
