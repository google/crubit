// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IR_FFI_SUPPORT_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IR_FFI_SUPPORT_H_

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <initializer_list>
#include <new>
#include <ostream>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/log/check.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "common/ffi_types.h"

namespace crubit::ffi {

constexpr bool IsValidUtf8(absl::string_view sv) {
  auto byte = [&](size_t idx) { return static_cast<unsigned char>(sv[idx]); };
  size_t i = 0;
  const size_t len = sv.size();
  while (i < len) {
    unsigned char b0 = byte(i);
    if (b0 < 0x80) {
      i += 1;
    } else if (b0 >= 0xC2 && b0 <= 0xDF) {
      if (i + 1 >= len || (byte(i + 1) & 0xC0) != 0x80) return false;
      i += 2;
    } else if (b0 == 0xE0) {
      if (i + 2 >= len || byte(i + 1) < 0xA0 || byte(i + 1) > 0xBF ||
          (byte(i + 2) & 0xC0) != 0x80) {
        return false;
      }
      i += 3;
    } else if ((b0 >= 0xE1 && b0 <= 0xEC) || (b0 >= 0xEE && b0 <= 0xEF)) {
      if (i + 2 >= len || (byte(i + 1) & 0xC0) != 0x80 ||
          (byte(i + 2) & 0xC0) != 0x80) {
        return false;
      }
      i += 3;
    } else if (b0 == 0xED) {
      if (i + 2 >= len || byte(i + 1) < 0x80 || byte(i + 1) > 0x9F ||
          (byte(i + 2) & 0xC0) != 0x80) {
        return false;
      }
      i += 3;
    } else if (b0 == 0xF0) {
      if (i + 3 >= len || byte(i + 1) < 0x90 || byte(i + 1) > 0xBF ||
          (byte(i + 2) & 0xC0) != 0x80 || (byte(i + 3) & 0xC0) != 0x80) {
        return false;
      }
      i += 4;
    } else if (b0 >= 0xF1 && b0 <= 0xF3) {
      if (i + 3 >= len || (byte(i + 1) & 0xC0) != 0x80 ||
          (byte(i + 2) & 0xC0) != 0x80 || (byte(i + 3) & 0xC0) != 0x80) {
        return false;
      }
      i += 4;
    } else if (b0 == 0xF4) {
      if (i + 3 >= len || byte(i + 1) < 0x80 || byte(i + 1) > 0x8F ||
          (byte(i + 2) & 0xC0) != 0x80 || (byte(i + 3) & 0xC0) != 0x80) {
        return false;
      }
      i += 4;
    } else {
      return false;
    }
  }
  return true;
}

// An owned UTF-8 string with a C-compatible layout shared between C++ and Rust.
// Memory is allocated and freed via `AllocFfiBytes` / `FreeFfiBytes` so
// ownership can be transferred across FFI in either direction without copying.
class FfiStr {
 public:
  FfiStr() : ptr_(reinterpret_cast<const uint8_t*>(1)), len_(0) {}

