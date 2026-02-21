// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector,
// supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_TRAIT_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_TRAIT_EXAMPLE_CRATE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/str_ref.h"
#include "support/rs_std/traits.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace example_crate {

// Generated from:
// examples/rust/trait/example.rs;l=12
struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate_golden :: MyStruct") alignas(
    4) [[clang::trivial_abi]] MyStruct final {
 public:
  // Default::default
  MyStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyStruct() = default;
  MyStruct(MyStruct&&) = default;
  ::example_crate::MyStruct& operator=(MyStruct&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  MyStruct(const MyStruct&) = default;
  ::example_crate::MyStruct& operator=(const MyStruct&) = default;
  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // examples/rust/trait/example.rs;l=17
  static ::example_crate::MyStruct new_(std::int32_t x);

 private:
  union {
    // Generated from:
    // examples/rust/trait/example.rs;l=13
    std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from: examples/rust/trait/example.rs;l=5
struct CRUBIT_INTERNAL_RUST_TYPE(":: example_crate_golden :: MyTrait") MyTrait {
  template <typename T>
  using impl = rs_std::impl<T, MyTrait>;
};

static_assert(
    sizeof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::example_crate::MyStruct* __ret_ptr);
}
inline ::example_crate::MyStruct::MyStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<MyStruct>);
static_assert(
    std::is_trivially_move_constructible_v<::example_crate::MyStruct>);
static_assert(std::is_trivially_move_assignable_v<::example_crate::MyStruct>);
static_assert(
    std::is_trivially_copy_constructible_v<::example_crate::MyStruct>);
static_assert(std::is_trivially_copy_assignable_v<::example_crate::MyStruct>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(std::int32_t,
                                   ::example_crate::MyStruct* __ret_ptr);
}
inline ::example_crate::MyStruct MyStruct::new_(std::int32_t x) {
  crubit::Slot<::example_crate::MyStruct> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void MyStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyStruct, x));
}
}  // namespace example_crate

template <>
struct rs_std::impl<::example_crate::MyStruct, ::example_crate::MyTrait> {
  static constexpr bool kIsImplemented = true;

  // Generated from:
  // examples/rust/trait/example.rs;l=23
  static std::int32_t add_with(::example_crate::MyStruct const& self,
                               std::int32_t y);

  // Generated from:
  // examples/rust/trait/example.rs;l=27
  static rs_std::StrRef describe(::example_crate::MyStruct const& self);
};

namespace example_crate {
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_MyTrait_uadd_uwith(
    ::example_crate::MyStruct const&, std::int32_t);
}
}  // namespace example_crate
inline std::int32_t
rs_std::impl<::example_crate::MyStruct, ::example_crate::MyTrait>::add_with(
    ::example_crate::MyStruct const& self, std::int32_t y) {
  return example_crate::__crubit_internal::__crubit_thunk_MyTrait_uadd_uwith(
      self, y);
}

namespace example_crate {
namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_MyTrait_udescribe(
    ::example_crate::MyStruct const&);
}
}  // namespace example_crate
inline rs_std::StrRef
rs_std::impl<::example_crate::MyStruct, ::example_crate::MyTrait>::describe(
    ::example_crate::MyStruct const& self) {
  return example_crate::__crubit_internal::__crubit_thunk_MyTrait_udescribe(
      self);
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_TRAIT_EXAMPLE_CRATE_GOLDEN
