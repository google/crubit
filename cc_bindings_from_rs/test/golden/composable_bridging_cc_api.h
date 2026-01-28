// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// composable_bridging_rust_golden
// Features: assume_lifetimes, custom_ffi_types, experimental, non_unpin_ctor,
// std_unique_ptr, std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_COMPOSABLE_BRIDGING_RUST_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_COMPOSABLE_BRIDGING_RUST_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#include "support/bridge.h"
#include "support/rs_std/slice_ref.h"

#include <cstdint>
#include <optional>

namespace composable_bridging_rust {

// Error generating bindings for `MyOptionRust` defined at
// cc_bindings_from_rs/test/golden/composable_bridging.rs;l=27:
// Type bindings for MyOptionRust suppressed due to being mapped to an existing
// C++ type (std::optional)

// Error generating bindings for `MyOptionRustAbi` defined at
// cc_bindings_from_rs/test/golden/composable_bridging.rs;l=38:
// Generic types are not supported yet (b/259749095)

// Generated from:
// cc_bindings_from_rs/test/golden/composable_bridging.rs;l=29
std::optional<std::int32_t> make_my_option_rust();

// Generated from:
// cc_bindings_from_rs/test/golden/composable_bridging.rs;l=33
std::optional<rs_std::SliceRef<const std::int32_t>> maybe_int_slice();

// Generated from:
// cc_bindings_from_rs/test/golden/composable_bridging.rs;l=20
std::optional<std::int32_t> option_increments(std::optional<std::int32_t> x);

// Generated from:
// cc_bindings_from_rs/test/golden/composable_bridging.rs;l=12
std::optional<std::int32_t> returns_no_int();

// Generated from:
// cc_bindings_from_rs/test/golden/composable_bridging.rs;l=8
std::optional<std::int32_t> returns_some_int();

// Generated from:
// cc_bindings_from_rs/test/golden/composable_bridging.rs;l=16
std::int32_t unwrap_or_zero(std::optional<std::int32_t> x);

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_umy_uoption_urust(unsigned char* __ret_ptr);
}
inline std::optional<std::int32_t> make_my_option_rust() {
  unsigned char __return_value_storage
      [crubit::OptionalAbi<::crubit::TransmuteAbi<std::int32_t>>::kSize];
  __crubit_internal::__crubit_thunk_make_umy_uoption_urust(
      __return_value_storage);
  return ::crubit::internal::Decode<
      crubit::OptionalAbi<::crubit::TransmuteAbi<std::int32_t>>>(
      crubit::OptionalAbi<::crubit::TransmuteAbi<std::int32_t>>(
          ::crubit::TransmuteAbi<std::int32_t>()),
      __return_value_storage);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_maybe_uint_uslice(unsigned char* __ret_ptr);
}
inline std::optional<rs_std::SliceRef<const std::int32_t>> maybe_int_slice() {
  unsigned char __return_value_storage[::crubit::OptionAbi<
      ::crubit::TransmuteAbi<::rs_std::SliceRef<const std::int32_t>>>::kSize];
  __crubit_internal::__crubit_thunk_maybe_uint_uslice(__return_value_storage);
  return ::crubit::internal::Decode<::crubit::OptionAbi<
      ::crubit::TransmuteAbi<::rs_std::SliceRef<const std::int32_t>>>>(
      ::crubit::OptionAbi<
          ::crubit::TransmuteAbi<::rs_std::SliceRef<const std::int32_t>>>(
          ::crubit::TransmuteAbi<::rs_std::SliceRef<const std::int32_t>>()),
      __return_value_storage);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_option_uincrements(unsigned char*,
                                                  unsigned char* __ret_ptr);
}
inline std::optional<std::int32_t> option_increments(
    std::optional<std::int32_t> x) {
  unsigned char x_buffer
      [::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>::kSize];
  ::crubit::internal::Encode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>(
          ::crubit::TransmuteAbi<std::int32_t>()),
      x_buffer, x);
  unsigned char __return_value_storage
      [::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>::kSize];
  __crubit_internal::__crubit_thunk_option_uincrements(x_buffer,
                                                       __return_value_storage);
  return ::crubit::internal::Decode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>(
          ::crubit::TransmuteAbi<std::int32_t>()),
      __return_value_storage);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_returns_uno_uint(unsigned char* __ret_ptr);
}
inline std::optional<std::int32_t> returns_no_int() {
  unsigned char __return_value_storage
      [::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>::kSize];
  __crubit_internal::__crubit_thunk_returns_uno_uint(__return_value_storage);
  return ::crubit::internal::Decode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>(
          ::crubit::TransmuteAbi<std::int32_t>()),
      __return_value_storage);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_returns_usome_uint(unsigned char* __ret_ptr);
}
inline std::optional<std::int32_t> returns_some_int() {
  unsigned char __return_value_storage
      [::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>::kSize];
  __crubit_internal::__crubit_thunk_returns_usome_uint(__return_value_storage);
  return ::crubit::internal::Decode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>(
          ::crubit::TransmuteAbi<std::int32_t>()),
      __return_value_storage);
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_unwrap_uor_uzero(unsigned char*);
}
inline std::int32_t unwrap_or_zero(std::optional<std::int32_t> x) {
  unsigned char x_buffer
      [::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>::kSize];
  ::crubit::internal::Encode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<std::int32_t>>(
          ::crubit::TransmuteAbi<std::int32_t>()),
      x_buffer, x);
  return __crubit_internal::__crubit_thunk_unwrap_uor_uzero(x_buffer);
}

}  // namespace composable_bridging_rust
#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_COMPOSABLE_BRIDGING_RUST_GOLDEN
