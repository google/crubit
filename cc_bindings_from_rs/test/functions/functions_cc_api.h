// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// functions_golden

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
#include "support/internal/offsetof.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/char.h"
#include "support/rs_std/slice_ref.h"
#include "support/rs_std/traits.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

#include "support/ctor.h"
#include "support/rs_std/rs_core.h"

namespace functions::fn_abi_tests {

//  Testing `#[unsafe(export_name = ...)]` - the generated bindings need to
//  forward/proxy the call into a function with a different name.
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
::std::int32_t add_i32_via_extern_c_with_mangling(::std::int32_t x,
                                                  ::std::int32_t y);

//  Testing the default / Rust ABI (one used in absence of `extern "C"`).
::std::int32_t add_i32_via_rust_abi(::std::int32_t x, ::std::int32_t y);

//  Testing one of simpler function bindings:
//  - `extern "C"` means that no thunk is required
//  - `#[unsafe(no_mangle)]` means that the function is already exposed with
//    the desired, public name (and just needs to be redeclared in C++).
extern "C" double get_42_as_f64_via_no_mangle_extern_c();

}  // namespace functions::fn_abi_tests

namespace functions::fn_attribute_tests {

[[deprecated("★ Deprecated note for add_i32 ★")]] ::std::int32_t add_i32(
    ::std::int32_t x, ::std::int32_t y);

}

namespace functions::fn_must_use_tests {

[[nodiscard("woohoo")]] ::std::int32_t msg_add(::std::int32_t x,
                                               ::std::int32_t y);

[[nodiscard]] ::std::int32_t no_msg_add(::std::int32_t x, ::std::int32_t y);

}  // namespace functions::fn_must_use_tests

namespace functions::fn_param_ty_tests {

//  Testing a type that maps to a built-in C++ type (spelled with a
//  keyword). `float` is one such example.
double add_f64(double x, double y);

//  Testing a type that requires `#include`ing a standard C++ header.
//  `std::int32_t` is one such example - it requires `#include <cstdint>`.
::std::int32_t add_i32(::std::int32_t x, ::std::int32_t y);

void add_i32_via_ptr(::std::int32_t const* x, ::std::int32_t const* y,
                     ::std::int32_t* sum);

::std::int32_t apply_binary_i32_op(
    ::std::int32_t x, ::std::int32_t y,
    crubit::type_identity_t<::std::int32_t(::std::int32_t, ::std::int32_t)>& f);

rs_std::char_ char_to_ascii_lowercase(rs_std::char_ c);

::std::int32_t const& $(__anon1) get_identical_ref_with_inferred_lifetime(
    ::std::int32_t const* $(__anon1) crubit_nonnull x CRUBIT_LIFETIME_BOUND);

::std::int32_t const& $a
get_ref_to_smaller_int(::std::int32_t const* $a crubit_nonnull x,
                       ::std::int32_t const* $a crubit_nonnull y);

void set_mut_ref_to_sum_of_ints(::std::int32_t& sum, ::std::int32_t x,
                                ::std::int32_t y);

}  // namespace functions::fn_param_ty_tests

namespace functions::generic_fn_tests::as_mut_trait_tests {

void prefix_sums(rs_std::SliceRef<::std::int32_t> arg);

}

namespace functions::generic_fn_tests::as_ref_trait_tests {

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

  static ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct new_(
      ::std::int32_t x);

