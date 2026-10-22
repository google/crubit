// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// move_golden

// clang-format off
#ifndef CC_BINDINGS_FROM_RS_TEST_MOVE_SEMANTICS_MOVE_GOLDEN
#define CC_BINDINGS_FROM_RS_TEST_MOVE_SEMANTICS_MOVE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/movable.h"
#include "support/rs_std/result.h"

#include <array>
#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace move {

struct CRUBIT_INTERNAL_RUST_TYPE(":: move_golden :: Copyable") alignas(1)
    [[clang::trivial_abi]] Copyable final {
 public:
  static ::move::Copyable from_byte(::std::uint8_t byte);

  //  Typically, `self`-by-value methods turn into `&&`-qualified methods in
  //  C++. However, for `Copy` types, there's no need to consume the argument,
  //  as it will be copied regardless.
  ::std::uint8_t consume_self() const;

  ::std::uint8_t field{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: move_golden :: Foo") alignas(4)
    [[clang::trivial_abi]] Foo final {
 public:
  // Default::default
  Foo();

  // Drop::drop
  ~Foo();

  Foo(Foo&&) noexcept;
  ::move::Foo& operator=(Foo&&) noexcept;

  // `move_golden::Foo` doesn't implement the `Clone` trait
  Foo(const Foo&) = delete;
  Foo& operator=(const Foo&) = delete;
  Foo(::crubit::UnsafeRelocateTag, Foo&& value);

  static ::move::Foo from_byte(::std::uint8_t byte);

  ::std::uint8_t read_byte() const;

  ::std::uint8_t into_byte() &&;

 private:
  // Field type has been replaced with a blob of bytes: Field is private
  ::std::array<unsigned char, 4> buf;

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: move_golden :: UnmovableFoo") alignas(4)
    [[clang::trivial_abi]] UnmovableFoo final {
 public:
  // `move_golden::UnmovableFoo` doesn't implement the `Default` trait
  UnmovableFoo() = delete;

  // Drop::drop
  ~UnmovableFoo();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  UnmovableFoo(UnmovableFoo&&) = delete;
  ::move::UnmovableFoo& operator=(UnmovableFoo&&) = delete;
  // `move_golden::UnmovableFoo` doesn't implement the `Clone` trait
  UnmovableFoo(const UnmovableFoo&) = delete;
  UnmovableFoo& operator=(const UnmovableFoo&) = delete;
  UnmovableFoo(::crubit::UnsafeRelocateTag, UnmovableFoo&& value);

  static ::rs::Movable<::move::UnmovableFoo> from_byte(::std::uint8_t byte);

  ::std::uint8_t read_byte() const;

 private:
  // Field type has been replaced with a blob of bytes: Field is private
  ::std::array<unsigned char, 4> buf;

 private:
  static void __crubit_field_offset_assertions();
};

void consume_foo(::move::Foo _foo);

::std::uint8_t consume_unmovable_foo(::rs::Movable<::move::UnmovableFoo> foo);

//  Initializes an `UnmovableFoo` in the given memory location.
//
//  # Safety
//
//  * `out` must be valid for writes.
//  * `out` must be properly aligned.
//  * `out` must point to uninitialized memory (otherwise the previous value
//  will be leaked).
void initialize_unmovable_foo(::move::UnmovableFoo* out, ::std::uint8_t byte);

::rs::Movable<::move::UnmovableFoo> new_unmovable_foo(::std::uint8_t byte);

::rs::Movable<rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>>
new_unmovable_foo_result(::std::uint8_t byte);

}  // namespace move

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020move_x00000020_x0000003a_x0000003a_x00000020UnmovableFoo_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020move_x00000020_x0000003a_x0000003a_x00000020UnmovableFoo_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: result :: Result < :: move_golden :: UnmovableFoo , u8 >")
    rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>
    : public rs_std::ResultBase<
          rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>,
          ::move::UnmovableFoo, ::std::uint8_t> {
 public:
  // `core::result::Result` doesn't implement the `Clone` trait
  Result(const Result&) = delete;
  Result& operator=(const Result&) = delete;
  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  Result(Result&&) = delete;
  rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>& operator=(Result&&) =
      delete;
  Result(::crubit::UnsafeRelocateTag, Result&& value);

 public:
  using base_type =
      rs_std::ResultBase<rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>,
                         ::move::UnmovableFoo, ::std::uint8_t>;
  template <typename U>
    requires(
        rs_std::ResultForwardConstructible<Result, ::move::UnmovableFoo, U>)
  explicit constexpr Result(U&& ok) noexcept;
  template <typename U>
    requires(
        rs_std::ResultForwardConstructible<Result, ::move::UnmovableFoo, U>)
  constexpr Result& operator=(U&& ok) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  explicit constexpr Result(rs_std::unexpected<F>&& err) noexcept;
  template <typename F>
    requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
  constexpr Result& operator=(rs_std::unexpected<F>&& err) noexcept;
  template <typename... Args>
  explicit constexpr Result(::std::in_place_t ip, Args&&... args) noexcept;
  template <typename... Args>
  explicit constexpr Result(rs_std::unexpect_t u, Args&&... args) noexcept;
  ~Result() noexcept;

 private:
  friend base_type;
  bool has_value_impl() const noexcept { return tag() == 0; }
  ::move::UnmovableFoo* ok_ptr() noexcept {
    return reinterpret_cast<::move::UnmovableFoo*>(__storage + 4);
  }
  ::move::UnmovableFoo const* ok_const_ptr() const noexcept {
    return reinterpret_cast<::move::UnmovableFoo const*>(__storage + 4);
  }
  ::std::uint8_t* err_ptr() noexcept {
    return reinterpret_cast<::std::uint8_t*>(__storage + 1);
  }
  ::std::uint8_t const* err_const_ptr() const noexcept {
    return reinterpret_cast<::std::uint8_t const*>(__storage + 1);
  }
  void set_ok_tag() noexcept { set_tag(0); }
  void set_err_tag() noexcept { set_tag(1); }
  constexpr ::std::uint8_t tag() const& noexcept;
  constexpr void set_tag(::std::uint8_t tag) noexcept;

 private:
  unsigned char __storage[8];
};
#endif

namespace move {

static_assert(
    sizeof(::move::Copyable) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::move::Copyable) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::move::Copyable>);
static_assert(::std::is_trivially_move_constructible_v<::move::Copyable>);
static_assert(::std::is_trivially_move_assignable_v<::move::Copyable>);
static_assert(::std::is_trivially_copy_constructible_v<::move::Copyable>);
static_assert(::std::is_trivially_copy_assignable_v<::move::Copyable>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_from_ubyte(::std::uint8_t,
                                          ::move::Copyable* __ret_ptr);
}
inline ::move::Copyable(Copyable::from_byte)(::std::uint8_t byte) {
  crubit::Slot<::move::Copyable> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_from_ubyte(byte, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_consume_uself(::move::Copyable*);
}
inline ::std::uint8_t(Copyable::consume_self)() const {
  auto& self = const_cast<::std::remove_cvref_t<decltype(*this)>&>(*this);
  return __crubit_internal::__crubit_thunk_consume_uself(&self);
}
inline void ::move::Copyable::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::move::Copyable;
  static_assert(0 == offsetof(__crubit_assert_type, field));
}
static_assert(
    sizeof(::move::Foo) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::move::Foo) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_umove_ugolden_x0000003a_x0000003aFoo(
    ::move::Foo* __ret_ptr);
}
inline ::move::Foo::Foo() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_umove_ugolden_x0000003a_x0000003aFoo(
          this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_Drop_udrop_umove_ugolden_x0000003a_x0000003aFoo(
    ::move::Foo&);
}
inline ::move::Foo::~Foo() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_umove_ugolden_x0000003a_x0000003aFoo(*this);
}
inline ::move::Foo::Foo(Foo&& other) noexcept : Foo() {
  *this = ::std::move(other);
}
inline ::move::Foo& ::move::Foo::operator=(Foo&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::move::Foo::Foo(::crubit::UnsafeRelocateTag, Foo&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_from_ubyte(::std::uint8_t,
                                          ::move::Foo* __ret_ptr);
}
inline ::move::Foo(Foo::from_byte)(::std::uint8_t byte) {
  crubit::Slot<::move::Foo> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_from_ubyte(byte, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_read_ubyte(::move::Foo const&);
}
inline ::std::uint8_t(Foo::read_byte)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_read_ubyte(self);
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_into_ubyte(::move::Foo*);
}
inline ::std::uint8_t(Foo::into_byte)() && {
  auto&& self = *this;
  crubit::Slot self_slot((::std::move(self)));
  return __crubit_internal::__crubit_thunk_into_ubyte(self_slot.Get());
}
inline void ::move::Foo::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::move::Foo;
  static_assert(0 == offsetof(__crubit_assert_type, buf));
}
static_assert(
    sizeof(::move::UnmovableFoo) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::move::UnmovableFoo) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_umove_ugolden_x0000003a_x0000003aUnmovableFoo(
    ::move::UnmovableFoo&);
}
inline ::move::UnmovableFoo::~UnmovableFoo() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_umove_ugolden_x0000003a_x0000003aUnmovableFoo(
          *this);
}
inline ::move::UnmovableFoo::UnmovableFoo(::crubit::UnsafeRelocateTag,
                                          UnmovableFoo&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_from_ubyte(::std::uint8_t,
                                          ::move::UnmovableFoo* __ret_ptr);
}
inline ::rs::Movable<::move::UnmovableFoo>(UnmovableFoo::from_byte)(
    ::std::uint8_t byte) {
  crubit::Slot<::move::UnmovableFoo> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_from_ubyte(byte, __return_value_storage);
  return ::rs::Movable<::move::UnmovableFoo>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_read_ubyte(
    ::move::UnmovableFoo const&);
}
inline ::std::uint8_t(UnmovableFoo::read_byte)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_read_ubyte(self);
}
inline void ::move::UnmovableFoo::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::move::UnmovableFoo;
  static_assert(0 == offsetof(__crubit_assert_type, buf));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_consume_ufoo(::move::Foo*);
}
inline void consume_foo(::move::Foo _foo) {
  crubit::Slot _foo_slot((::std::move(_foo)));
  return __crubit_internal::__crubit_thunk_consume_ufoo(_foo_slot.Get());
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_consume_uunmovable_ufoo(
    ::move::UnmovableFoo*);
}
inline ::std::uint8_t consume_unmovable_foo(
    ::rs::Movable<::move::UnmovableFoo> foo) {
  crubit::Slot<::move::UnmovableFoo> foo_slot;
  ::std::move(foo).MoveToSlot(foo_slot);
  return __crubit_internal::__crubit_thunk_consume_uunmovable_ufoo(
      foo_slot.Get());
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_initialize_uunmovable_ufoo(::move::UnmovableFoo*,
                                                          ::std::uint8_t);
}
inline void initialize_unmovable_foo(::move::UnmovableFoo* out,
                                     ::std::uint8_t byte) {
  return __crubit_internal::__crubit_thunk_initialize_uunmovable_ufoo(out,
                                                                      byte);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new_uunmovable_ufoo(
    ::std::uint8_t, ::move::UnmovableFoo* __ret_ptr);
}
inline ::rs::Movable<::move::UnmovableFoo> new_unmovable_foo(
    ::std::uint8_t byte) {
  crubit::Slot<::move::UnmovableFoo> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new_uunmovable_ufoo(byte,
                                                        __return_value_storage);
  return ::rs::Movable<::move::UnmovableFoo>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new_uunmovable_ufoo_uresult(
    ::std::uint8_t,
    rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>* __ret_ptr);
}
inline ::rs::Movable<rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>>
new_unmovable_foo_result(::std::uint8_t byte) {
  crubit::Slot<rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new_uunmovable_ufoo_uresult(
      byte, __return_value_storage);
  return ::rs::Movable<rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>>::
      TakeFromSlot(::std::move(__return_value_ret_val_holder));
}

}  // namespace move

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020move_x00000020_x0000003a_x0000003a_x00000020UnmovableFoo_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Result_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020move_x00000020_x0000003a_x0000003a_x00000020UnmovableFoo_x00000020_x0000002c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020uint8_ut_x00000020_x0000003e
inline rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>::Result(
    ::crubit::UnsafeRelocateTag, Result&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline rs_std::Result<::move::UnmovableFoo,
                      ::std::uint8_t>::~Result() noexcept {
  this->Reset();
}
inline constexpr ::std::uint8_t
rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>::tag() const& noexcept {
  std::array<unsigned char, sizeof(::std::uint8_t)> __bytes = {};
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __bytes[i] = __storage[0 + i];
  }
  return std::bit_cast<::std::uint8_t>(__bytes);
}
inline constexpr void
rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>::set_tag(
    ::std::uint8_t tag) noexcept {
  auto __bytes =
      std::bit_cast<std::array<unsigned char, sizeof(::std::uint8_t)>>(tag);
  for (std::size_t i = 0; i < sizeof(::std::uint8_t); ++i) {
    __storage[0 + i] = __bytes[i];
  }
}

template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>,
           ::move::UnmovableFoo, U>)
inline constexpr rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>::Result(
    U&& ok) noexcept
    : base_type(::std::forward<U>(ok)) {}
template <typename U>
  requires(rs_std::ResultForwardConstructible<
           rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>,
           ::move::UnmovableFoo, U>)
inline constexpr rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>&
rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>::operator=(
    U&& ok) noexcept {
  base_type::operator=(::std::forward<U>(ok));
  return *this;
}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>::Result(
    rs_std::unexpected<F>&& err) noexcept
    : base_type(::std::move(err)) {}
template <typename F>
  requires(rs_std::ResultUnexpectedConstructible<::std::uint8_t, F>)
inline constexpr rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>&
rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>::operator=(
    rs_std::unexpected<F>&& err) noexcept {
  base_type::operator=(::std::move(err));
  return *this;
}
template <typename... Args>
inline constexpr rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>::Result(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}
template <typename... Args>
inline constexpr rs_std::Result<::move::UnmovableFoo, ::std::uint8_t>::Result(
    rs_std::unexpect_t u, Args&&... args) noexcept
    : base_type(u, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // CC_BINDINGS_FROM_RS_TEST_MOVE_SEMANTICS_MOVE_GOLDEN
