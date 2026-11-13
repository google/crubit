// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// option_references_golden

// clang-format off
#ifndef CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_REFERENCES_GOLDEN
#define CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_REFERENCES_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/iterator_adapter.h"
#include "support/rs_std/option.h"
#include "support/rs_std/traits.h"

#include <array>
#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <optional>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_core.h"

namespace option_references {
struct ValuesIter;
struct CRUBIT_INTERNAL_RUST_TYPE(":: option_references_golden :: Value")
    [[clang::trivial_abi]] Value final {
 public:
  ::std::int32_t x{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_references_golden :: Values") alignas(4) [[clang::trivial_abi]]
Values final {
 public:
  // Default::default
  Values();

  // No custom `Drop` impl and no custom "drop glue" required
  ~Values() = default;
  Values(Values&&) = default;
  Values& operator=(Values&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Values(const Values&) = default;
  Values& operator=(const Values&) = default;
  Values(::crubit::UnsafeRelocateTag, Values&& value);

  static ::option_references::Values new_(::std::int32_t a, ::std::int32_t b,
                                          ::std::int32_t c);

  ::rs::Option<::std::int32_t const & $static> get(
      ::std::uintptr_t index) const& $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::rs::Option<::std::int32_t& $static> get_mut(::std::uintptr_t index) &
      $(__anon1) CRUBIT_LIFETIME_BOUND;

  ::option_references::ValuesIter iter() const& $(__anon1)
      CRUBIT_LIFETIME_BOUND;

 private:
  // Field type has been replaced with a blob of bytes: Field is private
  ::std::array<unsigned char, 12> values;

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: option_references_golden :: ValuesIter") alignas(4)
    [[clang::trivial_abi]] ValuesIter final {
 public:
  // `option_references_golden::ValuesIter` doesn't implement the `Default`
  // trait
  ValuesIter() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~ValuesIter() = default;
  ValuesIter(ValuesIter&&) = default;
  ValuesIter& operator=(ValuesIter&&) = default;

  // `option_references_golden::ValuesIter` doesn't implement the `Clone` trait
  ValuesIter(const ValuesIter&) = delete;
  ValuesIter& operator=(const ValuesIter&) = delete;
  ValuesIter(::crubit::UnsafeRelocateTag, ValuesIter&& value);
  template <typename TAdaptedSelf_ = ValuesIter>
  inline ::rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return ::rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline ::rs::IteratorEnd end() & { return ::rs::IteratorEnd(); }

 private:
  // Field type has been replaced with a blob of bytes: Field is private
  ::std::array<unsigned char, 8> values;

 private:
  static void __crubit_field_offset_assertions();
};

//  `Option<*const Value>` used to have the same C++ type as `Option<&Value>`,
//  even though their layouts differ (b/575916406).
::rs::Option<::option_references::Value const*> as_ptr(
    ::option_references::Value const& v);

//  Returns `v` if `v.x` is positive.
::rs::Option<::option_references::Value const & $static> if_positive(
    ::option_references::Value const* $(__anon1)
        crubit_nonnull v CRUBIT_LIFETIME_BOUND);

//  Returns `v` if `v.x` is positive.
::rs::Option<::option_references::Value & $static> if_positive_mut(
    ::option_references::Value* $(__anon1)
        crubit_nonnull v CRUBIT_LIFETIME_BOUND);

void increment(::rs::Option<::option_references::Value & $static> v);

bool same_value(::rs::Option<::option_references::Value const & $static> a,
                ::rs::Option<::option_references::Value const & $static> b);

::std::int32_t x_or(::rs::Option<::option_references::Value const & $static> v,
                    ::std::int32_t default_);

}

template <>
struct ::rs::impl<::option_references::ValuesIter, ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<option_references_golden::ValuesIter<'a> as :: core :: iter :: "
      "Iterator>::Item") = ::std::int32_t const& $a;

  static ::rs::Option<::std::int32_t const & $static> next(
      ::option_references::ValuesIter& self);
};
#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x00000026_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x00000026_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("core :: option :: Option < & 'static i32 >")::
        rs::Option<::std::int32_t const & $static>
    : public ::rs::OptionBase<::rs::Option<::std::int32_t const & $static>,
                              ::std::int32_t const & $static> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::std::int32_t const & $static>,
                       ::std::int32_t const & $static>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionRefBindable<::std::int32_t const & $static, U>
  constexpr Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionRefBindable<::std::int32_t const & $static, U>
  constexpr Option& operator=(U&& value) noexcept;
  ~Option() noexcept = default;
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x00000026_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x00000026_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < & 'static mut i32 >")::rs::
    Option<::std::int32_t& $static>
    : public ::rs::OptionBase<::rs::Option<::std::int32_t& $static>,
                              ::std::int32_t& $static> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<::rs::Option<::std::int32_t& $static>,
                                     ::std::int32_t& $static>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionRefBindable<::std::int32_t& $static, U>
  constexpr Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionRefBindable<::std::int32_t& $static, U>
  constexpr Option& operator=(U&& value) noexcept;
  ~Option() noexcept = default;
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_ureferences_x00000020_x0000003a_x0000003a_x00000020Value_x00000020_x00000026_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_ureferences_x00000020_x0000003a_x0000003a_x00000020Value_x00000020_x00000026_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < & 'static mut :: option_references_golden :: "
    "Value >")::rs::Option<::option_references::Value & $static>
    : public ::rs::OptionBase<
          ::rs::Option<::option_references::Value & $static>,
          ::option_references::Value & $static> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::option_references::Value & $static>,
                       ::option_references::Value & $static>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionRefBindable<::option_references::Value & $static, U>
  constexpr Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionRefBindable<::option_references::Value & $static, U>
  constexpr Option& operator=(U&& value) noexcept;
  ~Option() noexcept = default;
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_ureferences_x00000020_x0000003a_x0000003a_x00000020Value_x00000020const_x00000020_x00000026_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_ureferences_x00000020_x0000003a_x0000003a_x00000020Value_x00000020const_x00000020_x00000026_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < & 'static :: option_references_golden :: Value "
    ">")::rs::Option<::option_references::Value const & $static>
    : public ::rs::OptionBase<
          ::rs::Option<::option_references::Value const & $static>,
          ::option_references::Value const & $static> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::option_references::Value const & $static>,
                       ::option_references::Value const & $static>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionRefBindable<::option_references::Value const & $static,
                                     U>
  constexpr Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionRefBindable<::option_references::Value const & $static,
                                     U>
  constexpr Option& operator=(U&& value) noexcept;
  ~Option() noexcept = default;
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_ureferences_x00000020_x0000003a_x0000003a_x00000020Value_x00000020const_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_ureferences_x00000020_x0000003a_x0000003a_x00000020Value_x00000020const_x00000020_x0000002a_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < * const :: option_references_golden :: Value "
    ">")::rs::Option<::option_references::Value const*>
    : public ::rs::OptionBase<::rs::Option<::option_references::Value const*>,
                              ::option_references::Value const*> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::option_references::Value const*>,
                       ::option_references::Value const*>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, ::option_references::Value const*, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, ::option_references::Value const*, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option_references::Value const*, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::option_references::Value const*, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = 0;
  ::option_references::Value const** some_ptr() noexcept {
    return reinterpret_cast<::option_references::Value const**>(storage_ + 4);
  }
  ::option_references::Value const* const* some_const_ptr() const noexcept {
    return reinterpret_cast<::option_references::Value const* const*>(storage_ +
                                                                      4);
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

namespace option_references {

static_assert(
    sizeof(::option_references::Value) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option_references::Value) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::option_references::Value>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option_references::Value>);
static_assert(
    ::std::is_trivially_move_assignable_v<::option_references::Value>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::option_references::Value>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::option_references::Value>);
inline void ::option_references::Value::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option_references::Value;
  static_assert(0 == offsetof(__crubit_assert_type, x));
}
static_assert(
    sizeof(::option_references::Values) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option_references::Values) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uoption_ureferences_ugolden_x0000003a_x0000003aValues(
    ::option_references::Values* __ret_ptr);
}
inline ::option_references::Values::Values() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uoption_ureferences_ugolden_x0000003a_x0000003aValues(
          this);
}
static_assert(::std::is_trivially_destructible_v<::option_references::Values>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option_references::Values>);
static_assert(
    ::std::is_trivially_move_assignable_v<::option_references::Values>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::option_references::Values>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::option_references::Values>);
