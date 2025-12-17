// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// move_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_MOVE_SEMANTICS_MOVE_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_MOVE_SEMANTICS_MOVE_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace move {
struct Foo;

// Generated from:
// cc_bindings_from_rs/test/move_semantics/move.rs;l=27
void consume_foo(::move::Foo _foo);

// Generated from:
// cc_bindings_from_rs/test/move_semantics/move.rs;l=9
struct CRUBIT_INTERNAL_RUST_TYPE(":: move_golden :: Foo") alignas(8)
    [[clang::trivial_abi]] Foo final {
 public:
  // Default::default
  Foo();

  // Drop::drop
  ~Foo();

  Foo(Foo&&);
  Foo& operator=(Foo&&);

  // `move_golden::Foo` doesn't implement the `Clone` trait
  Foo(const Foo&) = delete;
  Foo& operator=(const Foo&) = delete;
  Foo(::crubit::UnsafeRelocateTag, Foo&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/move_semantics/move.rs;l=14
  static ::move::Foo from_byte(std::uint8_t byte);

  // Generated from:
  // cc_bindings_from_rs/test/move_semantics/move.rs;l=18
  std::uint8_t read_byte() const;

  // Generated from:
  // cc_bindings_from_rs/test/move_semantics/move.rs;l=22
  std::uint8_t into_byte() &&;

 private:
  // Field type has been replaced with a blob of bytes: Generic types are not
  // supported yet (b/259749095)
  unsigned char buf[8];

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/move_semantics/move.rs;l=30
struct CRUBIT_INTERNAL_RUST_TYPE(":: move_golden :: Copyable") alignas(1)
    [[clang::trivial_abi]] Copyable final {
 public:
  // Default::default
  Copyable();

  // No custom `Drop` impl and no custom "drop glue" required
  ~Copyable() = default;
  Copyable(Copyable&&) = default;
  Copyable& operator=(Copyable&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Copyable(const Copyable&) = default;
  Copyable& operator=(const Copyable&) = default;
  Copyable(::crubit::UnsafeRelocateTag, Copyable&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/move_semantics/move.rs;l=35
  static ::move::Copyable from_byte(std::uint8_t byte);

  //  Typically, `self`-by-value methods turn into `&&`-qualified methods in
  //  C++.
  //
  //  However, for `Copy` types, there's no need to consume the argument, as it
  //  will be copied
  //
  //  regardless.
  //
  // Generated from:
  // cc_bindings_from_rs/test/move_semantics/move.rs;l=42
  std::uint8_t consume_self() const;

  union {
    // Generated from:
    // cc_bindings_from_rs/test/move_semantics/move.rs;l=31
    std::uint8_t field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

namespace __crubit_internal {
extern "C" void __crubit_thunk_consume_ufoo(::move::Foo*);
}
inline void consume_foo(::move::Foo _foo) {
  crubit::Slot _foo_slot((std::move(_foo)));
  return __crubit_internal::__crubit_thunk_consume_ufoo(_foo_slot.Get());
}

static_assert(
    sizeof(Foo) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Foo) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::move::Foo* __ret_ptr);
}
inline Foo::Foo() { __crubit_internal::__crubit_thunk_default(this); }
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::move::Foo&);
}
inline Foo::~Foo() { __crubit_internal::__crubit_thunk_drop(*this); }
inline Foo::Foo(Foo&& other) : Foo() { *this = std::move(other); }
inline Foo& Foo::operator=(Foo&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_from_ubyte(std::uint8_t, ::move::Foo* __ret_ptr);
}
inline ::move::Foo Foo::from_byte(std::uint8_t byte) {
  crubit::Slot<::move::Foo> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_from_ubyte(byte, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::uint8_t __crubit_thunk_read_ubyte(::move::Foo const&);
}
inline std::uint8_t Foo::read_byte() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_read_ubyte(self);
}

namespace __crubit_internal {
extern "C" std::uint8_t __crubit_thunk_into_ubyte(::move::Foo*);
}
inline std::uint8_t Foo::into_byte() && {
  auto&& self = *this;
  crubit::Slot self_slot((std::move(self)));
  return __crubit_internal::__crubit_thunk_into_ubyte(self_slot.Get());
}
inline void Foo::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Foo, buf));
}
static_assert(
    sizeof(Copyable) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Copyable) == 1,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::move::Copyable* __ret_ptr);
}
inline Copyable::Copyable() { __crubit_internal::__crubit_thunk_default(this); }
static_assert(std::is_trivially_destructible_v<Copyable>);
static_assert(std::is_trivially_move_constructible_v<Copyable>);
static_assert(std::is_trivially_move_assignable_v<Copyable>);
static_assert(std::is_trivially_copy_constructible_v<Copyable>);
static_assert(std::is_trivially_copy_assignable_v<Copyable>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_from_ubyte(std::uint8_t,
                                          ::move::Copyable* __ret_ptr);
}
inline ::move::Copyable Copyable::from_byte(std::uint8_t byte) {
  crubit::Slot<::move::Copyable> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_from_ubyte(byte, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" std::uint8_t __crubit_thunk_consume_uself(::move::Copyable*);
}
inline std::uint8_t Copyable::consume_self() const {
  auto& self = const_cast<std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_consume_uself(&self);
}
inline void Copyable::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Copyable, field));
}
}  // namespace move
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_MOVE_SEMANTICS_MOVE_GOLDEN
