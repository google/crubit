// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// functions_golden
// Features: custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr,
// std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_FUNCTIONS_FUNCTIONS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_FUNCTIONS_FUNCTIONS_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/cxx20_backports.h"
#include "support/rs_std/char.h"

#include <cstdint>

namespace functions {

namespace fn_param_ty_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=72
rs_std::char_ char_to_ascii_lowercase(rs_std::char_ c);

}  // namespace fn_param_ty_tests

namespace fn_attribute_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=128
[[deprecated("★ Deprecated note for add_i32 ★")]] std::int32_t add_i32(
    std::int32_t x, std::int32_t y);

}  // namespace fn_attribute_tests

namespace fn_abi_tests {

//  Testing one of simpler function bindings:
//
//  - `extern "C"` means that no thunk is required
//
//  - `#[unsafe(no_mangle)]` means that the function is already exposed with
//
//    the desired, public name (and just needs to be redeclared in C++).
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=20
extern "C" double get_42_as_f64_via_no_mangle_extern_c();

//  Testing the default / Rust ABI (one used in absence of `extern "C"`).
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=46
std::int32_t add_i32_via_rust_abi(std::int32_t x, std::int32_t y);

}  // namespace fn_abi_tests

namespace fn_param_ty_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=65
void add_i32_via_ptr(std::int32_t const* x, std::int32_t const* y,
                     std::int32_t* sum);

//  Testing a type that requires `#include`ing a standard C++ header.
//
//  `std::int32_t` is one such example - it requires `#include <cstdint>`.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=61
std::int32_t add_i32(std::int32_t x, std::int32_t y);

//  Testing a type that maps to a built-in C++ type (spelled with a
//
//  keyword). `float` is one such example.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=55
double add_f64(double x, double y);

}  // namespace fn_param_ty_tests

namespace other_fn_param_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=121
std::int32_t add_i32_via_rust_abi_with_duplicated_param_names(
    std::int32_t x, std::int32_t y, std::int32_t __param_2,
    std::int32_t __param_3);

}  // namespace other_fn_param_tests

namespace unit_ret_ty_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=110
void set_global_i32_via_extern_c_with_export_name(std::int32_t x);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=115
extern "C" std::int32_t get_global_i32_via_extern_c_with_export_name();

}  // namespace unit_ret_ty_tests

namespace fn_must_use_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=146
[[nodiscard]] std::int32_t no_msg_add(std::int32_t x, std::int32_t y);

}  // namespace fn_must_use_tests

namespace fn_param_ty_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=80
std::int32_t apply_binary_i32_op(
    std::int32_t x, std::int32_t y,
    crubit::type_identity_t<std::int32_t(std::int32_t, std::int32_t)>& f);

}  // namespace fn_param_ty_tests

namespace fn_abi_tests {

//  Testing bindings for an `extern "C"` function (no thunk required) with a
//
//  mangled name. This test verifies that:
//
//  * `cc_bindings_from_rs` can correctly discover mangled names that
//
//    `rustc` produces
//
//  * Bazel support for `cc_bindings_from_rs` invokes it with the same
//
//    command line flags as the ones used when invoking `rustc` when
//
//    building the `functions` crate.
//
//
//
//  TODO(b/262904507): Bazel integration is currently broken and the
//
//  coresponding test is commented out in `functions_test.cc`.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=41
std::int32_t add_i32_via_extern_c_with_mangling(std::int32_t x, std::int32_t y);

}  // namespace fn_abi_tests

namespace fn_param_ty_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=92
std::int32_t const& [[clang::annotate_type(
    "lifetime",
    "__anon1")]] get_identical_ref_with_inferred_lifetime(std::
                                                              int32_t const* [[clang::annotate_type(
                                                                  "lifetime",
                                                                  "__"
                                                                  "anon"
                                                                  "1")]] crubit_nonnull
                                                                  x CRUBIT_LIFETIME_BOUND);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=84
std::int32_t const& [[clang::annotate_type(
    "lifetime",
    "a")]] get_ref_to_smaller_int(std::
                                      int32_t const* [[clang::annotate_type(
                                          "lifetime", "a")]] crubit_nonnull x,
                                  std::int32_t const* [[clang::annotate_type(
                                      "lifetime", "a")]] crubit_nonnull y);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=96
void set_mut_ref_to_sum_of_ints(std::int32_t& sum, std::int32_t x,
                                std::int32_t y);

}  // namespace fn_param_ty_tests

