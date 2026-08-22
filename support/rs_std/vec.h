// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs_std/vec.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_VEC_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_VEC_H_

#include <cstddef>
#include <cstring>
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

template <typename Derived, typename T>
class VecBase {
 public:
  T* data() noexcept { return derived().data(); }
  const T* data() const noexcept { return derived().data(); }
  std::size_t size() const noexcept { return derived().size(); }
  std::size_t capacity() const noexcept { return derived().capacity(); }

  T& operator[](std::size_t index) noexcept {
    CRUBIT_CHECK(index < size());
    return data()[index];
  }
  const T& operator[](std::size_t index) const noexcept {
    CRUBIT_CHECK(index < size());
    return data()[index];
  }

  T* begin() noexcept { return data(); }
  const T* begin() const noexcept { return data(); }
  T* end() noexcept { return data() + size(); }
  const T* end() const noexcept { return data() + size(); }

  void destroy() noexcept {
    std::destroy(data(), data() + size());
    if (capacity() > 0) {
      crubit_dealloc(data(), capacity() * sizeof(T), alignof(T));
    }
  }

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
    derived().set_ptr(new_ptr);
    derived().set_cap(new_cap);
  }

  void push_back(const T& value) { emplace_back(value); }

  void push_back(T&& value) { emplace_back(std::move(value)); }

  template <typename... Args>
  T& emplace_back(Args&&... args) {
    reserve_additional_capacity(1);
    T* target = data() + size();
    std::construct_at(target, std::forward<Args>(args)...);
    derived().set_len(size() + 1);
    return *target;
  }

  void insert(std::size_t index, const T& value) {
    CRUBIT_CHECK(index <= size());
    reserve_additional_capacity(1);
    if (index < size()) {
      std::memmove(data() + index + 1, data() + index,
                   (size() - index) * sizeof(T));
    }
    std::construct_at(data() + index, value);
    derived().set_len(size() + 1);
  }

  void insert(std::size_t index, T&& value) {
    CRUBIT_CHECK(index <= size());
    reserve_additional_capacity(1);
    if (index < size()) {
      std::memmove(data() + index + 1, data() + index,
                   (size() - index) * sizeof(T));
    }
    std::construct_at(data() + index, std::move(value));
    derived().set_len(size() + 1);
  }

  void clear() noexcept {
    std::destroy(data(), data() + size());
    derived().set_len(0);
  }

  void pop_back() noexcept {
    CRUBIT_CHECK(size() > 0);
    std::destroy_at(data() + size() - 1);
    derived().set_len(size() - 1);
  }

 private:
  Derived& derived() { return *static_cast<Derived*>(this); }
  const Derived& derived() const { return *static_cast<const Derived*>(this); }
};

template <typename T>
struct Vec final {
  static_assert(false,
                "This type should only be used via a generated specialization");
};

}  // namespace rs_std

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_VEC_H_
