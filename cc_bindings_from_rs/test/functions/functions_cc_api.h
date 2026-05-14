// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// functions_golden
// Features: assume_lifetimes, assume_this_lifetimes, callables,
// check_default_initialized, experimental, fmt, leading_colons_for_cpp_type,
// supported, types, unsafe_view, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_FUNCTIONS_FUNCTIONS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_FUNCTIONS_FUNCTIONS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/check_no_mutable_aliasing.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/char.h"
#include "support/rs_std/slice_ref.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>

namespace functions::fn_abi_tests {

//  Testing `#[unsafe(export_name = ...)]` - the generated bindings need to
//  forward/proxy the call into a function with a different name.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=27
::std::int32_t add_i32_via_extern_c_with_export_name(::std::int32_t x,
                                                     ::std::int32_t y);

//  Testing bindings for an `extern "C"` function (no thunk required) with a
//  mangled name. This test verifies that:
//  * `cc_bindings_from_rs` can correctly discover mangled names that
//    `rustc` produces
//  * Bazel support for `cc_bindings_from_rs` invokes it with the same
//    command line flags as the ones used when invoking `rustc` when
//    building the `functions` crate.
//
//  TODO(b/262904507): Bazel integration is currently broken and the
//  coresponding test is commented out in `functions_test.cc`.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=41
::std::int32_t add_i32_via_extern_c_with_mangling(::std::int32_t x,
                                                  ::std::int32_t y);

//  Testing the default / Rust ABI (one used in absence of `extern "C"`).
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=46
::std::int32_t add_i32_via_rust_abi(::std::int32_t x, ::std::int32_t y);

//  Testing one of simpler function bindings:
//  - `extern "C"` means that no thunk is required
//  - `#[unsafe(no_mangle)]` means that the function is already exposed with
//    the desired, public name (and just needs to be redeclared in C++).
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=20
extern "C" double get_42_as_f64_via_no_mangle_extern_c();

}  // namespace functions::fn_abi_tests

namespace functions::fn_attribute_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=128
[[deprecated("★ Deprecated note for add_i32 ★")]] ::std::int32_t add_i32(
    ::std::int32_t x, ::std::int32_t y);

}  // namespace functions::fn_attribute_tests

namespace functions::fn_must_use_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=151
[[nodiscard("woohoo")]] ::std::int32_t msg_add(::std::int32_t x,
                                               ::std::int32_t y);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=146
[[nodiscard]] ::std::int32_t no_msg_add(::std::int32_t x, ::std::int32_t y);

}  // namespace functions::fn_must_use_tests

namespace functions::fn_param_ty_tests {

//  Testing a type that maps to a built-in C++ type (spelled with a
//  keyword). `float` is one such example.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=55
double add_f64(double x, double y);

//  Testing a type that requires `#include`ing a standard C++ header.
//  `std::int32_t` is one such example - it requires `#include <cstdint>`.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=61
::std::int32_t add_i32(::std::int32_t x, ::std::int32_t y);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=65
void add_i32_via_ptr(::std::int32_t const* x, ::std::int32_t const* y,
                     ::std::int32_t* sum);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=80
::std::int32_t apply_binary_i32_op(
    ::std::int32_t x, ::std::int32_t y,
    crubit::type_identity_t<::std::int32_t(::std::int32_t, ::std::int32_t)>& f);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=72
rs_std::char_ char_to_ascii_lowercase(rs_std::char_ c);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=92
::std::int32_t const& $(__anon1) get_identical_ref_with_inferred_lifetime(
    ::std::int32_t const* $(__anon1) crubit_nonnull x CRUBIT_LIFETIME_BOUND);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=84
::std::int32_t const& $a
get_ref_to_smaller_int(::std::int32_t const* $a crubit_nonnull x,
                       ::std::int32_t const* $a crubit_nonnull y);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=96
void set_mut_ref_to_sum_of_ints(::std::int32_t& sum, ::std::int32_t x,
                                ::std::int32_t y);

}  // namespace functions::fn_param_ty_tests