namespace fn_must_use_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=151
[[nodiscard("woohoo")]] std::int32_t msg_add(std::int32_t x, std::int32_t y);

}  // namespace fn_must_use_tests

namespace fn_abi_tests {

//  Testing `#[unsafe(export_name = ...)]` - the generated bindings need to
//
//  forward/proxy the call into a function with a different name.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=27
std::int32_t add_i32_via_extern_c_with_export_name(std::int32_t x,
                                                   std::int32_t y);

}  // namespace fn_abi_tests

namespace unsafe_fn_tests {

//  # Safety
//
//
//
//  This function has no safety requirements - it is only marked as `unsafe`
//
//  to facilitate minimal testing of bindings generated for such functions.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=138
std::int32_t unsafe_add(std::int32_t x, std::int32_t y);

}  // namespace unsafe_fn_tests

namespace fn_param_ty_tests {

namespace __crubit_internal {
extern "C" rs_std::char_ __crubit_thunk_char_uto_uascii_ulowercase(
    rs_std::char_);
}
inline rs_std::char_ char_to_ascii_lowercase(rs_std::char_ c) {
  return __crubit_internal::__crubit_thunk_char_uto_uascii_ulowercase(c);
}

}  // namespace fn_param_ty_tests

namespace fn_attribute_tests {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_add_ui32(std::int32_t, std::int32_t);
}
inline std::int32_t add_i32(std::int32_t x, std::int32_t y) {
  return __crubit_internal::__crubit_thunk_add_ui32(x, y);
}

}  // namespace fn_attribute_tests

namespace fn_abi_tests {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_add_ui32_uvia_urust_uabi(std::int32_t,
                                                                std::int32_t);
}
inline std::int32_t add_i32_via_rust_abi(std::int32_t x, std::int32_t y) {
  return __crubit_internal::__crubit_thunk_add_ui32_uvia_urust_uabi(x, y);
}

}  // namespace fn_abi_tests

namespace fn_param_ty_tests {

namespace __crubit_internal {
extern "C" void __crubit_thunk_add_ui32_uvia_uptr(std::int32_t const*,
                                                  std::int32_t const*,
                                                  std::int32_t*);
}
inline void add_i32_via_ptr(std::int32_t const* x, std::int32_t const* y,
                            std::int32_t* sum) {
  return __crubit_internal::__crubit_thunk_add_ui32_uvia_uptr(x, y, sum);
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_add_ui32(std::int32_t, std::int32_t);
}
inline std::int32_t add_i32(std::int32_t x, std::int32_t y) {
  return __crubit_internal::__crubit_thunk_add_ui32(x, y);
}

namespace __crubit_internal {
extern "C" double __crubit_thunk_add_uf64(double, double);
}
inline double add_f64(double x, double y) {
  return __crubit_internal::__crubit_thunk_add_uf64(x, y);
}

}  // namespace fn_param_ty_tests

namespace other_fn_param_tests {

namespace __crubit_internal {
extern "C" std::int32_t
__crubit_thunk_add_ui32_uvia_urust_uabi_uwith_uduplicated_uparam_unames(
    std::int32_t, std::int32_t, std::int32_t, std::int32_t);
}
inline std::int32_t add_i32_via_rust_abi_with_duplicated_param_names(
    std::int32_t x, std::int32_t y, std::int32_t __param_2,
    std::int32_t __param_3) {
  return __crubit_internal::
      __crubit_thunk_add_ui32_uvia_urust_uabi_uwith_uduplicated_uparam_unames(
          x, y, __param_2, __param_3);
}

}  // namespace other_fn_param_tests

namespace unit_ret_ty_tests {

namespace __crubit_internal {
extern "C" void custom_export_name_for_get_global_i32(std::int32_t);
}
inline void set_global_i32_via_extern_c_with_export_name(std::int32_t x) {
  return __crubit_internal::custom_export_name_for_get_global_i32(x);
}

}  // namespace unit_ret_ty_tests

namespace fn_must_use_tests {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_no_umsg_uadd(std::int32_t, std::int32_t);
}
inline std::int32_t no_msg_add(std::int32_t x, std::int32_t y) {
  return __crubit_internal::__crubit_thunk_no_umsg_uadd(x, y);
}

}  // namespace fn_must_use_tests

