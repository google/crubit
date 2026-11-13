// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_IR_FFI_SUPPORT_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_IR_FFI_SUPPORT_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <initializer_list>
#include <new>
#include <ostream>
#include <string>
#include <string_view>
#include <type_traits>
#include <utility>
#include <vector>

#if __has_cpp_attribute(clang::trivial_abi)
#define CRUBIT_FFI_TRIVIAL_ABI [[clang::trivial_abi]]
#else
#define CRUBIT_FFI_TRIVIAL_ABI
#endif
#include "common/check.h"
#include "common/ffi_types.h"
#include "support/rs_std/internal/is_utf8.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseMapInfo.h"
#include "llvm/ADT/Hashing.h"
#include "llvm/ADT/StringRef.h"

namespace crubit::ffi {

// An owned heap-allocated UTF-8 string with a C-compatible layout shared
// between C++ and Rust (`FfiString`). Memory is allocated and freed via
// `AllocFfiBytes` / `FreeFfiBytes` so ownership can be transferred across FFI
// in either direction without copying.
class CRUBIT_FFI_TRIVIAL_ABI FfiString {
 public:
  constexpr FfiString() : dangling_ptr_(1), len_(0) {}

  explicit FfiString(llvm::StringRef sv) : dangling_ptr_(1), len_(0) {
    CRUBIT_CHECK(rs::internal::IsUtf8(sv));
    if (!sv.empty()) {
      uint8_t* buf = static_cast<uint8_t*>(AllocFfiBytes(sv.size(), 1));
      std::memcpy(buf, sv.data(), sv.size());
      ptr_ = buf;
      len_ = sv.size();
    }
  }

  static FfiString FromUtf8(llvm::StringRef sv) { return FfiString(sv); }

  FfiString(const FfiString& other) : dangling_ptr_(1), len_(0) {
    if (other.len_ > 0) {
      uint8_t* buf = static_cast<uint8_t*>(AllocFfiBytes(other.len_, 1));
      std::memcpy(buf, other.ptr_, other.len_);
      ptr_ = buf;
      len_ = other.len_;
    }
  }

  FfiString(FfiString&& other) noexcept : len_(other.len_) {
    if (other.len_ > 0) {
      ptr_ = other.ptr_;
      other.dangling_ptr_ = 1;
      other.len_ = 0;
    } else {
      dangling_ptr_ = 1;
    }
  }

  FfiString& operator=(const FfiString& other) {
    if (this != &other) {
      *this = FfiString(other);
    }
    return *this;
  }

  FfiString& operator=(FfiString&& other) noexcept {
    if (this != &other) {
      Reset();
      if (other.len_ > 0) {
        ptr_ = other.ptr_;
        len_ = other.len_;
        other.dangling_ptr_ = 1;
        other.len_ = 0;
      }
    }
    return *this;
  }

  ~FfiString() { Reset(); }

  llvm::StringRef AsView() const { return llvm::StringRef(data(), len_); }

  // NOLINTNEXTLINE(google-explicit-constructor)
  operator llvm::StringRef() const { return AsView(); }

  std::string ToString() const { return std::string(AsView()); }
  explicit operator std::string() const { return ToString(); }

  const char* data() const {
    return len_ > 0 ? reinterpret_cast<const char*>(ptr_) : nullptr;
  }
  size_t size() const { return len_; }
  bool empty() const { return len_ == 0; }
  void clear() { Reset(); }

  friend bool operator==(const FfiString& lhs, const FfiString& rhs) {
    return lhs.AsView() == rhs.AsView();
  }
  friend bool operator!=(const FfiString& lhs, const FfiString& rhs) {
    return !(lhs == rhs);
  }
  friend bool operator==(const FfiString& lhs, llvm::StringRef rhs) {
    return lhs.AsView() == rhs;
  }
  friend bool operator!=(const FfiString& lhs, llvm::StringRef rhs) {
    return lhs.AsView() != rhs;
  }
  friend bool operator==(llvm::StringRef lhs, const FfiString& rhs) {
    return lhs == rhs.AsView();
  }
  friend bool operator!=(llvm::StringRef lhs, const FfiString& rhs) {
    return lhs != rhs.AsView();
  }
  friend bool operator<(const FfiString& lhs, const FfiString& rhs) {
    return lhs.AsView() < rhs.AsView();
  }
  friend bool operator<(const FfiString& lhs, llvm::StringRef rhs) {
    return lhs.AsView() < rhs;
  }
  friend bool operator<(llvm::StringRef lhs, const FfiString& rhs) {
    return lhs < rhs.AsView();
  }