inline ::option_references::Values::Values(::crubit::UnsafeRelocateTag,
                                           Values&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t, ::std::int32_t,
                                   ::std::int32_t,
                                   ::option_references::Values* __ret_ptr);
}
inline ::option_references::Values(Values::new_)(::std::int32_t a,
                                                 ::std::int32_t b,
                                                 ::std::int32_t c) {
  crubit::Slot<::option_references::Values> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(a, b, c, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_get(
    ::option_references::Values const&, ::std::uintptr_t,
    ::rs::Option<::std::int32_t const & $static>* __ret_ptr);
}
inline ::rs::Option<::std::int32_t const & $static>(Values::get)(
    ::std::uintptr_t index) const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  crubit::Slot<::rs::Option<::std::int32_t const & $static>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_get(self, index, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_get_umut(
    ::option_references::Values&, ::std::uintptr_t,
    ::rs::Option<::std::int32_t& $static>* __ret_ptr);
}
inline ::rs::Option<::std::int32_t& $static>(Values::get_mut)(
    ::std::uintptr_t index) &
    $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  crubit::Slot<::rs::Option<::std::int32_t& $static>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_get_umut(self, index,
                                             __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_iter(::option_references::Values const&,
                                    ::option_references::ValuesIter* __ret_ptr);
}
inline ::option_references::ValuesIter(Values::iter)() const& $(__anon1)
    CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  crubit::Slot<::option_references::ValuesIter> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_iter(self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void ::option_references::Values::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option_references::Values;
  static_assert(0 == offsetof(__crubit_assert_type, values));
}
static_assert(
    sizeof(::option_references::ValuesIter) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::option_references::ValuesIter) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::option_references::ValuesIter>);
