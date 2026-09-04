// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// callables_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_CALLABLES_CALLABLES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_CALLABLES_CALLABLES_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/fn.h"
#include "support/rs_std/fn_ref.h"
#include "support/rs_std/option.h"
#include "support/rs_std/str_ref.h"
#include "support/rs_std/traits.h"
#include "support/rs_std/tuple.h"

#include <array>
#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <memory>
#include <new>
#include <optional>
#include <tuple>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_core.h"

namespace callables {

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: callables_golden :: CallbackHolder") alignas(4) [[clang::trivial_abi]]
CallbackHolder final {
 public:
  // Default::default
  CallbackHolder();

  // Drop::drop
  ~CallbackHolder();

  CallbackHolder(CallbackHolder&&);
  ::callables::CallbackHolder& operator=(CallbackHolder&&);

  // `callables_golden::CallbackHolder` doesn't implement the `Clone` trait
  CallbackHolder(const CallbackHolder&) = delete;
  CallbackHolder& operator=(const CallbackHolder&) = delete;
  CallbackHolder(::crubit::UnsafeRelocateTag, CallbackHolder&& value);

  // CRUBIT_ANNOTATE: must_bind=
  static ::callables::CallbackHolder new_();

  // CRUBIT_ANNOTATE: must_bind=
  void set_callback(::rs::Fn<void() const> f);

  // CRUBIT_ANNOTATE: must_bind=
  void call() const;

  // CRUBIT_ANNOTATE: must_bind=
  void drop_callback();

 private:
  // Field type has been replaced with a blob of bytes: Callable types are only
  // supported in function parameter position
  ::std::array<unsigned char, 8> cb;

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: callables_golden :: CppMovableDrop") alignas(4) [[clang::trivial_abi]]
CppMovableDrop final {
 public:
  // Type is not a C++ aggregate: Type implements `Drop`

  // Default::default
  CppMovableDrop();

  // Synthesized tuple constructor
  explicit CppMovableDrop(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // Drop::drop
  ~CppMovableDrop();

  CppMovableDrop(CppMovableDrop&&);
  ::callables::CppMovableDrop& operator=(CppMovableDrop&&);

  // `callables_golden::CppMovableDrop` doesn't implement the `Clone` trait
  CppMovableDrop(const CppMovableDrop&) = delete;
  CppMovableDrop& operator=(const CppMovableDrop&) = delete;
  CppMovableDrop(::crubit::UnsafeRelocateTag, CppMovableDrop&& value);

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: callables_golden :: NonCppMovable") alignas(4) [[clang::trivial_abi]]
NonCppMovable final {
 public:
  // Type is not a C++ aggregate: Type implements `Drop`

  // `callables_golden::NonCppMovable` doesn't implement the `Default` trait
  NonCppMovable() = delete;

  // Synthesized tuple constructor
  explicit NonCppMovable(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // Drop::drop
  ~NonCppMovable();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  NonCppMovable(NonCppMovable&&) = delete;
  ::callables::NonCppMovable& operator=(NonCppMovable&&) = delete;
  // `callables_golden::NonCppMovable` doesn't implement the `Clone` trait
  NonCppMovable(const NonCppMovable&) = delete;
  NonCppMovable& operator=(const NonCppMovable&) = delete;
  NonCppMovable(::crubit::UnsafeRelocateTag, NonCppMovable&& value);

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: callables_golden :: Point") alignas(4)
    [[clang::trivial_abi]] Point final {
 public:
  bool operator==(::callables::Point const& other) const;

  ::std::int32_t x = {};
  ::std::int32_t y = {};

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
::callables::NonCppMovable call_and_return_non_movable(
    ::rs::FnRef<::callables::NonCppMovable() const> f);

// CRUBIT_ANNOTATE: must_bind=
::callables::NonCppMovable call_and_return_non_movable_box_fn(
    ::rs::Fn<::callables::NonCppMovable() const> f);

// CRUBIT_ANNOTATE: must_bind=
::callables::NonCppMovable call_and_return_non_movable_boxed(
    ::rs::Fn<::callables::NonCppMovable() &&> f);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_box_dyn_fn(::rs::Fn<::std::int32_t(::std::int32_t) const> f,
                               ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_box_dyn_fn_mut(::rs::Fn<::std::int32_t(::std::int32_t)> f,
                                   ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_box_dyn_fn_once(
    ::rs::Fn<::std::int32_t(::std::int32_t) &&> f, ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_dyn_fn(::rs::FnRef<::std::int32_t(::std::int32_t) const> f,
                           ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_dyn_fn_mut(::rs::FnRef<::std::int32_t(::std::int32_t)> f,
                               ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_impl_fn(::rs::FnRef<::std::int32_t(::std::int32_t) const> f,
                            ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_impl_fn_mut(::rs::FnRef<::std::int32_t(::std::int32_t)> f,
                                ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_impl_fn_once(::rs::FnRef<::std::int32_t(::std::int32_t)> f,
                                 ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_impl_fn_once_static(
    ::rs::Fn<::std::int32_t(::std::int32_t) &&> f, ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_impl_with_tuple_option(
    ::rs::Fn<rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>(
        rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>) &&>
        f);

// CRUBIT_ANNOTATE: must_bind=
::callables::Point call_int_to_point(
    ::rs::FnRef<::callables::Point(::std::int32_t) const> f, ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
::callables::Point call_point_mut(
    ::rs::FnRef<::callables::Point(::callables::Point)> f,
    ::callables::Point pt);

// CRUBIT_ANNOTATE: must_bind=
::callables::Point call_point_once_static(
    ::rs::Fn<::callables::Point(::callables::Point) &&> f,
    ::callables::Point pt);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_point_to_int(
    ::rs::FnRef<::std::int32_t(::callables::Point) const> f,
    ::callables::Point pt);

// CRUBIT_ANNOTATE: must_bind=
void call_point_void(::rs::FnRef<void(::callables::Point) const> f,
                     ::callables::Point pt);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_two_args(
    ::rs::FnRef<::std::int32_t(::std::int32_t, ::std::int32_t) const> f,
    ::std::int32_t a, ::std::int32_t b);

// CRUBIT_ANNOTATE: must_bind=
::callables::Point call_two_points(
    ::rs::FnRef<::callables::Point(::callables::Point, ::callables::Point)
                    const>
        f,
    ::callables::Point a, ::callables::Point b);

// CRUBIT_ANNOTATE: must_bind=
void call_void(::rs::FnRef<void(::std::int32_t) const> f, ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
void call_void_mut(::rs::FnRef<void(::std::int32_t)> f, ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
void call_void_once(::rs::FnRef<void(::std::int32_t)> f, ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_with_hrtb_str(
    ::rs::FnRef<::std::int32_t(rs_std::StrRef) const> f, rs_std::StrRef s);

// CRUBIT_ANNOTATE: must_bind=
::std::uintptr_t call_with_hrtb_str_to_str(
    ::rs::FnRef<rs_std::StrRef(rs_std::StrRef) const> f, rs_std::StrRef s);

// CRUBIT_ANNOTATE: must_bind=
void call_with_movable_drop(
    ::rs::FnRef<void(::callables::CppMovableDrop) const> f, ::std::int32_t x);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_with_non_movable_ref(
    ::rs::FnRef<void(::callables::NonCppMovable const* $static crubit_nonnull)
                    const>
        f,
    ::callables::NonCppMovable const& x);

// CRUBIT_ANNOTATE: must_bind=
::callables::Point call_with_point(
    ::rs::FnRef<::callables::Point(::callables::Point) const> f,
    ::callables::Point pt);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_with_str(
    ::rs::FnRef<::std::int32_t(rs_std::StrRef) const> f, rs_std::StrRef s);

// CRUBIT_ANNOTATE: must_bind=
rs_std::StrRef call_with_str_to_str(
    ::rs::FnRef<rs_std::StrRef(rs_std::StrRef) const> f, rs_std::StrRef s);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t call_with_tuple_option(
    ::rs::Fn<rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>(
        rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>) &&>
        f);

}  // namespace callables

template <>
struct rs_std::impl<::callables::Point, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::callables::Point, ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function `<callables_golden::Point
  // as std::fmt::Debug>::fmt` defined at
  // cc_bindings_from_rs/test/callables/callables.rs;l=74:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Generic types are not supported yet (b/259749095)
};
#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < i32 >") rs_std::Option<::std::int32_t>
    : public rs_std::OptionBase<rs_std::Option<::std::int32_t>,
                                ::std::int32_t> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<::std::int32_t>, ::std::int32_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, ::std::int32_t, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option, ::std::int32_t, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::int32_t, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::std::int32_t, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = 0;
  ::std::int32_t* some_ptr() noexcept {
    return reinterpret_cast<::std::int32_t*>(storage_ + 4);
  }
  ::std::int32_t const* some_const_ptr() const noexcept {
    return reinterpret_cast<::std::int32_t const*>(storage_ + 4);
  }
  void set_some_tag() noexcept { set_tag(1); }
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char storage_[8];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("(i32 , :: core :: option :: Option < i32 > ,)")
        rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>> {
 public:
  // Default::default
  Tuple();

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Tuple(const Tuple&) = default;
  Tuple& operator=(const Tuple&) = default;
  Tuple(Tuple&&) = default;
  Tuple& operator=(Tuple&&) = default;

  Tuple(::crubit::UnsafeRelocateTag, Tuple&& value);
  Tuple(std::tuple<::std::int32_t, rs_std::Option<::std::int32_t>>&&
            tuple) noexcept;
  ~Tuple() = default;
  operator std::tuple<::std::int32_t,
                      rs_std::Option<::std::int32_t>>() && noexcept;
  template <std::size_t I>
  constexpr decltype(auto) get() & noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return (this->__field0);
    } else if constexpr (I == 1) {
      return (this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() && noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  template <std::size_t I>
  constexpr decltype(auto) get() const&& noexcept {
    static_assert(I < 2, "Tuple index out of bounds");
    if constexpr (I == 0) {
      return std::move(this->__field0);
    } else if constexpr (I == 1) {
      return std::move(this->__field1);
    } else {
      CRUBIT_UNREACHABLE();
    }
  }
  union {
    ::std::int32_t __field0;
  };
  union {
    rs_std::Option<::std::int32_t> __field1;
  };

 private:
  static void __crubit_field_offset_assertions();
};
#endif

namespace callables {

static_assert(
    sizeof(CallbackHolder) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CallbackHolder) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_ucallables_ugolden_x0000003a_x0000003aCallbackHolder(
    ::callables::CallbackHolder* __ret_ptr);
}
inline ::callables::CallbackHolder::CallbackHolder() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_ucallables_ugolden_x0000003a_x0000003aCallbackHolder(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_ucallables_ugolden_x0000003a_x0000003aCallbackHolder(
    ::callables::CallbackHolder&);
}
inline CallbackHolder::~CallbackHolder() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_ucallables_ugolden_x0000003a_x0000003aCallbackHolder(
          *this);
}
inline ::callables::CallbackHolder::CallbackHolder(CallbackHolder&& other)
    : CallbackHolder() {
  *this = ::std::move(other);
}
inline ::callables::CallbackHolder& ::callables::CallbackHolder::operator=(
    CallbackHolder&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::callables::CallbackHolder::CallbackHolder(::crubit::UnsafeRelocateTag,
                                                   CallbackHolder&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::callables::CallbackHolder* __ret_ptr);
}
inline ::callables::CallbackHolder CallbackHolder::new_() {
  crubit::Slot<::callables::CallbackHolder> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_set_ucallback(::callables::CallbackHolder&,
                                             ::rs::internal::FnPayload);
}
inline void CallbackHolder::set_callback(::rs::Fn<void() const> f) {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_set_ucallback(
      self, ::std::move(f).release_payload());
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call(::callables::CallbackHolder const&);
}
inline void CallbackHolder::call() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_call(self);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_drop_ucallback(::callables::CallbackHolder&);
}
inline void CallbackHolder::drop_callback() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_drop_ucallback(self);
}
inline void CallbackHolder::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CallbackHolder, cb));
}
static_assert(
    sizeof(CppMovableDrop) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(CppMovableDrop) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_ucallables_ugolden_x0000003a_x0000003aCppMovableDrop(
    ::callables::CppMovableDrop* __ret_ptr);
}
inline ::callables::CppMovableDrop::CppMovableDrop() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_ucallables_ugolden_x0000003a_x0000003aCppMovableDrop(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_ucallables_ugolden_x0000003a_x0000003aCppMovableDrop(
    ::callables::CppMovableDrop&);
}
inline CppMovableDrop::~CppMovableDrop() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_ucallables_ugolden_x0000003a_x0000003aCppMovableDrop(
          *this);
}
inline ::callables::CppMovableDrop::CppMovableDrop(CppMovableDrop&& other)
    : CppMovableDrop() {
  *this = ::std::move(other);
}
inline ::callables::CppMovableDrop& ::callables::CppMovableDrop::operator=(
    CppMovableDrop&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::callables::CppMovableDrop::CppMovableDrop(::crubit::UnsafeRelocateTag,
                                                   CppMovableDrop&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void CppMovableDrop::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(CppMovableDrop, __field0));
}
static_assert(
    sizeof(NonCppMovable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonCppMovable) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_ucallables_ugolden_x0000003a_x0000003aNonCppMovable(
    ::callables::NonCppMovable&);
}
inline NonCppMovable::~NonCppMovable() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_ucallables_ugolden_x0000003a_x0000003aNonCppMovable(
          *this);
}
inline ::callables::NonCppMovable::NonCppMovable(::crubit::UnsafeRelocateTag,
                                                 NonCppMovable&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void NonCppMovable::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NonCppMovable, __field0));
}
static_assert(
    sizeof(Point) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Point) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<Point>);
static_assert(::std::is_trivially_move_constructible_v<::callables::Point>);
static_assert(::std::is_trivially_move_assignable_v<::callables::Point>);
static_assert(::std::is_trivially_copy_constructible_v<::callables::Point>);
static_assert(::std::is_trivially_copy_assignable_v<::callables::Point>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_ucallables_ugolden_x0000003a_x0000003aPoint_ucallables_ugolden_x0000003a_x0000003aPoint(
    ::callables::Point const&, ::callables::Point const&);
}
inline bool Point::operator==(::callables::Point const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_ucallables_ugolden_x0000003a_x0000003aPoint_ucallables_ugolden_x0000003a_x0000003aPoint(
          self, other);
}
inline void Point::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Point, x));
  static_assert(4 == offsetof(Point, y));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_call_uand_ureturn_unon_umovable(
    ::rs::internal::FnRefPayload, ::callables::NonCppMovable* __ret_ptr);
}
inline ::callables::NonCppMovable call_and_return_non_movable(
    ::rs::FnRef<::callables::NonCppMovable() const> f) {
  auto __f_invoker = [](void* __data, void* __ret_ptr) -> void {
    new (__ret_ptr)::callables::NonCppMovable(
        (*reinterpret_cast<::rs::FnRef<::callables::NonCppMovable() const>*>(
            __data))());
  };
  ::rs::internal::FnRefPayload __f_payload{
      const_cast<void*>(reinterpret_cast<const void*>(&f)),
      reinterpret_cast<void (*)()>(+__f_invoker),
  };
  crubit::Slot<::callables::NonCppMovable> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_call_uand_ureturn_unon_umovable(
      __f_payload, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call_uand_ureturn_unon_umovable_ubox_ufn(
    ::rs::internal::FnPayload, ::callables::NonCppMovable* __ret_ptr);
}
inline ::callables::NonCppMovable call_and_return_non_movable_box_fn(
    ::rs::Fn<::callables::NonCppMovable() const> f) {
  auto* __f_heap_fn =
      new ::rs::Fn<::callables::NonCppMovable() const>(::std::move(f));
  auto __f_invoker = [](void* __data, void* __ret_ptr) -> void {
    new (__ret_ptr)::callables::NonCppMovable(
        (*reinterpret_cast<::rs::Fn<::callables::NonCppMovable() const>*>(
            __data))());
  };
  auto __f_destroyer = [](void* __data) noexcept {
    delete reinterpret_cast<::rs::Fn<::callables::NonCppMovable() const>*>(
        __data);
  };
  ::rs::internal::FnPayload __f_payload{
      __f_heap_fn,
      reinterpret_cast<void (*)()>(+__f_invoker),
      +__f_destroyer,
  };
  crubit::Slot<::callables::NonCppMovable> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_call_uand_ureturn_unon_umovable_ubox_ufn(
      __f_payload, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call_uand_ureturn_unon_umovable_uboxed(
    ::rs::internal::FnPayload, ::callables::NonCppMovable* __ret_ptr);
}
inline ::callables::NonCppMovable call_and_return_non_movable_boxed(
    ::rs::Fn<::callables::NonCppMovable() &&> f) {
  auto* __f_heap_fn =
      new ::rs::Fn<::callables::NonCppMovable() &&>(::std::move(f));
  auto __f_invoker = [](void* __data, void* __ret_ptr) -> void {
    new (__ret_ptr)::callables::NonCppMovable(::std::move(
        *reinterpret_cast<::rs::Fn<::callables::NonCppMovable() &&>*>(
            __data))());
  };
  auto __f_destroyer = [](void* __data) noexcept {
    delete reinterpret_cast<::rs::Fn<::callables::NonCppMovable() &&>*>(__data);
  };
  ::rs::internal::FnPayload __f_payload{
      __f_heap_fn,
      reinterpret_cast<void (*)()>(+__f_invoker),
      +__f_destroyer,
  };
  crubit::Slot<::callables::NonCppMovable> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_call_uand_ureturn_unon_umovable_uboxed(
      __f_payload, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_ubox_udyn_ufn(
    ::rs::internal::FnPayload, ::std::int32_t);
}
inline ::std::int32_t call_box_dyn_fn(
    ::rs::Fn<::std::int32_t(::std::int32_t) const> f, ::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_call_ubox_udyn_ufn(
      ::std::move(f).release_payload(), x);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_ubox_udyn_ufn_umut(
    ::rs::internal::FnPayload, ::std::int32_t);
}
inline ::std::int32_t call_box_dyn_fn_mut(
    ::rs::Fn<::std::int32_t(::std::int32_t)> f, ::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_call_ubox_udyn_ufn_umut(
      ::std::move(f).release_payload(), x);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_ubox_udyn_ufn_uonce(
    ::rs::internal::FnPayload, ::std::int32_t);
}
inline ::std::int32_t call_box_dyn_fn_once(
    ::rs::Fn<::std::int32_t(::std::int32_t) &&> f, ::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_call_ubox_udyn_ufn_uonce(
      ::std::move(f).release_payload(), x);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_udyn_ufn(
    ::rs::internal::FnRefPayload, ::std::int32_t);
}
inline ::std::int32_t call_dyn_fn(
    ::rs::FnRef<::std::int32_t(::std::int32_t) const> f, ::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_call_udyn_ufn(f.payload(), x);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_udyn_ufn_umut(
    ::rs::internal::FnRefPayload, ::std::int32_t);
}
inline ::std::int32_t call_dyn_fn_mut(
    ::rs::FnRef<::std::int32_t(::std::int32_t)> f, ::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_call_udyn_ufn_umut(f.payload(), x);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_uimpl_ufn(
    ::rs::internal::FnRefPayload, ::std::int32_t);
}
inline ::std::int32_t call_impl_fn(
    ::rs::FnRef<::std::int32_t(::std::int32_t) const> f, ::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_call_uimpl_ufn(f.payload(), x);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_uimpl_ufn_umut(
    ::rs::internal::FnRefPayload, ::std::int32_t);
}
inline ::std::int32_t call_impl_fn_mut(
    ::rs::FnRef<::std::int32_t(::std::int32_t)> f, ::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_call_uimpl_ufn_umut(f.payload(), x);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_uimpl_ufn_uonce(
    ::rs::internal::FnRefPayload, ::std::int32_t);
}
inline ::std::int32_t call_impl_fn_once(
    ::rs::FnRef<::std::int32_t(::std::int32_t)> f, ::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_call_uimpl_ufn_uonce(f.payload(), x);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_uimpl_ufn_uonce_ustatic(
    ::rs::internal::FnPayload, ::std::int32_t);
}
inline ::std::int32_t call_impl_fn_once_static(
    ::rs::Fn<::std::int32_t(::std::int32_t) &&> f, ::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_call_uimpl_ufn_uonce_ustatic(
      ::std::move(f).release_payload(), x);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_uimpl_uwith_utuple_uoption(
    ::rs::internal::FnPayload);
}
inline ::std::int32_t call_impl_with_tuple_option(
    ::rs::Fn<rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>(
        rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>) &&>
        f) {
  auto* __f_heap_fn = new ::rs::Fn<
      rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>(
          rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>) &&>(
      ::std::move(f));
  auto __f_invoker =
      [](void* __data,
         rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>* __arg_0,
         void* __ret_ptr) -> void {
    new (__ret_ptr) rs_std::Tuple<::std::int32_t,
                                  rs_std::Option<::std::int32_t>>(::std::move(
        *reinterpret_cast<::rs::Fn<
            rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>(
                rs_std::Tuple<::std::int32_t,
                              rs_std::Option<::std::int32_t>>) &&>*>(__data))(
        ::std::move(*__arg_0)));
  };
  auto __f_destroyer = [](void* __data) noexcept {
    delete reinterpret_cast<::rs::Fn<rs_std::Tuple<
        ::std::int32_t, rs_std::Option<::std::int32_t>>(
        rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>) &&>*>(
        __data);
  };
  ::rs::internal::FnPayload __f_payload{
      __f_heap_fn,
      reinterpret_cast<void (*)()>(+__f_invoker),
      +__f_destroyer,
  };
  return __crubit_internal::__crubit_thunk_call_uimpl_uwith_utuple_uoption(
      __f_payload);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call_uint_uto_upoint(
    ::rs::internal::FnRefPayload, ::std::int32_t,
    ::callables::Point* __ret_ptr);
}
inline ::callables::Point call_int_to_point(
    ::rs::FnRef<::callables::Point(::std::int32_t) const> f, ::std::int32_t x) {
  auto __f_invoker = [](void* __data, ::std::int32_t __arg_0,
                        void* __ret_ptr) -> void {
    new (__ret_ptr)::callables::Point(
        (*reinterpret_cast<
            ::rs::FnRef<::callables::Point(::std::int32_t) const>*>(__data))(
            ::std::move(__arg_0)));
  };
  ::rs::internal::FnRefPayload __f_payload{
      const_cast<void*>(reinterpret_cast<const void*>(&f)),
      reinterpret_cast<void (*)()>(+__f_invoker),
  };
  crubit::Slot<::callables::Point> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_call_uint_uto_upoint(
      __f_payload, x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call_upoint_umut(::rs::internal::FnRefPayload,
                                                ::callables::Point*,
                                                ::callables::Point* __ret_ptr);
}
inline ::callables::Point call_point_mut(
    ::rs::FnRef<::callables::Point(::callables::Point)> f,
    ::callables::Point pt) {
  auto __f_invoker = [](void* __data, ::callables::Point* __arg_0,
                        void* __ret_ptr) -> void {
    new (__ret_ptr)::callables::Point((
        *reinterpret_cast<::rs::FnRef<::callables::Point(::callables::Point)>*>(
            __data))(::std::move(*__arg_0)));
  };
  ::rs::internal::FnRefPayload __f_payload{
      const_cast<void*>(reinterpret_cast<const void*>(&f)),
      reinterpret_cast<void (*)()>(+__f_invoker),
  };
  crubit::Slot<::callables::Point> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_call_upoint_umut(__f_payload, &pt,
                                                     __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call_upoint_uonce_ustatic(
    ::rs::internal::FnPayload, ::callables::Point*,
    ::callables::Point* __ret_ptr);
}
inline ::callables::Point call_point_once_static(
    ::rs::Fn<::callables::Point(::callables::Point) &&> f,
    ::callables::Point pt) {
  auto* __f_heap_fn =
      new ::rs::Fn<::callables::Point(::callables::Point) &&>(::std::move(f));
  auto __f_invoker = [](void* __data, ::callables::Point* __arg_0,
                        void* __ret_ptr) -> void {
    new (__ret_ptr)::callables::Point(::std::move(
        *reinterpret_cast<::rs::Fn<::callables::Point(::callables::Point) &&>*>(
            __data))(::std::move(*__arg_0)));
  };
  auto __f_destroyer = [](void* __data) noexcept {
    delete reinterpret_cast<
        ::rs::Fn<::callables::Point(::callables::Point) &&>*>(__data);
  };
  ::rs::internal::FnPayload __f_payload{
      __f_heap_fn,
      reinterpret_cast<void (*)()>(+__f_invoker),
      +__f_destroyer,
  };
  crubit::Slot<::callables::Point> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_call_upoint_uonce_ustatic(
      __f_payload, &pt, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_upoint_uto_uint(
    ::rs::internal::FnRefPayload, ::callables::Point*);
}
inline ::std::int32_t call_point_to_int(
    ::rs::FnRef<::std::int32_t(::callables::Point) const> f,
    ::callables::Point pt) {
  auto __f_invoker = [](void* __data,
                        ::callables::Point* __arg_0) -> ::std::int32_t {
    return (*reinterpret_cast<
            ::rs::FnRef<::std::int32_t(::callables::Point) const>*>(__data))(
        ::std::move(*__arg_0));
  };
  ::rs::internal::FnRefPayload __f_payload{
      const_cast<void*>(reinterpret_cast<const void*>(&f)),
      reinterpret_cast<void (*)()>(+__f_invoker),
  };
  return __crubit_internal::__crubit_thunk_call_upoint_uto_uint(__f_payload,
                                                                &pt);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call_upoint_uvoid(::rs::internal::FnRefPayload,
                                                 ::callables::Point*);
}
inline void call_point_void(::rs::FnRef<void(::callables::Point) const> f,
                            ::callables::Point pt) {
  auto __f_invoker = [](void* __data, ::callables::Point* __arg_0) -> void {
    (*reinterpret_cast<::rs::FnRef<void(::callables::Point) const>*>(__data))(
        ::std::move(*__arg_0));
  };
  ::rs::internal::FnRefPayload __f_payload{
      const_cast<void*>(reinterpret_cast<const void*>(&f)),
      reinterpret_cast<void (*)()>(+__f_invoker),
  };
  return __crubit_internal::__crubit_thunk_call_upoint_uvoid(__f_payload, &pt);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_utwo_uargs(
    ::rs::internal::FnRefPayload, ::std::int32_t, ::std::int32_t);
}
inline ::std::int32_t call_two_args(
    ::rs::FnRef<::std::int32_t(::std::int32_t, ::std::int32_t) const> f,
    ::std::int32_t a, ::std::int32_t b) {
  return __crubit_internal::__crubit_thunk_call_utwo_uargs(f.payload(), a, b);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call_utwo_upoints(::rs::internal::FnRefPayload,
                                                 ::callables::Point*,
                                                 ::callables::Point*,
                                                 ::callables::Point* __ret_ptr);
}
inline ::callables::Point call_two_points(
    ::rs::FnRef<::callables::Point(::callables::Point, ::callables::Point)
                    const>
        f,
    ::callables::Point a, ::callables::Point b) {
  auto __f_invoker = [](void* __data, ::callables::Point* __arg_0,
                        ::callables::Point* __arg_1, void* __ret_ptr) -> void {
    new (__ret_ptr)::callables::Point(
        (*reinterpret_cast<::rs::FnRef<::callables::Point(
             ::callables::Point, ::callables::Point) const>*>(__data))(
            ::std::move(*__arg_0), ::std::move(*__arg_1)));
  };
  ::rs::internal::FnRefPayload __f_payload{
      const_cast<void*>(reinterpret_cast<const void*>(&f)),
      reinterpret_cast<void (*)()>(+__f_invoker),
  };
  crubit::Slot<::callables::Point> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_call_utwo_upoints(__f_payload, &a, &b,
                                                      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call_uvoid(::rs::internal::FnRefPayload,
                                          ::std::int32_t);
}
inline void call_void(::rs::FnRef<void(::std::int32_t) const> f,
                      ::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_call_uvoid(f.payload(), x);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call_uvoid_umut(::rs::internal::FnRefPayload,
                                               ::std::int32_t);
}
inline void call_void_mut(::rs::FnRef<void(::std::int32_t)> f,
                          ::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_call_uvoid_umut(f.payload(), x);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call_uvoid_uonce(::rs::internal::FnRefPayload,
                                                ::std::int32_t);
}
inline void call_void_once(::rs::FnRef<void(::std::int32_t)> f,
                           ::std::int32_t x) {
  return __crubit_internal::__crubit_thunk_call_uvoid_uonce(f.payload(), x);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_uwith_uhrtb_ustr(
    ::rs::internal::FnRefPayload, rs_std::StrRef);
}
inline ::std::int32_t call_with_hrtb_str(
    ::rs::FnRef<::std::int32_t(rs_std::StrRef) const> f, rs_std::StrRef s) {
  return __crubit_internal::__crubit_thunk_call_uwith_uhrtb_ustr(f.payload(),
                                                                 s);
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_call_uwith_uhrtb_ustr_uto_ustr(
    ::rs::internal::FnRefPayload, rs_std::StrRef);
}
inline ::std::uintptr_t call_with_hrtb_str_to_str(
    ::rs::FnRef<rs_std::StrRef(rs_std::StrRef) const> f, rs_std::StrRef s) {
  return __crubit_internal::__crubit_thunk_call_uwith_uhrtb_ustr_uto_ustr(
      f.payload(), s);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call_uwith_umovable_udrop(
    ::rs::internal::FnRefPayload, ::std::int32_t);
}
inline void call_with_movable_drop(
    ::rs::FnRef<void(::callables::CppMovableDrop) const> f, ::std::int32_t x) {
  auto __f_invoker = [](void* __data,
                        ::callables::CppMovableDrop* __arg_0) -> void {
    (*reinterpret_cast<::rs::FnRef<void(::callables::CppMovableDrop) const>*>(
        __data))(::std::move(*__arg_0));
  };
  ::rs::internal::FnRefPayload __f_payload{
      const_cast<void*>(reinterpret_cast<const void*>(&f)),
      reinterpret_cast<void (*)()>(+__f_invoker),
  };
  return __crubit_internal::__crubit_thunk_call_uwith_umovable_udrop(
      __f_payload, x);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_uwith_unon_umovable_uref(
    ::rs::internal::FnRefPayload, ::callables::NonCppMovable const&);
}
inline ::std::int32_t call_with_non_movable_ref(
    ::rs::FnRef<void(::callables::NonCppMovable const* $static crubit_nonnull)
                    const>
        f,
    ::callables::NonCppMovable const& x) {
  return __crubit_internal::__crubit_thunk_call_uwith_unon_umovable_uref(
      f.payload(), x);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_call_uwith_upoint(::rs::internal::FnRefPayload,
                                                 ::callables::Point*,
                                                 ::callables::Point* __ret_ptr);
}
inline ::callables::Point call_with_point(
    ::rs::FnRef<::callables::Point(::callables::Point) const> f,
    ::callables::Point pt) {
  auto __f_invoker = [](void* __data, ::callables::Point* __arg_0,
                        void* __ret_ptr) -> void {
    new (__ret_ptr)::callables::Point(
        (*reinterpret_cast<
            ::rs::FnRef<::callables::Point(::callables::Point) const>*>(
            __data))(::std::move(*__arg_0)));
  };
  ::rs::internal::FnRefPayload __f_payload{
      const_cast<void*>(reinterpret_cast<const void*>(&f)),
      reinterpret_cast<void (*)()>(+__f_invoker),
  };
  crubit::Slot<::callables::Point> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_call_uwith_upoint(__f_payload, &pt,
                                                      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_uwith_ustr(
    ::rs::internal::FnRefPayload, rs_std::StrRef);
}
inline ::std::int32_t call_with_str(
    ::rs::FnRef<::std::int32_t(rs_std::StrRef) const> f, rs_std::StrRef s) {
  return __crubit_internal::__crubit_thunk_call_uwith_ustr(f.payload(), s);
}

namespace __crubit_internal {
extern "C" rs_std::StrRef __crubit_thunk_call_uwith_ustr_uto_ustr(
    ::rs::internal::FnRefPayload, rs_std::StrRef);
}
inline rs_std::StrRef call_with_str_to_str(
    ::rs::FnRef<rs_std::StrRef(rs_std::StrRef) const> f, rs_std::StrRef s) {
  return __crubit_internal::__crubit_thunk_call_uwith_ustr_uto_ustr(f.payload(),
                                                                    s);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_call_uwith_utuple_uoption(
    ::rs::internal::FnPayload);
}
inline ::std::int32_t call_with_tuple_option(
    ::rs::Fn<rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>(
        rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>) &&>
        f) {
  auto* __f_heap_fn = new ::rs::Fn<
      rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>(
          rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>) &&>(
      ::std::move(f));
  auto __f_invoker =
      [](void* __data,
         rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>* __arg_0,
         void* __ret_ptr) -> void {
    new (__ret_ptr) rs_std::Tuple<::std::int32_t,
                                  rs_std::Option<::std::int32_t>>(::std::move(
        *reinterpret_cast<::rs::Fn<
            rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>(
                rs_std::Tuple<::std::int32_t,
                              rs_std::Option<::std::int32_t>>) &&>*>(__data))(
        ::std::move(*__arg_0)));
  };
  auto __f_destroyer = [](void* __data) noexcept {
    delete reinterpret_cast<::rs::Fn<rs_std::Tuple<
        ::std::int32_t, rs_std::Option<::std::int32_t>>(
        rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>) &&>*>(
        __data);
  };
  ::rs::internal::FnPayload __f_payload{
      __f_heap_fn,
      reinterpret_cast<void (*)()>(+__f_invoker),
      +__f_destroyer,
  };
  return __crubit_internal::__crubit_thunk_call_uwith_utuple_uoption(
      __f_payload);
}

}  // namespace callables

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Tuple_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002c_x00000020rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_u_x00000028i32_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x00000029(
    rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>* __ret_ptr);
}
inline ::rs_std::Tuple<::std::int32_t,
                       rs_std::Option<::std::int32_t>>::Tuple() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_u_x00000028i32_x0000002c_x00000020std_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003ci32_x0000003e_x00000029(
          this);
}
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>>);
inline ::rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>::Tuple(
    ::crubit::UnsafeRelocateTag, Tuple&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>::Tuple(
    std::tuple<::std::int32_t, rs_std::Option<::std::int32_t>>&&
        tuple) noexcept {
  std::construct_at(&this->__field0, std::move(std::get<0>(tuple)));
  std::construct_at(&this->__field1, std::move(std::get<1>(tuple)));
}
inline rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>::
operator std::tuple<::std::int32_t,
                    rs_std::Option<::std::int32_t>>() && noexcept {
  return std::tuple<::std::int32_t, rs_std::Option<::std::int32_t>>(
      std::move(this->__field0), std::move(this->__field1));
}

inline void ::rs_std::Tuple<::std::int32_t, rs_std::Option<::std::int32_t>>::
    __crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Tuple, __field0));
  static_assert(4 == offsetof(Tuple, __field1));
}
#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e
static_assert(
    ::std::is_trivially_copy_constructible_v<rs_std::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_copy_assignable_v<rs_std::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_constructible_v<rs_std::Option<::std::int32_t>>);
static_assert(
    ::std::is_trivially_move_assignable_v<rs_std::Option<::std::int32_t>>);
inline rs_std::Option<::std::int32_t>::Option(::crubit::UnsafeRelocateTag,
                                              Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    ::std::is_trivially_destructible_v<rs_std::Option<::std::int32_t>>);
inline constexpr ::std::uint32_t rs_std::Option<::std::int32_t>::tag()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void rs_std::Option<::std::int32_t>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::std::int32_t>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::std::int32_t>&
rs_std::Option<::std::int32_t>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<::std::int32_t>,
                                              ::std::int32_t, U>)
inline rs_std::Option<::std::int32_t>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<rs_std::Option<::std::int32_t>,
                                              ::std::int32_t, U>)
inline rs_std::Option<::std::int32_t>&
rs_std::Option<::std::int32_t>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::int32_t, Opt>)
inline rs_std::Option<::std::int32_t>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::int32_t, Opt>)
inline rs_std::Option<::std::int32_t>&
rs_std::Option<::std::int32_t>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::std::int32_t>::Option(::std::in_place_t ip,
                                              Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_CALLABLES_CALLABLES_GOLDEN
