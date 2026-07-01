// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// async_fn_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ASYNC_FN_ASYNC_FN_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ASYNC_FN_ASYNC_FN_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/rs_std/dyn_erased_future.h"
#include "support/rs_std/run_crubit_future.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <utility>

#include "cc_bindings_from_rs/test/async_fn/async_fn_existing_cpp_types.h"

namespace async_fn {

// Error generating bindings for struct
// `async_fn_golden::AsyncFnRustConvertible` defined at
// cc_bindings_from_rs/test/async_fn/async_fn.rs;l=54:
// Type bindings for async_fn_golden::AsyncFnRustConvertible suppressed due to
// being mapped to an existing C++ type (crubit::test::AsyncFnCppConvertible)

// Error generating bindings for struct
// `async_fn_golden::AsyncFnRustLayoutEquivalent` defined at
// cc_bindings_from_rs/test/async_fn/async_fn.rs;l=96:
// Type bindings for async_fn_golden::AsyncFnRustLayoutEquivalent suppressed due
// to being mapped to an existing C++ type
// (crubit::test::AsyncFnCppLayoutEquivalent)

struct CRUBIT_INTERNAL_RUST_TYPE(":: async_fn_golden :: NotCppMovable") alignas(
    4) [[clang::trivial_abi]] NotCppMovable final {
 public:
  // `async_fn_golden::NotCppMovable` doesn't implement the `Default` trait
  NotCppMovable() = delete;

  // Synthesized tuple constructor
  explicit NotCppMovable(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // Drop::drop
  ~NotCppMovable();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  NotCppMovable(NotCppMovable&&) = delete;
  ::async_fn::NotCppMovable& operator=(NotCppMovable&&) = delete;
  // `async_fn_golden::NotCppMovable` doesn't implement the `Clone` trait
  NotCppMovable(const NotCppMovable&) = delete;
  NotCppMovable& operator=(const NotCppMovable&) = delete;
  NotCppMovable(::crubit::UnsafeRelocateTag, NotCppMovable&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: async_fn_golden :: StructWithDrop") alignas(4) [[clang::trivial_abi]]
StructWithDrop final {
 public:
  // Default::default
  StructWithDrop();

  // Drop::drop
  ~StructWithDrop();

  StructWithDrop(StructWithDrop&&);
  ::async_fn::StructWithDrop& operator=(StructWithDrop&&);

  // `async_fn_golden::StructWithDrop` doesn't implement the `Clone` trait
  StructWithDrop(const StructWithDrop&) = delete;
  StructWithDrop& operator=(const StructWithDrop&) = delete;
  StructWithDrop(::crubit::UnsafeRelocateTag, StructWithDrop&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

::crubit::DynErasedFuture<::std::int32_t> add(::std::int32_t x,
                                              ::std::int32_t y);

//  # Safety
//
//  `cpp_in` must be a valid pointer to a `AsyncFnCppConvertible` that will not
//  be used again. `rs_out` must be a valid pointer to uninitialized memory
//  suitable for writing a `AsyncFnRustConvertible`.
extern "C" void convert_cpp_to_rust_async_fn(const void* _cpp_in,
                                             void* _rs_out);

//  # Safety
//
//  `rs_in` must be a valid pointer to an `AsyncFnRustConvertible` that will not
//  be used again. `cpp_out` must be a valid pointer to uninitialized memory
//  suitable for writing a `AsyncFnCppConvertible`.
extern "C" void convert_rust_to_cpp_async_fn(const void* _rs_in,
                                             void* _cpp_out);

::crubit::DynErasedFuture<void> do_nothing();

// Error generating bindings for function `async_fn_golden::non_send_return`
// defined at
// cc_bindings_from_rs/test/async_fn/async_fn.rs;l=130:
// Crubit currently only supports async functions that return a Send future.

::crubit::DynErasedFuture<::std::int32_t> pend_5_times();

// Error generating bindings for function
// `async_fn_golden::return_box_dyn_future` defined at
// cc_bindings_from_rs/test/async_fn/async_fn.rs;l=125:
// Error formatting function return type `std::boxed::Box<(dyn
// std::future::Future<Output = i32> + 'static)>`: Generic types are not
// supported yet (b/259749095)

// Error generating bindings for function
// `async_fn_golden::return_bridged_convertible` defined at
// cc_bindings_from_rs/test/async_fn/async_fn.rs;l=58:
// Crubit currently does not support async functions returning bridged types
// that require conversion thunks, found
// `async_fn_golden::AsyncFnRustConvertible`.

::crubit::DynErasedFuture<crubit::test::AsyncFnCppLayoutEquivalent>
return_cpp_layout_equivalent(::std::int32_t x);

// Error generating bindings for function `async_fn_golden::return_impl_future`
// defined at
// cc_bindings_from_rs/test/async_fn/async_fn.rs;l=120:
// Error formatting function return type `impl std::future::Future<Output =
// i32>`: Generic types are not supported yet (b/259749095)

::crubit::DynErasedFuture<::async_fn::StructWithDrop> return_struct_with_drop(
    ::std::int32_t x);

// Error generating bindings for function `async_fn_golden::return_unmovable`
// defined at
// cc_bindings_from_rs/test/async_fn/async_fn.rs;l=115:
// Can't pass a type by value without a move constructor. See
// crubit.rs/rust/movable_types for what types are C++ movable.

static_assert(
    sizeof(NotCppMovable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NotCppMovable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_uasync_ufn_ugolden_x0000003a_x0000003aNotCppMovable(
    ::async_fn::NotCppMovable&);
}
inline NotCppMovable::~NotCppMovable() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uasync_ufn_ugolden_x0000003a_x0000003aNotCppMovable(
          *this);
}
inline void NotCppMovable::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NotCppMovable, __field0));
}
static_assert(
    sizeof(StructWithDrop) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithDrop) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uasync_ufn_ugolden_x0000003a_x0000003aStructWithDrop(
    ::async_fn::StructWithDrop* __ret_ptr);
}
inline ::async_fn::StructWithDrop::StructWithDrop() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uasync_ufn_ugolden_x0000003a_x0000003aStructWithDrop(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_uasync_ufn_ugolden_x0000003a_x0000003aStructWithDrop(
    ::async_fn::StructWithDrop&);
}
inline StructWithDrop::~StructWithDrop() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uasync_ufn_ugolden_x0000003a_x0000003aStructWithDrop(
          *this);
}
inline ::async_fn::StructWithDrop::StructWithDrop(StructWithDrop&& other)
    : StructWithDrop() {
  *this = ::std::move(other);
}
inline ::async_fn::StructWithDrop& ::async_fn::StructWithDrop::operator=(
    StructWithDrop&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline void StructWithDrop::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithDrop, field));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_add(
    ::std::int32_t, ::std::int32_t,
    ::crubit::DynErasedFuture<::std::int32_t>* __ret_ptr);
}
inline ::crubit::DynErasedFuture<::std::int32_t> add(::std::int32_t x,
                                                     ::std::int32_t y) {
  ::crubit::Slot<::crubit::DynErasedFuture<::std::int32_t>>
      __return_value_ret_val_holder;
  __crubit_internal::__crubit_thunk_add(x, y,
                                        __return_value_ret_val_holder.Get());
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_do_unothing(
    ::crubit::DynErasedFuture<void>* __ret_ptr);
}
inline ::crubit::DynErasedFuture<void> do_nothing() {
  ::crubit::Slot<::crubit::DynErasedFuture<void>> __return_value_ret_val_holder;
  __crubit_internal::__crubit_thunk_do_unothing(
      __return_value_ret_val_holder.Get());
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_pend_u5_utimes(
    ::crubit::DynErasedFuture<::std::int32_t>* __ret_ptr);
}
inline ::crubit::DynErasedFuture<::std::int32_t> pend_5_times() {
  ::crubit::Slot<::crubit::DynErasedFuture<::std::int32_t>>
      __return_value_ret_val_holder;
  __crubit_internal::__crubit_thunk_pend_u5_utimes(
      __return_value_ret_val_holder.Get());
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_ucpp_ulayout_uequivalent(
    ::std::int32_t,
    ::crubit::DynErasedFuture<crubit::test::AsyncFnCppLayoutEquivalent>*
        __ret_ptr);
}
inline ::crubit::DynErasedFuture<crubit::test::AsyncFnCppLayoutEquivalent>
return_cpp_layout_equivalent(::std::int32_t x) {
  ::crubit::Slot<
      ::crubit::DynErasedFuture<crubit::test::AsyncFnCppLayoutEquivalent>>
      __return_value_ret_val_holder;
  __crubit_internal::__crubit_thunk_return_ucpp_ulayout_uequivalent(
      x, __return_value_ret_val_holder.Get());
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_ustruct_uwith_udrop(
    ::std::int32_t,
    ::crubit::DynErasedFuture<::async_fn::StructWithDrop>* __ret_ptr);
}
inline ::crubit::DynErasedFuture<::async_fn::StructWithDrop>
return_struct_with_drop(::std::int32_t x) {
  ::crubit::Slot<::crubit::DynErasedFuture<::async_fn::StructWithDrop>>
      __return_value_ret_val_holder;
  __crubit_internal::__crubit_thunk_return_ustruct_uwith_udrop(
      x, __return_value_ret_val_holder.Get());
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace async_fn

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ASYNC_FN_ASYNC_FN_GOLDEN
