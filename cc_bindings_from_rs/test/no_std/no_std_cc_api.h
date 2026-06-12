// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// no_std_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_NO_STD_NO_STD_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_NO_STD_NO_STD_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/offsetof.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/str_ref.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <utility>

#include "support/rs_std/rs_alloc.h"

namespace no_std {

struct CRUBIT_INTERNAL_RUST_TYPE(":: no_std_golden :: NoStdStruct") alignas(8)
    [[clang::trivial_abi]] NoStdStruct final {
 public:
  // `no_std_golden::NoStdStruct` doesn't implement the `Default` trait
  NoStdStruct() = delete;

  // Drop::drop
  ~NoStdStruct();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  NoStdStruct(NoStdStruct&&) = delete;
  ::no_std::NoStdStruct& operator=(NoStdStruct&&) = delete;
  // `no_std_golden::NoStdStruct` doesn't implement the `Clone` trait
  NoStdStruct(const NoStdStruct&) = delete;
  NoStdStruct& operator=(const NoStdStruct&) = delete;
  NoStdStruct(::crubit::UnsafeRelocateTag, NoStdStruct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  static ::no_std::NoStdStruct new_(::std::int32_t x, float y);

  rs_std::StrRef display() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  union {
    ::rs::alloc::string::String test;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(NoStdStruct) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NoStdStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::no_std::NoStdStruct&);
}
inline NoStdStruct::~NoStdStruct() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t, float,
                                   ::no_std::NoStdStruct* __ret_ptr);
}
inline ::no_std::NoStdStruct NoStdStruct::new_(::std::int32_t x, float y) {
  crubit::Slot<::no_std::NoStdStruct> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, y, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_display(::no_std::NoStdStruct const&);
}
inline rs_std::StrRef NoStdStruct::display() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_display(self);
}
inline void NoStdStruct::__crubit_field_offset_assertions() {
  CRUBIT_WARNING_PUSH("-Wno-invalid-offsetof")
  static_assert(0 == offsetof(NoStdStruct, test));
  CRUBIT_WARNING_POP
}
}  // namespace no_std

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_NO_STD_NO_STD_GOLDEN
