// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// uses_rust_golden
// Features: experimental, infer_operator_lifetimes, non_unpin_ctor,
// std_unique_ptr, std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_USES_RUST_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_USES_RUST_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

namespace uses_rust {

namespace test_use_glob {

// Generated from:
// cc_bindings_from_rs/test/golden/uses.rs;l=8
std::int32_t f1();

// Generated from:
// cc_bindings_from_rs/test/golden/uses.rs;l=12
std::int32_t f2();

// Generated from:
// cc_bindings_from_rs/test/golden/uses.rs;l=20
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: uses_rust_golden :: test_use_glob :: X1") alignas(4)
    [[clang::trivial_abi]] X1 final {
 public:
  // `test_use_glob::X1` doesn't implement the `Default` trait
  X1() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~X1() = default;
  X1(X1&&) = default;
  X1& operator=(X1&&) = default;

  // `test_use_glob::X1` doesn't implement the `Clone` trait
  X1(const X1&) = delete;
  X1& operator=(const X1&) = delete;
  X1(::crubit::UnsafeRelocateTag, X1&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/golden/uses.rs;l=21
    std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace test_use_glob

// Generated from:
// cc_bindings_from_rs/test/golden/uses.rs;l=32
struct CRUBIT_INTERNAL_RUST_TYPE(":: uses_rust_golden :: Bar") alignas(4)
    [[clang::trivial_abi]] Bar final {
 public:
  // `private_module::Bar` doesn't implement the `Default` trait
  Bar() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Bar() = default;
  Bar(Bar&&) = default;
  Bar& operator=(Bar&&) = default;

  // `private_module::Bar` doesn't implement the `Clone` trait
  Bar(const Bar&) = delete;
  Bar& operator=(const Bar&) = delete;
  Bar(::crubit::UnsafeRelocateTag, Bar&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/golden/uses.rs;l=33
    std::int32_t i;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/golden/uses.rs;l=35
struct CRUBIT_INTERNAL_RUST_TYPE(":: uses_rust_golden :: Foo") alignas(4)
    [[clang::trivial_abi]] Foo final {
 public:
  // `private_module::Foo` doesn't implement the `Default` trait
  Foo() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Foo() = default;
  Foo(Foo&&) = default;
  Foo& operator=(Foo&&) = default;

  // `private_module::Foo` doesn't implement the `Clone` trait
  Foo(const Foo&) = delete;
  Foo& operator=(const Foo&) = delete;
  Foo(::crubit::UnsafeRelocateTag, Foo&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/golden/uses.rs;l=41
  static ::uses_rust::Foo create();

  // Generated from:
  // cc_bindings_from_rs/test/golden/uses.rs;l=45
  static ::uses_rust::Bar bar();

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/golden/uses.rs;l=36
    std::int32_t i;
  };

 public:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/golden/uses.rs;l=37
    ::uses_rust::Bar bar_;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/golden/uses.rs;l=54
std::int32_t g1();

// Generated from:
// cc_bindings_from_rs/test/golden/uses.rs;l=58
std::int32_t g2();

// Generated from:
// cc_bindings_from_rs/test/golden/uses.rs;l=67
struct CRUBIT_INTERNAL_RUST_TYPE(":: uses_rust_golden :: InnerX") alignas(4)
    [[clang::trivial_abi]] InnerX final {
 public:
  // `m1::m2::InnerX` doesn't implement the `Default` trait
  InnerX() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~InnerX() = default;
  InnerX(InnerX&&) = default;
  InnerX& operator=(InnerX&&) = default;

  // `m1::m2::InnerX` doesn't implement the `Clone` trait
  InnerX(const InnerX&) = delete;
  InnerX& operator=(const InnerX&) = delete;
  InnerX(::crubit::UnsafeRelocateTag, InnerX&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/golden/uses.rs;l=68
    std::int32_t field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

namespace test_mod {

// Generated from:
// cc_bindings_from_rs/test/golden/uses.rs;l=77
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: uses_rust_golden :: test_mod :: S") alignas(4) [[clang::trivial_abi]]
S final {
 public:
  // `test_mod::S` doesn't implement the `Default` trait
  S() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~S() = default;
  S(S&&) = default;
  S& operator=(S&&) = default;

  // `test_mod::S` doesn't implement the `Clone` trait
  S(const S&) = delete;
  S& operator=(const S&) = delete;
  S(::crubit::UnsafeRelocateTag, S&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/golden/uses.rs;l=78
    std::int32_t field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace test_mod

using ::uses_rust::test_use_glob::f1;

namespace test_use_glob {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_f1();
}
inline std::int32_t f1() { return __crubit_internal::__crubit_thunk_f1(); }

}  // namespace test_use_glob

using ::uses_rust::test_use_glob::f2;

namespace test_use_glob {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_f2();
}
inline std::int32_t f2() { return __crubit_internal::__crubit_thunk_f2(); }

}  // namespace test_use_glob

using X1 CRUBIT_INTERNAL_RUST_TYPE(
    ":: uses_rust_golden :: test_use_glob :: X1") =
    ::uses_rust::test_use_glob::X1;

namespace test_use_glob {

static_assert(
    sizeof(X1) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(X1) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<X1>);
static_assert(std::is_trivially_move_constructible_v<X1>);
static_assert(std::is_trivially_move_assignable_v<X1>);
inline void X1::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(X1, x));
}
}  // namespace test_use_glob

static_assert(
    sizeof(Bar) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Bar) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<Bar>);
static_assert(std::is_trivially_move_constructible_v<Bar>);
static_assert(std::is_trivially_move_assignable_v<Bar>);
inline void Bar::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Bar, i));
}
static_assert(
    sizeof(Foo) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Foo) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<Foo>);
static_assert(std::is_trivially_move_constructible_v<Foo>);
static_assert(std::is_trivially_move_assignable_v<Foo>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(::uses_rust::Foo* __ret_ptr);
}
inline ::uses_rust::Foo Foo::create() {
  crubit::Slot<::uses_rust::Foo> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_bar(::uses_rust::Bar* __ret_ptr);
}
inline ::uses_rust::Bar Foo::bar() {
  crubit::Slot<::uses_rust::Bar> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_bar(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void Foo::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Foo, i));
  static_assert(4 == offsetof(Foo, bar_));
}
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_g1();
}
inline std::int32_t g1() { return __crubit_internal::__crubit_thunk_g1(); }

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_g2();
}
inline std::int32_t g2() { return __crubit_internal::__crubit_thunk_g2(); }

static_assert(
    sizeof(InnerX) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(InnerX) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<InnerX>);
static_assert(std::is_trivially_move_constructible_v<InnerX>);
static_assert(std::is_trivially_move_assignable_v<InnerX>);
inline void InnerX::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(InnerX, field));
}

using G CRUBIT_INTERNAL_RUST_TYPE(":: uses_rust_golden :: test_mod :: S") =
    ::uses_rust::test_mod::S;

namespace test_mod {

static_assert(
    sizeof(S) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(S) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<S>);
static_assert(std::is_trivially_move_constructible_v<S>);
static_assert(std::is_trivially_move_assignable_v<S>);
inline void S::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(S, field));
}
}  // namespace test_mod

}  // namespace uses_rust
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_USES_RUST_GOLDEN
