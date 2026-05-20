// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// stdlib_golden
// Features: callables, supported, types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_STDLIB_STDLIB_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_STDLIB_STDLIB_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/bridge.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/rs_std/traits.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <optional>
#include <utility>

#include "support/rs_std/rs_core.h"

namespace stdlib {

// Generated from:
// cc_bindings_from_rs/test/traits/stdlib/stdlib.rs;l=19
struct CRUBIT_INTERNAL_RUST_TYPE(":: stdlib_golden :: MyStruct") alignas(4)
    [[clang::trivial_abi]] MyStruct final {
 public:
  // Default::default
  MyStruct();

  // Drop::drop
  ~MyStruct();

  MyStruct(MyStruct&&);
  ::stdlib::MyStruct& operator=(MyStruct&&);

  // Clone::clone
  MyStruct(const MyStruct&);

  // Clone::clone_from
  ::stdlib::MyStruct& operator=(const MyStruct&);

  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/traits/stdlib/stdlib.rs;l=24
  static ::stdlib::MyStruct new_(::std::int32_t x);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/traits/stdlib/stdlib.rs;l=20
    ::std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/traits/stdlib/stdlib.rs;l=5
struct CRUBIT_INTERNAL_RUST_TYPE(":: stdlib_golden :: MyTrait") MyTrait {
  template <typename T>
  using impl = rs_std::impl<T, MyTrait>;
};

}  // namespace stdlib

template <>
struct rs_std::impl<::stdlib::MyStruct, ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  // Generated from:
  // cc_bindings_from_rs/test/traits/stdlib/stdlib.rs;l=47
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<stdlib_golden::MyStruct as :: core :: iter :: Iterator>::Item") =
      ::std::int32_t;

  // Generated from:
  // cc_bindings_from_rs/test/traits/stdlib/stdlib.rs;l=49
  static ::std::optional<::std::int32_t> next(::stdlib::MyStruct& self);
};

namespace stdlib {

static_assert(
    sizeof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::stdlib::MyStruct* __ret_ptr);
}
inline ::stdlib::MyStruct::MyStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::stdlib::MyStruct&);
}
inline MyStruct::~MyStruct() { __crubit_internal::__crubit_thunk_drop(*this); }
inline ::stdlib::MyStruct::MyStruct(MyStruct&& other) : MyStruct() {
  *this = ::std::move(other);
}
inline ::stdlib::MyStruct& ::stdlib::MyStruct::operator=(MyStruct&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone(::stdlib::MyStruct const&,
                                     ::stdlib::MyStruct* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_ufrom(::stdlib::MyStruct&,
                                           ::stdlib::MyStruct const&);
}
inline ::stdlib::MyStruct::MyStruct(const MyStruct& other) {
  __crubit_internal::__crubit_thunk_clone(other, this);
}
inline ::stdlib::MyStruct& ::stdlib::MyStruct::operator=(
    const MyStruct& other) {
  if (this != &other) {
    __crubit_internal::__crubit_thunk_clone_ufrom(*this, other);
  }
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t,
                                   ::stdlib::MyStruct* __ret_ptr);
}
inline ::stdlib::MyStruct MyStruct::new_(::std::int32_t x) {
  crubit::Slot<::stdlib::MyStruct> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void MyStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyStruct, x));
}
}  // namespace stdlib

namespace stdlib {
namespace __crubit_internal {
extern "C" void __crubit_thunk_Iterator_unext(::stdlib::MyStruct&,
                                              unsigned char* __ret_ptr);
}
}  // namespace stdlib
inline ::std::optional<::std::int32_t>
rs_std::impl<::stdlib::MyStruct, ::rs::core::iter::Iterator>::next(
    ::stdlib::MyStruct& self) {
  unsigned char __return_value_storage
      [::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>::kSize];
  stdlib::__crubit_internal::__crubit_thunk_Iterator_unext(
      self, __return_value_storage);
  return ::crubit::internal::Decode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>(
          ::crubit::TransmuteAbi<::std::int32_t>()),
      __return_value_storage);
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_STDLIB_STDLIB_GOLDEN
