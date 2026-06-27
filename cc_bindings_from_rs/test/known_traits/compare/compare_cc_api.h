// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// compare_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_COMPARE_COMPARE_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_COMPARE_COMPARE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/option.h"
#include "support/rs_std/traits.h"

#include <bit>
#include <compare>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <optional>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_core.h"

namespace compare {

struct CRUBIT_INTERNAL_RUST_TYPE(":: compare_golden :: MyOrd") alignas(4)
    [[clang::trivial_abi]] MyOrd final {
 public:
  // `compare_golden::MyOrd` doesn't implement the `Default` trait
  MyOrd() = delete;

  // Synthesized tuple constructor
  explicit MyOrd(::std::int32_t __field0) : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyOrd() = default;
  MyOrd(MyOrd&&) = default;
  MyOrd& operator=(MyOrd&&) = default;

  // `compare_golden::MyOrd` doesn't implement the `Clone` trait
  MyOrd(const MyOrd&) = delete;
  MyOrd& operator=(const MyOrd&) = delete;
  MyOrd(::crubit::UnsafeRelocateTag, MyOrd&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool operator==(::compare::MyOrd const& other) const;

  ::std::strong_ordering operator<=>(const MyOrd& other) const;

  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: compare_golden :: MyUnordered") alignas(4)
    [[clang::trivial_abi]] MyUnordered final {
 public:
  // `compare_golden::MyUnordered` doesn't implement the `Default` trait
  MyUnordered() = delete;

  // Synthesized tuple constructor
  explicit MyUnordered(float __field0) : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyUnordered() = default;
  MyUnordered(MyUnordered&&) = default;
  MyUnordered& operator=(MyUnordered&&) = default;

  // `compare_golden::MyUnordered` doesn't implement the `Clone` trait
  MyUnordered(const MyUnordered&) = delete;
  MyUnordered& operator=(const MyUnordered&) = delete;
  MyUnordered(::crubit::UnsafeRelocateTag, MyUnordered&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool operator==(::compare::MyUnordered const& other) const;

  ::std::partial_ordering operator<=>(
      ::compare::MyUnordered const& other) const;

  union {
    float __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace compare

template <>
struct rs_std::impl<::compare::MyOrd, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct rs_std::impl<::compare::MyOrd, ::rs::core::cmp::Ord> {
  static constexpr bool kIsImplemented = true;

  static ::rs::core::cmp::Ordering cmp(::compare::MyOrd const& self,
                                       ::compare::MyOrd const& other);
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

  Option(::crubit::UnsafeRelocateTag, Option&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  using base_type =
      rs_std::OptionBase<rs_std::Option<::std::int32_t>, ::std::int32_t>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(!std::is_base_of_v<Option, std::decay_t<U>> &&
             !std::is_same_v<std::decay_t<U>, ::std::nullopt_t> &&
             !std::is_same_v<std::decay_t<U>, ::std::in_place_t> &&
             std::is_constructible_v<::std::int32_t, U>)
  Option(U&& value) noexcept : base_type(::std::forward<U>(value)) {}
  template <typename U>
    requires(!std::is_base_of_v<Option, std::decay_t<U>> &&
             !std::is_same_v<std::decay_t<U>, ::std::nullopt_t> &&
             !std::is_same_v<std::decay_t<U>, ::std::in_place_t> &&
             std::is_constructible_v<::std::int32_t, U>)
  Option& operator=(U&& value) noexcept {
    base_type::operator=(::std::forward<U>(value));
    return *this;
  }
  template <typename Opt>
    requires(
        std::is_same_v<std::decay_t<Opt>, ::std::optional<::std::int32_t>> &&
        !std::is_lvalue_reference_v<Opt>)
  Option(Opt&& value) noexcept : base_type(::std::forward<Opt>(value)) {}
  template <typename Opt>
    requires(
        std::is_same_v<std::decay_t<Opt>, ::std::optional<::std::int32_t>> &&
        !std::is_lvalue_reference_v<Opt>)
  Option& operator=(Opt&& value) noexcept {
    base_type::operator=(::std::forward<Opt>(value));
    return *this;
  }
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept
      : base_type(ip, ::std::forward<Args>(args)...) {}
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

namespace compare {

struct CRUBIT_INTERNAL_RUST_TYPE(":: compare_golden :: MyPartialOrd") alignas(4)
    [[clang::trivial_abi]] MyPartialOrd final {
 public:
  // `compare_golden::MyPartialOrd` doesn't implement the `Default` trait
  MyPartialOrd() = delete;

  // Synthesized tuple constructor
  explicit MyPartialOrd(rs_std::Option<::std::int32_t> __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyPartialOrd() = default;
  MyPartialOrd(MyPartialOrd&&) = default;
  MyPartialOrd& operator=(MyPartialOrd&&) = default;

  // `compare_golden::MyPartialOrd` doesn't implement the `Clone` trait
  MyPartialOrd(const MyPartialOrd&) = delete;
  MyPartialOrd& operator=(const MyPartialOrd&) = delete;
  MyPartialOrd(::crubit::UnsafeRelocateTag, MyPartialOrd&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool operator==(::compare::MyPartialOrd const& other) const;

  ::std::partial_ordering operator<=>(
      ::compare::MyPartialOrd const& other) const;

  union {
    rs_std::Option<::std::int32_t> __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(MyOrd) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyOrd) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyOrd>);
static_assert(::std::is_trivially_move_constructible_v<::compare::MyOrd>);
static_assert(::std::is_trivially_move_assignable_v<::compare::MyOrd>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_ucompare_ugolden_x0000003a_x0000003aMyOrd_ucompare_ugolden_x0000003a_x0000003aMyOrd(
    ::compare::MyOrd const&, ::compare::MyOrd const&);
}
inline bool MyOrd::operator==(::compare::MyOrd const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_ucompare_ugolden_x0000003a_x0000003aMyOrd_ucompare_ugolden_x0000003a_x0000003aMyOrd(
          self, other);
}
namespace __crubit_internal {
extern "C" ::std::int8_t __crubit_thunk_cmp(::compare::MyOrd const&,
                                            ::compare::MyOrd const&);
}
inline ::std::strong_ordering MyOrd::operator<=>(const MyOrd& other) const {
  auto val = __crubit_internal::__crubit_thunk_cmp(*this, other);
  return val < 0 ? ::std::strong_ordering::less
                 : (val > 0 ? ::std::strong_ordering::greater
                            : ::std::strong_ordering::equal);
}
inline void MyOrd::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyOrd, __field0));
}
static_assert(
    sizeof(MyPartialOrd) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyPartialOrd) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyPartialOrd>);
static_assert(
    ::std::is_trivially_move_constructible_v<::compare::MyPartialOrd>);
static_assert(::std::is_trivially_move_assignable_v<::compare::MyPartialOrd>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_ucompare_ugolden_x0000003a_x0000003aMyPartialOrd_ucompare_ugolden_x0000003a_x0000003aMyPartialOrd(
    ::compare::MyPartialOrd const&, ::compare::MyPartialOrd const&);
}
inline bool MyPartialOrd::operator==(
    ::compare::MyPartialOrd const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_ucompare_ugolden_x0000003a_x0000003aMyPartialOrd_ucompare_ugolden_x0000003a_x0000003aMyPartialOrd(
          self, other);
}
namespace __crubit_internal {
extern "C" ::std::int8_t
__crubit_thunk_partial_ucmp_ucompare_ugolden_x0000003a_x0000003aMyPartialOrd(
    ::compare::MyPartialOrd const&, ::compare::MyPartialOrd const&);
}
inline ::std::partial_ordering MyPartialOrd::operator<=>(
    ::compare::MyPartialOrd const& other) const {
  auto val = __crubit_internal::
      __crubit_thunk_partial_ucmp_ucompare_ugolden_x0000003a_x0000003aMyPartialOrd(
          *this, other);
  return val == 2 ? ::std::partial_ordering::unordered
                  : (val < 0 ? ::std::partial_ordering::less
                             : (val > 0 ? ::std::partial_ordering::greater
                                        : ::std::partial_ordering::equivalent));
}
inline void MyPartialOrd::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyPartialOrd, __field0));
}
static_assert(
    sizeof(MyUnordered) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyUnordered) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyUnordered>);
static_assert(::std::is_trivially_move_constructible_v<::compare::MyUnordered>);
static_assert(::std::is_trivially_move_assignable_v<::compare::MyUnordered>);
namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_ucompare_ugolden_x0000003a_x0000003aMyUnordered_ucompare_ugolden_x0000003a_x0000003aMyUnordered(
    ::compare::MyUnordered const&, ::compare::MyUnordered const&);
}
inline bool MyUnordered::operator==(::compare::MyUnordered const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_ucompare_ugolden_x0000003a_x0000003aMyUnordered_ucompare_ugolden_x0000003a_x0000003aMyUnordered(
          self, other);
}
namespace __crubit_internal {
extern "C" ::std::int8_t
__crubit_thunk_partial_ucmp_ucompare_ugolden_x0000003a_x0000003aMyUnordered(
    ::compare::MyUnordered const&, ::compare::MyUnordered const&);
}
inline ::std::partial_ordering MyUnordered::operator<=>(
    ::compare::MyUnordered const& other) const {
  auto val = __crubit_internal::
      __crubit_thunk_partial_ucmp_ucompare_ugolden_x0000003a_x0000003aMyUnordered(
          *this, other);
  return val == 2 ? ::std::partial_ordering::unordered
                  : (val < 0 ? ::std::partial_ordering::less
                             : (val > 0 ? ::std::partial_ordering::greater
                                        : ::std::partial_ordering::equivalent));
}
inline void MyUnordered::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyUnordered, __field0));
}
}  // namespace compare

namespace compare {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Ord_ucmp_ucompare_ugolden_x0000003a_x0000003aMyOrd(
    ::compare::MyOrd const&, ::compare::MyOrd const&,
    ::rs::core::cmp::Ordering* __ret_ptr);
}
}  // namespace compare
inline ::rs::core::cmp::Ordering
rs_std::impl<::compare::MyOrd, ::rs::core::cmp::Ord>::cmp(
    ::compare::MyOrd const& self, ::compare::MyOrd const& other) {
  crubit::Slot<::rs::core::cmp::Ordering> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  compare::__crubit_internal::
      __crubit_thunk_Ord_ucmp_ucompare_ugolden_x0000003a_x0000003aMyOrd(
          self, other, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

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

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_COMPARE_COMPARE_GOLDEN
