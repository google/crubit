// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// IWYU pragma: private, include "support/rs/vec.h"

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_VEC_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_VEC_H_

#include <algorithm>
#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <functional>
#include <initializer_list>
#include <iterator>
#include <memory>
#include <ranges>  // NOLINT(build/c++20)
#include <type_traits>
#include <utility>
#include <vector>

#include "support/internal/check.h"
#include "support/internal/cpp_config.h"

extern "C" {
void* crubit_alloc(std::size_t size, std::size_t align) noexcept;
void crubit_dealloc(void* ptr, std::size_t size, std::size_t align) noexcept;
void* crubit_realloc(void* ptr, std::size_t old_size, std::size_t old_align,
                     std::size_t new_size) noexcept;
}

#ifdef CRUBIT_INTERNAL_RS_NAMESPACE
#define CRUBIT_RS_STD_PRIMARY_NS rs
#else
#define CRUBIT_RS_STD_PRIMARY_NS rs_std
#endif

namespace CRUBIT_RS_STD_PRIMARY_NS {

template <typename T>
struct Vec;

// Base class that defines the public C++ API and common container operations
// for `Vec<T>`.
template <typename T>
class VecBase {
 public:
  using value_type = T;
  using size_type = ::std::size_t;
  using difference_type = ::std::ptrdiff_t;
  using reference = T&;
  using const_reference = const T&;
  using pointer = T*;
  using const_pointer = const T*;
  using iterator = T*;
  using const_iterator = const T*;
  using reverse_iterator = ::std::reverse_iterator<iterator>;
  using const_reverse_iterator = ::std::reverse_iterator<const_iterator>;

  // Constructs a vector with a copy of the elements of `other`.
  //
  // Like the rest of `rs::Vec`, assumes element constructors do not throw
  // (terminating if they do).
  template <typename Alloc, typename Elem = T>
    requires(::std::is_copy_constructible_v<Elem>)
  explicit VecBase(const ::std::vector<T, Alloc>& other) noexcept : VecBase() {
    init_empty();
    reserve(other.size());
    for (const T& elem : other) {
      emplace_back(elem);
    }
  }

  // Constructs a vector by moving the elements of `other`.
  //
  // Like the rest of `rs::Vec`, assumes element constructors do not throw
  // (terminating if they do).
  template <typename Alloc, typename Elem = T>
    requires(::std::is_move_constructible_v<Elem>)
  explicit VecBase(::std::vector<T, Alloc>&& other) noexcept : VecBase() {
    init_empty();
    reserve(other.size());
    for (T& elem : other) {
      emplace_back(::std::move(elem));
    }
  }

  // Constructs a vector with a copy of the elements of `init`.
  //
  // Like the rest of `rs::Vec`, assumes element constructors do not throw
  // (terminating if they do).
  VecBase(::std::initializer_list<T> init) noexcept : VecBase() {
    init_empty();
    reserve(init.size());
    for (const T& elem : init) {
      emplace_back(elem);
    }
  }

  // Constructs a vector with `count` value-initialized elements.
  //
  // Like the rest of `rs::Vec`, assumes element constructors do not throw
  // (terminating if they do).
  //
  // This is deliberately unconstrained, like `std::vector`'s: a single-argument
  // constructor is a candidate whenever a `Vec<T>` is copied or moved, and a
  // `requires` clause naming `T` would then be evaluated even while `T` is
  // still incomplete.
  explicit VecBase(size_type count) noexcept : VecBase() {
    init_empty();
    resize(count);
  }

  // Constructs a vector with `count` copies of `value`.
  //
  // Like the rest of `rs::Vec`, assumes element constructors do not throw
  // (terminating if they do).
  VecBase(size_type count, const T& value) noexcept
    requires(::std::is_copy_constructible_v<T>)
      : VecBase() {
    init_empty();
    if (count > 0) {
      reserve(count);
      ::std::uninitialized_fill_n(data(), count, value);
      set_len(count);
    }
  }

