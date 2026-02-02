// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// traits_golden
// Features: custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr,
// std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_TRAITS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_TRAITS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"

#include <cstddef>
#include <cstdint>
#include <tuple>
#include <type_traits>
#include <utility>

namespace traits {

// Generated from:
// cc_bindings_from_rs/test/traits/traits.rs;l=20
template <typename Type>
struct CRUBIT_INTERNAL_RUST_TYPE(":: traits_golden :: DifferentTraitSameName")
    DifferentTraitSameName {
  static constexpr bool is_implemented = false;
};

// Generated from:
// cc_bindings_from_rs/test/traits/traits.rs;l=25
struct CRUBIT_INTERNAL_RUST_TYPE(":: traits_golden :: Foo") alignas(4)
    [[clang::trivial_abi]] Foo final {
 public:
  // Default::default
  Foo();

  // No custom `Drop` impl and no custom "drop glue" required
  ~Foo() = default;
  Foo(Foo&&) = default;
  Foo& operator=(Foo&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Foo(const Foo&) = default;
  Foo& operator=(const Foo&) = default;
  Foo(::crubit::UnsafeRelocateTag, Foo&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=29
  static ::traits::Foo new_(std::int32_t x, std::int32_t y);

 private:
  // Field type has been replaced with a blob of bytes: Tuple types cannot be
  // used inside of compound data types, because std::tuple is not
  // layout-compatible with a Rust tuple.
  unsigned char a[8];

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for `traits_golden::GenericTrait` defined at
// cc_bindings_from_rs/test/traits/traits.rs;l=39:
// Trait is not yet supported

// Generated from:
// cc_bindings_from_rs/test/traits/traits.rs;l=100
struct CRUBIT_INTERNAL_RUST_TYPE(":: traits_golden :: LifetimeStruct") alignas(
    8) [[clang::trivial_abi]] LifetimeStruct final {
 public:
  // `traits_golden::LifetimeStruct` doesn't implement the `Default` trait
  LifetimeStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~LifetimeStruct() = default;
  LifetimeStruct(LifetimeStruct&&) = default;
  LifetimeStruct& operator=(LifetimeStruct&&) = default;

  // `traits_golden::LifetimeStruct` doesn't implement the `Clone` trait
  LifetimeStruct(const LifetimeStruct&) = delete;
  LifetimeStruct& operator=(const LifetimeStruct&) = delete;
  LifetimeStruct(::crubit::UnsafeRelocateTag, LifetimeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  // Field type has been replaced with a blob of bytes: Can't format `&i32`,
  // because references are only supported in function parameter types, return
  // types, and consts (b/286256327)
  unsigned char x[8];

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/traits/traits.rs;l=43
template <typename Type>
struct CRUBIT_INTERNAL_RUST_TYPE(":: traits_golden :: LifetimeTrait")
    LifetimeTrait {
  static constexpr bool is_implemented = false;
};

// Generated from:
// cc_bindings_from_rs/test/traits/traits.rs;l=50
struct CRUBIT_INTERNAL_RUST_TYPE(":: traits_golden :: MyStruct") alignas(4)
    [[clang::trivial_abi]] MyStruct final {
 public:
  // Default::default
  MyStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyStruct() = default;
  MyStruct(MyStruct&&) = default;
  MyStruct& operator=(MyStruct&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  MyStruct(const MyStruct&) = default;
  MyStruct& operator=(const MyStruct&) = default;
  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=55
  static ::traits::MyStruct new_(std::int32_t x);

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/traits/traits.rs;l=51
    std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/traits/traits.rs;l=115
struct CRUBIT_INTERNAL_RUST_TYPE(":: traits_golden :: MyStruct2") alignas(4)
    [[clang::trivial_abi]] MyStruct2 final {
 public:
  // Default::default
  MyStruct2();

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyStruct2() = default;
  MyStruct2(MyStruct2&&) = default;
  MyStruct2& operator=(MyStruct2&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  MyStruct2(const MyStruct2&) = default;
  MyStruct2& operator=(const MyStruct2&) = default;
  MyStruct2(::crubit::UnsafeRelocateTag, MyStruct2&& value) {
    memcpy(this, &value, sizeof(value));
  }

 private:
  union {
    // Generated from:
    // cc_bindings_from_rs/test/traits/traits.rs;l=116
    std::int32_t y;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/traits/traits.rs;l=8
template <typename Type>
struct CRUBIT_INTERNAL_RUST_TYPE(":: traits_golden :: MyTrait") MyTrait {
  static constexpr bool is_implemented = false;
};

static_assert(
    sizeof(Foo) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Foo) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::traits::Foo* __ret_ptr);
}
inline Foo::Foo() { __crubit_internal::__crubit_thunk_default(this); }
static_assert(std::is_trivially_destructible_v<Foo>);
static_assert(std::is_trivially_move_constructible_v<Foo>);
static_assert(std::is_trivially_move_assignable_v<Foo>);
static_assert(std::is_trivially_copy_constructible_v<Foo>);
static_assert(std::is_trivially_copy_assignable_v<Foo>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(std::int32_t, std::int32_t,
                                   ::traits::Foo* __ret_ptr);
}
inline ::traits::Foo Foo::new_(std::int32_t x, std::int32_t y) {
  crubit::Slot<::traits::Foo> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, y, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void Foo::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Foo, a));
}
static_assert(
    sizeof(LifetimeStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(LifetimeStruct) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<LifetimeStruct>);
static_assert(std::is_trivially_move_constructible_v<LifetimeStruct>);
static_assert(std::is_trivially_move_assignable_v<LifetimeStruct>);
inline void LifetimeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(LifetimeStruct, x));
}
static_assert(
    sizeof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::traits::MyStruct* __ret_ptr);
}
inline MyStruct::MyStruct() { __crubit_internal::__crubit_thunk_default(this); }
static_assert(std::is_trivially_destructible_v<MyStruct>);
static_assert(std::is_trivially_move_constructible_v<MyStruct>);
static_assert(std::is_trivially_move_assignable_v<MyStruct>);
static_assert(std::is_trivially_copy_constructible_v<MyStruct>);
static_assert(std::is_trivially_copy_assignable_v<MyStruct>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_new(std::int32_t, ::traits::MyStruct* __ret_ptr);
}
inline ::traits::MyStruct MyStruct::new_(std::int32_t x) {
  crubit::Slot<::traits::MyStruct> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void MyStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyStruct, x));
}
static_assert(
    sizeof(MyStruct2) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyStruct2) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::traits::MyStruct2* __ret_ptr);
}
inline MyStruct2::MyStruct2() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<MyStruct2>);
static_assert(std::is_trivially_move_constructible_v<MyStruct2>);
static_assert(std::is_trivially_move_assignable_v<MyStruct2>);
static_assert(std::is_trivially_copy_constructible_v<MyStruct2>);
static_assert(std::is_trivially_copy_assignable_v<MyStruct2>);
inline void MyStruct2::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyStruct2, y));
}
}  // namespace traits

