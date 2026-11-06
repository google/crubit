// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// option_specialization_collision_golden

// clang-format off
#ifndef CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_SPECIALIZATION_COLLISION_GOLDEN
#define CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_SPECIALIZATION_COLLISION_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/option.h"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <optional>
#include <type_traits>
#include <utility>

namespace option_specialization_collision {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_specialization_collision_golden :: Value") [[clang::trivial_abi]]
Value final {
 public:
  ::rs::Option<::std::int32_t const * $static crubit_nonnull> get() const& $(
      __anon1) CRUBIT_LIFETIME_BOUND;

  // Error generating bindings for associated function
  // `option_specialization_collision_golden::Value::get_ptr`: Error formatting
  // function return type `std::option::Option<*const i32>`:
  // `std::option::Option<*const i32>` can't be bound because it would have the
  // same C++ type as `std::option::Option<&i32>`, which is also used by this
  // crate (b/575916406). Enabling the `option_references` Crubit feature makes
  // `Option<&T>` a distinct C++ type, `rs::Option<T const&>`

  ::std::int32_t value{};

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for function
// `option_specialization_collision_golden::by_ptr`: Error formatting function
// return type `std::option::Option<*const i32>`: `std::option::Option<*const
// i32>` can't be bound because it would have the same C++ type as
// `std::option::Option<&i32>`, which is also used by this crate (b/575916406).
// Enabling the `option_references` Crubit feature makes `Option<&T>` a distinct
// C++ type, `rs::Option<T const&>`

::rs::Option<::std::int32_t const * $static crubit_nonnull> by_ref(
    ::std::int32_t const* $(__anon1)
        crubit_nonnull value CRUBIT_LIFETIME_BOUND);

}

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
template <>
struct alignas(8)
    CRUBIT_INTERNAL_RUST_TYPE("core :: option :: Option < & 'static i32 >")::
        rs::Option<::std::int32_t const * $static crubit_nonnull>
    : public ::rs::OptionBase<
          ::rs::Option<::std::int32_t const * $static crubit_nonnull>,
          ::std::int32_t const * $static crubit_nonnull> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<
      ::rs::Option<::std::int32_t const * $static crubit_nonnull>,
      ::std::int32_t const * $static crubit_nonnull>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, ::std::int32_t const * $static crubit_nonnull, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, ::std::int32_t const * $static crubit_nonnull, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<
        ::std::int32_t const * $static crubit_nonnull, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<
        ::std::int32_t const * $static crubit_nonnull, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = 0;
  ::std::int32_t const* $static crubit_nonnull* some_ptr() noexcept {
    return reinterpret_cast<::std::int32_t const * $static crubit_nonnull*>(
        storage_);
  }
  ::std::int32_t const* $static crubit_nonnull const* some_const_ptr()
      const noexcept {
    return reinterpret_cast<::std::int32_t const *
                            $static crubit_nonnull const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[8];
};
#endif

namespace option_specialization_collision {

static_assert(
    sizeof(::option_specialization_collision::Value) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option_specialization_collision::Value) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::option_specialization_collision::Value>);
static_assert(::std::is_trivially_move_constructible_v<
              ::option_specialization_collision::Value>);
static_assert(::std::is_trivially_move_assignable_v<
              ::option_specialization_collision::Value>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::option_specialization_collision::Value>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::option_specialization_collision::Value>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_get(
    ::option_specialization_collision::Value const&,
    ::rs::Option<::std::int32_t const * $static crubit_nonnull>* __ret_ptr);
}
inline ::rs::Option<::std::int32_t const * $static crubit_nonnull>(
    Value::get)() const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  crubit::Slot<::rs::Option<::std::int32_t const * $static crubit_nonnull>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_get(self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::option_specialization_collision::Value::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option_specialization_collision::Value;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_by_uref(
    ::std::int32_t const* $(__anon1) crubit_nonnull,
    ::rs::Option<::std::int32_t const * $static crubit_nonnull>* __ret_ptr);
}
inline ::rs::Option<::std::int32_t const * $static crubit_nonnull> by_ref(
    ::std::int32_t const* $(__anon1)
        crubit_nonnull value CRUBIT_LIFETIME_BOUND) {
  crubit::Slot<::rs::Option<::std::int32_t const * $static crubit_nonnull>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_by_uref(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Option<::std::int32_t const * $static crubit_nonnull>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Option<::std::int32_t const * $static crubit_nonnull>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<::std::int32_t const * $static crubit_nonnull>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Option<::std::int32_t const * $static crubit_nonnull>>);
inline ::rs::Option<::std::int32_t const * $static crubit_nonnull>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              ::rs::Option<::std::int32_t const * $static crubit_nonnull>>);
inline constexpr ::std::uint64_t(
    ::rs::Option<::std::int32_t const * $static crubit_nonnull>::tag)()
    const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void(
    ::rs::Option<::std::int32_t const * $static crubit_nonnull>::set_tag)(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::std::int32_t const * $static crubit_nonnull>::
    Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::std::int32_t const * $static crubit_nonnull>& ::
rs::Option<::std::int32_t const * $static crubit_nonnull>::operator=(
    ::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::std::int32_t const * $static crubit_nonnull>,
      ::std::int32_t const * $static crubit_nonnull, U>
inline ::rs::Option<::std::int32_t const * $static crubit_nonnull>::Option(
    U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::std::int32_t const * $static crubit_nonnull>,
      ::std::int32_t const * $static crubit_nonnull, U>
inline ::rs::Option<::std::int32_t const * $static crubit_nonnull>& ::rs::
    Option<::std::int32_t const * $static crubit_nonnull>::operator=(
        U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<
      ::std::int32_t const * $static crubit_nonnull, Opt>
inline ::rs::Option<::std::int32_t const * $static crubit_nonnull>::Option(
    Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<
      ::std::int32_t const * $static crubit_nonnull, Opt>
inline ::rs::Option<::std::int32_t const * $static crubit_nonnull>& ::rs::
    Option<::std::int32_t const * $static crubit_nonnull>::operator=(
        Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::std::int32_t const * $static crubit_nonnull>::Option(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_SPECIALIZATION_COLLISION_GOLDEN