  // Constructs a vector with the contents of the range `[first, last)`.
  //
  // Like the rest of `rs::Vec`, assumes element constructors do not throw
  // (terminating if they do).
  template <typename InputIt>
    requires(::std::input_iterator<InputIt> &&
             ::std::is_constructible_v<T, ::std::iter_reference_t<InputIt>>)
  VecBase(InputIt first, InputIt last) noexcept : VecBase() {
    init_empty();
    append_iterator_range(first, last);
  }

  // Returns a pointer to the contiguous element buffer, or a dangling pointer
  // if empty.
  T* data() noexcept {
    return ::std::bit_cast<T*>(*reinterpret_cast<const ::std::uintptr_t*>(
        &derived().storage_[Vec<T>::kPtrOffset]));
  }

  const T* data() const noexcept {
    return ::std::bit_cast<const T*>(*reinterpret_cast<const ::std::uintptr_t*>(
        &derived().storage_[Vec<T>::kPtrOffset]));
  }

  // Returns the number of elements in the vector.
  ::std::size_t size() const noexcept {
    return *reinterpret_cast<const ::std::size_t*>(
        &derived().storage_[Vec<T>::kLenOffset]);
  }

  // Returns the total number of elements that the vector can hold without
  // allocating.
  ::std::size_t capacity() const noexcept {
    return *reinterpret_cast<const ::std::size_t*>(
        &derived().storage_[Vec<T>::kCapOffset]);
  }

  // Returns true if the vector contains no elements.
  [[nodiscard]] bool empty() const noexcept { return size() == 0; }

  // Returns the largest number of elements the vector can hold: like Rust's
  // `Vec`, an allocation may not exceed `isize::MAX` bytes.
  ::std::size_t max_size() const noexcept {
    return static_cast<::std::size_t>(PTRDIFF_MAX) / sizeof(T);
  }

