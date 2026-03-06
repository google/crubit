// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// no_core_golden
// Features: supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_NO_STD_NO_CORE_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_NO_STD_NO_CORE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/str_ref.h"

#include <array>
#include <cstddef>
#include <utility>

namespace no_core {

// Generated from:
// cc_bindings_from_rs/test/no_std/no_core.rs;l=12
struct CRUBIT_INTERNAL_RUST_TYPE(":: no_core_golden :: Test") alignas(8)
    [[clang::trivial_abi]] Test final {
 public:
  // `no_core_golden::Test` doesn't implement the `Default` trait
  Test() = delete;

  // Drop::drop
  ~Test();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Test(Test&&) = delete;
  ::no_core::Test& operator=(Test&&) = delete;
  // `no_core_golden::Test` doesn't implement the `Clone` trait
  Test(const Test&) = delete;
  Test& operator=(const Test&) = delete;
  Test(::crubit::UnsafeRelocateTag, Test&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/no_std/no_core.rs;l=17
  static ::no_core::Test new_();

  // Generated from:
  // cc_bindings_from_rs/test/no_std/no_core.rs;l=21
  rs_std::StrRef s() const& $(__anon1) CRUBIT_LIFETIME_BOUND;

 private:
  // Field type has been replaced with a blob of bytes: Type
  // `std::string::String` comes from the `alloc` crate, but no `--crate-header`
  // was specified for this crate
  std::array<unsigned char, 24> s_;

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(Test) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Test) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::no_core::Test&);
}
inline Test::~Test() { __crubit_internal::__crubit_thunk_drop(*this); }
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::no_core::Test* __ret_ptr);
}
inline ::no_core::Test Test::new_() {
  crubit::Slot<::no_core::Test> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_s(::no_core::Test const&);
}
inline rs_std::StrRef Test::s() const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_s(self);
}
inline void Test::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Test, s_));
}
}  // namespace no_core

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_NO_STD_NO_CORE_GOLDEN