static_assert(
    ::std::is_trivially_move_constructible_v<::option_references::ValuesIter>);
static_assert(
    ::std::is_trivially_move_assignable_v<::option_references::ValuesIter>);
inline ::option_references::ValuesIter::ValuesIter(::crubit::UnsafeRelocateTag,
                                                   ValuesIter&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::option_references::ValuesIter::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::option_references::ValuesIter;
  static_assert(0 == offsetof(__crubit_assert_type, values));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_as_uptr(
    ::option_references::Value const&,
    ::rs::Option<::option_references::Value const*>* __ret_ptr);
}
inline ::rs::Option<::option_references::Value const*> as_ptr(
    ::option_references::Value const& v) {
  crubit::Slot<::rs::Option<::option_references::Value const*>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_as_uptr(v, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_if_upositive(
    ::option_references::Value const* $(__anon1) crubit_nonnull,
    ::rs::Option<::option_references::Value const & $static>* __ret_ptr);
}
inline ::rs::Option<::option_references::Value const & $static> if_positive(
    ::option_references::Value const* $(__anon1)
        crubit_nonnull v CRUBIT_LIFETIME_BOUND) {
  crubit::Slot<::rs::Option<::option_references::Value const & $static>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_if_upositive(v, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_if_upositive_umut(
    ::option_references::Value* $(__anon1) crubit_nonnull,
    ::rs::Option<::option_references::Value & $static>* __ret_ptr);
}
inline ::rs::Option<::option_references::Value & $static> if_positive_mut(
    ::option_references::Value* $(__anon1)
        crubit_nonnull v CRUBIT_LIFETIME_BOUND) {
  crubit::Slot<::rs::Option<::option_references::Value & $static>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_if_upositive_umut(v,
                                                      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_increment(
    ::rs::Option<::option_references::Value & $static>*);
}
inline void increment(::rs::Option<::option_references::Value & $static> v) {
  return __crubit_internal::__crubit_thunk_increment(&v);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_same_uvalue(
    ::rs::Option<::option_references::Value const & $static>*,
    ::rs::Option<::option_references::Value const & $static>*);
}
inline bool same_value(
    ::rs::Option<::option_references::Value const & $static> a,
    ::rs::Option<::option_references::Value const & $static> b) {
  return __crubit_internal::__crubit_thunk_same_uvalue(&a, &b);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_x_uor(
    ::rs::Option<::option_references::Value const & $static>*, ::std::int32_t);
}
inline ::std::int32_t x_or(
    ::rs::Option<::option_references::Value const & $static> v,
    ::std::int32_t default_) {
  return __crubit_internal::__crubit_thunk_x_uor(&v, default_);
}

}

namespace option_references {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_uoption_ureferences_ugolden_x0000003a_x0000003aValuesIter_x0000003c_x00000027a_x0000003e_u_x00000027a(
    ::option_references::ValuesIter&,
    ::rs::Option<::std::int32_t const & $static>* __ret_ptr);
}
}
inline ::rs::Option<::std::int32_t const & $static>(
    ::rs::impl<::option_references::ValuesIter,
               ::rs::core::iter::Iterator>::next)(
    ::option_references::ValuesIter& self) {
  crubit::Slot<::rs::Option<::std::int32_t const & $static>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  option_references::__crubit_internal::
      __crubit_thunk_Iterator_unext_uoption_ureferences_ugolden_x0000003a_x0000003aValuesIter_x0000003c_x00000027a_x0000003e_u_x00000027a(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x00000026_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x00000026_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Option<::std::int32_t const & $static>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Option<::std::int32_t const & $static>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<::std::int32_t const & $static>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Option<::std::int32_t const & $static>>);
inline ::rs::Option<::std::int32_t const & $static>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(sizeof(::rs::Option<::std::int32_t const & $static>) == 4);
static_assert(alignof(::rs::Option<::std::int32_t const & $static>) == 4);
static_assert(::std::is_trivially_destructible_v<
              ::rs::Option<::std::int32_t const & $static>>);
inline constexpr ::rs::Option<::std::int32_t const & $static>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::std::int32_t const & $static>& ::rs::Option<
    ::std::int32_t const & $static>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionRefBindable<::std::int32_t const & $static, U>
inline constexpr ::rs::Option<::std::int32_t const & $static>::Option(
    U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionRefBindable<::std::int32_t const & $static, U>
inline constexpr ::rs::Option<::std::int32_t const & $static>& ::rs::Option<
    ::std::int32_t const & $static>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x00000026_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x00000026_x00000020_x0000003e
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<::std::int32_t& $static>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Option<::std::int32_t& $static>>);
inline ::rs::Option<::std::int32_t& $static>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(sizeof(::rs::Option<::std::int32_t& $static>) == 4);
static_assert(alignof(::rs::Option<::std::int32_t& $static>) == 4);
static_assert(
    ::std::is_trivially_destructible_v<::rs::Option<::std::int32_t& $static>>);
inline constexpr ::rs::Option<::std::int32_t& $static>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::std::int32_t& $static>& ::rs::Option<
    ::std::int32_t& $static>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionRefBindable<::std::int32_t& $static, U>
inline constexpr ::rs::Option<::std::int32_t& $static>::Option(
    U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionRefBindable<::std::int32_t& $static, U>
inline constexpr ::rs::Option<::std::int32_t& $static>& ::rs::Option<
    ::std::int32_t& $static>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_ureferences_x00000020_x0000003a_x0000003a_x00000020Value_x00000020_x00000026_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_ureferences_x00000020_x0000003a_x0000003a_x00000020Value_x00000020_x00000026_x00000020_x0000003e
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<::option_references::Value & $static>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Option<::option_references::Value & $static>>);
inline ::rs::Option<::option_references::Value & $static>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(sizeof(::rs::Option<::option_references::Value & $static>) == 4);
static_assert(alignof(::rs::Option<::option_references::Value & $static>) == 4);
static_assert(::std::is_trivially_destructible_v<
              ::rs::Option<::option_references::Value & $static>>);
inline constexpr ::rs::Option<::option_references::Value & $static>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::option_references::Value & $static>& ::rs::
    Option<::option_references::Value & $static>::operator=(
        ::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionRefBindable<::option_references::Value & $static, U>
inline constexpr ::rs::Option<::option_references::Value & $static>::Option(
    U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionRefBindable<::option_references::Value & $static, U>
inline constexpr ::rs::Option<::option_references::Value & $static>& ::rs::
    Option<::option_references::Value & $static>::operator=(
        U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_ureferences_x00000020_x0000003a_x0000003a_x00000020Value_x00000020const_x00000020_x00000026_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_ureferences_x00000020_x0000003a_x0000003a_x00000020Value_x00000020const_x00000020_x00000026_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Option<::option_references::Value const & $static>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Option<::option_references::Value const & $static>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<::option_references::Value const & $static>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Option<::option_references::Value const & $static>>);
inline ::rs::Option<::option_references::Value const & $static>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(
    sizeof(::rs::Option<::option_references::Value const & $static>) == 4);
static_assert(
    alignof(::rs::Option<::option_references::Value const & $static>) == 4);
static_assert(::std::is_trivially_destructible_v<
              ::rs::Option<::option_references::Value const & $static>>);
inline constexpr ::rs::Option<::option_references::Value const &
                              $static>::Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::option_references::Value const & $static>& ::
rs::Option<::option_references::Value const & $static>::operator=(
    ::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionRefBindable<::option_references::Value const & $static,
                                   U>
inline constexpr ::rs::Option<::option_references::Value const &
                              $static>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionRefBindable<::option_references::Value const & $static,
                                   U>
inline constexpr ::rs::Option<::option_references::Value const & $static>& ::
rs::Option<::option_references::Value const & $static>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_ureferences_x00000020_x0000003a_x0000003a_x00000020Value_x00000020const_x00000020_x0000002a_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020option_ureferences_x00000020_x0000003a_x0000003a_x00000020Value_x00000020const_x00000020_x0000002a_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              ::rs::Option<::option_references::Value const*>>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::rs::Option<::option_references::Value const*>>);
static_assert(::std::is_trivially_move_constructible_v<
              ::rs::Option<::option_references::Value const*>>);
static_assert(::std::is_trivially_move_assignable_v<
              ::rs::Option<::option_references::Value const*>>);
inline ::rs::Option<::option_references::Value const*>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              ::rs::Option<::option_references::Value const*>>);
inline constexpr ::std::uint32_t(
    ::rs::Option<::option_references::Value const*>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void(::rs::Option<::option_references::Value const*>::set_tag)(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::option_references::Value const*>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::option_references::Value const*>& ::rs::Option<
    ::option_references::Value const*>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::option_references::Value const*>,
      ::option_references::Value const*, U>
inline ::rs::Option<::option_references::Value const*>::Option(
    U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::option_references::Value const*>,
      ::option_references::Value const*, U>
inline ::rs::Option<::option_references::Value const*>& ::rs::Option<
    ::option_references::Value const*>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option_references::Value const*, Opt>
inline ::rs::Option<::option_references::Value const*>::Option(
    Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::option_references::Value const*, Opt>
inline ::rs::Option<::option_references::Value const*>& ::rs::Option<
    ::option_references::Value const*>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::option_references::Value const*>::Option(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // CC_BINDINGS_FROM_RS_TEST_ENUMS_OPTION_REFERENCES_GOLDEN