namespace functions::generic_fn_tests::as_mut_trait_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=269
void prefix_sums(rs_std::SliceRef<::std::int32_t> arg);

}  // namespace functions::generic_fn_tests::as_mut_trait_tests

namespace functions::generic_fn_tests::as_ref_trait_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=249
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: functions_golden :: generic_fn_tests :: as_ref_trait_tests :: "
    "MyStruct") alignas(4) [[clang::trivial_abi]] MyStruct final {
 public:
  // `functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct` doesn't
  // implement the `Default` trait
  MyStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyStruct() = default;
  MyStruct(MyStruct&&) = default;
  MyStruct& operator=(MyStruct&&) = default;

  // `functions_golden::generic_fn_tests::as_ref_trait_tests::MyStruct` doesn't
  // implement the `Clone` trait
  MyStruct(const MyStruct&) = delete;
  MyStruct& operator=(const MyStruct&) = delete;
  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/functions/functions.rs;l=252
  static ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct new_(
      ::std::int32_t x);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/functions/functions.rs;l=249
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

//  The substitution `impl AsRef<[i32]>` => `&[u32]` needs to "conjure" a new,
//  late-bound lifetime/region.  The test below is an ad-hoc attempt to test
//  that the new region doesn't somehow clobber/conflict with existing implicit
//  or explicit lifetimes. `impl AsRef<[i32]>` is "sandwiched" in the middle to
//  increase the chances that a conflict would be caught somehow.  The test
//  never failed, so it's unclear how useful it is.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=219
void diverse_lifetimes(rs_std::SliceRef<const ::std::int32_t> arg1,
                       rs_std::SliceRef<const ::std::int32_t> arg2,
                       rs_std::SliceRef<const ::std::int32_t> arg3,
                       rs_std::SliceRef<::std::int32_t> result);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=208
::std::int32_t slice_ref_sum(rs_std::SliceRef<const ::std::int32_t> arg);

//  This is an attempt to trigger an error seen in
//  https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=42303eaaafe4a3538ad259e9e9b67f05
//
//  Today the error doesn't happen in Crubit, because the thunks explicitly
//  declare all their lifetimes as `'static` - see `fn
//  replace_all_regions_with_static`.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=241
::std::int32_t static_lifetime_requirement(
    rs_std::SliceRef<const ::std::int32_t> arg);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=263
::std::int32_t struct_ref(
    ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct const& arg);

}  // namespace functions::generic_fn_tests::as_ref_trait_tests

namespace functions::generic_fn_tests::into_trait_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=158
::std::int32_t basic_test(::std::int32_t arg);

//  This test was initially added to cover/verify the call to
//  `super_visit_with` from an `impl` of `GenericParamsFinder` in
//  `get_generic_args.rs`.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=184
::std::int32_t generic_param_nested_deeper_in_param_ty(
    ::std::array<::std::int32_t, 3> xs);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=173
::std::int32_t multiple_generic_params(::std::int32_t x, ::std::int32_t y);

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=177
::std::int32_t return_type();

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=169
::std::int32_t reused_generic_param(::std::int32_t x, ::std::int32_t y);

// Error generating bindings for function
// `functions_golden::generic_fn_tests::into_trait_tests::unused_generic_param`
// defined at
// cc_bindings_from_rs/test/functions/functions.rs;l=204:
// No support for replacing an _unused_ generic type param: `T`

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=162
::std::int32_t where_clause(::std::int32_t x);

}  // namespace functions::generic_fn_tests::into_trait_tests

namespace functions::other_fn_param_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=121
::std::int32_t add_i32_via_rust_abi_with_duplicated_param_names(
    ::std::int32_t x, ::std::int32_t y, ::std::int32_t __param_2,
    ::std::int32_t __param_3);

}  // namespace functions::other_fn_param_tests

namespace functions::unit_ret_ty_tests {

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=115
extern "C" ::std::int32_t get_global_i32_via_extern_c_with_export_name();

// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=110
void set_global_i32_via_extern_c_with_export_name(::std::int32_t x);

}  // namespace functions::unit_ret_ty_tests

