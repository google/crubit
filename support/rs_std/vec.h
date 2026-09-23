// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/vec.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_VEC_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_VEC_H_

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <iterator>
#include <memory>
#include <utility>

#include "support/internal/check.h"

extern "C" {
void* crubit_alloc(std::size_t size, std::size_t align) noexcept;
void crubit_dealloc(void* ptr, std::size_t size, std::size_t align) noexcept;
void* crubit_realloc(void* ptr, std::size_t old_size, std::size_t old_align,
                     std::size_t new_size) noexcept;
}

namespace rs_std {

template <typename T>
struct Vec;

// Base class that defines the public C++ API and common container operations
// for `Vec<T>`.
template <typename T>
class VecBase {
 public:
  using value_type = T;
  using size_type = std::size_t;
  using difference_type = std::ptrdiff_t;
  using reference = T&;
  using const_reference = const T&;
  using pointer = T*;
  using const_pointer = const T*;
  using iterator = T*;
  using const_iterator = const T*;
  using reverse_iterator = std::reverse_iterator<iterator>;
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;

  // Returns a pointer to the contiguous element buffer, or a dangling pointer
  // if empty.
  T* data() noexcept {
    return std::bit_cast<T*>(*reinterpret_cast<const std::uintptr_t*>(
        &derived().storage_[Vec<T>::kPtrOffset]));
  }

  const T* data() const noexcept {
    return std::bit_cast<const T*>(*reinterpret_cast<const std::uintptr_t*>(
        &derived().storage_[Vec<T>::kPtrOffset]));
  }

  // Returns the number of elements in the vector.
  std::size_t size() const noexcept {
    return *reinterpret_cast<const std::size_t*>(
        &derived().storage_[Vec<T>::kLenOffset]);
  }

  // Returns the total number of elements that the vector can hold without
  // allocating.
  std::size_t capacity() const noexcept {
    return *reinterpret_cast<const std::size_t*>(
        &derived().storage_[Vec<T>::kCapOffset]);
  }

  // Returns true if the vector contains no elements.
  [[nodiscard]] bool empty() const noexcept { return size() == 0; }

  // Returns a reference to the element at index `index`. Panics if index >=
  // size().
  T& operator[](std::size_t index) noexcept {
    CRUBIT_CHECK(index < size());
    return data()[index];
  }
  const T& operator[](std::size_t index) const noexcept {
    CRUBIT_CHECK(index < size());
    return data()[index];
  }

  // Returns a reference to the first element. Panics if the vector is empty.
  T& front() noexcept {
    CRUBIT_CHECK(!empty());
    return *data();
  }
  const T& front() const noexcept {
    CRUBIT_CHECK(!empty());
    return *data();
  }

  // Returns a reference to the last element. Panics if the vector is empty.
  T& back() noexcept {
    CRUBIT_CHECK(!empty());
    return *(data() + size() - 1);
  }
  const T& back() const noexcept {
    CRUBIT_CHECK(!empty());
    return *(data() + size() - 1);
  }

  // Standard iterator accessors providing contiguous, random-access traversal.
  iterator begin() noexcept { return data(); }
  const_iterator begin() const noexcept { return data(); }
  iterator end() noexcept { return data() + size(); }
  const_iterator end() const noexcept { return data() + size(); }

  const_iterator cbegin() const noexcept { return begin(); }
  const_iterator cend() const noexcept { return end(); }

  reverse_iterator rbegin() noexcept { return reverse_iterator(end()); }
  const_reverse_iterator rbegin() const noexcept { return crbegin(); }
  reverse_iterator rend() noexcept { return reverse_iterator(begin()); }
  const_reverse_iterator rend() const noexcept { return crend(); }

  const_reverse_iterator crbegin() const noexcept {
    return const_reverse_iterator(end());
  }
  const_reverse_iterator crend() const noexcept {
    return const_reverse_iterator(begin());
  }