  friend bool operator==(const FfiString& lhs, std::string_view rhs) {
    return llvm::StringRef(lhs.AsView()) == llvm::StringRef(rhs);
  }
  friend bool operator!=(const FfiString& lhs, std::string_view rhs) {
    return llvm::StringRef(lhs.AsView()) != llvm::StringRef(rhs);
  }
  friend bool operator==(std::string_view lhs, const FfiString& rhs) {
    return llvm::StringRef(lhs) == llvm::StringRef(rhs.AsView());
  }
  friend bool operator!=(std::string_view lhs, const FfiString& rhs) {
    return llvm::StringRef(lhs) != llvm::StringRef(rhs.AsView());
  }
  friend bool operator<(const FfiString& lhs, std::string_view rhs) {
    return llvm::StringRef(lhs.AsView()) < llvm::StringRef(rhs);
  }
  friend bool operator<(std::string_view lhs, const FfiString& rhs) {
    return llvm::StringRef(lhs) < llvm::StringRef(rhs.AsView());
  }

  // Operators for const char* and std::string to avoid ambiguity between
  // StringRef and string_view.
  friend bool operator==(const FfiString& lhs, const char* rhs) {
    return lhs == std::string_view(rhs);
  }
  friend bool operator!=(const FfiString& lhs, const char* rhs) {
    return lhs != std::string_view(rhs);
  }
  friend bool operator==(const char* lhs, const FfiString& rhs) {
    return std::string_view(lhs) == rhs;
  }
  friend bool operator!=(const char* lhs, const FfiString& rhs) {
    return std::string_view(lhs) != rhs;
  }
  friend bool operator<(const FfiString& lhs, const char* rhs) {
    return lhs < std::string_view(rhs);
  }
  friend bool operator<(const char* lhs, const FfiString& rhs) {
    return std::string_view(lhs) < rhs;
  }

  friend bool operator==(const FfiString& lhs, const std::string& rhs) {
    return lhs == std::string_view(rhs);
  }
  friend bool operator!=(const FfiString& lhs, const std::string& rhs) {
    return lhs != std::string_view(rhs);
  }
  friend bool operator==(const std::string& lhs, const FfiString& rhs) {
    return std::string_view(lhs) == rhs;
  }
  friend bool operator!=(const std::string& lhs, const FfiString& rhs) {
    return std::string_view(lhs) != rhs;
  }
  friend bool operator<(const FfiString& lhs, const std::string& rhs) {
    return lhs < std::string_view(rhs);
  }
  friend bool operator<(const std::string& lhs, const FfiString& rhs) {
    return std::string_view(lhs) < rhs;
  }

  template <typename H>
  friend H AbslHashValue(H h, const FfiString& s) {
    return H::combine(std::move(h), std::string_view(s.data(), s.size()));
  }

  friend std::ostream& operator<<(std::ostream& os, const FfiString& s) {
    return os << std::string_view(s.AsView().data(), s.AsView().size());
  }

  friend struct llvm::DenseMapInfo<FfiString>;
  friend struct llvm::DenseMapInfo<class BazelLabel>;
  constexpr FfiString(uintptr_t sentinel_tag, std::nullptr_t)
      : dangling_ptr_(sentinel_tag), len_(0) {}

 private:
  void Reset() {
    if (len_ > 0) {
      FreeFfiBytes(const_cast<uint8_t*>(ptr_), len_, 1);
      dangling_ptr_ = 1;
      len_ = 0;
    }
  }

  // Stick to the following invariants when changing the data member values:
  // (1) `ptr_` and `dangling_ptr_` must never be 0 (to mirror `&str` and
  //     `NonNull<u8>` in Rust).
  // (2) If `len_ > 0`, `ptr_` points to `len_` valid UTF-8 bytes allocated via
  //     `AllocFfiBytes`; otherwise `dangling_ptr_` is 1.
  //
  // `dangling_ptr_` is never dereferenced, and `ptr_` must only be read when
  // `len_ > 0`.
  union {
    const uint8_t* ptr_;
    uintptr_t dangling_ptr_;
  };
  size_t len_;
};

static_assert(sizeof(FfiString) == 2 * sizeof(size_t));
static_assert(alignof(FfiString) == alignof(size_t));
static_assert(std::is_standard_layout_v<FfiString>);

// Representation of a Bazel label (for example //foo/bar:baz) with a
// C-compatible layout matching `BazelLabel` in Rust (`FfiString`).
class CRUBIT_FFI_TRIVIAL_ABI BazelLabel {
 public:
  constexpr BazelLabel() = default;
  explicit BazelLabel(FfiString value) noexcept : value_(std::move(value)) {}
  explicit BazelLabel(llvm::StringRef value) : value_(value) {}
  explicit BazelLabel(std::string_view value)
      : value_(llvm::StringRef(value.data(), value.size())) {}
  explicit BazelLabel(const std::string& value)
      : value_(llvm::StringRef(value)) {}
  explicit BazelLabel(const char* value) : value_(llvm::StringRef(value)) {}

