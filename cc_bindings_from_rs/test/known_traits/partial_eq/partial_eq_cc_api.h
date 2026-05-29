// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// partial_eq_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_PARTIAL_EQ_PARTIAL_EQ_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_PARTIAL_EQ_PARTIAL_EQ_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace partial_eq::basic_test {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: partial_eq_golden :: basic_test :: MyStruct") alignas(8)
    [[clang::trivial_abi]] MyStruct final {
 public:
  // `partial_eq_golden::basic_test::MyStruct` doesn't implement the `Default`
  // trait
  MyStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyStruct() = default;
  MyStruct(MyStruct&&) = default;
  MyStruct& operator=(MyStruct&&) = default;

  // `partial_eq_golden::basic_test::MyStruct` doesn't implement the `Clone`
  // trait
  MyStruct(const MyStruct&) = delete;
  MyStruct& operator=(const MyStruct&) = delete;
  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  static ::partial_eq::basic_test::MyStruct new_(::std::uintptr_t val);

  bool operator==(::partial_eq::basic_test::MyStruct const& other) const;

 private:
  union {
    ::std::uintptr_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace partial_eq::basic_test

namespace partial_eq::usize_rhs {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: partial_eq_golden :: usize_rhs :: MyStruct") alignas(8)
    [[clang::trivial_abi]] MyStruct final {
 public:
  // `partial_eq_golden::usize_rhs::MyStruct` doesn't implement the `Default`
  // trait
  MyStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyStruct() = default;
  MyStruct(MyStruct&&) = default;
  MyStruct& operator=(MyStruct&&) = default;

  // `partial_eq_golden::usize_rhs::MyStruct` doesn't implement the `Clone`
  // trait
  MyStruct(const MyStruct&) = delete;
  MyStruct& operator=(const MyStruct&) = delete;
  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  static ::partial_eq::usize_rhs::MyStruct new_(::std::uintptr_t val);

  bool operator==(::std::uintptr_t const& other) const;

 private:
  union {
    ::std::uintptr_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace partial_eq::usize_rhs

namespace partial_eq::basic_test {

static_assert(
    sizeof(MyStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyStruct>);
static_assert(::std::is_trivially_move_constructible_v<
              ::partial_eq::basic_test::MyStruct>);
static_assert(
    ::std::is_trivially_move_assignable_v<::partial_eq::basic_test::MyStruct>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(
    ::std::uintptr_t, ::partial_eq::basic_test::MyStruct* __ret_ptr);
}
inline ::partial_eq::basic_test::MyStruct MyStruct::new_(::std::uintptr_t val) {
  crubit::Slot<::partial_eq::basic_test::MyStruct>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_PartialEq_ueq(
    ::partial_eq::basic_test::MyStruct const&,
    ::partial_eq::basic_test::MyStruct const&);
}
inline bool MyStruct::operator==(
    ::partial_eq::basic_test::MyStruct const& other) const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_PartialEq_ueq(self, other);
}
inline void MyStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyStruct, __field0));
}
}  // namespace partial_eq::basic_test

namespace partial_eq::usize_rhs {

static_assert(
    sizeof(MyStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyStruct>);
static_assert(::std::is_trivially_move_constructible_v<
              ::partial_eq::usize_rhs::MyStruct>);
static_assert(
    ::std::is_trivially_move_assignable_v<::partial_eq::usize_rhs::MyStruct>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(
    ::std::uintptr_t, ::partial_eq::usize_rhs::MyStruct* __ret_ptr);
}
inline ::partial_eq::usize_rhs::MyStruct MyStruct::new_(::std::uintptr_t val) {
  crubit::Slot<::partial_eq::usize_rhs::MyStruct> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(val, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_PartialEq_ueq(
    ::partial_eq::usize_rhs::MyStruct const&, ::std::uintptr_t const&);
}
inline bool MyStruct::operator==(::std::uintptr_t const& other) const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_PartialEq_ueq(self, other);
}
inline void MyStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyStruct, __field0));
}
}  // namespace partial_eq::usize_rhs

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_PARTIAL_EQ_PARTIAL_EQ_GOLDEN