namespace functions::unsafe_fn_tests {

//  # Safety
//
//  This function has no safety requirements - it is only marked as `unsafe`
//  to facilitate minimal testing of bindings generated for such functions.
//
// Generated from:
// cc_bindings_from_rs/test/functions/functions.rs;l=138
::std::int32_t unsafe_add(::std::int32_t x, ::std::int32_t y);

}  // namespace functions::unsafe_fn_tests

namespace functions::fn_abi_tests {

namespace __crubit_internal {
extern "C" ::std::int32_t custom_export_name_for_add_i32(::std::int32_t,
                                                         ::std::int32_t);
}
inline ::std::int32_t add_i32_via_extern_c_with_export_name(::std::int32_t x,
                                                            ::std::int32_t y) {
  return __crubit_internal::custom_export_name_for_add_i32(x, y);
}

namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_add_ui32_uvia_uextern_uc_uwith_umangling(::std::int32_t,
                                                        ::std::int32_t);
}
inline ::std::int32_t add_i32_via_extern_c_with_mangling(::std::int32_t x,
                                                         ::std::int32_t y) {
  return __crubit_internal::
      __crubit_thunk_add_ui32_uvia_uextern_uc_uwith_umangling(x, y);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_add_ui32_uvia_urust_uabi(
    ::std::int32_t, ::std::int32_t);
}
inline ::std::int32_t add_i32_via_rust_abi(::std::int32_t x, ::std::int32_t y) {
  return __crubit_internal::__crubit_thunk_add_ui32_uvia_urust_uabi(x, y);
}

}  // namespace functions::fn_abi_tests

namespace functions::fn_attribute_tests {

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_add_ui32(::std::int32_t,
                                                  ::std::int32_t);
}
inline ::std::int32_t add_i32(::std::int32_t x, ::std::int32_t y) {
  return __crubit_internal::__crubit_thunk_add_ui32(x, y);
}

}  // namespace functions::fn_attribute_tests

namespace functions::fn_must_use_tests {

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_msg_uadd(::std::int32_t,
                                                  ::std::int32_t);
}
inline ::std::int32_t msg_add(::std::int32_t x, ::std::int32_t y) {
  return __crubit_internal::__crubit_thunk_msg_uadd(x, y);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_no_umsg_uadd(::std::int32_t,
                                                      ::std::int32_t);
}
inline ::std::int32_t no_msg_add(::std::int32_t x, ::std::int32_t y) {
  return __crubit_internal::__crubit_thunk_no_umsg_uadd(x, y);
}

}  // namespace functions::fn_must_use_tests

