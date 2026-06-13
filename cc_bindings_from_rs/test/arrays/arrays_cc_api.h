// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// arrays_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ARRAYS_ARRAYS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ARRAYS_ARRAYS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/rs_std/tuple.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <tuple>
#include <type_traits>
#include <utility>

namespace arrays {

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
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::array<::std::int32_t, 2> array;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: arrays_golden :: HasDrop") alignas(4)
    [[clang::trivial_abi]] HasDrop final {
 public:
  // `arrays_golden::HasDrop` doesn't implement the `Default` trait
  HasDrop() = delete;

  // Drop::drop
  ~HasDrop();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  HasDrop(HasDrop&&) = delete;
  ::arrays::HasDrop& operator=(HasDrop&&) = delete;
  // `arrays_golden::HasDrop` doesn't implement the `Clone` trait
  HasDrop(const HasDrop&) = delete;
  HasDrop& operator=(const HasDrop&) = delete;
  HasDrop(::crubit::UnsafeRelocateTag, HasDrop&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  static ::arrays::HasDrop new_(::std::int32_t x);

  union {
    ::std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: arrays_golden :: HasDropAndDefault") alignas(4) [[clang::trivial_abi]]
HasDropAndDefault final {
 public:
  // Default::default
  HasDropAndDefault();

  // Drop::drop
  ~HasDropAndDefault();

  HasDropAndDefault(HasDropAndDefault&&);
  ::arrays::HasDropAndDefault& operator=(HasDropAndDefault&&);

  // `arrays_golden::HasDropAndDefault` doesn't implement the `Clone` trait
  HasDropAndDefault(const HasDropAndDefault&) = delete;
  HasDropAndDefault& operator=(const HasDropAndDefault&) = delete;
  HasDropAndDefault(::crubit::UnsafeRelocateTag, HasDropAndDefault&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
::std::array<::std::int32_t, 2> function_with_array_id(
    ::std::array<::std::int32_t, 2> array);

// CRUBIT_ANNOTATE: must_bind=
::arrays::ArrayStruct function_with_array_struct_id(
    ::arrays::ArrayStruct array_struct);

// CRUBIT_ANNOTATE: must_bind=
::std::tuple<::std::array<::std::int32_t, 2>, ::std::array<::std::int32_t, 2>>
function_with_array_tuple_id(::std::tuple<::std::array<::std::int32_t, 2>,
                                          ::std::array<::std::int32_t, 2>>
                                 array_tup);

// CRUBIT_ANNOTATE: must_bind=
::std::array<::std::int32_t, 2> const* function_with_const_array_ptr_id(
    ::std::array<::std::int32_t, 2> const* array_ptr);

// CRUBIT_ANNOTATE: must_bind=
::std::array<::std::int32_t, 0> function_with_empty_array(
    ::std::array<::std::int32_t, 0> array);

// CRUBIT_ANNOTATE: must_bind=
::std::array<::arrays::HasDropAndDefault, 2>
function_with_has_drop_and_default_array_id(
    ::std::array<::arrays::HasDropAndDefault, 2> array);

// CRUBIT_ANNOTATE: must_bind=
::std::array<::arrays::HasDrop, 2> function_with_has_drop_array_id(
    ::std::array<::arrays::HasDrop, 2> array);

// CRUBIT_ANNOTATE: must_bind=
::std::array<::arrays::HasDrop, 2> function_with_has_drop_ret_only();

// CRUBIT_ANNOTATE: must_bind=
::std::array<::std::int32_t, 3> const*
function_with_mut_array_named_size_ptr_id(
    ::std::array<::std::int32_t, 3> const* array_ptr);

// CRUBIT_ANNOTATE: must_bind=
::std::array<::std::array<::std::int32_t, 2>, 2> function_with_nested_arrays(
    ::std::array<::std::array<::std::int32_t, 2>, 2> array);

// CRUBIT_ANNOTATE: must_bind=
::std::array<::std::array<::arrays::HasDropAndDefault, 2>, 2>
function_with_nested_drop_default_arrays(
    ::std::array<::std::array<::arrays::HasDropAndDefault, 2>, 2> array);

// Error generating bindings for function
// `arrays_golden::function_with_nested_droponly_arrays` defined at
// cc_bindings_from_rs/test/arrays/arrays.rs;l=98:
// b/260128806 - nested array [arrays_golden::HasDrop; 2] is not supported
// because it contains a type that implements Drop but not Default

// Error generating bindings for function
// `arrays_golden::function_with_nested_nested_droponly_arrays` defined at
// cc_bindings_from_rs/test/arrays/arrays.rs;l=104:
// b/260128806 - nested array [[arrays_golden::HasDrop; 2]; 2] is not supported
// because it contains a type that implements Drop but not Default

::std::array<rs::Tuple<::std::int32_t, ::std::int32_t>, 2>
function_with_tuple_array_id(
    ::std::array<rs::Tuple<::std::int32_t, ::std::int32_t>, 2> tup_array);

}  // namespace arrays

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "(i32 , i32 ,)") rs::Tuple<::std::int32_t, ::std::int32_t> {
 public:
  // Default::default
  Tuple();

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Tuple(const Tuple&) = default;
  Tuple& operator=(const Tuple&) = default;
  Tuple(Tuple&&) = default;
  Tuple& operator=(Tuple&&) = default;

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  Tuple(::std::tuple<::std::int32_t, ::std::int32_t>&& tuple) noexcept;
  ~Tuple() = default;
  operator ::std::tuple<::std::int32_t, ::std::int32_t>() && noexcept;

 private:
  unsigned char storage_[8];
};
#endif

namespace arrays {

static_assert(
    sizeof(ArrayStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ArrayStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::arrays::ArrayStruct* __ret_ptr);
}
inline ::arrays::ArrayStruct::ArrayStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_destructible_v<ArrayStruct>);
static_assert(::std::is_trivially_move_constructible_v<::arrays::ArrayStruct>);
static_assert(::std::is_trivially_move_assignable_v<::arrays::ArrayStruct>);
static_assert(::std::is_trivially_copy_constructible_v<::arrays::ArrayStruct>);
static_assert(::std::is_trivially_copy_assignable_v<::arrays::ArrayStruct>);
inline void ArrayStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ArrayStruct, array));
}
static_assert(
    sizeof(HasDrop) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasDrop) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::arrays::HasDrop&);
}
inline HasDrop::~HasDrop() { __crubit_internal::__crubit_thunk_drop(*this); }
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t,
                                   ::arrays::HasDrop* __ret_ptr);
}
inline ::arrays::HasDrop HasDrop::new_(::std::int32_t x) {
  crubit::Slot<::arrays::HasDrop> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void HasDrop::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasDrop, x));
}
static_assert(
    sizeof(HasDropAndDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(HasDropAndDefault) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::arrays::HasDropAndDefault* __ret_ptr);
}
inline ::arrays::HasDropAndDefault::HasDropAndDefault() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::arrays::HasDropAndDefault&);
}
inline HasDropAndDefault::~HasDropAndDefault() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline ::arrays::HasDropAndDefault::HasDropAndDefault(HasDropAndDefault&& other)
    : HasDropAndDefault() {
  *this = ::std::move(other);
}
inline ::arrays::HasDropAndDefault& ::arrays::HasDropAndDefault::operator=(
    HasDropAndDefault&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline void HasDropAndDefault::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(HasDropAndDefault, x));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_function_uwith_uarray_uid(void*,
                                                         void* __ret_ptr);
}
inline ::std::array<::std::int32_t, 2> function_with_array_id(
    ::std::array<::std::int32_t, 2> array) {
  crubit::Slot<::std::array<::std::int32_t, 2>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_function_uwith_uarray_uid(
      &array, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
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
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_function_uwith_uarray_utuple_uid(
    void**, void** __ret_ptr);
}
inline ::std::tuple<::std::array<::std::int32_t, 2>,
                    ::std::array<::std::int32_t, 2>>
function_with_array_tuple_id(::std::tuple<::std::array<::std::int32_t, 2>,
                                          ::std::array<::std::int32_t, 2>>
                                 array_tup) {
  auto&& array_tup_0 = ::std::get<0>(array_tup);
  auto&& array_tup_cabi_0 = &array_tup_0;
  auto&& array_tup_1 = ::std::get<1>(array_tup);
  auto&& array_tup_cabi_1 = &array_tup_1;
  void* array_tup_cabi[] = {&array_tup_cabi_0, &array_tup_cabi_1};
  crubit::Slot<::std::array<::std::int32_t, 2>> __return_value_0_ret_val_holder;
  auto* __return_value_0_storage = __return_value_0_ret_val_holder.Get();
  crubit::Slot<::std::array<::std::int32_t, 2>> __return_value_1_ret_val_holder;
  auto* __return_value_1_storage = __return_value_1_ret_val_holder.Get();
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage};
  __crubit_internal::__crubit_thunk_function_uwith_uarray_utuple_uid(
      array_tup_cabi, __return_value_storage);
  return ::std::make_tuple(
      ::std::move(__return_value_0_ret_val_holder).AssumeInitAndTakeValue(),
      ::std::move(__return_value_1_ret_val_holder).AssumeInitAndTakeValue());
}

