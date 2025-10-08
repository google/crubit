// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// arrays_golden
// Features: supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ARRAYS_ARRAYS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ARRAYS_ARRAYS_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace arrays {

// Generated from:
// cc_bindings_from_rs/test/arrays/arrays.rs;l=8
std::array<std::int32_t, 2> const* function_with_const_array_ptr_id(
    std::array<std::int32_t, 2> const* array_ptr);

// Error generating bindings for `function_with_array_id` defined at
// cc_bindings_from_rs/test/arrays/arrays.rs;l=12:
// Unknown type

// Error generating bindings for `function_with_array_tuple_id` defined at
// cc_bindings_from_rs/test/arrays/arrays.rs;l=16:
// Attempted to write out unknown type from Rust to C

// Generated from:
// cc_bindings_from_rs/test/arrays/arrays.rs;l=21
struct CRUBIT_INTERNAL_RUST_TYPE(":: arrays_golden :: ArrayStruct") alignas(4)
    [[clang::trivial_abi]] ArrayStruct final {
 public:
  // Default::default
  ArrayStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~ArrayStruct() = default;
  ArrayStruct(ArrayStruct&&) = default;
  ArrayStruct& operator=(ArrayStruct&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  ArrayStruct(const ArrayStruct&) = default;
  ArrayStruct& operator=(const ArrayStruct&) = default;
  ArrayStruct(::crubit::UnsafeRelocateTag, ArrayStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/arrays/arrays.rs;l=22
    std::array<std::int32_t, 2> array;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/arrays/arrays.rs;l=25
::arrays::ArrayStruct function_with_array_struct_id(
    ::arrays::ArrayStruct array_struct);

namespace __crubit_internal {
extern "C" std::array<std::int32_t, 2> const*
__crubit_thunk_function_uwith_uconst_uarray_uptr_uid(
    std::array<std::int32_t, 2> const*);
}
inline std::array<std::int32_t, 2> const* function_with_const_array_ptr_id(
    std::array<std::int32_t, 2> const* array_ptr) {
  return __crubit_internal::
      __crubit_thunk_function_uwith_uconst_uarray_uptr_uid(array_ptr);
}

static_assert(
    sizeof(ArrayStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ArrayStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::arrays::ArrayStruct* __ret_ptr);
}
inline ArrayStruct::ArrayStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<ArrayStruct>);
static_assert(std::is_trivially_move_constructible_v<ArrayStruct>);
static_assert(std::is_trivially_move_assignable_v<ArrayStruct>);
static_assert(std::is_trivially_copy_constructible_v<ArrayStruct>);
static_assert(std::is_trivially_copy_assignable_v<ArrayStruct>);
inline void ArrayStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ArrayStruct, array));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_function_uwith_uarray_ustruct_uid(
    ::arrays::ArrayStruct*, ::arrays::ArrayStruct* __ret_ptr);
}
inline ::arrays::ArrayStruct function_with_array_struct_id(
    ::arrays::ArrayStruct array_struct) {
  crubit::Slot<::arrays::ArrayStruct> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_function_uwith_uarray_ustruct_uid(
      &array_struct, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace arrays
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ARRAYS_ARRAYS_GOLDEN
