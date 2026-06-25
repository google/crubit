// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// uses_reexport_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_REEXPORT_USES_REEXPORT_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_REEXPORT_USES_REEXPORT_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace uses_reexport {

struct CRUBIT_INTERNAL_RUST_TYPE(":: uses_reexport_golden :: Bar") alignas(4)
    [[clang::trivial_abi]] Bar final {
 public:
  // `uses_reexport_golden::Bar` doesn't implement the `Default` trait
  Bar() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Bar() = default;
  Bar(Bar&&) = default;
  Bar& operator=(Bar&&) = default;

  // `uses_reexport_golden::Bar` doesn't implement the `Clone` trait
  Bar(const Bar&) = delete;
  Bar& operator=(const Bar&) = delete;
  Bar(::crubit::UnsafeRelocateTag, Bar&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    ::std::int32_t i;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: uses_reexport_golden :: Foo") alignas(4)
    [[clang::trivial_abi]] Foo final {
 public:
  // `uses_reexport_golden::Foo` doesn't implement the `Default` trait
  Foo() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Foo() = default;
  Foo(Foo&&) = default;
  Foo& operator=(Foo&&) = default;

  // `uses_reexport_golden::Foo` doesn't implement the `Clone` trait
  Foo(const Foo&) = delete;
  Foo& operator=(const Foo&) = delete;
  Foo(::crubit::UnsafeRelocateTag, Foo&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  static ::uses_reexport::Foo create();

  static ::uses_reexport::Bar bar();

 private:
  union {
    ::std::int32_t i;
  };

 public:
  union {
    ::uses_reexport::Bar bar_;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace uses_reexport

namespace uses_reexport {

struct CRUBIT_INTERNAL_RUST_TYPE(":: uses_reexport_golden :: G") alignas(4)
    [[clang::trivial_abi]] G final {
 public:
  // `uses_reexport_golden::G` doesn't implement the `Default` trait
  G() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~G() = default;
  G(G&&) = default;
  G& operator=(G&&) = default;

  // `uses_reexport_golden::G` doesn't implement the `Clone` trait
  G(const G&) = delete;
  G& operator=(const G&) = delete;
  G(::crubit::UnsafeRelocateTag, G&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace uses_reexport

namespace uses_reexport {

struct CRUBIT_INTERNAL_RUST_TYPE(":: uses_reexport_golden :: InnerX") alignas(4)
    [[clang::trivial_abi]] InnerX final {
 public:
  // `uses_reexport_golden::InnerX` doesn't implement the `Default` trait
  InnerX() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~InnerX() = default;
  InnerX(InnerX&&) = default;
  InnerX& operator=(InnerX&&) = default;

  // `uses_reexport_golden::InnerX` doesn't implement the `Clone` trait
  InnerX(const InnerX&) = delete;
  InnerX& operator=(const InnerX&) = delete;
  InnerX(::crubit::UnsafeRelocateTag, InnerX&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace uses_reexport

namespace uses_reexport {

struct CRUBIT_INTERNAL_RUST_TYPE(":: uses_reexport_golden :: X1") alignas(4)
    [[clang::trivial_abi]] X1 final {
 public:
  // `uses_reexport_golden::X1` doesn't implement the `Default` trait
  X1() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~X1() = default;
  X1(X1&&) = default;
  X1& operator=(X1&&) = default;

  // `uses_reexport_golden::X1` doesn't implement the `Clone` trait
  X1(const X1&) = delete;
  X1& operator=(const X1&) = delete;
  X1(::crubit::UnsafeRelocateTag, X1&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    ::std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

::std::int32_t f1();

::std::int32_t f2();

}  // namespace uses_reexport

namespace uses_reexport {

::std::int32_t g1();

::std::int32_t g2();

static_assert(
    sizeof(Bar) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Bar) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Bar>);
static_assert(::std::is_trivially_move_constructible_v<::uses_reexport::Bar>);
static_assert(::std::is_trivially_move_assignable_v<::uses_reexport::Bar>);
inline void Bar::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Bar, i));
}
static_assert(
    sizeof(Foo) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Foo) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Foo>);
static_assert(::std::is_trivially_move_constructible_v<::uses_reexport::Foo>);
static_assert(::std::is_trivially_move_assignable_v<::uses_reexport::Foo>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_create(::uses_reexport::Foo* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::uses_reexport::Foo Foo::create() {
  crubit::Slot<::uses_reexport::Foo> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_bar(::uses_reexport::Bar* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::uses_reexport::Bar Foo::bar() {
  crubit::Slot<::uses_reexport::Bar> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_bar(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void Foo::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Foo, i));
  static_assert(4 == offsetof(Foo, bar_));
}
}  // namespace uses_reexport

namespace uses_reexport::test_mod {

using S CRUBIT_INTERNAL_RUST_TYPE(":: uses_reexport_golden :: G") =
    ::uses_reexport::G;
}

namespace uses_reexport {

static_assert(
    sizeof(G) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(G) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<G>);
static_assert(::std::is_trivially_move_constructible_v<::uses_reexport::G>);
static_assert(::std::is_trivially_move_assignable_v<::uses_reexport::G>);
inline void G::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(G, field));
}
}  // namespace uses_reexport

namespace uses_reexport {

static_assert(
    sizeof(InnerX) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(InnerX) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<InnerX>);
static_assert(
    ::std::is_trivially_move_constructible_v<::uses_reexport::InnerX>);
static_assert(::std::is_trivially_move_assignable_v<::uses_reexport::InnerX>);
inline void InnerX::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(InnerX, field));
}
}  // namespace uses_reexport

namespace uses_reexport::test_use_glob {

using X1 CRUBIT_INTERNAL_RUST_TYPE(":: uses_reexport_golden :: X1") =
    ::uses_reexport::X1;
}

namespace uses_reexport {

static_assert(
    sizeof(X1) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(X1) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<X1>);
static_assert(::std::is_trivially_move_constructible_v<::uses_reexport::X1>);
static_assert(::std::is_trivially_move_assignable_v<::uses_reexport::X1>);
inline void X1::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(X1, x));
}
}  // namespace uses_reexport

namespace uses_reexport::test_use_glob {
using ::uses_reexport::f1;
}

namespace uses_reexport {

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_f1();
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t f1() { return __crubit_internal::__crubit_thunk_f1(); }

}  // namespace uses_reexport

namespace uses_reexport::test_use_glob {
using ::uses_reexport::f2;
}

namespace uses_reexport {

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_f2();
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t f2() { return __crubit_internal::__crubit_thunk_f2(); }

}  // namespace uses_reexport

namespace uses_reexport {

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_g1();
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t g1() { return __crubit_internal::__crubit_thunk_g1(); }

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_g2();
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t g2() { return __crubit_internal::__crubit_thunk_g2(); }

}  // namespace uses_reexport

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_REEXPORT_USES_REEXPORT_GOLDEN