template <>
struct ::traits::MyTrait<::traits::MyStruct> {
  static constexpr bool is_implemented = true;

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=61
  static std::int32_t do_something(::traits::MyStruct const& self);

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=65
  static std::int32_t consume_self(::traits::MyStruct self);

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=69
  static ::traits::MyStruct const& $(__anon1)
      return_self(::traits::MyStruct const& self);

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=73
  static std::int32_t no_self();

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=77
  static std::tuple<std::int32_t, std::int32_t> take_and_return_other_types(
      ::traits::MyStruct const& self, ::traits::Foo x);
};

template <>
struct ::traits::MyTrait<::traits::MyStruct2> {
  static constexpr bool is_implemented = true;

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=120
  static std::int32_t do_something(::traits::MyStruct2 const& self);

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=124
  static std::int32_t consume_self(::traits::MyStruct2 self);

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=128
  static ::traits::MyStruct2 const& $(__anon1)
      return_self(::traits::MyStruct2 const& self);

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=132
  static std::int32_t no_self();

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=136
  static std::tuple<std::int32_t, std::int32_t> take_and_return_other_types(
      ::traits::MyStruct2 const& self, ::traits::Foo x);
};

template <>
struct ::traits::DifferentTraitSameName<::traits::MyStruct> {
  static constexpr bool is_implemented = true;

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=83
  static std::int32_t do_something(::traits::MyStruct const& self);
};