namespace functions::fn_param_ty_tests {

namespace __crubit_internal {
extern "C" double __crubit_thunk_add_uf64(double, double);
}
inline double add_f64(double x, double y) {
  return __crubit_internal::__crubit_thunk_add_uf64(x, y);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_add_ui32(::std::int32_t,
                                                  ::std::int32_t);
}
inline ::std::int32_t add_i32(::std::int32_t x, ::std::int32_t y) {
  return __crubit_internal::__crubit_thunk_add_ui32(x, y);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_add_ui32_uvia_uptr(::std::int32_t const*,
                                                  ::std::int32_t const*,
                                                  ::std::int32_t*);
}
inline void add_i32_via_ptr(::std::int32_t const* x, ::std::int32_t const* y,
                            ::std::int32_t* sum) {
  return __crubit_internal::__crubit_thunk_add_ui32_uvia_uptr(x, y, sum);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_apply_ubinary_ui32_uop(
    ::std::int32_t, ::std::int32_t,
    crubit::type_identity_t<::std::int32_t(::std::int32_t, ::std::int32_t)>&);
}
inline ::std::int32_t apply_binary_i32_op(
    ::std::int32_t x, ::std::int32_t y,
    crubit::type_identity_t<::std::int32_t(::std::int32_t, ::std::int32_t)>&
        f) {
  return __crubit_internal::__crubit_thunk_apply_ubinary_ui32_uop(x, y, f);
}

namespace __crubit_internal {
extern "C" rs_std::char_ __crubit_thunk_char_uto_uascii_ulowercase(
    rs_std::char_);
}
inline rs_std::char_ char_to_ascii_lowercase(rs_std::char_ c) {
  return __crubit_internal::__crubit_thunk_char_uto_uascii_ulowercase(c);
}

namespace __crubit_internal {
extern "C" ::std::int32_t const& $(__anon1)
    __crubit_thunk_get_uidentical_uref_uwith_uinferred_ulifetime(
        ::std::int32_t const* $(__anon1) crubit_nonnull);
}
inline ::std::int32_t const& $(__anon1)
    get_identical_ref_with_inferred_lifetime(::std::int32_t const* $(
        __anon1) crubit_nonnull x CRUBIT_LIFETIME_BOUND) {
  return __crubit_internal::
      __crubit_thunk_get_uidentical_uref_uwith_uinferred_ulifetime(x);
}

namespace __crubit_internal {
extern "C" ::std::int32_t const& $a __crubit_thunk_get_uref_uto_usmaller_uint(
    ::std::int32_t const* $a crubit_nonnull,
    ::std::int32_t const* $a crubit_nonnull);
}
inline ::std::int32_t const& $a
get_ref_to_smaller_int(::std::int32_t const* $a crubit_nonnull x,
                       ::std::int32_t const* $a crubit_nonnull y) {
  return __crubit_internal::__crubit_thunk_get_uref_uto_usmaller_uint(x, y);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_set_umut_uref_uto_usum_uof_uints(::std::int32_t&,
                                                                ::std::int32_t,
                                                                ::std::int32_t);
}
inline void set_mut_ref_to_sum_of_ints(::std::int32_t& sum, ::std::int32_t x,
                                       ::std::int32_t y) {
  return __crubit_internal::__crubit_thunk_set_umut_uref_uto_usum_uof_uints(
      sum, x, y);
}

}  // namespace functions::fn_param_ty_tests

namespace functions::generic_fn_tests::as_mut_trait_tests {

namespace __crubit_internal {
extern "C" void __crubit_thunk_prefix_usums(rs_std::SliceRef<::std::int32_t>);
}
inline void prefix_sums(rs_std::SliceRef<::std::int32_t> arg) {
  return __crubit_internal::__crubit_thunk_prefix_usums(arg);
}

}  // namespace functions::generic_fn_tests::as_mut_trait_tests

namespace functions::generic_fn_tests::as_ref_trait_tests {

static_assert(
    sizeof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyStruct>);
static_assert(::std::is_trivially_move_constructible_v<
              ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct>);
static_assert(::std::is_trivially_move_assignable_v<
              ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct>);
namespace __crubit_internal {
extern "C" ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct
__crubit_thunk_new(::std::int32_t);
}
inline ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct
MyStruct::new_(::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_new(x);
}
inline void MyStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyStruct, __field0));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_diverse_ulifetimes(
    rs_std::SliceRef<const ::std::int32_t>,
    rs_std::SliceRef<const ::std::int32_t>,
    rs_std::SliceRef<const ::std::int32_t>, rs_std::SliceRef<::std::int32_t>);
}
inline void diverse_lifetimes(rs_std::SliceRef<const ::std::int32_t> arg1,
                              rs_std::SliceRef<const ::std::int32_t> arg2,
                              rs_std::SliceRef<const ::std::int32_t> arg3,
                              rs_std::SliceRef<::std::int32_t> result) {
  crubit::internal::CheckNoMutableAliasing(
      crubit::internal::AsMutPtrDatas<rs_std::SliceRef<::std::int32_t>>(result),
      crubit::internal::AsPtrDatas<rs_std::SliceRef<const ::std::int32_t>,
                                   rs_std::SliceRef<const ::std::int32_t>,
                                   rs_std::SliceRef<const ::std::int32_t>>(
          arg1, arg2, arg3));
  return __crubit_internal::__crubit_thunk_diverse_ulifetimes(arg1, arg2, arg3,
                                                              result);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_slice_uref_usum(
    rs_std::SliceRef<const ::std::int32_t>);
}
inline ::std::int32_t slice_ref_sum(
    rs_std::SliceRef<const ::std::int32_t> arg) {
  return __crubit_internal::__crubit_thunk_slice_uref_usum(arg);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_static_ulifetime_urequirement(
    rs_std::SliceRef<const ::std::int32_t>);
}
inline ::std::int32_t static_lifetime_requirement(
    rs_std::SliceRef<const ::std::int32_t> arg) {
  return __crubit_internal::__crubit_thunk_static_ulifetime_urequirement(arg);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_struct_uref(
    ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct const&);
}
inline ::std::int32_t struct_ref(
    ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct const& arg) {
  return __crubit_internal::__crubit_thunk_struct_uref(arg);
}

}  // namespace functions::generic_fn_tests::as_ref_trait_tests

