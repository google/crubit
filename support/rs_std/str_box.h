// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_STR_BOX_H_
#define THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_STR_BOX_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <string_view>
#include <utility>

#include "support/movable.h"
#include "support/unsafe_relocate_tag.h"
#include "support/annotations_internal.h"
#include "support/rs_std/internal/is_utf8.h"
#include "support/rs_std/str_ref.h"

extern "C" {
void* crubit_nonnull crubit_alloc(::std::size_t size,
                                  ::std::size_t align) noexcept;
void crubit_dealloc(void* crubit_nonnull ptr, ::std::size_t size,
                    ::std::size_t align) noexcept;
}

namespace rs {

// `rs::StrBox` represents a Rust `Box<str>` in C++.
// It is non-C++-movable, but can be relocated via `crubit::UnsafeRelocateTag`
// and passed by-value via `rs::Movable`. Memory is managed by the Rust global
// allocator.
class CRUBIT_INTERNAL_RUST_TYPE("::alloc::boxed::Box<str>") StrBox final {
 public:
  StrBox() noexcept : dangling_ptr_(alignof(char)), size_(0) {}

  // Allocates a `StrBox` copying `utf8`. Because `StrRef`'s implicit
  // conversions from string literals and `std::string_view`-like types are
  // `consteval`, `StrBox("hello")` validates UTF-8 at compile time while
  // rejecting runtime `std::string_view` values (use `StrBox::FromUtf8` for
  // fallible runtime UTF-8 validation).
  explicit StrBox(StrRef utf8)
      : StrBox(UnsafePromiseUtf8(), utf8.to_string_view()) {}

  // Returns a `Movable<StrBox>` containing a copy of `string_view`, or
  // `std::nullopt` if `string_view` is not valid UTF-8.
  static ::std::optional<::rs::Movable<StrBox>> FromUtf8(
      ::std::string_view string_view) noexcept {
    if (!internal::IsUtf8(string_view)) {
      return ::std::nullopt;
    }
    return FromUtf8Unchecked(string_view);
  }

  // Returns a `Movable<StrBox>` containing a copy of `string_view` without
  // performing UTF-8 validation.
  //
  // NOTE: other code relies on `StrBox` being valid UTF-8, so calls to this
  // function may result in undefined behavior if `string_view` is not UTF-8.
  static ::rs::Movable<StrBox> FromUtf8Unchecked(
      ::std::string_view string_view) noexcept {
    return ::rs::Movable<StrBox>(::std::in_place, UnsafePromiseUtf8(),
                                 string_view);
  }

  // Takes ownership of a pointer allocated by `crubit_alloc(size,
  // alignof(char))`. SAFETY: The pointer must be non-null if size > 0, and
  // valid for the lifetime of this `StrBox`. The buffer must contain valid
  // UTF-8.
  static StrBox unsafe_from_raw(char* crubit_nullable ptr,
                                size_t size) noexcept {
    return StrBox(ptr, size);
  }

  ~StrBox() {
    if (size_ > 0) {
      crubit_dealloc(ptr_, size_, alignof(char));
    }
  }

  StrBox(const StrBox&) = delete;
  StrBox& operator=(const StrBox&) = delete;
  StrBox(StrBox&&) = delete;
  StrBox& operator=(StrBox&&) = delete;

  StrBox(::crubit::UnsafeRelocateTag, StrBox&& other) noexcept
      : StrBox(other.size_ > 0 ? other.ptr_ : nullptr, other.size_) {}

  const char* crubit_nullable data() const noexcept {
    return size_ > 0 ? ptr_ : nullptr;
  }
  size_t size() const noexcept { return size_; }
  bool empty() const noexcept { return size_ == 0; }

  StrRef as_str() const noexcept {
    return StrRef::FromUtf8Unchecked(
        ::std::string_view(data() ? data() : "", size()));
  }

  // NOLINTNEXTLINE(google-explicit-constructor)
  operator StrRef() const noexcept { return as_str(); }

  // NOLINTNEXTLINE(google-explicit-constructor)
  operator ::std::string_view() const noexcept {
    return ::std::string_view(data() ? data() : "", size());
  }

 private:
  friend class ::rs::Movable<StrBox>;

  struct UnsafePromiseUtf8 {};

  StrBox(UnsafePromiseUtf8, ::std::string_view utf8)
      : dangling_ptr_(alignof(char)), size_(utf8.size()) {
    if (size_ > 0) {
      ptr_ = static_cast<char*>(crubit_alloc(size_, alignof(char)));
      ::std::uninitialized_copy_n(utf8.data(), size_, ptr_);
    }
  }

  explicit StrBox(char* crubit_nullable ptr, size_t size)
      : dangling_ptr_(alignof(char)), size_(size) {
    if (size_ > 0) {
      ptr_ = ptr;
    }
  }

  union {
    char* crubit_nonnull ptr_;
    uintptr_t dangling_ptr_;
  };
  size_t size_;
};

}  // namespace rs

#endif  // THIRD_PARTY_CRUBIT_SUPPORT_RS_STD_STR_BOX_H_