template <>
struct ::traits::LifetimeTrait<::traits::LifetimeStruct> {
  static constexpr bool is_implemented = true;

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=105
  static std::int32_t const& $a
  trait_do_something(::traits::LifetimeStruct const& self);

  // Generated from:
  // cc_bindings_from_rs/test/traits/traits.rs;l=109
  static std::int32_t const& $(__anon1)
      function_do_something(::traits::LifetimeStruct const& self);
};

namespace traits {
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_MyTrait_udo_usomething(
    ::traits::MyStruct const&);
}
}  // namespace traits
inline std::int32_t(::traits::MyTrait<::traits::MyStruct>::do_something)(
    ::traits::MyStruct const& self) {
  return traits::__crubit_internal::__crubit_thunk_MyTrait_udo_usomething(self);
}

namespace traits {
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_MyTrait_uconsume_uself(
    ::traits::MyStruct*);
}
}  // namespace traits
inline std::int32_t(::traits::MyTrait<::traits::MyStruct>::consume_self)(
    ::traits::MyStruct self) {
  return traits::__crubit_internal::__crubit_thunk_MyTrait_uconsume_uself(
      &self);
}

namespace traits {
namespace __crubit_internal {
extern "C" ::traits::MyStruct const& $(__anon1)
    __crubit_thunk_MyTrait_ureturn_uself(::traits::MyStruct const&);
}
}  // namespace traits
inline ::traits::MyStruct const& $(__anon1)(
    ::traits::MyTrait<::traits::MyStruct>::return_self)(
    ::traits::MyStruct const& self) {
  return traits::__crubit_internal::__crubit_thunk_MyTrait_ureturn_uself(self);
}

namespace traits {
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_MyTrait_uno_uself();
}
}  // namespace traits
inline std::int32_t(::traits::MyTrait<::traits::MyStruct>::no_self)() {
  return traits::__crubit_internal::__crubit_thunk_MyTrait_uno_uself();
}

namespace traits {
namespace __crubit_internal {
extern "C" void __crubit_thunk_MyTrait_utake_uand_ureturn_uother_utypes(
    ::traits::MyStruct const&, ::traits::Foo*, void** __ret_ptr);
}
}  // namespace traits
inline std::tuple<std::int32_t, std::int32_t>(
    ::traits::MyTrait<::traits::MyStruct>::take_and_return_other_types)(
    ::traits::MyStruct const& self, ::traits::Foo x) {
  std::int32_t __return_value_0_ret_val_holder;
  std::int32_t* __return_value_0_storage = &__return_value_0_ret_val_holder;
  std::int32_t __return_value_1_ret_val_holder;
  std::int32_t* __return_value_1_storage = &__return_value_1_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage};
  traits::__crubit_internal::
      __crubit_thunk_MyTrait_utake_uand_ureturn_uother_utypes(
          self, &x, __return_value_storage);
  return std::make_tuple(*__return_value_0_storage, *__return_value_1_storage);
}

namespace traits {
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_MyTrait_udo_usomething(
    ::traits::MyStruct2 const&);
}
}  // namespace traits
inline std::int32_t(::traits::MyTrait<::traits::MyStruct2>::do_something)(
    ::traits::MyStruct2 const& self) {
  return traits::__crubit_internal::__crubit_thunk_MyTrait_udo_usomething(self);
}

