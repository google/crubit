// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// boxes_golden

// clang-format off
#ifndef CC_BINDINGS_FROM_RS_TEST_BOX_BOXES_GOLDEN
#define CC_BINDINGS_FROM_RS_TEST_BOX_BOXES_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/movable.h"
#include "support/rs/box.h"
#include "support/rs/pin_box.h"
#include "support/rs/slice_box.h"
#include "support/rs/str_box.h"
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

namespace boxes {

struct CRUBIT_INTERNAL_RUST_TYPE(":: boxes_golden :: Holder") alignas(8)
    [[clang::trivial_abi]] Holder final {
 public:
  // Type is not a C++ aggregate: Type has a manual `Default` implementation

  // Default::default
  Holder();

  // Drop::drop
  ~Holder();

  Holder(Holder&&) noexcept;
  ::boxes::Holder& operator=(Holder&&) noexcept;

  // `boxes_golden::Holder` doesn't implement the `Clone` trait
  Holder(const Holder&) = delete;
  Holder& operator=(const Holder&) = delete;
  Holder(::crubit::UnsafeRelocateTag, Holder&& value);

  union {
    ::rs::Box<::std::int32_t> b;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: boxes_golden :: NotUnpin") alignas(4)
    [[clang::trivial_abi]] NotUnpin final {
 public:
  // Type is not a C++ aggregate: Field `_pin` is not public

  // `boxes_golden::NotUnpin` doesn't implement the `Default` trait
  NotUnpin() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NotUnpin() = default;
  NotUnpin(NotUnpin&&) = default;
  NotUnpin& operator=(NotUnpin&&) = default;

  // `boxes_golden::NotUnpin` doesn't implement the `Clone` trait
  NotUnpin(const NotUnpin&) = delete;
  NotUnpin& operator=(const NotUnpin&) = delete;
  NotUnpin(::crubit::UnsafeRelocateTag, NotUnpin&& value);

  union {
    ::std::int32_t val;
  };
  // Field `_pin` omitted: C++ does not support zero-sized types.
 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: boxes_golden :: Point") alignas(4)
    [[clang::trivial_abi]] Point final {
 public:
  ::std::int32_t x{};
  ::std::int32_t y{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: boxes_golden :: Tracked") alignas(8)
    [[clang::trivial_abi]] Tracked final {
 public:
  // `boxes_golden::Tracked` doesn't implement the `Default` trait
  Tracked() = delete;

  // Drop::drop
  ~Tracked();

  // Clone::clone
  Tracked(const Tracked&) noexcept;

  // Clone::clone_from
  ::boxes::Tracked& operator=(const Tracked&) noexcept;

  Tracked(::crubit::UnsafeRelocateTag, Tracked&& value);

 private:
  // Field type has been replaced with a blob of bytes: Field is private
  ::std::array<unsigned char, 8> __field0;

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: boxes_golden :: UnmovableHolder") alignas(
    8) [[clang::trivial_abi]] UnmovableHolder final {
 public:
  // Type is not a C++ aggregate: Field `b` requires drop glue but is not
  // C++-movable

  // `boxes_golden::UnmovableHolder` doesn't implement the `Default` trait
  UnmovableHolder() = delete;

  // Drop::drop
  ~UnmovableHolder();

  // C++ move operations are unavailable for this type. See
  // http://crubit.rs/rust/movable_types for an explanation of Rust types that
  // are C++ movable.
  UnmovableHolder(UnmovableHolder&&) = delete;
  ::boxes::UnmovableHolder& operator=(UnmovableHolder&&) = delete;
  // `boxes_golden::UnmovableHolder` doesn't implement the `Clone` trait
  UnmovableHolder(const UnmovableHolder&) = delete;
  UnmovableHolder& operator=(const UnmovableHolder&) = delete;
  UnmovableHolder(::crubit::UnsafeRelocateTag, UnmovableHolder&& value);

  union {
    ::rs::Box<::std::int32_t> b;
  };

 private:
  static void __crubit_field_offset_assertions();
};

::boxes::Tracked clone_handle(::boxes::Tracked const& t);

::std::int32_t consume_box(::rs::Movable<::rs::Box<::std::int32_t>> b);

::std::int32_t consume_opt_box(::rs::Option<::rs::Box<::std::int32_t>> o);

::std::uintptr_t consume_slice_box(
    ::rs::Movable<::rs::SliceBox<::std::int32_t>> b);

::std::uintptr_t consume_str_box(::rs::Movable<::rs::StrBox> s);

void consume_tracked(::rs::Movable<::rs::Box<::boxes::Tracked>> _b);

::std::int32_t holder_value(::boxes::Holder const& h);

::rs::Movable<::rs::Box<::std::int32_t>> make_box(::std::int32_t x);

::rs::Movable<::rs::Box<::boxes::NotUnpin>> make_box_not_unpin();

::rs::Movable<::rs::Box<::boxes::Point>> make_boxed_point();

::boxes::Holder make_holder();

::rs::Option<::rs::Box<::std::int32_t>> make_opt_box(bool some);

::rs::Option<::rs::SliceBox<::std::int32_t>> make_opt_box_slice(bool some);

::rs::Option<::rs::StrBox> make_opt_box_str(bool some);

::rs::Option<::rs::PinBox<::boxes::NotUnpin>> make_opt_pin_box(bool some);

::rs::Movable<::rs::PinBox<::std::int32_t>> make_pin_box();

::rs::Movable<::rs::PinBox<::boxes::NotUnpin>> make_pin_box_not_unpin();

::rs::Movable<::rs::SliceBox<::std::int32_t>> make_slice_box(
    ::std::uintptr_t n);

::rs::Movable<::rs::StrBox> make_str_box();

::rs::Movable<::rs::Box<::boxes::Tracked>> make_tracked_box();

::rs::Movable<::boxes::UnmovableHolder> make_unmovable_holder();

void mutate_box(::rs::Box<::std::int32_t>& b);

::std::int32_t opt_box_value(
    ::rs::Option<::rs::Box<::std::int32_t> const * $static crubit_nonnull> o);

::std::int32_t point_x(::rs::Movable<::rs::Box<::boxes::Point>> b);

::std::int32_t read_box(::rs::Box<::std::int32_t> const& b);

::std::int32_t read_pin_box(::rs::PinBox<::std::int32_t> const& b);

::rs::Movable<::rs::Box<::std::int32_t>> roundtrip_box(
    ::rs::Movable<::rs::Box<::std::int32_t>> b);

::std::uintptr_t str_box_len(::rs::StrBox const& s);

::std::int32_t sum_slice_box(::rs::SliceBox<::std::int32_t> const& b);

::std::uintptr_t tracked_strong_count(::boxes::Tracked const& t);

}  // namespace boxes

template <>
struct ::rs::impl<::boxes::NotUnpin, ::rs::core::marker::Unpin> {
  static constexpr bool kIsImplemented = false;
};
#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Box_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020const_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Box_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020const_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < & 'static :: alloc :: boxed :: Box < i32 , :: "
    "alloc :: alloc :: Global > >")::rs::
    Option<::rs::Box<::std::int32_t> const * $static crubit_nonnull>
    : public ::rs::OptionBase<::rs::Option<::rs::Box<::std::int32_t> const *
                                           $static crubit_nonnull>,
                              ::rs::Box<::std::int32_t> const *
                                  $static crubit_nonnull> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<
      ::rs::Option<::rs::Box<::std::int32_t> const * $static crubit_nonnull>,
      ::rs::Box<::std::int32_t> const * $static crubit_nonnull>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, ::rs::Box<::std::int32_t> const * $static crubit_nonnull, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, ::rs::Box<::std::int32_t> const * $static crubit_nonnull, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<
        ::rs::Box<::std::int32_t> const * $static crubit_nonnull, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<
        ::rs::Box<::std::int32_t> const * $static crubit_nonnull, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = 0;
  ::rs::Box<::std::int32_t> const* $static crubit_nonnull* some_ptr() noexcept {
    return reinterpret_cast<::rs::Box<::std::int32_t> const *
                            $static crubit_nonnull*>(storage_);
  }
  ::rs::Box<::std::int32_t> const* $static crubit_nonnull const*
  some_const_ptr() const noexcept {
    return reinterpret_cast<::rs::Box<::std::int32_t> const *
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

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020SliceBox_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020SliceBox_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: alloc :: boxed :: Box < [i32] , :: alloc :: "
    "alloc :: Global > >")::rs::Option<::rs::SliceBox<::std::int32_t>>
    : public ::rs::OptionBase<::rs::Option<::rs::SliceBox<::std::int32_t>>,
                              ::rs::SliceBox<::std::int32_t>> {
 public:
  // Clone::clone
  Option(const Option&) noexcept;

  // Clone::clone_from
  ::rs::Option<::rs::SliceBox<::std::int32_t>>& operator=(
      const Option&) noexcept;

  Option(Option&&) noexcept;
  ::rs::Option<::rs::SliceBox<::std::int32_t>>& operator=(Option&&) noexcept;
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::rs::SliceBox<::std::int32_t>>,
                       ::rs::SliceBox<::std::int32_t>>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option,
                                              ::rs::SliceBox<::std::int32_t>, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option,
                                              ::rs::SliceBox<::std::int32_t>, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::SliceBox<::std::int32_t>, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::SliceBox<::std::int32_t>, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = 0;
  ::rs::SliceBox<::std::int32_t>* some_ptr() noexcept {
    return reinterpret_cast<::rs::SliceBox<::std::int32_t>*>(storage_);
  }
  ::rs::SliceBox<::std::int32_t> const* some_const_ptr() const noexcept {
    return reinterpret_cast<::rs::SliceBox<::std::int32_t> const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Box_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Box_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: alloc :: boxed :: Box < i32 , :: alloc :: "
    "alloc :: Global > >")::rs::Option<::rs::Box<::std::int32_t>>
    : public ::rs::OptionBase<::rs::Option<::rs::Box<::std::int32_t>>,
                              ::rs::Box<::std::int32_t>> {
 public:
  // Clone::clone
  Option(const Option&) noexcept;

  // Clone::clone_from
  ::rs::Option<::rs::Box<::std::int32_t>>& operator=(const Option&) noexcept;

  Option(Option&&) noexcept;
  ::rs::Option<::rs::Box<::std::int32_t>>& operator=(Option&&) noexcept;
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<::rs::Option<::rs::Box<::std::int32_t>>,
                                     ::rs::Box<::std::int32_t>>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::rs::Box<::std::int32_t>,
                                              U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::rs::Box<::std::int32_t>,
                                              U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::Box<::std::int32_t>, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::Box<::std::int32_t>, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = 0;
  ::rs::Box<::std::int32_t>* some_ptr() noexcept {
    return reinterpret_cast<::rs::Box<::std::int32_t>*>(storage_);
  }
  ::rs::Box<::std::int32_t> const* some_const_ptr() const noexcept {
    return reinterpret_cast<::rs::Box<::std::int32_t> const*>(storage_);
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

namespace boxes {

struct CRUBIT_INTERNAL_RUST_TYPE(":: boxes_golden :: OptHolder") alignas(8)
    [[clang::trivial_abi]] OptHolder final {
 public:
  ::rs::Option<::rs::Box<::std::int32_t>> o{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace boxes

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020StrBox_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020StrBox_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: alloc :: boxed :: Box < str , :: alloc :: "
    "alloc :: Global > >")::rs::Option<::rs::StrBox>
    : public ::rs::OptionBase<::rs::Option<::rs::StrBox>, ::rs::StrBox> {
 public:
  // Clone::clone
  Option(const Option&) noexcept;

  // Clone::clone_from
  ::rs::Option<::rs::StrBox>& operator=(const Option&) noexcept;

  Option(Option&&) noexcept;
  ::rs::Option<::rs::StrBox>& operator=(Option&&) noexcept;
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = ::rs::OptionBase<::rs::Option<::rs::StrBox>, ::rs::StrBox>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::rs::StrBox, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<Option, ::rs::StrBox, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::StrBox, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::StrBox, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = 0;
  ::rs::StrBox* some_ptr() noexcept {
    return reinterpret_cast<::rs::StrBox*>(storage_);
  }
  ::rs::StrBox const* some_const_ptr() const noexcept {
    return reinterpret_cast<::rs::StrBox const*>(storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint64_t tag() const& noexcept;
  constexpr void set_tag(::std::uint64_t tag) noexcept;

 private:
  unsigned char storage_[16];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020PinBox_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020boxes_x00000020_x0000003a_x0000003a_x00000020NotUnpin_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020PinBox_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020boxes_x00000020_x0000003a_x0000003a_x00000020NotUnpin_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    "core :: option :: Option < :: core :: pin :: Pin < :: alloc :: boxed :: "
    "Box < :: boxes_golden :: NotUnpin , :: alloc :: alloc :: Global > > >")::
    rs::Option<::rs::PinBox<::boxes::NotUnpin>>
    : public ::rs::OptionBase<::rs::Option<::rs::PinBox<::boxes::NotUnpin>>,
                              ::rs::PinBox<::boxes::NotUnpin>> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) noexcept;
  ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>& operator=(Option&&) noexcept;
  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      ::rs::OptionBase<::rs::Option<::rs::PinBox<::boxes::NotUnpin>>,
                       ::rs::PinBox<::boxes::NotUnpin>>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, ::rs::PinBox<::boxes::NotUnpin>, U>
  Option(U&& value) noexcept;
  template <typename U>
    requires ::rs::OptionForwardConstructible<
        Option, ::rs::PinBox<::boxes::NotUnpin>, U>
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::PinBox<::boxes::NotUnpin>, Opt>
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires ::rs::OptionFromStdOptional<::rs::PinBox<::boxes::NotUnpin>, Opt>
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  constexpr ~Option() noexcept;

 private:
  friend base_type;
  using tag_type = ::std::uint64_t;
  static constexpr tag_type kNoneVal = 0;
  ::rs::PinBox<::boxes::NotUnpin>* some_ptr() noexcept {
    return reinterpret_cast<::rs::PinBox<::boxes::NotUnpin>*>(storage_);
  }
  ::rs::PinBox<::boxes::NotUnpin> const* some_const_ptr() const noexcept {
    return reinterpret_cast<::rs::PinBox<::boxes::NotUnpin> const*>(storage_);
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

namespace boxes {

static_assert(
    sizeof(::boxes::Holder) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::boxes::Holder) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uboxes_ugolden_x0000003a_x0000003aHolder(
    ::boxes::Holder* __ret_ptr);
}
inline ::boxes::Holder::Holder() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uboxes_ugolden_x0000003a_x0000003aHolder(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_uboxes_ugolden_x0000003a_x0000003aHolder(
    ::boxes::Holder&);
}
inline ::boxes::Holder::~Holder() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uboxes_ugolden_x0000003a_x0000003aHolder(*this);
}
inline ::boxes::Holder::Holder(Holder&& other) noexcept : Holder() {
  *this = ::std::move(other);
}
inline ::boxes::Holder& ::boxes::Holder::operator=(Holder&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::boxes::Holder::Holder(::crubit::UnsafeRelocateTag, Holder&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::boxes::Holder::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::boxes::Holder;
  static_assert(0 == offsetof(__crubit_assert_type, b));
}
static_assert(
    sizeof(::boxes::NotUnpin) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::boxes::NotUnpin) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::boxes::NotUnpin>);
static_assert(::std::is_trivially_move_constructible_v<::boxes::NotUnpin>);
static_assert(::std::is_trivially_move_assignable_v<::boxes::NotUnpin>);
inline ::boxes::NotUnpin::NotUnpin(::crubit::UnsafeRelocateTag,
                                   NotUnpin&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::boxes::NotUnpin::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::boxes::NotUnpin;
  static_assert(0 == offsetof(__crubit_assert_type, val));
}
static_assert(
    sizeof(::boxes::OptHolder) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::boxes::OptHolder) == 8,
    "Verify that ADT layout didn't change since this header got generated");
inline void ::boxes::OptHolder::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::boxes::OptHolder;
  static_assert(0 == offsetof(__crubit_assert_type, o));
}
static_assert(
    sizeof(::boxes::Point) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::boxes::Point) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::boxes::Point>);
static_assert(::std::is_trivially_move_constructible_v<::boxes::Point>);
static_assert(::std::is_trivially_move_assignable_v<::boxes::Point>);
inline void ::boxes::Point::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::boxes::Point;
  static_assert(0 == offsetof(__crubit_assert_type, x));
  static_assert(4 == offsetof(__crubit_assert_type, y));
}
static_assert(
    sizeof(::boxes::Tracked) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::boxes::Tracked) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_uboxes_ugolden_x0000003a_x0000003aTracked(
    ::boxes::Tracked&);
}
inline ::boxes::Tracked::~Tracked() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uboxes_ugolden_x0000003a_x0000003aTracked(
          *this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_uboxes_ugolden_x0000003a_x0000003aTracked(
    ::boxes::Tracked const&, ::boxes::Tracked* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_uboxes_ugolden_x0000003a_x0000003aTracked(
    ::boxes::Tracked&, ::boxes::Tracked const&);
}
inline ::boxes::Tracked::Tracked(const Tracked& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_uboxes_ugolden_x0000003a_x0000003aTracked(
          other, this);
}
inline ::boxes::Tracked& ::boxes::Tracked::operator=(
    const Tracked& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_uboxes_ugolden_x0000003a_x0000003aTracked(
            *this, other);
  }
  return *this;
}
inline ::boxes::Tracked::Tracked(::crubit::UnsafeRelocateTag, Tracked&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::boxes::Tracked::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::boxes::Tracked;
  static_assert(0 == offsetof(__crubit_assert_type, __field0));
}
static_assert(
    sizeof(::boxes::UnmovableHolder) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::boxes::UnmovableHolder) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_uboxes_ugolden_x0000003a_x0000003aUnmovableHolder(
    ::boxes::UnmovableHolder&);
}
inline ::boxes::UnmovableHolder::~UnmovableHolder() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uboxes_ugolden_x0000003a_x0000003aUnmovableHolder(
          *this);
}
inline ::boxes::UnmovableHolder::UnmovableHolder(::crubit::UnsafeRelocateTag,
                                                 UnmovableHolder&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::boxes::UnmovableHolder::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::boxes::UnmovableHolder;
  static_assert(0 == offsetof(__crubit_assert_type, b));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_uhandle(::boxes::Tracked const&,
                                             ::boxes::Tracked* __ret_ptr);
}
inline ::boxes::Tracked clone_handle(::boxes::Tracked const& t) {
  crubit::Slot<::boxes::Tracked> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_clone_uhandle(t, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_consume_ubox(
    ::rs::Box<::std::int32_t>*);
}
inline ::std::int32_t consume_box(::rs::Movable<::rs::Box<::std::int32_t>> b) {
  crubit::Slot<::rs::Box<::std::int32_t>> b_slot;
  ::std::move(b).MoveToSlot(b_slot);
  return __crubit_internal::__crubit_thunk_consume_ubox(b_slot.Get());
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_consume_uopt_ubox(
    ::rs::Option<::rs::Box<::std::int32_t>>*);
}
inline ::std::int32_t consume_opt_box(
    ::rs::Option<::rs::Box<::std::int32_t>> o) {
  crubit::Slot o_slot((::std::move(o)));
  return __crubit_internal::__crubit_thunk_consume_uopt_ubox(o_slot.Get());
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_consume_uslice_ubox(
    ::rs::SliceBox<::std::int32_t>*);
}
inline ::std::uintptr_t consume_slice_box(
    ::rs::Movable<::rs::SliceBox<::std::int32_t>> b) {
  crubit::Slot<::rs::SliceBox<::std::int32_t>> b_slot;
  ::std::move(b).MoveToSlot(b_slot);
  return __crubit_internal::__crubit_thunk_consume_uslice_ubox(b_slot.Get());
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_consume_ustr_ubox(::rs::StrBox*);
}
inline ::std::uintptr_t consume_str_box(::rs::Movable<::rs::StrBox> s) {
  crubit::Slot<::rs::StrBox> s_slot;
  ::std::move(s).MoveToSlot(s_slot);
  return __crubit_internal::__crubit_thunk_consume_ustr_ubox(s_slot.Get());
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_consume_utracked(::rs::Box<::boxes::Tracked>*);
}
inline void consume_tracked(::rs::Movable<::rs::Box<::boxes::Tracked>> _b) {
  crubit::Slot<::rs::Box<::boxes::Tracked>> _b_slot;
  ::std::move(_b).MoveToSlot(_b_slot);
  return __crubit_internal::__crubit_thunk_consume_utracked(_b_slot.Get());
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_holder_uvalue(::boxes::Holder const&);
}
inline ::std::int32_t holder_value(::boxes::Holder const& h) {
  return __crubit_internal::__crubit_thunk_holder_uvalue(h);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_ubox(::std::int32_t,
                                         ::rs::Box<::std::int32_t>* __ret_ptr);
}
inline ::rs::Movable<::rs::Box<::std::int32_t>> make_box(::std::int32_t x) {
  crubit::Slot<::rs::Box<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_ubox(x, __return_value_storage);
  return ::rs::Movable<::rs::Box<::std::int32_t>>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_ubox_unot_uunpin(
    ::rs::Box<::boxes::NotUnpin>* __ret_ptr);
}
inline ::rs::Movable<::rs::Box<::boxes::NotUnpin>> make_box_not_unpin() {
  crubit::Slot<::rs::Box<::boxes::NotUnpin>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_ubox_unot_uunpin(
      __return_value_storage);
  return ::rs::Movable<::rs::Box<::boxes::NotUnpin>>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_uboxed_upoint(
    ::rs::Box<::boxes::Point>* __ret_ptr);
}
inline ::rs::Movable<::rs::Box<::boxes::Point>> make_boxed_point() {
  crubit::Slot<::rs::Box<::boxes::Point>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uboxed_upoint(__return_value_storage);
  return ::rs::Movable<::rs::Box<::boxes::Point>>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_uholder(::boxes::Holder* __ret_ptr);
}
inline ::boxes::Holder make_holder() {
  crubit::Slot<::boxes::Holder> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uholder(__return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_uopt_ubox(
    bool, ::rs::Option<::rs::Box<::std::int32_t>>* __ret_ptr);
}
inline ::rs::Option<::rs::Box<::std::int32_t>> make_opt_box(bool some) {
  crubit::Slot<::rs::Option<::rs::Box<::std::int32_t>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uopt_ubox(some,
                                                   __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_uopt_ubox_uslice(
    bool, ::rs::Option<::rs::SliceBox<::std::int32_t>>* __ret_ptr);
}
inline ::rs::Option<::rs::SliceBox<::std::int32_t>> make_opt_box_slice(
    bool some) {
  crubit::Slot<::rs::Option<::rs::SliceBox<::std::int32_t>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uopt_ubox_uslice(
      some, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_uopt_ubox_ustr(
    bool, ::rs::Option<::rs::StrBox>* __ret_ptr);
}
inline ::rs::Option<::rs::StrBox> make_opt_box_str(bool some) {
  crubit::Slot<::rs::Option<::rs::StrBox>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uopt_ubox_ustr(some,
                                                        __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_uopt_upin_ubox(
    bool, ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>* __ret_ptr);
}
inline ::rs::Option<::rs::PinBox<::boxes::NotUnpin>> make_opt_pin_box(
    bool some) {
  crubit::Slot<::rs::Option<::rs::PinBox<::boxes::NotUnpin>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uopt_upin_ubox(some,
                                                        __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_upin_ubox(
    ::rs::PinBox<::std::int32_t>* __ret_ptr);
}
inline ::rs::Movable<::rs::PinBox<::std::int32_t>> make_pin_box() {
  crubit::Slot<::rs::PinBox<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_upin_ubox(__return_value_storage);
  return ::rs::Movable<::rs::PinBox<::std::int32_t>>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_upin_ubox_unot_uunpin(
    ::rs::PinBox<::boxes::NotUnpin>* __ret_ptr);
}
inline ::rs::Movable<::rs::PinBox<::boxes::NotUnpin>> make_pin_box_not_unpin() {
  crubit::Slot<::rs::PinBox<::boxes::NotUnpin>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_upin_ubox_unot_uunpin(
      __return_value_storage);
  return ::rs::Movable<::rs::PinBox<::boxes::NotUnpin>>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_uslice_ubox(
    ::std::uintptr_t, ::rs::SliceBox<::std::int32_t>* __ret_ptr);
}
inline ::rs::Movable<::rs::SliceBox<::std::int32_t>> make_slice_box(
    ::std::uintptr_t n) {
  crubit::Slot<::rs::SliceBox<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uslice_ubox(n, __return_value_storage);
  return ::rs::Movable<::rs::SliceBox<::std::int32_t>>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_ustr_ubox(::rs::StrBox* __ret_ptr);
}
inline ::rs::Movable<::rs::StrBox> make_str_box() {
  crubit::Slot<::rs::StrBox> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_ustr_ubox(__return_value_storage);
  return ::rs::Movable<::rs::StrBox>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_utracked_ubox(
    ::rs::Box<::boxes::Tracked>* __ret_ptr);
}
inline ::rs::Movable<::rs::Box<::boxes::Tracked>> make_tracked_box() {
  crubit::Slot<::rs::Box<::boxes::Tracked>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_utracked_ubox(__return_value_storage);
  return ::rs::Movable<::rs::Box<::boxes::Tracked>>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_uunmovable_uholder(
    ::boxes::UnmovableHolder* __ret_ptr);
}
inline ::rs::Movable<::boxes::UnmovableHolder> make_unmovable_holder() {
  crubit::Slot<::boxes::UnmovableHolder> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uunmovable_uholder(
      __return_value_storage);
  return ::rs::Movable<::boxes::UnmovableHolder>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_mutate_ubox(::rs::Box<::std::int32_t>&);
}
inline void mutate_box(::rs::Box<::std::int32_t>& b) {
  return __crubit_internal::__crubit_thunk_mutate_ubox(b);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_opt_ubox_uvalue(
    ::rs::Option<::rs::Box<::std::int32_t> const * $static crubit_nonnull>*);
}
inline ::std::int32_t opt_box_value(
    ::rs::Option<::rs::Box<::std::int32_t> const * $static crubit_nonnull> o) {
  return __crubit_internal::__crubit_thunk_opt_ubox_uvalue(&o);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_point_ux(::rs::Box<::boxes::Point>*);
}
inline ::std::int32_t point_x(::rs::Movable<::rs::Box<::boxes::Point>> b) {
  crubit::Slot<::rs::Box<::boxes::Point>> b_slot;
  ::std::move(b).MoveToSlot(b_slot);
  return __crubit_internal::__crubit_thunk_point_ux(b_slot.Get());
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_read_ubox(
    ::rs::Box<::std::int32_t> const&);
}
inline ::std::int32_t read_box(::rs::Box<::std::int32_t> const& b) {
  return __crubit_internal::__crubit_thunk_read_ubox(b);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_read_upin_ubox(
    ::rs::PinBox<::std::int32_t> const&);
}
inline ::std::int32_t read_pin_box(::rs::PinBox<::std::int32_t> const& b) {
  return __crubit_internal::__crubit_thunk_read_upin_ubox(b);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_roundtrip_ubox(
    ::rs::Box<::std::int32_t>*, ::rs::Box<::std::int32_t>* __ret_ptr);
}
inline ::rs::Movable<::rs::Box<::std::int32_t>> roundtrip_box(
    ::rs::Movable<::rs::Box<::std::int32_t>> b) {
  crubit::Slot<::rs::Box<::std::int32_t>> b_slot;
  ::std::move(b).MoveToSlot(b_slot);
  crubit::Slot<::rs::Box<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_roundtrip_ubox(b_slot.Get(),
                                                   __return_value_storage);
  return ::rs::Movable<::rs::Box<::std::int32_t>>::TakeFromSlot(
      ::std::move(__return_value_ret_val_holder));
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_str_ubox_ulen(::rs::StrBox const&);
}
inline ::std::uintptr_t str_box_len(::rs::StrBox const& s) {
  return __crubit_internal::__crubit_thunk_str_ubox_ulen(s);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_sum_uslice_ubox(
    ::rs::SliceBox<::std::int32_t> const&);
}
inline ::std::int32_t sum_slice_box(::rs::SliceBox<::std::int32_t> const& b) {
  return __crubit_internal::__crubit_thunk_sum_uslice_ubox(b);
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_tracked_ustrong_ucount(
    ::boxes::Tracked const&);
}
inline ::std::uintptr_t tracked_strong_count(::boxes::Tracked const& t) {
  return __crubit_internal::__crubit_thunk_tracked_ustrong_ucount(t);
}

}  // namespace boxes

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Box_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020const_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Box_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020const_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<::rs::Option<
                  ::rs::Box<::std::int32_t> const * $static crubit_nonnull>>);
static_assert(::std::is_trivially_copy_assignable_v<::rs::Option<
                  ::rs::Box<::std::int32_t> const * $static crubit_nonnull>>);
static_assert(::std::is_trivially_move_constructible_v<::rs::Option<
                  ::rs::Box<::std::int32_t> const * $static crubit_nonnull>>);
static_assert(::std::is_trivially_move_assignable_v<::rs::Option<
                  ::rs::Box<::std::int32_t> const * $static crubit_nonnull>>);
inline ::rs::Option<::rs::Box<::std::int32_t> const *
                    $static crubit_nonnull>::Option(::crubit::UnsafeRelocateTag,
                                                    Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<::rs::Option<
                  ::rs::Box<::std::int32_t> const * $static crubit_nonnull>>);
inline constexpr ::std::uint64_t(
    ::rs::Option<::rs::Box<::std::int32_t> const *
                 $static crubit_nonnull>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void(::rs::Option<::rs::Box<::std::int32_t> const *
                                   $static crubit_nonnull>::set_tag)(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<
    ::rs::Box<::std::int32_t> const *
    $static crubit_nonnull>::Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::rs::Box<::std::int32_t> const *
                              $static crubit_nonnull>& ::
rs::Option<::rs::Box<::std::int32_t> const * $static crubit_nonnull>::operator=(
    ::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::rs::Box<::std::int32_t> const * $static crubit_nonnull>,
      ::rs::Box<::std::int32_t> const * $static crubit_nonnull, U>
inline ::rs::Option<::rs::Box<::std::int32_t> const *
                    $static crubit_nonnull>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::rs::Box<::std::int32_t> const * $static crubit_nonnull>,
      ::rs::Box<::std::int32_t> const * $static crubit_nonnull, U>
inline ::rs::Option<::rs::Box<::std::int32_t> const *
                    $static crubit_nonnull>& ::
rs::Option<::rs::Box<::std::int32_t> const * $static crubit_nonnull>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<
      ::rs::Box<::std::int32_t> const * $static crubit_nonnull, Opt>
inline ::rs::Option<::rs::Box<::std::int32_t> const *
                    $static crubit_nonnull>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<
      ::rs::Box<::std::int32_t> const * $static crubit_nonnull, Opt>
inline ::rs::Option<::rs::Box<::std::int32_t> const *
                    $static crubit_nonnull>& ::
rs::Option<::rs::Box<::std::int32_t> const * $static crubit_nonnull>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::rs::Box<::std::int32_t> const *
                    $static crubit_nonnull>::Option(::std::in_place_t ip,
                                                    Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020SliceBox_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020SliceBox_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003c_x0000005bi32_x0000005d_x0000003e_x0000003e(
    ::rs::Option<::rs::SliceBox<::std::int32_t>> const&,
    ::rs::Option<::rs::SliceBox<::std::int32_t>>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003c_x0000005bi32_x0000005d_x0000003e_x0000003e(
    ::rs::Option<::rs::SliceBox<::std::int32_t>>&,
    ::rs::Option<::rs::SliceBox<::std::int32_t>> const&);
}
inline ::rs::Option<::rs::SliceBox<::std::int32_t>>::Option(
    const Option& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003c_x0000005bi32_x0000005d_x0000003e_x0000003e(
          other, this);
}
inline ::rs::Option<::rs::SliceBox<::std::int32_t>>& ::rs::Option<
    ::rs::SliceBox<::std::int32_t>>::operator=(const Option& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003c_x0000005bi32_x0000005d_x0000003e_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Option<::rs::SliceBox<::std::int32_t>>::Option(
    Option&& other) noexcept
    : Option() {
  *this = ::std::move(other);
}
inline ::rs::Option<::rs::SliceBox<::std::int32_t>>& ::rs::Option<
    ::rs::SliceBox<::std::int32_t>>::operator=(Option&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::rs::Option<::rs::SliceBox<::std::int32_t>>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr ::rs::Option<
    ::rs::SliceBox<::std::int32_t>>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint64_t(
    ::rs::Option<::rs::SliceBox<::std::int32_t>>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void(::rs::Option<::rs::SliceBox<::std::int32_t>>::set_tag)(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::rs::SliceBox<::std::int32_t>>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::rs::SliceBox<::std::int32_t>>& ::rs::Option<
    ::rs::SliceBox<::std::int32_t>>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::rs::SliceBox<::std::int32_t>>,
      ::rs::SliceBox<::std::int32_t>, U>
inline ::rs::Option<::rs::SliceBox<::std::int32_t>>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::rs::SliceBox<::std::int32_t>>,
      ::rs::SliceBox<::std::int32_t>, U>
inline ::rs::Option<::rs::SliceBox<::std::int32_t>>& ::rs::Option<
    ::rs::SliceBox<::std::int32_t>>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::SliceBox<::std::int32_t>, Opt>
inline ::rs::Option<::rs::SliceBox<::std::int32_t>>::Option(
    Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::SliceBox<::std::int32_t>, Opt>
inline ::rs::Option<::rs::SliceBox<::std::int32_t>>& ::rs::Option<
    ::rs::SliceBox<::std::int32_t>>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::rs::SliceBox<::std::int32_t>>::Option(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Box_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Box_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003ci32_x0000003e_x0000003e(
    ::rs::Option<::rs::Box<::std::int32_t>> const&,
    ::rs::Option<::rs::Box<::std::int32_t>>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003ci32_x0000003e_x0000003e(
    ::rs::Option<::rs::Box<::std::int32_t>>&,
    ::rs::Option<::rs::Box<::std::int32_t>> const&);
}
inline ::rs::Option<::rs::Box<::std::int32_t>>::Option(
    const Option& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003ci32_x0000003e_x0000003e(
          other, this);
}
inline ::rs::Option<::rs::Box<::std::int32_t>>& ::rs::Option<
    ::rs::Box<::std::int32_t>>::operator=(const Option& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003ci32_x0000003e_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Option<::rs::Box<::std::int32_t>>::Option(Option&& other) noexcept
    : Option() {
  *this = ::std::move(other);
}
inline ::rs::Option<::rs::Box<::std::int32_t>>& ::rs::Option<
    ::rs::Box<::std::int32_t>>::operator=(Option&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::rs::Option<::rs::Box<::std::int32_t>>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr ::rs::Option<::rs::Box<::std::int32_t>>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint64_t(
    ::rs::Option<::rs::Box<::std::int32_t>>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void(::rs::Option<::rs::Box<::std::int32_t>>::set_tag)(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::rs::Box<::std::int32_t>>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::rs::Box<::std::int32_t>>& ::rs::Option<
    ::rs::Box<::std::int32_t>>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::rs::Box<::std::int32_t>>, ::rs::Box<::std::int32_t>, U>
inline ::rs::Option<::rs::Box<::std::int32_t>>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::rs::Box<::std::int32_t>>, ::rs::Box<::std::int32_t>, U>
inline ::rs::Option<::rs::Box<::std::int32_t>>& ::rs::Option<
    ::rs::Box<::std::int32_t>>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::Box<::std::int32_t>, Opt>
inline ::rs::Option<::rs::Box<::std::int32_t>>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::Box<::std::int32_t>, Opt>
inline ::rs::Option<::rs::Box<::std::int32_t>>& ::rs::Option<
    ::rs::Box<::std::int32_t>>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::rs::Box<::std::int32_t>>::Option(::std::in_place_t ip,
                                                       Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020StrBox_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020StrBox_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003cstr_x0000003e_x0000003e(
    ::rs::Option<::rs::StrBox> const&, ::rs::Option<::rs::StrBox>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003cstr_x0000003e_x0000003e(
    ::rs::Option<::rs::StrBox>&, ::rs::Option<::rs::StrBox> const&);
}
inline ::rs::Option<::rs::StrBox>::Option(const Option& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003cstr_x0000003e_x0000003e(
          other, this);
}
inline ::rs::Option<::rs::StrBox>& ::rs::Option<::rs::StrBox>::operator=(
    const Option& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003aoption_x0000003a_x0000003aOption_x0000003cstd_x0000003a_x0000003aboxed_x0000003a_x0000003aBox_x0000003cstr_x0000003e_x0000003e(
            *this, other);
  }
  return *this;
}
inline ::rs::Option<::rs::StrBox>::Option(Option&& other) noexcept : Option() {
  *this = ::std::move(other);
}
inline ::rs::Option<::rs::StrBox>& ::rs::Option<::rs::StrBox>::operator=(
    Option&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::rs::Option<::rs::StrBox>::Option(::crubit::UnsafeRelocateTag,
                                          Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr ::rs::Option<::rs::StrBox>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint64_t(
    ::rs::Option<::rs::StrBox>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void(::rs::Option<::rs::StrBox>::set_tag)(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::rs::StrBox>::Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::rs::StrBox>& ::rs::Option<
    ::rs::StrBox>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::rs::StrBox>,
                                            ::rs::StrBox, U>
inline ::rs::Option<::rs::StrBox>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<::rs::Option<::rs::StrBox>,
                                            ::rs::StrBox, U>
inline ::rs::Option<::rs::StrBox>& ::rs::Option<::rs::StrBox>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::StrBox, Opt>
inline ::rs::Option<::rs::StrBox>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::StrBox, Opt>
inline ::rs::Option<::rs::StrBox>& ::rs::Option<::rs::StrBox>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::rs::StrBox>::Option(::std::in_place_t ip,
                                          Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#ifndef _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020PinBox_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020boxes_x00000020_x0000003a_x0000003a_x00000020NotUnpin_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL__x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020rs_x00000020_x0000003a_x0000003a_x00000020PinBox_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020boxes_x00000020_x0000003a_x0000003a_x00000020NotUnpin_x00000020_x0000003e_x00000020_x0000003e
inline ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>::Option(
    Option&& other) noexcept
    : Option() {
  *this = ::std::move(other);
}
inline ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>& ::rs::Option<
    ::rs::PinBox<::boxes::NotUnpin>>::operator=(Option&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline constexpr ::rs::Option<
    ::rs::PinBox<::boxes::NotUnpin>>::~Option() noexcept {
  this->reset();
}
inline constexpr ::std::uint64_t(
    ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>::tag)() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void(::rs::Option<::rs::PinBox<::boxes::NotUnpin>>::set_tag)(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>& ::rs::Option<
    ::rs::PinBox<::boxes::NotUnpin>>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>,
      ::rs::PinBox<::boxes::NotUnpin>, U>
inline ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires ::rs::OptionForwardConstructible<
      ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>,
      ::rs::PinBox<::boxes::NotUnpin>, U>
inline ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>& ::rs::Option<
    ::rs::PinBox<::boxes::NotUnpin>>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::PinBox<::boxes::NotUnpin>, Opt>
inline ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>::Option(
    Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires ::rs::OptionFromStdOptional<::rs::PinBox<::boxes::NotUnpin>, Opt>
inline ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>& ::rs::Option<
    ::rs::PinBox<::boxes::NotUnpin>>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline ::rs::Option<::rs::PinBox<::boxes::NotUnpin>>::Option(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // CC_BINDINGS_FROM_RS_TEST_BOX_BOXES_GOLDEN