  // Reserves capacity for at least `additional` more elements beyond the
  // current size, matching the semantics of Rust's `Vec::reserve(additional)`.
  //
  // This is named `reserve_additional_capacity` to avoid ambiguity with C++'s
  // `std::vector::reserve(n)`, which takes the *total* desired capacity rather
  // than additional elements. For standard C++ total capacity semantics, use
  // `reserve(new_cap)` instead.
  void reserve_additional_capacity(std::size_t additional) {
    std::size_t needed = size() + additional;
    if (needed <= capacity()) return;
    std::size_t new_cap = capacity() == 0 ? 4 : capacity() * 2;
    if (new_cap < needed) {
      new_cap = needed;
    }
    std::size_t old_cap = capacity();
    T* old_ptr = data();
    T* new_ptr;
    if (old_cap == 0) {
      new_ptr =
          reinterpret_cast<T*>(crubit_alloc(new_cap * sizeof(T), alignof(T)));
    } else {
      new_ptr = reinterpret_cast<T*>(crubit_realloc(
          old_ptr, old_cap * sizeof(T), alignof(T), new_cap * sizeof(T)));
    }
    set_ptr(new_ptr);
    set_cap(new_cap);
  }

  // Increases the capacity of the vector to at least `new_cap` total elements,
  // matching the semantics of `std::vector::reserve(new_cap)`. Use
  // `reserve_additional_capacity` if you want the Rust `Vec::reserve`
  // semantics.
  void reserve(std::size_t new_cap) {
    if (new_cap <= capacity()) return;
    reserve_additional_capacity(new_cap - size());
  }

  // Appends an element to the end of the vector.
  void push_back(const T& value) { emplace_back(value); }
  void push_back(T&& value) { emplace_back(std::move(value)); }

  // Constructs an element in-place at the end of the vector.
  template <typename... Args>
  T& emplace_back(Args&&... args) {
    reserve_additional_capacity(1);
    T* target = data() + size();
    std::construct_at(target, std::forward<Args>(args)...);
    set_len(size() + 1);
    return *target;
  }

  // Inserts an element at the specified index, shifting existing elements
  // rightward.
  void insert(std::size_t index, const T& value) {
    CRUBIT_CHECK(index <= size());
    reserve_additional_capacity(1);
    if (index < size()) {
      std::memmove(data() + index + 1, data() + index,
                   (size() - index) * sizeof(T));
    }
    std::construct_at(data() + index, value);
    set_len(size() + 1);
  }

  void insert(std::size_t index, T&& value) {
    CRUBIT_CHECK(index <= size());
    reserve_additional_capacity(1);
    if (index < size()) {
      std::memmove(data() + index + 1, data() + index,
                   (size() - index) * sizeof(T));
    }
    std::construct_at(data() + index, std::move(value));
    set_len(size() + 1);
  }

  // Destroys all elements in the vector without freeing the allocated buffer.
  void clear() noexcept {
    std::destroy(data(), data() + size());
    set_len(0);
  }

  // Removes and destroys the last element. Panics if the vector is empty.
  void pop_back() noexcept {
    CRUBIT_CHECK(!empty());
    std::destroy_at(data() + size() - 1);
    set_len(size() - 1);
  }

 protected:
  // Destroys all elements, deallocates the underlying buffer, and reinitializes
  // the vector to the empty state (`ptr = alignof(T), len = 0, cap = 0`).
  void destroy() noexcept {
    std::destroy(data(), data() + size());
    if (capacity() > 0) {
      crubit_dealloc(data(), capacity() * sizeof(T), alignof(T));
    }
    init_empty();
  }

  void set_ptr(T* ptr) noexcept {
    *reinterpret_cast<std::uintptr_t*>(
        &derived().storage_[Vec<T>::kPtrOffset]) =
        std::bit_cast<std::uintptr_t>(ptr);
  }
  void set_len(std::size_t len) noexcept {
    *reinterpret_cast<std::size_t*>(&derived().storage_[Vec<T>::kLenOffset]) =
        len;
  }
  void set_cap(std::size_t cap) noexcept {
    *reinterpret_cast<std::size_t*>(&derived().storage_[Vec<T>::kCapOffset]) =
        cap;
  }
  void init_empty() noexcept {
    set_ptr(reinterpret_cast<T*>(alignof(T)));
    set_len(0);
    set_cap(0);
  }

 private:
  Vec<T>& derived() { return *static_cast<Vec<T>*>(this); }
  const Vec<T>& derived() const { return *static_cast<const Vec<T>*>(this); }
};

template <typename T>
struct Vec final {
  static_assert(false,
                "This type should only be used via a generated specialization");
};

}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_VEC_H_
