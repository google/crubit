// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// trait_definition_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_IN_DEPENDENT_CRATE_TRAIT_DEFINITION_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_IN_DEPENDENT_CRATE_TRAIT_DEFINITION_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/offsetof.h"
#include "support/internal/slot.h"
#include "support/rs_std/traits.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>

namespace trait_definition {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: trait_definition_golden :: MyStruct") alignas(4) [[clang::trivial_abi]]
MyStruct final {
 public:
  // `trait_definition_golden::MyStruct` doesn't implement the `Default` trait
  MyStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyStruct() = default;
  MyStruct(MyStruct&&) = default;
  MyStruct& operator=(MyStruct&&) = default;

  // `trait_definition_golden::MyStruct` doesn't implement the `Clone` trait
  MyStruct(const MyStruct&) = delete;
  MyStruct& operator=(const MyStruct&) = delete;
  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    ::std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: trait_definition_golden :: MyTrait")
    MyTrait {
  template <typename T>
  using impl = rs_std::impl<T, MyTrait>;
};

}  // namespace trait_definition

template <>
struct rs_std::impl<::trait_definition::MyStruct, ::trait_definition::MyTrait> {
  static constexpr bool kIsImplemented = true;

  static ::std::int32_t do_something(::trait_definition::MyStruct const& self);
};

namespace trait_definition {

static_assert(
    sizeof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyStruct>);
static_assert(
    ::std::is_trivially_move_constructible_v<::trait_definition::MyStruct>);
static_assert(
    ::std::is_trivially_move_assignable_v<::trait_definition::MyStruct>);
inline void MyStruct::__crubit_field_offset_assertions() {
  CRUBIT_WARNING_PUSH("-Wno-invalid-offsetof")
  static_assert(0 == offsetof(MyStruct, y));
  CRUBIT_WARNING_POP
}
}  // namespace trait_definition

namespace trait_definition {
namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_MyTrait_udo_usomething_utrait_udefinition_ugolden_x0000003a_x0000003aMyStruct(
    ::trait_definition::MyStruct const&);
}
}  // namespace trait_definition
inline ::std::int32_t
rs_std::impl<::trait_definition::MyStruct, ::trait_definition::MyTrait>::
    do_something(::trait_definition::MyStruct const& self) {
  return trait_definition::__crubit_internal::
      __crubit_thunk_MyTrait_udo_usomething_utrait_udefinition_ugolden_x0000003a_x0000003aMyStruct(
          self);
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_IN_DEPENDENT_CRATE_TRAIT_DEFINITION_GOLDEN