namespace traits {
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_MyTrait_uconsume_uself(
    ::traits::MyStruct2*);
}
}  // namespace traits
inline std::int32_t(::traits::MyTrait<::traits::MyStruct2>::consume_self)(
    ::traits::MyStruct2 self) {
  return traits::__crubit_internal::__crubit_thunk_MyTrait_uconsume_uself(
      &self);
}

namespace traits {
namespace __crubit_internal {
extern "C" ::traits::MyStruct2 const& $(__anon1)
    __crubit_thunk_MyTrait_ureturn_uself(::traits::MyStruct2 const&);
}
}  // namespace traits
inline ::traits::MyStruct2 const& $(__anon1)(
    ::traits::MyTrait<::traits::MyStruct2>::return_self)(
    ::traits::MyStruct2 const& self) {
  return traits::__crubit_internal::__crubit_thunk_MyTrait_ureturn_uself(self);
}

namespace traits {
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_MyTrait_uno_uself();
}
}  // namespace traits
inline std::int32_t(::traits::MyTrait<::traits::MyStruct2>::no_self)() {
  return traits::__crubit_internal::__crubit_thunk_MyTrait_uno_uself();
}

namespace traits {
namespace __crubit_internal {
extern "C" void __crubit_thunk_MyTrait_utake_uand_ureturn_uother_utypes(
    ::traits::MyStruct2 const&, ::traits::Foo*, void** __ret_ptr);
}
}  // namespace traits
inline std::tuple<std::int32_t, std::int32_t>(
    ::traits::MyTrait<::traits::MyStruct2>::take_and_return_other_types)(
    ::traits::MyStruct2 const& self, ::traits::Foo x) {
  std::int32_t __return_value_0_ret_val_holder;
  std::int32_t* __return_value_0_storage = &__return_value_0_ret_val_holder;
  std::int32_t __return_value_1_ret_val_holder;
  std::int32_t* __return_value_1_storage = &__return_value_1_ret_val_holder;
  void* __return_value_storage[] = {__return_value_0_storage,
                                    __return_value_1_storage};
  traits::__crubit_internal::
      __crubit_thunk_MyTrait_utake_uand_ureturn_uother_utypes(
          self, &x, __return_value_storage);
  return std::make_tuple(*__return_value_0_storage, *__return_value_1_storage);
}

namespace traits {
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_DifferentTraitSameName_udo_usomething(
    ::traits::MyStruct const&);
}
}  // namespace traits
inline std::int32_t(
    ::traits::DifferentTraitSameName<::traits::MyStruct>::do_something)(
    ::traits::MyStruct const& self) {
  return traits::__crubit_internal::
      __crubit_thunk_DifferentTraitSameName_udo_usomething(self);
}

namespace traits {
namespace __crubit_internal {
extern "C" std::int32_t const& $a
__crubit_thunk_LifetimeTrait_utrait_udo_usomething(
    ::traits::LifetimeStruct const&);
}
}  // namespace traits
inline std::int32_t const& $a(
    ::traits::LifetimeTrait<::traits::LifetimeStruct>::trait_do_something)(
    ::traits::LifetimeStruct const& self) {
  return traits::__crubit_internal::
      __crubit_thunk_LifetimeTrait_utrait_udo_usomething(self);
}

namespace traits {
namespace __crubit_internal {
extern "C" std::int32_t const& $(__anon1)
    __crubit_thunk_LifetimeTrait_ufunction_udo_usomething(
        ::traits::LifetimeStruct const&);
}
}  // namespace traits
inline std::int32_t const& $(__anon1)(
    ::traits::LifetimeTrait<::traits::LifetimeStruct>::function_do_something)(
    ::traits::LifetimeStruct const& self) {
  return traits::__crubit_internal::
      __crubit_thunk_LifetimeTrait_ufunction_udo_usomething(self);
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_TRAITS_GOLDEN