  // Returns a reference to the element at index `index`. Panics if index >=
  // size().
  T& operator[](::std::size_t index) noexcept {
    CRUBIT_CHECK(index < size());
    return data()[index];
  }
  const T& operator[](::std::size_t index) const noexcept {
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
  //
  // Terminates if the resulting size would exceed `max_size()`.
  void reserve_additional_capacity(::std::size_t additional) {
    // Checked before adding so that `size() + additional` cannot wrap around.
    CRUBIT_CHECK(additional <= max_size() - size());
    ::std::size_t needed = size() + additional;
    if (needed <= capacity()) return;
    // `capacity() <= max_size() <= PTRDIFF_MAX`, so doubling cannot wrap.
    ::std::size_t new_cap = capacity() == 0 ? 4 : capacity() * 2;
    if (new_cap > max_size()) {
      new_cap = max_size();
    }
    if (new_cap < needed) {
      new_cap = needed;
    }
    // `new_cap <= max_size()`, so `new_cap * sizeof(T)` cannot wrap and is a
    // valid Rust allocation size.
    ::std::size_t old_cap = capacity();
    T* old_ptr = data();
    T* new_ptr;
    if (old_cap == 0) {
      new_ptr =
          reinterpret_cast<T*>(crubit_alloc(new_cap * sizeof(T), alignof(T)));
    } else {
      new_ptr = reinterpret_cast<T*>(crubit_realloc(
          old_ptr, old_cap * sizeof(T), alignof(T), new_cap * sizeof(T)));
    }
    CRUBIT_CHECK(new_ptr != nullptr);
    set_ptr(new_ptr);
    set_cap(new_cap);
  }

  // Increases the capacity of the vector to at least `new_cap` total elements,
  // matching the semantics of `std::vector::reserve(new_cap)`. Use
  // `reserve_additional_capacity` if you want the Rust `Vec::reserve`
  // semantics.
  void reserve(::std::size_t new_cap) {
    if (new_cap <= capacity()) return;
    reserve_additional_capacity(new_cap - size());
  }

  // Appends an element to the end of the vector.
  void push_back(const T& value) { emplace_back(value); }
  void push_back(T&& value) { emplace_back(::std::move(value)); }

  // Constructs an element in-place at the end of the vector.
  template <typename... Args>
  T& emplace_back(Args&&... args) {
    reserve_additional_capacity(1);
    T* target = data() + size();
    ::std::construct_at(target, ::std::forward<Args>(args)...);
    set_len(size() + 1);
    return *target;
  }

  // Inserts an element at the specified index, shifting existing elements
  // rightward.
  void insert(::std::size_t index, const T& value) {
    CRUBIT_CHECK(index <= size());
    reserve_additional_capacity(1);
    if (index < size()) {
      ::std::memmove(data() + index + 1, data() + index,
                     (size() - index) * sizeof(T));
    }
    ::std::construct_at(data() + index, value);
    set_len(size() + 1);
  }

  void insert(::std::size_t index, T&& value) {
    CRUBIT_CHECK(index <= size());
    reserve_additional_capacity(1);
    if (index < size()) {
      ::std::memmove(data() + index + 1, data() + index,
                     (size() - index) * sizeof(T));
    }
    ::std::construct_at(data() + index, ::std::move(value));
    set_len(size() + 1);
  }

  // Appends the elements of the range `rg` to the end of the vector.
  //
  // The behavior is undefined if `rg` overlaps with `*this` (i.e. refers to
  // elements inside this vector). Contiguous ranges of trivially copyable `T`
  // are copied with a single `memcpy`.
  //
  // Like the rest of `rs::Vec`, assumes element constructors do not throw
  // (terminating if they do).
  template <typename R>
  // NOLINTNEXTLINE(build/c++20)
    requires(::std::ranges::input_range<R> &&
             // NOLINTNEXTLINE(build/c++20)
             ::std::is_constructible_v<T, ::std::ranges::range_reference_t<R>>)
  void append_range(R&& rg) {
    if constexpr (::std::ranges::sized_range<R>) {  // NOLINT(build/c++20)
      auto first = ::std::ranges::begin(rg);        // NOLINT(build/c++20)
      auto last = ::std::ranges::end(rg);           // NOLINT(build/c++20)
      const auto count = ::std::ranges::size(rg);   // NOLINT(build/c++20)
      append_sized_range(first, last, static_cast<size_type>(count));
      // NOLINTNEXTLINE(build/c++20)
    } else if constexpr (::std::ranges::forward_range<R>) {
      auto first = ::std::ranges::begin(rg);  // NOLINT(build/c++20)
      auto last = ::std::ranges::end(rg);     // NOLINT(build/c++20)
      // NOLINTNEXTLINE(build/c++20)
      const auto count = ::std::ranges::distance(first, last);
      append_sized_range(first, last, static_cast<size_type>(count));
    } else {
      for (auto&& elem : rg) {
        emplace_back(::std::forward<decltype(elem)>(elem));
      }
    }
  }

  // Replaces the contents with `count` copies of `value`.
  //
  // Like the rest of `rs::Vec`, assumes element constructors and assignment
  // operators do not throw (terminating if they do).
  void assign(size_type count, const T& value)
    requires(::std::is_copy_constructible_v<T>)
  {
    if (aliases_element(value)) {
      T copy(value);
      assign_fill_non_aliasing(count, copy);
    } else {
      assign_fill_non_aliasing(count, value);
    }
  }

  // Replaces the contents with copies of those in the range `[first, last)`.
  //
  // The behavior is undefined if either `first` or `last` is an iterator into
  // `*this`.
  //
  // Like the rest of `rs::Vec`, assumes element constructors and assignment
  // operators do not throw (terminating if they do).
  template <typename InputIt>
    requires(::std::input_iterator<InputIt> &&
             ::std::is_constructible_v<T, ::std::iter_reference_t<InputIt>>)
  void assign(InputIt first, InputIt last) {
    if constexpr (::std::forward_iterator<InputIt>) {
      const size_type count =
          static_cast<size_type>(::std::distance(first, last));
      if (count > capacity()) {
        destroy();
        append_sized_range(first, last, count);
        return;
      }
      if constexpr (::std::contiguous_iterator<InputIt> &&
                    ::std::is_trivially_copyable_v<T> &&
                    ::std::is_same_v<
                        ::std::remove_const_t<::std::remove_reference_t<
                            ::std::iter_reference_t<InputIt>>>,
                        T>) {
        clear();
        append_sized_range(first, last, count);
        return;
      }
      if constexpr (::std::is_assignable_v<T&,
                                           ::std::iter_reference_t<InputIt>>) {
        const size_type old_size = size();
        T* dst = data();
        if (count <= old_size) {
          for (size_type i = 0; i < count; ++i, ++first, ++dst) {
            *dst = *first;
          }
          ::std::destroy(data() + count, data() + old_size);
        } else {
          for (size_type i = 0; i < old_size; ++i, ++first, ++dst) {
            *dst = *first;
          }
          for (; first != last; ++first, ++dst) {
            ::std::construct_at(dst, *first);
          }
        }
        set_len(count);
      } else {
        clear();
        append_sized_range(first, last, count);
      }
    } else {
      if constexpr (::std::is_assignable_v<T&,
                                           ::std::iter_reference_t<InputIt>>) {
        const size_type old_size = size();
        size_type i = 0;
        T* dst = data();
        for (; first != last && i < old_size; ++first, ++dst, ++i) {
          *dst = *first;
        }
        if (first == last) {
          ::std::destroy(data() + i, data() + old_size);
          set_len(i);
          return;
        }
      } else {
        clear();
      }
      for (; first != last; ++first) {
        emplace_back(*first);
      }
    }
  }

  // Resizes the vector to contain `count` elements.
  //
  // If `count` is less than `size()`, excess trailing elements are destroyed.
  // If `count` is greater than `size()`, additional value-initialized elements
  // are appended.
  //
  // Like the rest of `rs::Vec`, assumes element constructors do not throw
  // (terminating if they do).
  void resize(size_type count)
    requires(::std::is_default_constructible_v<T>)
  {
    const size_type old_size = size();
    if (count < old_size) {
      ::std::destroy(data() + count, data() + old_size);
      set_len(count);
    } else if (count > old_size) {
      reserve(count);
      ::std::uninitialized_value_construct_n(data() + old_size,
                                             count - old_size);
      set_len(count);
    }
  }

  // Resizes the vector to contain `count` elements.
  //
  // If `count` is less than `size()`, excess trailing elements are destroyed.
  // If `count` is greater than `size()`, additional copies of `value` are
  // appended.
  //
  // Like the rest of `rs::Vec`, assumes element constructors do not throw
  // (terminating if they do).
  void resize(size_type count, const T& value)
    requires(::std::is_copy_constructible_v<T>)
  {
    const size_type old_size = size();
    if (count < old_size) {
      ::std::destroy(data() + count, data() + old_size);
      set_len(count);
    } else if (count > old_size) {
      if (count > capacity() && aliases_element(value)) {
        T copy(value);
        reserve(count);
        ::std::uninitialized_fill_n(data() + old_size, count - old_size, copy);
      } else {
        reserve(count);
        ::std::uninitialized_fill_n(data() + old_size, count - old_size, value);
      }
      set_len(count);
    }
  }

  // Destroys all elements in the vector without freeing the allocated buffer.
  void clear() noexcept {
    ::std::destroy(data(), data() + size());
    set_len(0);
  }

  // Removes and destroys the last element. Panics if the vector is empty.
  void pop_back() noexcept {
    CRUBIT_CHECK(!empty());
    ::std::destroy_at(data() + size() - 1);
    set_len(size() - 1);
  }

 protected:
  VecBase() noexcept = default;
  VecBase(const VecBase&) noexcept = default;
  VecBase(VecBase&&) noexcept = default;
  VecBase& operator=(const VecBase&) noexcept = default;
  VecBase& operator=(VecBase&&) noexcept = default;
  ~VecBase() = default;

  // Destroys all elements, deallocates the underlying buffer, and reinitializes
  // the vector to the empty state (`ptr = alignof(T), len = 0, cap = 0`).
  void destroy() noexcept {
    ::std::destroy(data(), data() + size());
    if (capacity() > 0) {
      crubit_dealloc(data(), capacity() * sizeof(T), alignof(T));
    }
    init_empty();
  }

  void set_ptr(T* ptr) noexcept {
    *reinterpret_cast<::std::uintptr_t*>(
        &derived().storage_[Vec<T>::kPtrOffset]) =
        ::std::bit_cast<::std::uintptr_t>(ptr);
  }
  void set_len(::std::size_t len) noexcept {
    *reinterpret_cast<::std::size_t*>(&derived().storage_[Vec<T>::kLenOffset]) =
        len;
  }
  void set_cap(::std::size_t cap) noexcept {
    *reinterpret_cast<::std::size_t*>(&derived().storage_[Vec<T>::kCapOffset]) =
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

  bool aliases_element(const T& value) const noexcept {
    if (empty()) return false;
    const T* ptr = ::std::addressof(value);
    return !::std::less<const T*>{}(ptr, data()) &&
           ::std::less<const T*>{}(ptr, data() + size());
  }

  void assign_fill_non_aliasing(size_type count, const T& value) {
    if (count == 0) {
      clear();
      return;
    }
    if (count > capacity()) {
      destroy();
      reserve(count);
      ::std::uninitialized_fill_n(data(), count, value);
      set_len(count);
      return;
    }
    if constexpr (::std::is_copy_assignable_v<T>) {
      const size_type old_size = size();
      if (count <= old_size) {
        ::std::fill_n(data(), count, value);
        ::std::destroy(data() + count, data() + old_size);
      } else {
        ::std::fill_n(data(), old_size, value);
        ::std::uninitialized_fill_n(data() + old_size, count - old_size, value);
      }
      set_len(count);
    } else {
      clear();
      ::std::uninitialized_fill_n(data(), count, value);
      set_len(count);
    }
  }

  template <typename InputIt>
  void append_iterator_range(InputIt first, InputIt last) {
    if constexpr (::std::forward_iterator<InputIt>) {
      append_sized_range(first, last,
                         static_cast<size_type>(::std::distance(first, last)));
    } else {
      for (; first != last; ++first) {
        emplace_back(*first);
      }
    }
  }

  template <typename It, typename Sent>
  void append_sized_range(It first, Sent last, size_type count) {
    if (count == 0) return;
    reserve_additional_capacity(count);
    if constexpr (::std::contiguous_iterator<It> &&
                  ::std::is_trivially_copyable_v<T> &&
                  ::std::is_same_v<
                      ::std::remove_const_t<::std::remove_reference_t<
                          ::std::iter_reference_t<It>>>,
                      T>) {
      ::std::memcpy(data() + size(), ::std::to_address(first),
                    count * sizeof(T));
      set_len(size() + count);
    } else {
      size_type current_len = size();
      T* base = data();
      for (; first != last; ++first, ++current_len) {
        ::std::construct_at(base + current_len, *first);
      }
      set_len(current_len);
    }
  }
};

template <typename T>
struct Vec final {
  static_assert(false,
                "This type should only be used via a generated specialization");
};

}  // namespace CRUBIT_RS_STD_PRIMARY_NS

#undef CRUBIT_RS_STD_PRIMARY_NS

namespace rs_std {
#ifdef CRUBIT_INTERNAL_RS_NAMESPACE
template <typename T>
using Vec [[deprecated("Use rs::Vec instead")]] = ::rs::Vec<T>;
template <typename T>
using VecBase [[deprecated("Use rs::VecBase instead")]] = ::rs::VecBase<T>;
#endif
}  // namespace rs_std

#ifndef CRUBIT_INTERNAL_RS_NAMESPACE
namespace rs {
template <typename T>
using Vec = ::rs_std::Vec<T>;
template <typename T>
using VecBase = ::rs_std::VecBase<T>;
}  // namespace rs
#endif

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_VEC_H_