  // NOLINTNEXTLINE(google-explicit-constructor)
  FfiStr(absl::string_view sv) : FfiStr() {
    CHECK(IsValidUtf8(sv));
    if (!sv.empty()) {
      uint8_t* buf = static_cast<uint8_t*>(AllocFfiBytes(sv.size(), 1));
      std::memcpy(buf, sv.data(), sv.size());
      ptr_ = buf;
      len_ = sv.size();
    }
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  FfiStr(const std::string& s) : FfiStr(absl::string_view(s)) {}
  // NOLINTNEXTLINE(google-explicit-constructor)
  FfiStr(const char* s) : FfiStr(absl::string_view(s)) {}

  FfiStr(const FfiStr& other) : FfiStr(other.AsView()) {}

  FfiStr(FfiStr&& other) noexcept : ptr_(other.ptr_), len_(other.len_) {
    other.ptr_ = reinterpret_cast<const uint8_t*>(1);
    other.len_ = 0;
  }

  FfiStr& operator=(const FfiStr& other) {
    if (this != &other) {
      *this = FfiStr(other);
    }
    return *this;
  }

  FfiStr& operator=(FfiStr&& other) noexcept {
    if (this != &other) {
      FfiStr tmp(std::move(other));
      std::swap(ptr_, tmp.ptr_);
      std::swap(len_, tmp.len_);
    }
    return *this;
  }

  FfiStr& operator=(absl::string_view sv) {
    *this = FfiStr(sv);
    return *this;
  }

  FfiStr& operator=(const std::string& s) {
    return *this = absl::string_view(s);
  }

  FfiStr& operator=(const char* s) { return *this = absl::string_view(s); }

  ~FfiStr() { Reset(); }

  absl::string_view AsView() const {
    return absl::string_view(reinterpret_cast<const char*>(ptr_), len_);
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  operator absl::string_view() const { return AsView(); }

  std::string ToString() const { return std::string(AsView()); }
  explicit operator std::string() const { return ToString(); }

  const char* data() const { return reinterpret_cast<const char*>(ptr_); }
  size_t size() const { return len_; }
  bool empty() const { return len_ == 0; }
  void clear() { Reset(); }

  friend bool operator==(const FfiStr& lhs, const FfiStr& rhs) {
    return lhs.AsView() == rhs.AsView();
  }
  friend bool operator!=(const FfiStr& lhs, const FfiStr& rhs) {
    return !(lhs == rhs);
  }
  friend bool operator==(const FfiStr& lhs, absl::string_view rhs) {
    return lhs.AsView() == rhs;
  }
  friend bool operator!=(const FfiStr& lhs, absl::string_view rhs) {
    return lhs.AsView() != rhs;
  }
  friend bool operator==(absl::string_view lhs, const FfiStr& rhs) {
    return lhs == rhs.AsView();
  }
  friend bool operator!=(absl::string_view lhs, const FfiStr& rhs) {
    return lhs != rhs.AsView();
  }
  friend bool operator<(const FfiStr& lhs, const FfiStr& rhs) {
    return lhs.AsView() < rhs.AsView();
  }

  template <typename H>
  friend H AbslHashValue(H h, const FfiStr& s) {
    return H::combine(std::move(h), s.AsView());
  }

  friend std::ostream& operator<<(std::ostream& os, const FfiStr& s) {
    return os << s.AsView();
  }

 private:
  void Reset() {
    if (len_ > 0) {
      FreeFfiBytes(const_cast<uint8_t*>(ptr_), len_, 1);
      ptr_ = reinterpret_cast<const uint8_t*>(1);
      len_ = 0;
    }
  }

  const uint8_t* ptr_;
  size_t len_;
};

static_assert(sizeof(FfiStr) == 2 * sizeof(size_t));
static_assert(alignof(FfiStr) == alignof(size_t));
static_assert(std::is_standard_layout_v<FfiStr>);

// An owned contiguous vector with a `#[repr(C)]` layout shared between C++ and
// Rust.
template <typename T>
class FfiVec {
 public:
  using value_type = T;
  using reference = T&;
  using const_reference = const T&;
  using iterator = T*;
  using const_iterator = const T*;
  using size_type = size_t;
  using difference_type = ptrdiff_t;

  constexpr FfiVec() : ptr_(nullptr), len_(0), cap_(0) {}

  FfiVec(std::initializer_list<T> init) : FfiVec() {
    reserve(init.size());
    for (const T& item : init) {
      push_back(item);
    }
  }

  template <typename Iter>
  FfiVec(Iter first, Iter last) : FfiVec() {
    for (; first != last; ++first) {
      push_back(T(*first));
    }
  }

  FfiVec(const FfiVec& other) : FfiVec() {
    reserve(other.len_);
    for (size_t i = 0; i < other.len_; ++i) {
      new (ptr_ + i) T(other.ptr_[i]);
    }
    len_ = other.len_;
  }

  FfiVec(FfiVec&& other) noexcept
      : ptr_(other.ptr_), len_(other.len_), cap_(other.cap_) {
    other.ptr_ = nullptr;
    other.len_ = 0;
    other.cap_ = 0;
  }

  FfiVec& operator=(const FfiVec& other) {
    if (this != &other) {
      *this = FfiVec(other);
    }
    return *this;
  }

  FfiVec& operator=(FfiVec&& other) noexcept {
    if (this != &other) {
      FfiVec tmp(std::move(other));
      std::swap(ptr_, tmp.ptr_);
      std::swap(len_, tmp.len_);
      std::swap(cap_, tmp.cap_);
    }
    return *this;
  }

  ~FfiVec() { Reset(); }

  void reserve(size_t new_cap) {
    if (new_cap <= cap_) return;
    if constexpr (sizeof(T) > 0) {
      CHECK(new_cap <= static_cast<size_t>(-1) / sizeof(T));
    }
    T* new_ptr =
        static_cast<T*>(AllocFfiBytes(new_cap * sizeof(T), alignof(T)));
    for (size_t i = 0; i < len_; ++i) {
      new (new_ptr + i) T(std::move(ptr_[i]));
      ptr_[i].~T();
    }
    if (cap_ > 0) {
      FreeFfiBytes(ptr_, cap_ * sizeof(T), alignof(T));
    }
    ptr_ = new_ptr;
    cap_ = new_cap;
  }

  void Reserve(size_t new_cap) { reserve(new_cap); }

  void push_back(T value) {
    if (len_ == cap_) {
      CHECK(cap_ <= static_cast<size_t>(-1) / 2);
      reserve(cap_ == 0 ? 4 : cap_ * 2);
    }
    new (ptr_ + len_) T(std::move(value));
    ++len_;
  }

  T* Add() {
    if (len_ == cap_) {
      CHECK(cap_ <= static_cast<size_t>(-1) / 2);
      reserve(cap_ == 0 ? 4 : cap_ * 2);
    }
    T* elem = new (ptr_ + len_) T();
    ++len_;
    return elem;
  }

  void Add(T value) { push_back(std::move(value)); }

  template <typename Iter>
  void Add(Iter first, Iter last) {
    for (; first != last; ++first) {
      push_back(T(*first));
    }
  }

  void clear() {
    for (size_t i = 0; i < len_; ++i) {
      ptr_[i].~T();
    }
    len_ = 0;
  }

  void Clear() { clear(); }

  size_t size() const { return len_; }
  bool empty() const { return len_ == 0; }
  const T* data() const { return ptr_; }
  T* data() { return ptr_; }
  const T* begin() const { return ptr_; }
  const T* end() const { return ptr_ + len_; }
  T* begin() { return ptr_; }
  T* end() { return ptr_ + len_; }

  const T& operator[](size_t idx) const {
    CHECK(idx < len_);
    return ptr_[idx];
  }
  T& operator[](size_t idx) {
    CHECK(idx < len_);
    return ptr_[idx];
  }

  absl::Span<const T> AsSpan() const { return absl::Span<const T>(ptr_, len_); }

  friend bool operator==(const FfiVec& lhs, const FfiVec& rhs) {
    return lhs.AsSpan() == rhs.AsSpan();
  }
  friend bool operator!=(const FfiVec& lhs, const FfiVec& rhs) {
    return !(lhs == rhs);
  }

 private:
  void Reset() {
    for (size_t i = 0; i < len_; ++i) {
      ptr_[i].~T();
    }
    if (cap_ > 0) {
      FreeFfiBytes(ptr_, cap_ * sizeof(T), alignof(T));
    }
    ptr_ = nullptr;
    len_ = 0;
    cap_ = 0;
  }

  T* ptr_;
  size_t len_;
  size_t cap_;
};

static_assert(sizeof(FfiVec<uint8_t>) == 3 * sizeof(size_t));
static_assert(alignof(FfiVec<uint8_t>) == alignof(size_t));
static_assert(std::is_standard_layout_v<FfiVec<uint8_t>>);

// Control block allocated via `AllocFfiBytes` and shared with `FfiRc<T>` in
// Rust.
template <typename T>
struct FfiRcInner {
  size_t strong_count;
  T value;
};

// Single-threaded reference-counted nullable heap pointer with a `#[repr(C)]`
// layout matching `FfiRc<T>` in Rust. Allocated directly on the shared FFI heap
// so reference-counted IR nodes can cross FFI without moving or reallocating.
template <typename T>
class FfiRc {
 public:
  constexpr FfiRc() : ptr_(nullptr) {}

  explicit FfiRc(T value) {
    void* mem = AllocFfiBytes(sizeof(FfiRcInner<T>), alignof(FfiRcInner<T>));
    ptr_ = new (mem) FfiRcInner<T>{1, std::move(value)};
  }

  FfiRc(const FfiRc& other) noexcept : ptr_(other.ptr_) {
    if (ptr_ != nullptr) {
      ++ptr_->strong_count;
    }
  }

  FfiRc(FfiRc&& other) noexcept : ptr_(other.ptr_) { other.ptr_ = nullptr; }

  FfiRc& operator=(const FfiRc& other) {
    if (this != &other) {
      FfiRc tmp(other);
      std::swap(ptr_, tmp.ptr_);
    }
    return *this;
  }

  FfiRc& operator=(FfiRc&& other) noexcept {
    if (this != &other) {
      FfiRc tmp(std::move(other));
      std::swap(ptr_, tmp.ptr_);
    }
    return *this;
  }

  ~FfiRc() { Reset(); }

  bool has_value() const { return ptr_ != nullptr; }
  const T* get() const { return ptr_ != nullptr ? &ptr_->value : nullptr; }
  T* get() {
    if (ptr_ == nullptr) return nullptr;
    EnsureUnique();
    return &ptr_->value;
  }
  const T& operator*() const {
    CHECK(ptr_ != nullptr);
    return ptr_->value;
  }
  T& operator*() {
    CHECK(ptr_ != nullptr);
    EnsureUnique();
    return ptr_->value;
  }
  const T* operator->() const {
    CHECK(ptr_ != nullptr);
    return &ptr_->value;
  }
  T* operator->() {
    CHECK(ptr_ != nullptr);
    EnsureUnique();
    return &ptr_->value;
  }

  T& get_or_init() {
    if (ptr_ == nullptr) {
      void* mem = AllocFfiBytes(sizeof(FfiRcInner<T>), alignof(FfiRcInner<T>));
      ptr_ = new (mem) FfiRcInner<T>{1, T()};
    } else {
      EnsureUnique();
    }
    return ptr_->value;
  }

  void Reset() {
    if (ptr_ != nullptr) {
      if (--ptr_->strong_count == 0) {
        ptr_->~FfiRcInner<T>();
        FreeFfiBytes(ptr_, sizeof(FfiRcInner<T>), alignof(FfiRcInner<T>));
      }
      ptr_ = nullptr;
    }
  }

  template <typename U = T>
  friend bool operator==(const FfiRc& lhs, const FfiRc<U>& rhs) {
    if (lhs.ptr_ == rhs.ptr_) return true;
    if (lhs.ptr_ == nullptr || rhs.ptr_ == nullptr) return false;
    return lhs.ptr_->value == rhs.ptr_->value;
  }
  template <typename U = T>
  friend bool operator!=(const FfiRc& lhs, const FfiRc<U>& rhs) {
    return !(lhs == rhs);
  }

 private:
  void EnsureUnique() {
    if (ptr_ != nullptr && ptr_->strong_count > 1) {
      void* mem = AllocFfiBytes(sizeof(FfiRcInner<T>), alignof(FfiRcInner<T>));
      auto* copy = new (mem) FfiRcInner<T>{1, ptr_->value};
      --ptr_->strong_count;
      ptr_ = copy;
    }
  }

  FfiRcInner<T>* ptr_;
};

template <typename T>
using FfiBox = FfiRc<T>;

static_assert(sizeof(FfiRc<uint8_t>) == sizeof(size_t));
static_assert(alignof(FfiRc<uint8_t>) == alignof(size_t));
static_assert(std::is_standard_layout_v<FfiRc<uint8_t>>);

// Key-value pair entry for `FfiMap<K, V>`.
template <typename K, typename V>
struct FfiMapEntry {
  K first;
  V second;

  friend bool operator==(const FfiMapEntry& lhs, const FfiMapEntry& rhs) {
    return lhs.first == rhs.first && lhs.second == rhs.second;
  }
  friend bool operator!=(const FfiMapEntry& lhs, const FfiMapEntry& rhs) {
    return !(lhs == rhs);
  }
};

// An owned map backed by `FfiVec<FfiMapEntry<K, V>>` with a `#[repr(C)]` layout
// shared between C++ and Rust.
template <typename K, typename V>
class FfiMap {
 public:
  using key_type = K;
  using mapped_type = V;
  using value_type = FfiMapEntry<K, V>;
  using reference = value_type&;
  using const_reference = const value_type&;
  using iterator = value_type*;
  using const_iterator = const value_type*;
  using size_type = size_t;
  using difference_type = ptrdiff_t;

  constexpr FfiMap() = default;

  size_t size() const { return entries_.size(); }
  bool empty() const { return entries_.empty(); }
  void clear() { entries_.clear(); }
  void Clear() { entries_.clear(); }

  iterator begin() { return entries_.begin(); }
  iterator end() { return entries_.end(); }
  const_iterator begin() const { return entries_.begin(); }
  const_iterator end() const { return entries_.end(); }

  iterator find(absl::string_view key) {
    for (auto it = begin(); it != end(); ++it) {
      if (it->first == key) return it;
    }
    return end();
  }

  const_iterator find(absl::string_view key) const {
    for (auto it = begin(); it != end(); ++it) {
      if (it->first == key) return it;
    }
    return end();
  }

  bool contains(absl::string_view key) const { return find(key) != end(); }

  V& at(absl::string_view key) {
    auto it = find(key);
    CHECK(it != end());
    return it->second;
  }

  const V& at(absl::string_view key) const {
    auto it = find(key);
    CHECK(it != end());
    return it->second;
  }

  V& operator[](absl::string_view key) {
    auto it = find(key);
    if (it != end()) return it->second;
    value_type* entry = entries_.Add();
    entry->first = K(key);
    return entry->second;
  }

  absl::Span<const value_type> entries() const { return entries_.AsSpan(); }

  friend bool operator==(const FfiMap& lhs, const FfiMap& rhs) {
    if (lhs.size() != rhs.size()) return false;
    for (const auto& entry : lhs) {
      auto it = rhs.find(entry.first);
      if (it == rhs.end() || !(it->second == entry.second)) return false;
    }
    return true;
  }
  friend bool operator!=(const FfiMap& lhs, const FfiMap& rhs) {
    return !(lhs == rhs);
  }

 private:
  FfiVec<value_type> entries_;
};

}  // namespace crubit::ffi

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_FFI_SUPPORT_H_