 private:
  union {
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
void diverse_lifetimes(rs_std::SliceRef<const ::std::int32_t> arg1,
                       rs_std::SliceRef<const ::std::int32_t> arg2,
                       rs_std::SliceRef<const ::std::int32_t> arg3,
                       rs_std::SliceRef<::std::int32_t> result);

::std::int32_t slice_ref_sum(rs_std::SliceRef<const ::std::int32_t> arg);

//  This is an attempt to trigger an error seen in
//  https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=42303eaaafe4a3538ad259e9e9b67f05
//
//  Today the error doesn't happen in Crubit, because the thunks explicitly
//  declare all their lifetimes as `'static` - see `fn
//  replace_all_regions_with_static`.
::std::int32_t static_lifetime_requirement(
    rs_std::SliceRef<const ::std::int32_t> arg);

::std::int32_t struct_ref(
    ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct const& arg);

}  // namespace functions::generic_fn_tests::as_ref_trait_tests

namespace functions::generic_fn_tests::ctor_trait_tests {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: functions_golden :: generic_fn_tests :: ctor_trait_tests :: "
    "Movable") alignas(4) [[clang::trivial_abi]] Movable final {
 public:
  // `functions_golden::generic_fn_tests::ctor_trait_tests::Movable` doesn't
  // implement the `Default` trait
  Movable() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Movable() = default;
  Movable(Movable&&) = default;
  Movable& operator=(Movable&&) = default;

  // `functions_golden::generic_fn_tests::ctor_trait_tests::Movable` doesn't
  // implement the `Clone` trait
  Movable(const Movable&) = delete;
  Movable& operator=(const Movable&) = delete;
  Movable(::crubit::UnsafeRelocateTag, Movable&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: functions_golden :: generic_fn_tests :: ctor_trait_tests :: "
    "NonMovable") alignas(4) [[clang::trivial_abi]] NonMovable final {
 public:
  // `functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable` doesn't
  // implement the `Default` trait
  NonMovable() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NonMovable() = default;
  NonMovable(NonMovable&&) = default;
  NonMovable& operator=(NonMovable&&) = default;

  // `functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable` doesn't
  // implement the `Clone` trait
  NonMovable(const NonMovable&) = delete;
  NonMovable& operator=(const NonMovable&) = delete;
  NonMovable(::crubit::UnsafeRelocateTag, NonMovable&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  static ::functions::generic_fn_tests::ctor_trait_tests::NonMovable new_(
      ::std::int32_t value);

  union {
    ::std::int32_t value;
  };
  // Skipped bindings for field `_pinned`: ZST fields are not supported
  // (b/258259459)
 private:
  static void __crubit_field_offset_assertions();
};

::std::int32_t accept_ctor(
    ::functions::generic_fn_tests::ctor_trait_tests::NonMovable&& _c);

// Error generating bindings for function
// `functions_golden::generic_fn_tests::ctor_trait_tests::accept_ctor_movable`
// defined at
// cc_bindings_from_rs/test/functions/functions.rs;l=313:
// No valid non-generic replacement for generic type param `impl Ctor<Output =
// Movable>`

}  // namespace functions::generic_fn_tests::ctor_trait_tests

namespace functions::generic_fn_tests::into_trait_tests {

::std::int32_t basic_test(::std::int32_t arg);

//  This test was initially added to cover/verify the call to
//  `super_visit_with` from an `impl` of `GenericParamsFinder` in
//  `get_generic_args.rs`.
::std::int32_t generic_param_nested_deeper_in_param_ty(
    ::std::array<::std::int32_t, 3> xs);

::std::int32_t multiple_generic_params(::std::int32_t x, ::std::int32_t y);

::std::int32_t return_type();

::std::int32_t reused_generic_param(::std::int32_t x, ::std::int32_t y);

// Error generating bindings for function
// `functions_golden::generic_fn_tests::into_trait_tests::unused_generic_param`
// defined at
// cc_bindings_from_rs/test/functions/functions.rs;l=204:
// No support for replacing an _unused_ generic type param: `T`

::std::int32_t where_clause(::std::int32_t x);

}  // namespace functions::generic_fn_tests::into_trait_tests

namespace functions::other_fn_param_tests {

::std::int32_t add_i32_via_rust_abi_with_duplicated_param_names(
    ::std::int32_t x, ::std::int32_t y, ::std::int32_t __param_2,
    ::std::int32_t __param_3);

}

namespace functions::unit_ret_ty_tests {

extern "C" ::std::int32_t get_global_i32_via_extern_c_with_export_name();

void set_global_i32_via_extern_c_with_export_name(::std::int32_t x);

}  // namespace functions::unit_ret_ty_tests

namespace functions::unsafe_fn_tests {

//  # Safety
//
//  This function has no safety requirements - it is only marked as `unsafe`
//  to facilitate minimal testing of bindings generated for such functions.
::std::int32_t unsafe_add(::std::int32_t x, ::std::int32_t y);

}  // namespace functions::unsafe_fn_tests

template <>
struct rs_std::impl<
    ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct,
    ::rs::core::convert::AsRef<
        ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct>> {
  static constexpr bool kIsImplemented = true;

  static ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct const& $(
      __anon1)
      as_ref(::functions::generic_fn_tests::as_ref_trait_tests::MyStruct const&
                 self);
};

template <>
struct rs_std::impl<
    ::functions::generic_fn_tests::ctor_trait_tests::NonMovable,
    ::ctor::CtorNew<
        ::functions::generic_fn_tests::ctor_trait_tests::NonMovable&&>> {
  static constexpr bool kIsImplemented = true;
  using CtorType CRUBIT_INTERNAL_RUST_TYPE(
      "<functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable as :: "
      "ctor :: CtorNew>::CtorType") =
      ::functions::generic_fn_tests::ctor_trait_tests::NonMovable&&;
  using Error CRUBIT_INTERNAL_RUST_TYPE(
      "<functions_golden::generic_fn_tests::ctor_trait_tests::NonMovable as :: "
      "ctor :: CtorNew>::Error") = ::std::int32_t;

  static ::functions::generic_fn_tests::ctor_trait_tests::NonMovable&& ctor_new(
      ::functions::generic_fn_tests::ctor_trait_tests::NonMovable&& args);
};

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
  CRUBIT_WARNING_PUSH("-Wno-invalid-offsetof")
  static_assert(0 == offsetof(MyStruct, __field0));
  CRUBIT_WARNING_POP
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

namespace functions::generic_fn_tests::ctor_trait_tests {

static_assert(
    sizeof(Movable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Movable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Movable>);
static_assert(::std::is_trivially_move_constructible_v<
              ::functions::generic_fn_tests::ctor_trait_tests::Movable>);
static_assert(::std::is_trivially_move_assignable_v<
              ::functions::generic_fn_tests::ctor_trait_tests::Movable>);
inline void Movable::__crubit_field_offset_assertions() {
  CRUBIT_WARNING_PUSH("-Wno-invalid-offsetof")
  static_assert(0 == offsetof(Movable, value));
  CRUBIT_WARNING_POP
}
static_assert(
    sizeof(NonMovable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonMovable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NonMovable>);
static_assert(::std::is_trivially_move_constructible_v<
              ::functions::generic_fn_tests::ctor_trait_tests::NonMovable>);
static_assert(::std::is_trivially_move_assignable_v<
              ::functions::generic_fn_tests::ctor_trait_tests::NonMovable>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(
    ::std::int32_t,
    ::functions::generic_fn_tests::ctor_trait_tests::NonMovable* __ret_ptr);
}
inline ::functions::generic_fn_tests::ctor_trait_tests::NonMovable
NonMovable::new_(::std::int32_t value) {
  crubit::Slot<::functions::generic_fn_tests::ctor_trait_tests::NonMovable>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void NonMovable::__crubit_field_offset_assertions() {
  CRUBIT_WARNING_PUSH("-Wno-invalid-offsetof")
  static_assert(0 == offsetof(NonMovable, value));
  CRUBIT_WARNING_POP
}
namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_accept_uctor(
    ::functions::generic_fn_tests::ctor_trait_tests::NonMovable&&);
}
inline ::std::int32_t accept_ctor(
    ::functions::generic_fn_tests::ctor_trait_tests::NonMovable&& _c) {
  return __crubit_internal::__crubit_thunk_accept_uctor(::std::move(_c));
}

}  // namespace functions::generic_fn_tests::ctor_trait_tests

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

namespace functions {
namespace __crubit_internal {
extern "C" ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct const& $(
    __anon1)
    __crubit_thunk_AsRef_uas_uref_ufunctions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003aas_uref_utrait_utests_x0000003a_x0000003aMyStruct_ufunctions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003aas_uref_utrait_utests_x0000003a_x0000003aMyStruct(
        ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct const&);
}
}  // namespace functions
inline ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct const&
$(__anon1) rs_std::impl<
    ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct,
    ::rs::core::convert::AsRef<
        ::functions::generic_fn_tests::as_ref_trait_tests::MyStruct>>::
    as_ref(::functions::generic_fn_tests::as_ref_trait_tests::MyStruct const&
               self) {
  return functions::__crubit_internal::
      __crubit_thunk_AsRef_uas_uref_ufunctions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003aas_uref_utrait_utests_x0000003a_x0000003aMyStruct_ufunctions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003aas_uref_utrait_utests_x0000003a_x0000003aMyStruct(
          self);
}

namespace functions {
namespace __crubit_internal {
extern "C" ::functions::generic_fn_tests::ctor_trait_tests::NonMovable&&
__crubit_thunk_CtorNew_uctor_unew_ufunctions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003actor_utrait_utests_x0000003a_x0000003aNonMovable_uctor_x0000003a_x0000003aRvalueReference_x0000003c_x00000027a_x0000002c_x00000020functions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003actor_utrait_utests_x0000003a_x0000003aNonMovable_x0000003e(
    ::functions::generic_fn_tests::ctor_trait_tests::NonMovable&&);
}
}  // namespace functions
inline ::functions::generic_fn_tests::ctor_trait_tests::NonMovable&&
rs_std::impl<
    ::functions::generic_fn_tests::ctor_trait_tests::NonMovable,
    ::ctor::CtorNew<
        ::functions::generic_fn_tests::ctor_trait_tests::NonMovable&&>>::
    ctor_new(
        ::functions::generic_fn_tests::ctor_trait_tests::NonMovable&& args) {
  return functions::__crubit_internal::
      __crubit_thunk_CtorNew_uctor_unew_ufunctions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003actor_utrait_utests_x0000003a_x0000003aNonMovable_uctor_x0000003a_x0000003aRvalueReference_x0000003c_x00000027a_x0000002c_x00000020functions_ugolden_x0000003a_x0000003ageneric_ufn_utests_x0000003a_x0000003actor_utrait_utests_x0000003a_x0000003aNonMovable_x0000003e(
          ::std::move(args));
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_FUNCTIONS_FUNCTIONS_GOLDEN