  llvm::StringRef value() const { return value_.AsView(); }
  llvm::StringRef AsView() const { return value_.AsView(); }
  bool empty() const { return value_.empty(); }

  llvm::StringRef target_name() const { return TargetName(AsView()); }
  llvm::StringRef package_name() const { return PackageName(AsView()); }

  friend bool operator==(const BazelLabel& lhs, const BazelLabel& rhs) {
    return ComponentsOf(lhs.AsView()) == ComponentsOf(rhs.AsView());
  }
  friend bool operator!=(const BazelLabel& lhs, const BazelLabel& rhs) {
    return !(lhs == rhs);
  }
  friend bool operator<(const BazelLabel& lhs, const BazelLabel& rhs) {
    return ComponentsOf(lhs.AsView()) < ComponentsOf(rhs.AsView());
  }
  friend bool operator>(const BazelLabel& lhs, const BazelLabel& rhs) {
    return rhs < lhs;
  }
  friend bool operator<=(const BazelLabel& lhs, const BazelLabel& rhs) {
    return !(rhs < lhs);
  }
  friend bool operator>=(const BazelLabel& lhs, const BazelLabel& rhs) {
    return !(lhs < rhs);
  }

  friend bool operator==(const BazelLabel& lhs, llvm::StringRef rhs) {
    return ComponentsOf(lhs.AsView()) == ComponentsOf(rhs);
  }
  friend bool operator!=(const BazelLabel& lhs, llvm::StringRef rhs) {
    return !(lhs == rhs);
  }
  friend bool operator==(llvm::StringRef lhs, const BazelLabel& rhs) {
    return ComponentsOf(lhs) == ComponentsOf(rhs.AsView());
  }
  friend bool operator!=(llvm::StringRef lhs, const BazelLabel& rhs) {
    return !(lhs == rhs);
  }
  friend bool operator<(const BazelLabel& lhs, llvm::StringRef rhs) {
    return ComponentsOf(lhs.AsView()) < ComponentsOf(rhs);
  }
  friend bool operator<(llvm::StringRef lhs, const BazelLabel& rhs) {
    return ComponentsOf(lhs) < ComponentsOf(rhs.AsView());
  }

  friend bool operator==(const BazelLabel& lhs, std::string_view rhs) {
    return lhs == llvm::StringRef(rhs.data(), rhs.size());
  }
  friend bool operator!=(const BazelLabel& lhs, std::string_view rhs) {
    return lhs != llvm::StringRef(rhs.data(), rhs.size());
  }
  friend bool operator==(std::string_view lhs, const BazelLabel& rhs) {
    return llvm::StringRef(lhs.data(), lhs.size()) == rhs;
  }
  friend bool operator!=(std::string_view lhs, const BazelLabel& rhs) {
    return llvm::StringRef(lhs.data(), lhs.size()) != rhs;
  }
  friend bool operator<(const BazelLabel& lhs, std::string_view rhs) {
    return lhs < llvm::StringRef(rhs.data(), rhs.size());
  }
  friend bool operator<(std::string_view lhs, const BazelLabel& rhs) {
    return llvm::StringRef(lhs.data(), lhs.size()) < rhs;
  }

  friend bool operator==(const BazelLabel& lhs, const char* rhs) {
    return lhs == llvm::StringRef(rhs);
  }
  friend bool operator!=(const BazelLabel& lhs, const char* rhs) {
    return lhs != llvm::StringRef(rhs);
  }
  friend bool operator==(const char* lhs, const BazelLabel& rhs) {
    return llvm::StringRef(lhs) == rhs;
  }
  friend bool operator!=(const char* lhs, const BazelLabel& rhs) {
    return llvm::StringRef(lhs) != rhs;
  }
  friend bool operator<(const BazelLabel& lhs, const char* rhs) {
    return lhs < llvm::StringRef(rhs);
  }
  friend bool operator<(const char* lhs, const BazelLabel& rhs) {
    return llvm::StringRef(lhs) < rhs;
  }