namespace fn_param_ty_tests {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_apply_ubinary_ui32_uop(
    std::int32_t, std::int32_t,
    crubit::type_identity_t<std::int32_t(std::int32_t, std::int32_t)>&);
}
inline std::int32_t apply_binary_i32_op(
    std::int32_t x, std::int32_t y,
    crubit::type_identity_t<std::int32_t(std::int32_t, std::int32_t)>& f) {
  return __crubit_internal::__crubit_thunk_apply_ubinary_ui32_uop(x, y, f);
}

}  // namespace fn_param_ty_tests

namespace fn_abi_tests {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_add_ui32_uvia_uextern_uc_uwith_umangling(
    std::int32_t, std::int32_t);
}
inline std::int32_t add_i32_via_extern_c_with_mangling(std::int32_t x,
                                                       std::int32_t y) {
  return __crubit_internal::
      __crubit_thunk_add_ui32_uvia_uextern_uc_uwith_umangling(x, y);
}

}  // namespace fn_abi_tests

namespace fn_param_ty_tests {

namespace __crubit_internal {
extern "C" std::int32_t const& [[clang::annotate_type(
    "lifetime",
    "__anon1")]] __crubit_thunk_get_uidentical_uref_uwith_uinferred_ulifetime(std::
                                                                                  int32_t const* [[clang::annotate_type(
                                                                                      "lifetime",
                                                                                      "__anon1")]] crubit_nonnull);
}
inline std::int32_t const& [[clang::annotate_type(
    "lifetime",
    "__anon1")]] get_identical_ref_with_inferred_lifetime(std::
                                                              int32_t const* [[clang::annotate_type(
                                                                  "lifetime",
                                                                  "__"
                                                                  "anon"
                                                                  "1")]] crubit_nonnull
                                                                  x CRUBIT_LIFETIME_BOUND) {
  return __crubit_internal::
      __crubit_thunk_get_uidentical_uref_uwith_uinferred_ulifetime(x);
}

namespace __crubit_internal {
extern "C" std::int32_t const& [[clang::annotate_type(
    "lifetime",
    "a")]] __crubit_thunk_get_uref_uto_usmaller_uint(std::
                                                         int32_t const* [[clang::annotate_type(
                                                             "lifetime",
                                                             "a")]] crubit_nonnull,
                                                     std::int32_t const* [[clang::annotate_type(
                                                         "lifetime",
                                                         "a")]] crubit_nonnull);
}
inline std::int32_t const& [[clang::annotate_type(
    "lifetime",
    "a")]] get_ref_to_smaller_int(std::
                                      int32_t const* [[clang::annotate_type(
                                          "lifetime", "a")]] crubit_nonnull x,
                                  std::int32_t const* [[clang::annotate_type(
                                      "lifetime", "a")]] crubit_nonnull y) {
  return __crubit_internal::__crubit_thunk_get_uref_uto_usmaller_uint(x, y);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_set_umut_uref_uto_usum_uof_uints(std::int32_t&,
                                                                std::int32_t,
                                                                std::int32_t);
}
inline void set_mut_ref_to_sum_of_ints(std::int32_t& sum, std::int32_t x,
                                       std::int32_t y) {
  return __crubit_internal::__crubit_thunk_set_umut_uref_uto_usum_uof_uints(
      sum, x, y);
}

}  // namespace fn_param_ty_tests

namespace fn_must_use_tests {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_msg_uadd(std::int32_t, std::int32_t);
}
inline std::int32_t msg_add(std::int32_t x, std::int32_t y) {
  return __crubit_internal::__crubit_thunk_msg_uadd(x, y);
}

}  // namespace fn_must_use_tests

namespace fn_abi_tests {

namespace __crubit_internal {
extern "C" std::int32_t custom_export_name_for_add_i32(std::int32_t,
                                                       std::int32_t);
}
inline std::int32_t add_i32_via_extern_c_with_export_name(std::int32_t x,
                                                          std::int32_t y) {
  return __crubit_internal::custom_export_name_for_add_i32(x, y);
}

}  // namespace fn_abi_tests

namespace unsafe_fn_tests {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_unsafe_uadd(std::int32_t, std::int32_t);
}
inline std::int32_t unsafe_add(std::int32_t x, std::int32_t y) {
  return __crubit_internal::__crubit_thunk_unsafe_uadd(x, y);
}

}  // namespace unsafe_fn_tests

}  // namespace functions
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_FUNCTIONS_FUNCTIONS_GOLDEN