namespace __crubit_internal {
extern "C" ::std::array<::std::int32_t, 2> const*
__crubit_thunk_function_uwith_uconst_uarray_uptr_uid(
    ::std::array<::std::int32_t, 2> const*);
}
inline ::std::array<::std::int32_t, 2> const* function_with_const_array_ptr_id(
    ::std::array<::std::int32_t, 2> const* array_ptr) {
  return __crubit_internal::
      __crubit_thunk_function_uwith_uconst_uarray_uptr_uid(array_ptr);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_function_uwith_uempty_uarray(void*,
                                                            void* __ret_ptr);
}
inline ::std::array<::std::int32_t, 0> function_with_empty_array(
    ::std::array<::std::int32_t, 0> array) {
  crubit::Slot<::std::array<::std::int32_t, 0>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_function_uwith_uempty_uarray(
      &array, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void
__crubit_thunk_function_uwith_uhas_udrop_uand_udefault_uarray_uid(
    void*, void* __ret_ptr);
}
inline ::std::array<::arrays::HasDropAndDefault, 2>
function_with_has_drop_and_default_array_id(
    ::std::array<::arrays::HasDropAndDefault, 2> array) {
  crubit::Slot array_slot((::std::move(array)));
  crubit::Slot<::std::array<::arrays::HasDropAndDefault, 2>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_function_uwith_uhas_udrop_uand_udefault_uarray_uid(
          array_slot.Get(), __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_function_uwith_uhas_udrop_uarray_uid(
    void*, void* __ret_ptr);
}
inline ::std::array<::arrays::HasDrop, 2> function_with_has_drop_array_id(
    ::std::array<::arrays::HasDrop, 2> array) {
  crubit::Slot array_slot((::std::move(array)));
  crubit::Slot<::std::array<::arrays::HasDrop, 2>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_function_uwith_uhas_udrop_uarray_uid(
      array_slot.Get(), __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_function_uwith_uhas_udrop_uret_uonly(
    void* __ret_ptr);
}
inline ::std::array<::arrays::HasDrop, 2> function_with_has_drop_ret_only() {
  crubit::Slot<::std::array<::arrays::HasDrop, 2>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_function_uwith_uhas_udrop_uret_uonly(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::array<::std::int32_t, 3> const*
__crubit_thunk_function_uwith_umut_uarray_unamed_usize_uptr_uid(
    ::std::array<::std::int32_t, 3> const*);
}
inline ::std::array<::std::int32_t, 3> const*
function_with_mut_array_named_size_ptr_id(
    ::std::array<::std::int32_t, 3> const* array_ptr) {
  return __crubit_internal::
      __crubit_thunk_function_uwith_umut_uarray_unamed_usize_uptr_uid(
          array_ptr);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_function_uwith_unested_uarrays(void*,
                                                              void* __ret_ptr);
}
inline ::std::array<::std::array<::std::int32_t, 2>, 2>
function_with_nested_arrays(
    ::std::array<::std::array<::std::int32_t, 2>, 2> array) {
  crubit::Slot<::std::array<::std::array<::std::int32_t, 2>, 2>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_function_uwith_unested_uarrays(
      &array, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_function_uwith_unested_udrop_udefault_uarrays(
    void*, void* __ret_ptr);
}
inline ::std::array<::std::array<::arrays::HasDropAndDefault, 2>, 2>
function_with_nested_drop_default_arrays(
    ::std::array<::std::array<::arrays::HasDropAndDefault, 2>, 2> array) {
  crubit::Slot array_slot((::std::move(array)));
  crubit::Slot<::std::array<::std::array<::arrays::HasDropAndDefault, 2>, 2>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::
      __crubit_thunk_function_uwith_unested_udrop_udefault_uarrays(
          array_slot.Get(), __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_function_uwith_utuple_uarray_uid(
    void*, void* __ret_ptr);
}
inline ::std::array<rs::Tuple<::std::int32_t, ::std::int32_t>, 2>
function_with_tuple_array_id(
    ::std::array<rs::Tuple<::std::int32_t, ::std::int32_t>, 2> tup_array) {
  crubit::Slot<::std::array<rs::Tuple<::std::int32_t, ::std::int32_t>, 2>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_function_uwith_utuple_uarray_uid(
      &tup_array, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace arrays

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    rs::Tuple<::std::int32_t, ::std::int32_t>* __ret_ptr);
}
inline ::rs::Tuple<::std::int32_t, ::std::int32_t>::Tuple() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Tuple<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Tuple<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Tuple<::std::int32_t, ::std::int32_t>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Tuple<::std::int32_t, ::std::int32_t>>);
inline rs::Tuple<::std::int32_t, ::std::int32_t>::Tuple(
    ::std::tuple<::std::int32_t, ::std::int32_t>&& tuple) noexcept {
  ::std::construct_at(reinterpret_cast<::std::int32_t*>(storage_ + 0),
                      ::std::move(::std::get<0>(tuple)));
  ::std::construct_at(reinterpret_cast<::std::int32_t*>(storage_ + 4),
                      ::std::move(::std::get<1>(tuple)));
}
inline rs::Tuple<::std::int32_t, ::std::int32_t>::operator ::std::tuple<
    ::std::int32_t, ::std::int32_t>() && noexcept {
  return ::std::tuple<::std::int32_t, ::std::int32_t>(
      ::std::move(*reinterpret_cast<::std::int32_t*>(storage_ + 0)),
      ::std::move(*reinterpret_cast<::std::int32_t*>(storage_ + 4)));
}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ARRAYS_ARRAYS_GOLDEN