namespace functions::generic_fn_tests::into_trait_tests {

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_basic_utest(::std::int32_t);
}
inline ::std::int32_t basic_test(::std::int32_t arg) {
  return __crubit_internal::__crubit_thunk_basic_utest(arg);
}

namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_generic_uparam_unested_udeeper_uin_uparam_uty(void*);
}
inline ::std::int32_t generic_param_nested_deeper_in_param_ty(
    ::std::array<::std::int32_t, 3> xs) {
  return __crubit_internal::
      __crubit_thunk_generic_uparam_unested_udeeper_uin_uparam_uty(&xs);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_multiple_ugeneric_uparams(
    ::std::int32_t, ::std::int32_t);
}
inline ::std::int32_t multiple_generic_params(::std::int32_t x,
                                              ::std::int32_t y) {
  return __crubit_internal::__crubit_thunk_multiple_ugeneric_uparams(x, y);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_return_utype();
}
inline ::std::int32_t return_type() {
  return __crubit_internal::__crubit_thunk_return_utype();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_reused_ugeneric_uparam(::std::int32_t,
                                                                ::std::int32_t);
}
inline ::std::int32_t reused_generic_param(::std::int32_t x, ::std::int32_t y) {
  return __crubit_internal::__crubit_thunk_reused_ugeneric_uparam(x, y);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_where_uclause(::std::int32_t);
}
inline ::std::int32_t where_clause(::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_where_uclause(x);
}

}  // namespace functions::generic_fn_tests::into_trait_tests

namespace functions::other_fn_param_tests {

namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_add_ui32_uvia_urust_uabi_uwith_uduplicated_uparam_unames(
    ::std::int32_t, ::std::int32_t, ::std::int32_t, ::std::int32_t);
}
inline ::std::int32_t add_i32_via_rust_abi_with_duplicated_param_names(
    ::std::int32_t x, ::std::int32_t y, ::std::int32_t __param_2,
    ::std::int32_t __param_3) {
  return __crubit_internal::
      __crubit_thunk_add_ui32_uvia_urust_uabi_uwith_uduplicated_uparam_unames(
          x, y, __param_2, __param_3);
}

}  // namespace functions::other_fn_param_tests

namespace functions::unit_ret_ty_tests {

namespace __crubit_internal {
extern "C" void custom_export_name_for_get_global_i32(::std::int32_t);
}
inline void set_global_i32_via_extern_c_with_export_name(::std::int32_t x) {
  return __crubit_internal::custom_export_name_for_get_global_i32(x);
}

}  // namespace functions::unit_ret_ty_tests

namespace functions::unsafe_fn_tests {

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_unsafe_uadd(::std::int32_t,
                                                     ::std::int32_t);
}
inline ::std::int32_t unsafe_add(::std::int32_t x, ::std::int32_t y) {
  return __crubit_internal::__crubit_thunk_unsafe_uadd(x, y);
}

}  // namespace functions::unsafe_fn_tests

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_FUNCTIONS_FUNCTIONS_GOLDEN
