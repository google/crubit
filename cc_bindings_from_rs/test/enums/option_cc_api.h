// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// option_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector,
// supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#include "support/annotations_internal.h"
#include "support/bridge.h"
#include "support/internal/slot.h"
#include "support/rs_std/option.h"

#include <cstddef>
#include <cstdint>
#include <optional>
#include <type_traits>
#include <utility>

namespace option {

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=8
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: NonMaxU8") alignas(1)
    [[clang::trivial_abi]] NonMaxU8 final {
 public:
  // `option_golden::NonMaxU8` doesn't implement the `Default` trait
  NonMaxU8() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NonMaxU8() = default;
  NonMaxU8(NonMaxU8&&) = default;
  NonMaxU8& operator=(NonMaxU8&&) = default;

  // `option_golden::NonMaxU8` doesn't implement the `Clone` trait
  NonMaxU8(const NonMaxU8&) = delete;
  NonMaxU8& operator=(const NonMaxU8&) = delete;
  NonMaxU8(::crubit::UnsafeRelocateTag, NonMaxU8&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=11
  std::uint8_t value() const;

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=8
    std::uint8_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cu8_x0000003e
#define _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cu8_x0000003e
template <>
struct rs_std::Option<std::uint8_t> {
 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[2];

 private:
  static void __crubit_field_offset_assertions();

 public:
  std::optional<std::uint8_t> take_optional() noexcept {
    auto tag =
        reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 0);
    if (*tag == 0) {
      return std::nullopt;
    } else {
      *tag = 0;
      return std::optional<std::uint8_t>(*reinterpret_cast<const std::uint8_t*>(
          reinterpret_cast<const char*>(this) + 1));
    }
  }
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aNonMaxU8_x0000003e
#define _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aNonMaxU8_x0000003e
template <>
struct rs_std::Option<::option::NonMaxU8> {
 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[1];

 private:
  static void __crubit_field_offset_assertions();

 public:
  std::optional<::option::NonMaxU8> take_optional() noexcept {
    auto tag =
        reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 0);
    if (*tag - 251 == 0) {
      return std::nullopt;
    } else {
      ::option::NonMaxU8 value(
          std::move(*reinterpret_cast<::option::NonMaxU8*>(this)));
      *tag = 251;
      return std::make_optional(std::move(value));
    }
  }
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aNonMaxU8_x0000003e_x0000003e
#define _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aNonMaxU8_x0000003e_x0000003e
template <>
struct rs_std::Option<rs_std::Option<::option::NonMaxU8>> {
 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[1];

 private:
  static void __crubit_field_offset_assertions();

 public:
  std::optional<rs_std::Option<::option::NonMaxU8>> take_optional() noexcept {
    auto tag =
        reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 0);
    if (*tag - 252 == 0) {
      return std::nullopt;
    } else {
      rs_std::Option<::option::NonMaxU8> value(std::move(
          *reinterpret_cast<rs_std::Option<::option::NonMaxU8>*>(this)));
      *tag = 252;
      return std::make_optional(std::move(value));
    }
  }
};
#endif

namespace option {

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=16
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasOptions") alignas(1)
    [[clang::trivial_abi]] HasOptions final {
 public:
  // `option_golden::HasOptions` doesn't implement the `Default` trait
  HasOptions() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~HasOptions() = default;
  HasOptions(HasOptions&&) = default;
  HasOptions& operator=(HasOptions&&) = default;

  // `option_golden::HasOptions` doesn't implement the `Clone` trait
  HasOptions(const HasOptions&) = delete;
  HasOptions& operator=(const HasOptions&) = delete;
  HasOptions(::crubit::UnsafeRelocateTag, HasOptions&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=23
  static ::option::HasOptions new_(std::uint8_t value);

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=29
  static ::option::HasOptions with_option(std::optional<std::uint8_t> value);

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=39
  static ::option::HasOptions with_none();

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=19
    rs_std::Option<std::uint8_t> c;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=17
    rs_std::Option<::option::NonMaxU8> a;
  };
  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=18
    rs_std::Option<rs_std::Option<::option::NonMaxU8>> b;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace option

#ifndef _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasOptions_x0000003e
#define _CRUBIT_BINDINGS_FOR_std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003coption_ugolden_x0000003a_x0000003aHasOptions_x0000003e
template <>
struct rs_std::Option<::option::HasOptions> {
 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  unsigned char __opaque_blob_of_bytes[4];

 private:
  static void __crubit_field_offset_assertions();

 public:
  std::optional<::option::HasOptions> take_optional() noexcept {
    auto tag =
        reinterpret_cast<std::uint8_t*>(reinterpret_cast<char*>(this) + 0);
    if (*tag - 2 == 0) {
      return std::nullopt;
    } else {
      ::option::HasOptions value(
          std::move(*reinterpret_cast<::option::HasOptions*>(this)));
      *tag = 2;
      return std::make_optional(std::move(value));
    }
  }
};
#endif

namespace option {

// Generated from:
// cc_bindings_from_rs/test/enums/option.rs;l=44
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_golden :: HasHasOptions") alignas(1)
    [[clang::trivial_abi]] HasHasOptions final {
 public:
  // `option_golden::HasHasOptions` doesn't implement the `Default` trait
  HasHasOptions() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~HasHasOptions() = default;
  HasHasOptions(HasHasOptions&&) = default;
  HasHasOptions& operator=(HasHasOptions&&) = default;

  // `option_golden::HasHasOptions` doesn't implement the `Clone` trait
  HasHasOptions(const HasHasOptions&) = delete;
  HasHasOptions& operator=(const HasHasOptions&) = delete;
  HasHasOptions(::crubit::UnsafeRelocateTag, HasHasOptions&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/enums/option.rs;l=49
  static ::option::HasHasOptions new_(std::uint8_t value);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/enums/option.rs;l=45
    rs_std::Option<::option::HasOptions> me;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(HasHasOptions) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasHasOptions) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<HasHasOptions>);
static_assert(std::is_trivially_move_constructible_v<HasHasOptions>);
static_assert(std::is_trivially_move_assignable_v<HasHasOptions>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(std::uint8_t,
                                   ::option::HasHasOptions* __ret_ptr);
}
inline ::option::HasHasOptions HasHasOptions::new_(std::uint8_t value) {
  crubit::Slot<::option::HasHasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void HasHasOptions::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasHasOptions, me));
}
static_assert(
    sizeof(HasOptions) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasOptions) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<HasOptions>);
static_assert(std::is_trivially_move_constructible_v<HasOptions>);
static_assert(std::is_trivially_move_assignable_v<HasOptions>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(std::uint8_t,
                                   ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::new_(std::uint8_t value) {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_with_uoption(unsigned char*,
                                            ::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::with_option(
    std::optional<std::uint8_t> value) {
  unsigned char value_buffer
      [::crubit::OptionAbi<::crubit::TransmuteAbi<std::uint8_t>>::kSize];
  ::crubit::internal::Encode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::uint8_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::uint8_t>>(
          ::crubit::TransmuteAbi<std::uint8_t>()),
      value_buffer, value);
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_with_uoption(value_buffer,
                                                 __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_with_unone(::option::HasOptions* __ret_ptr);
}
inline ::option::HasOptions HasOptions::with_none() {
  crubit::Slot<::option::HasOptions> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_with_unone(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void HasOptions::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasOptions, c));
  static_assert(2 == offsetof(HasOptions, a));
  static_assert(3 == offsetof(HasOptions, b));
}
static_assert(
    sizeof(NonMaxU8) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonMaxU8) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<NonMaxU8>);
static_assert(std::is_trivially_move_constructible_v<NonMaxU8>);
static_assert(std::is_trivially_move_assignable_v<NonMaxU8>);
namespace __crubit_internal {
extern "C" std::uint8_t __crubit_thunk_value(::option::NonMaxU8 const&);
}
inline std::uint8_t NonMaxU8::value() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_value(self);
}
inline void NonMaxU8::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NonMaxU8, __field0));
}
}  // namespace option

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_GOLDEN
