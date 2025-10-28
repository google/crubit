// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// modules_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_MODULES_MODULES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_MODULES_MODULES_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace modules {

namespace basic_module {

// Generated from:
// cc_bindings_from_rs/test/modules/modules.rs;l=9
std::int32_t add_i32(std::int32_t x, std::int32_t y);

}  // namespace basic_module

namespace [[deprecated]] deprecated_module {

// Generated from:
// cc_bindings_from_rs/test/modules/modules.rs;l=16
[[deprecated]] std::int32_t add_i32(std::int32_t x, std::int32_t y);

}  // namespace deprecated_module

namespace outer {

namespace [[deprecated]] inner_deprecated {

// Error generating bindings for `outer::inner_deprecated::SomeType` defined at
// cc_bindings_from_rs/test/modules/modules.rs;l=26:
// Zero-sized types (ZSTs) are not supported (b/258259459)

}

}  // namespace outer

namespace outer::middle {

namespace [[deprecated]] innermost_deprecated {

// Error generating bindings for `outer::middle::innermost_deprecated::SomeType`
// defined at
// cc_bindings_from_rs/test/modules/modules.rs;l=32:
// Zero-sized types (ZSTs) are not supported (b/258259459)

}

}  // namespace outer::middle

namespace impl_in_separate_private_module {

// Generated from:
// cc_bindings_from_rs/test/modules/modules.rs;l=48
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: modules_golden :: impl_in_separate_private_module :: Foo") alignas(4)
    [[clang::trivial_abi]] Foo final {
 public:
  // `impl_in_separate_private_module::Foo` doesn't implement the `Default`
  // trait
  Foo() = delete;

  // Synthesized tuple constructor
  explicit Foo(std::int32_t __field0) : __field0(std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~Foo() = default;
  Foo(Foo&&) = default;
  Foo& operator=(Foo&&) = default;

  // `impl_in_separate_private_module::Foo` doesn't implement the `Clone` trait
  Foo(const Foo&) = delete;
  Foo& operator=(const Foo&) = delete;
  Foo(::crubit::UnsafeRelocateTag, Foo&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/modules/modules.rs;l=55
  static ::modules::impl_in_separate_private_module::Foo create(std::int32_t i);

  // Generated from:
  // cc_bindings_from_rs/test/modules/modules.rs;l=59
  static std::int32_t into_i32(
      ::modules::impl_in_separate_private_module::Foo s);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/modules/modules.rs;l=48
    std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace impl_in_separate_private_module

namespace basic_module {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_add_ui32(std::int32_t, std::int32_t);
}
inline std::int32_t add_i32(std::int32_t x, std::int32_t y) {
  return __crubit_internal::__crubit_thunk_add_ui32(x, y);
}

}  // namespace basic_module

namespace [[deprecated]] deprecated_module {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_add_ui32(std::int32_t, std::int32_t);
}
inline std::int32_t add_i32(std::int32_t x, std::int32_t y) {
  return __crubit_internal::__crubit_thunk_add_ui32(x, y);
}

}  // namespace deprecated_module

namespace outer {

namespace [[deprecated]] inner_deprecated {}

}  // namespace outer

namespace outer::middle {

namespace [[deprecated]] innermost_deprecated {}

}  // namespace outer::middle

namespace impl_in_separate_private_module {

static_assert(
    sizeof(Foo) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Foo) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<Foo>);
static_assert(std::is_trivially_move_constructible_v<Foo>);
static_assert(std::is_trivially_move_assignable_v<Foo>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(
    std::int32_t, ::modules::impl_in_separate_private_module::Foo* __ret_ptr);
}
inline ::modules::impl_in_separate_private_module::Foo Foo::create(
    std::int32_t i) {
  crubit::Slot<::modules::impl_in_separate_private_module::Foo>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(i, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_into_ui32(
    ::modules::impl_in_separate_private_module::Foo*);
}
inline std::int32_t Foo::into_i32(
    ::modules::impl_in_separate_private_module::Foo s) {
  return __crubit_internal::__crubit_thunk_into_ui32(&s);
}
inline void Foo::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Foo, __field0));
}
}  // namespace impl_in_separate_private_module

}  // namespace modules
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_MODULES_MODULES_GOLDEN
