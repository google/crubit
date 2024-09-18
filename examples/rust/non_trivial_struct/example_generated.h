// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate
// Features: <none>

// clang-format off
#pragma once

#include "support/internal/attribute_macros.h"
#include "support/internal/memswap.h"

#include <cstddef>
#include <cstdint>
#include <utility>

namespace example_crate {

// Generated from:
// examples/rust/non_trivial_struct/example.rs;l=6
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: example_crate :: NonTrivialStruct") alignas(4)
    [[clang::trivial_abi]] NonTrivialStruct final {
 public:
  // Default::default
  NonTrivialStruct();

  // Drop::drop
  ~NonTrivialStruct();

  NonTrivialStruct(NonTrivialStruct&&);
  NonTrivialStruct& operator=(NonTrivialStruct&&);

  // `NonTrivialStruct` doesn't implement the `Clone` trait
  NonTrivialStruct(const NonTrivialStruct&) = delete;
  NonTrivialStruct& operator=(const NonTrivialStruct&) = delete;

 public:
  union {
    // Generated from:
    // examples/rust/non_trivial_struct/example.rs;l=7
    std::int32_t a;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(NonTrivialStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonTrivialStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate16NonTrivialStructNtNtCs8sGNUgcxoFi_u4core7default7Default7defaultB4_u(
    ::example_crate::NonTrivialStruct* __ret_ptr);
}
inline NonTrivialStruct::NonTrivialStruct() {
  __crubit_internal::
      __crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate16NonTrivialStructNtNtCs8sGNUgcxoFi_u4core7default7Default7defaultB4_u(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate16NonTrivialStructNtNtNtCs8sGNUgcxoFi_u4core3ops4drop4Drop4dropB4_u(
    ::example_crate::NonTrivialStruct& [[clang::annotate_type("lifetime",
                                                              "__anon1")]]);
}
inline NonTrivialStruct::~NonTrivialStruct() {
  __crubit_internal::
      __crubit_thunk__uRNvYNtCsh2mUQSogkZl_u13example_ucrate16NonTrivialStructNtNtNtCs8sGNUgcxoFi_u4core3ops4drop4Drop4dropB4_u(
          *this);
}
inline NonTrivialStruct::NonTrivialStruct(NonTrivialStruct&& other)
    : NonTrivialStruct() {
  *this = std::move(other);
}
inline NonTrivialStruct& NonTrivialStruct::operator=(NonTrivialStruct&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline void NonTrivialStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NonTrivialStruct, a));
}
}  // namespace example_crate