  friend bool operator==(const BazelLabel& lhs, const std::string& rhs) {
    return lhs == llvm::StringRef(rhs);
  }
  friend bool operator!=(const BazelLabel& lhs, const std::string& rhs) {
    return lhs != llvm::StringRef(rhs);
  }
  friend bool operator==(const std::string& lhs, const BazelLabel& rhs) {
    return llvm::StringRef(lhs) == rhs;
  }
  friend bool operator!=(const std::string& lhs, const BazelLabel& rhs) {
    return llvm::StringRef(lhs) != rhs;
  }
  friend bool operator<(const BazelLabel& lhs, const std::string& rhs) {
    return lhs < llvm::StringRef(rhs);
  }
  friend bool operator<(const std::string& lhs, const BazelLabel& rhs) {
    return llvm::StringRef(lhs) < rhs;
  }

  template <typename H>
  friend H AbslHashValue(H h, const BazelLabel& s) {
    auto [target, pkg] = ComponentsOf(s.AsView());
    return H::combine(std::move(h),
                      std::string_view(target.data(), target.size()),
                      std::string_view(pkg.data(), pkg.size()));
  }

  friend std::ostream& operator<<(std::ostream& os, const BazelLabel& s) {
    return os << s.value_;
  }

  friend struct llvm::DenseMapInfo<BazelLabel>;

 private:
  static llvm::StringRef TargetName(llvm::StringRef s) {
    size_t colon = s.find(':');
    if (colon != llvm::StringRef::npos) {
      return s.substr(colon + 1);
    }
    size_t slash = s.rfind('/');
    if (slash != llvm::StringRef::npos) {
      return s.substr(slash + 1);
    }
    return s;
  }

  static llvm::StringRef PackageName(llvm::StringRef s) {
    size_t colon = s.rfind(':');
    if (colon != llvm::StringRef::npos) {
      return s.substr(0, colon);
    }
    return s;
  }

  static std::pair<llvm::StringRef, llvm::StringRef> ComponentsOf(
      llvm::StringRef s) {
    return {TargetName(s), PackageName(s)};
  }

  FfiString value_;
};

static_assert(sizeof(BazelLabel) == sizeof(FfiString));
static_assert(alignof(BazelLabel) == alignof(FfiString));
static_assert(std::is_standard_layout_v<BazelLabel>);

// An owned contiguous vector with a `#[repr(C)]` layout shared between C++ and
// Rust.
template <typename T>
class CRUBIT_FFI_TRIVIAL_ABI FfiVec {
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

  explicit FfiVec(llvm::ArrayRef<T> span) : FfiVec() {
    reserve(span.size());
    for (const T& item : span) {
      push_back(item);
    }
  }

