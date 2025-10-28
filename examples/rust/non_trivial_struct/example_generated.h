// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_NON_TRIVIAL_STRUCT_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_NON_TRIVIAL_STRUCT_EXAMPLE_CRATE_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <utility>

namespace example_crate {

// Generated from:
// examples/rust/non_trivial_struct/example.rs;l=6
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: example_crate_golden :: NonTrivialStruct") alignas(4)
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
  NonTrivialStruct(::crubit::UnsafeRelocateTag, NonTrivialStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }
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
extern "C" void __crubit_thunk_default(
    ::example_crate::NonTrivialStruct* __ret_ptr);
}
inline NonTrivialStruct::NonTrivialStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::example_crate::NonTrivialStruct&);
}
inline NonTrivialStruct::~NonTrivialStruct() {
  __crubit_internal::__crubit_thunk_drop(*this);
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
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_NON_TRIVIAL_STRUCT_EXAMPLE_CRATE_GOLDEN
