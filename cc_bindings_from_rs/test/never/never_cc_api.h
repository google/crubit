// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// never_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_NEVER_NEVER_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_NEVER_NEVER_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace never {

// Generated from:
// cc_bindings_from_rs/test/never/never.rs;l=8
[[noreturn]] void never_return();

// Generated from:
// cc_bindings_from_rs/test/never/never.rs;l=13
extern "C" [[noreturn]] void extern_never_return();

// Generated from:
// cc_bindings_from_rs/test/never/never.rs;l=18
struct CRUBIT_INTERNAL_RUST_TYPE(":: never_golden :: NeverStruct") alignas(4)
    [[clang::trivial_abi]] NeverStruct final {
 public:
  // Default::default
  NeverStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~NeverStruct() = default;
  NeverStruct(NeverStruct&&) = default;
  NeverStruct& operator=(NeverStruct&&) = default;

  // `NeverStruct` doesn't implement the `Clone` trait
  NeverStruct(const NeverStruct&) = delete;
  NeverStruct& operator=(const NeverStruct&) = delete;
  NeverStruct(::crubit::UnsafeRelocateTag, NeverStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/never/never.rs;l=25
  [[noreturn]] static void associated_fn_never_return();

  // Generated from:
  // cc_bindings_from_rs/test/never/never.rs;l=29
  [[noreturn]] void method_never_return() const;

 private:
  union {
    //  Having a non-ZST field avoids hitting the following error:
    //
    //  "Zero-sized types (ZSTs) are not supported (b/258259459)"
    //
    // Generated from:
    // cc_bindings_from_rs/test/never/never.rs;l=21
    std::int32_t _non_zst_field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

namespace __crubit_internal {
extern "C" [[noreturn]] void __crubit_thunk_never_ureturn();
}
inline void never_return() {
  __crubit_internal::__crubit_thunk_never_ureturn();
}

static_assert(
    sizeof(NeverStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NeverStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::never::NeverStruct* __ret_ptr);
}
inline NeverStruct::NeverStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<NeverStruct>);
static_assert(std::is_trivially_move_constructible_v<NeverStruct>);
static_assert(std::is_trivially_move_assignable_v<NeverStruct>);
namespace __crubit_internal {
extern "C" [[noreturn]] void __crubit_thunk_associated_ufn_unever_ureturn();
}
inline void NeverStruct::associated_fn_never_return() {
  __crubit_internal::__crubit_thunk_associated_ufn_unever_ureturn();
}

namespace __crubit_internal {
extern "C" [[noreturn]] void __crubit_thunk_method_unever_ureturn(
    ::never::NeverStruct const&);
}
inline void NeverStruct::method_never_return() const {
  auto&& self = *this;
  __crubit_internal::__crubit_thunk_method_unever_ureturn(self);
}
inline void NeverStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NeverStruct, _non_zst_field));
}
}  // namespace never
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_NEVER_NEVER_GOLDEN