  explicit FfiVec(std::vector<T>&& vec) : FfiVec() {
    reserve(vec.size());
    for (T& item : vec) {
      push_back(std::move(item));
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
      CRUBIT_CHECK(new_cap <= static_cast<size_t>(-1) / sizeof(T));
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

  template <typename... Args>
  T& emplace_back(Args&&... args) {
    if (len_ == cap_) {
      CRUBIT_CHECK(cap_ <= static_cast<size_t>(-1) / 2);
      reserve(cap_ == 0 ? 4 : cap_ * 2);
    }
    T* elem = new (ptr_ + len_) T(std::forward<Args>(args)...);
    ++len_;
    return *elem;
  }

  void push_back(T value) { emplace_back(std::move(value)); }

  void pop_back() {
    CRUBIT_CHECK(len_ > 0);
    --len_;
    ptr_[len_].~T();
  }

  T* insert(size_t idx, T value) {
    CRUBIT_CHECK(idx <= len_);
    if (len_ == cap_) {
      CRUBIT_CHECK(cap_ <= static_cast<size_t>(-1) / 2);
      reserve(cap_ == 0 ? 4 : cap_ * 2);
    }
    if (idx < len_) {
      new (ptr_ + len_) T(std::move(ptr_[len_ - 1]));
      for (size_t i = len_ - 1; i > idx; --i) {
        ptr_[i] = std::move(ptr_[i - 1]);
      }
      ptr_[idx] = std::move(value);
    } else {
      new (ptr_ + len_) T(std::move(value));
    }
    ++len_;
    return ptr_ + idx;
  }

  void clear() {
    for (size_t i = 0; i < len_; ++i) {
      ptr_[i].~T();
    }
    len_ = 0;
  }

  size_t size() const { return len_; }
  bool empty() const { return len_ == 0; }
  const T* data() const { return ptr_; }
  T* data() { return ptr_; }
  const T* begin() const { return ptr_; }
  const T* end() const { return ptr_ + len_; }
  T* begin() { return ptr_; }
  T* end() { return ptr_ + len_; }

  const T& front() const {
    CRUBIT_CHECK(len_ > 0);
    return ptr_[0];
  }
  T& front() {
    CRUBIT_CHECK(len_ > 0);
    return ptr_[0];
  }
  const T& back() const {
    CRUBIT_CHECK(len_ > 0);
    return ptr_[len_ - 1];
  }
  T& back() {
    CRUBIT_CHECK(len_ > 0);
    return ptr_[len_ - 1];
  }

  const T& operator[](size_t idx) const {
    CRUBIT_CHECK(idx < len_);
    return ptr_[idx];
  }
  T& operator[](size_t idx) {
    CRUBIT_CHECK(idx < len_);
    return ptr_[idx];
  }

  llvm::ArrayRef<T> AsSpan() const { return llvm::ArrayRef<T>(ptr_, len_); }

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

// Returns a process-lifetime default-constructed instance of `T` for empty
// union and `FfiRc<T>` getters.
template <typename T>
const T& DefaultValue() {
  static const T& kDefault = *new T();
  return kDefault;
}

// Control block allocated via `AllocFfiBytes` and shared with `FfiRc<T>` in
// Rust. Reference counting is single-threaded (non-atomic), matching Rust's
// `!Send + !Sync` `FfiRc<T>`.
template <typename T>
struct FfiRcInner {
  size_t strong_count;
  T value;
};

// Single-threaded reference-counted nullable heap pointer with a `#[repr(C)]`
// layout matching `FfiRc<T>` in Rust. Allocated directly on the shared FFI heap
// so reference-counted IR nodes can cross FFI on the same thread without moving
// or reallocating. Must not be shared across threads.
template <typename T>
class CRUBIT_FFI_TRIVIAL_ABI FfiRc {
 public:
  constexpr FfiRc() : ptr_(nullptr) {}

  explicit FfiRc(T value) {
    void* mem = AllocFfiBytes(sizeof(FfiRcInner<T>), alignof(FfiRcInner<T>));
    ptr_ = new (mem) FfiRcInner<T>{1, std::move(value)};
  }

  FfiRc(const FfiRc& other) noexcept : ptr_(other.ptr_) {
    if (ptr_ != nullptr) {
      CRUBIT_CHECK(ptr_->strong_count < static_cast<size_t>(-1) / 2);
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
    CRUBIT_CHECK(ptr_ != nullptr);
    return ptr_->value;
  }
  T& operator*() {
    CRUBIT_CHECK(ptr_ != nullptr);
    EnsureUnique();
    return ptr_->value;
  }
  const T* operator->() const {
    CRUBIT_CHECK(ptr_ != nullptr);
    return &ptr_->value;
  }
  T* operator->() {
    CRUBIT_CHECK(ptr_ != nullptr);
    EnsureUnique();
    return &ptr_->value;
  }

  const T& get_or_default() const {
    return ptr_ != nullptr ? ptr_->value : DefaultValue<T>();
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
    return lhs.get_or_default() == rhs.get_or_default();
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

static_assert(sizeof(FfiRc<uint8_t>) == sizeof(size_t));
static_assert(alignof(FfiRc<uint8_t>) == alignof(size_t));
static_assert(std::is_standard_layout_v<FfiRc<uint8_t>>);

// Key-value pair entry for `FfiMap<K, V>`.
template <typename K, typename V>
struct CRUBIT_FFI_TRIVIAL_ABI FfiMapEntry {
  K first;
  V second;

  friend bool operator==(const FfiMapEntry& lhs, const FfiMapEntry& rhs) {
    return lhs.first == rhs.first && lhs.second == rhs.second;
  }
  friend bool operator!=(const FfiMapEntry& lhs, const FfiMapEntry& rhs) {
    return !(lhs == rhs);
  }
};

// An owned sorted map backed by `FfiVec<FfiMapEntry<K, V>>` with a `#[repr(C)]`
// layout shared between C++ and Rust. Entries are kept sorted by key so
// iteration order, equality, and hashing are deterministic.
template <typename K, typename V>
class CRUBIT_FFI_TRIVIAL_ABI FfiMap {
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

  iterator begin() { return entries_.begin(); }
  iterator end() { return entries_.end(); }
  const_iterator begin() const { return entries_.begin(); }
  const_iterator end() const { return entries_.end(); }

  template <typename Key = K>
  iterator find(const Key& key) {
    auto it = std::lower_bound(
        begin(), end(), key,
        [](const value_type& entry, const Key& k) { return entry.first < k; });
    if (it != end() && it->first == key) return it;
    return end();
  }

  template <typename Key = K>
  const_iterator find(const Key& key) const {
    auto it = std::lower_bound(
        begin(), end(), key,
        [](const value_type& entry, const Key& k) { return entry.first < k; });
    if (it != end() && it->first == key) return it;
    return end();
  }

  template <typename Key = K>
  bool contains(const Key& key) const {
    return find(key) != end();
  }

  template <typename Key = K>
  V& at(const Key& key) {
    auto it = find(key);
    CRUBIT_CHECK(it != end());
    return it->second;
  }

  template <typename Key = K>
  const V& at(const Key& key) const {
    auto it = find(key);
    CRUBIT_CHECK(it != end());
    return it->second;
  }

  template <typename Key = K>
  V& operator[](const Key& key) {
    auto it = std::lower_bound(
        begin(), end(), key,
        [](const value_type& entry, const Key& k) { return entry.first < k; });
    if (it != end() && it->first == key) return it->second;
    size_t idx = static_cast<size_t>(it - begin());
    return entries_.insert(idx, value_type{K(key), V()})->second;
  }

  llvm::ArrayRef<value_type> entries() const { return entries_.AsSpan(); }

  friend bool operator==(const FfiMap& lhs, const FfiMap& rhs) {
    return lhs.entries_ == rhs.entries_;
  }
  friend bool operator!=(const FfiMap& lhs, const FfiMap& rhs) {
    return !(lhs == rhs);
  }

 private:
  FfiVec<value_type> entries_;
};

}  // namespace crubit::ffi

namespace llvm {
template <>
struct DenseMapInfo<crubit::ffi::FfiString> {
  static crubit::ffi::FfiString getEmptyKey() {
    return crubit::ffi::FfiString(2, nullptr);
  }
  static crubit::ffi::FfiString getTombstoneKey() {
    return crubit::ffi::FfiString(3, nullptr);
  }
  static unsigned getHashValue(const crubit::ffi::FfiString& Val) {
    return static_cast<unsigned>(llvm::hash_value(Val.AsView()));
  }
  static bool isEqual(const crubit::ffi::FfiString& LHS,
                      const crubit::ffi::FfiString& RHS) {
    if ((LHS.len_ == 0 && LHS.dangling_ptr_ != 1) ||
        (RHS.len_ == 0 && RHS.dangling_ptr_ != 1)) {
      return LHS.dangling_ptr_ == RHS.dangling_ptr_ && LHS.len_ == RHS.len_;
    }
    return LHS == RHS;
  }
};

template <>
struct DenseMapInfo<crubit::ffi::BazelLabel> {
  static crubit::ffi::BazelLabel getEmptyKey() {
    return crubit::ffi::BazelLabel(
        DenseMapInfo<crubit::ffi::FfiString>::getEmptyKey());
  }
  static crubit::ffi::BazelLabel getTombstoneKey() {
    return crubit::ffi::BazelLabel(
        DenseMapInfo<crubit::ffi::FfiString>::getTombstoneKey());
  }
  static unsigned getHashValue(const crubit::ffi::BazelLabel& Val) {
    return static_cast<unsigned>(
        llvm::hash_combine(Val.target_name(), Val.package_name()));
  }
  static bool isEqual(const crubit::ffi::BazelLabel& LHS,
                      const crubit::ffi::BazelLabel& RHS) {
    if ((LHS.value_.len_ == 0 && LHS.value_.dangling_ptr_ != 1) ||
        (RHS.value_.len_ == 0 && RHS.value_.dangling_ptr_ != 1)) {
      return DenseMapInfo<crubit::ffi::FfiString>::isEqual(LHS.value_,
                                                           RHS.value_);
    }
    return LHS == RHS;
  }
};
}  // namespace llvm

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_IR_FFI_SUPPORT_H_
