// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// stdlib_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_STDLIB_STDLIB_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_STDLIB_STDLIB_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/iterator_adapter.h"
#include "support/rs_std/option.h"
#include "support/rs_std/slice_ref.h"
#include "support/rs_std/traits.h"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <optional>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"
#include "support/rs_std/rs_core.h"

namespace stdlib {

struct CRUBIT_INTERNAL_RUST_TYPE(":: stdlib_golden :: MyStruct") alignas(4)
    [[clang::trivial_abi]] MyStruct final {
 public:
  // Default::default
  MyStruct();

  // Drop::drop
  ~MyStruct();

  MyStruct(MyStruct&&);
  ::stdlib::MyStruct& operator=(MyStruct&&);

  // Clone::clone
  MyStruct(const MyStruct&);

  // Clone::clone_from
  ::stdlib::MyStruct& operator=(const MyStruct&);

  MyStruct(::crubit::UnsafeRelocateTag, MyStruct&& value);

  static ::stdlib::MyStruct new_(::std::int32_t x);

  explicit MyStruct(::std::int32_t value);

  template <typename TAdaptedSelf_ = MyStruct>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }
  union {
    ::std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: stdlib_golden :: MyTrait") MyTrait {
  template <typename T>
  using impl = rs_std::impl<T, MyTrait>;
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: stdlib_golden :: NonCloneableIterator") alignas(4)
    [[clang::trivial_abi]] NonCloneableIterator final {
 public:
  // `stdlib_golden::NonCloneableIterator` doesn't implement the `Default` trait
  NonCloneableIterator() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NonCloneableIterator() = default;
  NonCloneableIterator(NonCloneableIterator&&) = default;
  NonCloneableIterator& operator=(NonCloneableIterator&&) = default;

  // `stdlib_golden::NonCloneableIterator` doesn't implement the `Clone` trait
  NonCloneableIterator(const NonCloneableIterator&) = delete;
  NonCloneableIterator& operator=(const NonCloneableIterator&) = delete;
  NonCloneableIterator(::crubit::UnsafeRelocateTag,
                       NonCloneableIterator&& value);

  static ::stdlib::NonCloneableIterator new_(::std::int32_t x);
  template <typename TAdaptedSelf_ = NonCloneableIterator>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }
  union {
    ::std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: stdlib_golden :: NonCloneableValue") alignas(4) [[clang::trivial_abi]]
NonCloneableValue final {
 public:
  // `stdlib_golden::NonCloneableValue` doesn't implement the `Default` trait
  NonCloneableValue() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NonCloneableValue() = default;
  NonCloneableValue(NonCloneableValue&&) = default;
  NonCloneableValue& operator=(NonCloneableValue&&) = default;

  // `stdlib_golden::NonCloneableValue` doesn't implement the `Clone` trait
  NonCloneableValue(const NonCloneableValue&) = delete;
  NonCloneableValue& operator=(const NonCloneableValue&) = delete;
  NonCloneableValue(::crubit::UnsafeRelocateTag, NonCloneableValue&& value);

  union {
    ::std::int32_t x;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: stdlib_golden :: RefIterator") alignas(8)
    [[clang::trivial_abi]] RefIterator final {
 public:
  // `stdlib_golden::RefIterator` doesn't implement the `Default` trait
  RefIterator() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~RefIterator() = default;
  RefIterator(RefIterator&&) = default;
  RefIterator& operator=(RefIterator&&) = default;

  // `stdlib_golden::RefIterator` doesn't implement the `Clone` trait
  RefIterator(const RefIterator&) = delete;
  RefIterator& operator=(const RefIterator&) = delete;
  RefIterator(::crubit::UnsafeRelocateTag, RefIterator&& value);

  static ::stdlib::RefIterator new_(
      rs_std::SliceRef<const ::std::int32_t> slice);
  template <typename TAdaptedSelf_ = RefIterator>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }
  union {
    rs_std::SliceRef<const ::std::int32_t> slice;
  };
  union {
    ::std::uintptr_t index;
  };

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace stdlib

template <>
struct rs_std::impl<::stdlib::MyStruct, ::rs::core::future::Future> {
  static constexpr bool kIsImplemented = true;
  using Output CRUBIT_INTERNAL_RUST_TYPE(
      "<stdlib_golden::MyStruct as :: core :: future :: Future>::Output") =
      ::std::int32_t;

  // Error generating bindings for associated function `<stdlib_golden::MyStruct
  // as std::future::Future>::poll` defined at
  // cc_bindings_from_rs/test/traits/stdlib/stdlib.rs;l=72:
  // Unsupported `self` type `std::pin::Pin<&'__anon1 mut
  // stdlib_golden::MyStruct>`
};

template <>
struct rs_std::impl<::stdlib::MyStruct, ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<stdlib_golden::MyStruct as :: core :: iter :: Iterator>::Item") =
      ::std::int32_t;

  static rs_std::Option<::std::int32_t> next(::stdlib::MyStruct& self);
};

template <>
struct rs_std::impl<::stdlib::MyStruct, ::rs::alloc::string::ToString> {
  static constexpr bool kIsImplemented = true;

  static ::rs::alloc::string::String to_string(::stdlib::MyStruct const& self);
};

template <>
struct rs_std::impl<::stdlib::NonCloneableIterator,
                    ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<stdlib_golden::NonCloneableIterator as :: core :: iter :: "
      "Iterator>::Item") = ::stdlib::NonCloneableValue;

  static rs_std::Option<::stdlib::NonCloneableValue> next(
      ::stdlib::NonCloneableIterator& self);
};

template <>
struct rs_std::impl<::stdlib::RefIterator, ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<stdlib_golden::RefIterator<'a> as :: core :: iter :: Iterator>::Item") =
      ::std::int32_t const* $a crubit_nonnull;

  static rs_std::Option<::std::int32_t const * $static crubit_nonnull> next(
      ::stdlib::RefIterator& self);
};
#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020_d_x00000020static_x00000020crubit_unonnull_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020_d_x00000020static_x00000020crubit_unonnull_x00000020_x0000003e
template <>
struct alignas(8)
    CRUBIT_INTERNAL_RUST_TYPE("std :: option :: Option < & 'static i32 >")
        rs_std::Option<::std::int32_t const * $static crubit_nonnull>
    : public rs_std::OptionBase<
          rs_std::Option<::std::int32_t const * $static crubit_nonnull>,
          ::std::int32_t const * $static crubit_nonnull> {
 public:
  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  Option(const Option&) = default;
  Option& operator=(const Option&) = default;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type = rs_std::OptionBase<
      rs_std::Option<::std::int32_t const * $static crubit_nonnull>,
      ::std::int32_t const * $static crubit_nonnull>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option, ::std::int32_t const * $static crubit_nonnull, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option, ::std::int32_t const * $static crubit_nonnull, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             ::std::int32_t const * $static crubit_nonnull, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             ::std::int32_t const * $static crubit_nonnull, Opt>)
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

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020stdlib_x00000020_x0000003a_x0000003a_x00000020NonCloneableValue_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020stdlib_x00000020_x0000003a_x0000003a_x00000020NonCloneableValue_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: stdlib_golden :: NonCloneableValue >")
    rs_std::Option<::stdlib::NonCloneableValue>
    : public rs_std::OptionBase<rs_std::Option<::stdlib::NonCloneableValue>,
                                ::stdlib::NonCloneableValue> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<::stdlib::NonCloneableValue>,
                         ::stdlib::NonCloneableValue>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option,
                                                ::stdlib::NonCloneableValue, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<Option,
                                                ::stdlib::NonCloneableValue, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::stdlib::NonCloneableValue, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<::stdlib::NonCloneableValue, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = 0;
  ::stdlib::NonCloneableValue* some_ptr() noexcept {
    return reinterpret_cast<::stdlib::NonCloneableValue*>(storage_ + 4);
  }
  ::stdlib::NonCloneableValue const* some_const_ptr() const noexcept {
    return reinterpret_cast<::stdlib::NonCloneableValue const*>(storage_ + 4);
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

namespace stdlib {

static_assert(
    sizeof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    ::stdlib::MyStruct* __ret_ptr);
}
inline ::stdlib::MyStruct::MyStruct() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    ::stdlib::MyStruct&);
}
inline MyStruct::~MyStruct() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
          *this);
}
inline ::stdlib::MyStruct::MyStruct(MyStruct&& other) : MyStruct() {
  *this = ::std::move(other);
}
inline ::stdlib::MyStruct& ::stdlib::MyStruct::operator=(MyStruct&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    ::stdlib::MyStruct const&, ::stdlib::MyStruct* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    ::stdlib::MyStruct&, ::stdlib::MyStruct const&);
}
inline ::stdlib::MyStruct::MyStruct(const MyStruct& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
          other, this);
}
inline ::stdlib::MyStruct& ::stdlib::MyStruct::operator=(
    const MyStruct& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
            *this, other);
  }
  return *this;
}
inline ::stdlib::MyStruct::MyStruct(::crubit::UnsafeRelocateTag,
                                    MyStruct&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t,
                                   ::stdlib::MyStruct* __ret_ptr);
}
inline ::stdlib::MyStruct MyStruct::new_(::std::int32_t x) {
  crubit::Slot<::stdlib::MyStruct> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_From_ufrom_ustdlib_ugolden_x0000003a_x0000003aMyStruct_ui32(
    ::std::int32_t, ::stdlib::MyStruct* __ret_ptr);
}
inline MyStruct::MyStruct(::std::int32_t value) {
  __crubit_internal::
      __crubit_thunk_From_ufrom_ustdlib_ugolden_x0000003a_x0000003aMyStruct_ui32(
          value, this);
}
inline void MyStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyStruct, x));
}
static_assert(
    sizeof(NonCloneableIterator) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonCloneableIterator) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NonCloneableIterator>);
static_assert(
    ::std::is_trivially_move_constructible_v<::stdlib::NonCloneableIterator>);
static_assert(
    ::std::is_trivially_move_assignable_v<::stdlib::NonCloneableIterator>);
inline ::stdlib::NonCloneableIterator::NonCloneableIterator(
    ::crubit::UnsafeRelocateTag, NonCloneableIterator&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(::std::int32_t,
                                   ::stdlib::NonCloneableIterator* __ret_ptr);
}
inline ::stdlib::NonCloneableIterator NonCloneableIterator::new_(
    ::std::int32_t x) {
  crubit::Slot<::stdlib::NonCloneableIterator> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void NonCloneableIterator::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NonCloneableIterator, x));
}
static_assert(
    sizeof(NonCloneableValue) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonCloneableValue) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NonCloneableValue>);
static_assert(
    ::std::is_trivially_move_constructible_v<::stdlib::NonCloneableValue>);
static_assert(
    ::std::is_trivially_move_assignable_v<::stdlib::NonCloneableValue>);
inline ::stdlib::NonCloneableValue::NonCloneableValue(
    ::crubit::UnsafeRelocateTag, NonCloneableValue&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void NonCloneableValue::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NonCloneableValue, x));
}
static_assert(
    sizeof(RefIterator) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(RefIterator) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<RefIterator>);
static_assert(::std::is_trivially_move_constructible_v<::stdlib::RefIterator>);
static_assert(::std::is_trivially_move_assignable_v<::stdlib::RefIterator>);
inline ::stdlib::RefIterator::RefIterator(::crubit::UnsafeRelocateTag,
                                          RefIterator&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_new(rs_std::SliceRef<const ::std::int32_t>,
                                   ::stdlib::RefIterator* __ret_ptr);
}
inline ::stdlib::RefIterator RefIterator::new_(
    rs_std::SliceRef<const ::std::int32_t> slice) {
  crubit::Slot<::stdlib::RefIterator> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_new(slice, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void RefIterator::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(RefIterator, slice));
  static_assert(16 == offsetof(RefIterator, index));
}
}  // namespace stdlib

namespace stdlib {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    ::stdlib::MyStruct&, rs_std::Option<::std::int32_t>* __ret_ptr);
}
}  // namespace stdlib
inline rs_std::Option<::std::int32_t>
rs_std::impl<::stdlib::MyStruct, ::rs::core::iter::Iterator>::next(
    ::stdlib::MyStruct& self) {
  crubit::Slot<rs_std::Option<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  stdlib::__crubit_internal::
      __crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace stdlib {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_ToString_uto_ustring_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
    ::stdlib::MyStruct const&, ::rs::alloc::string::String* __ret_ptr);
}
}  // namespace stdlib
inline ::rs::alloc::string::String
rs_std::impl<::stdlib::MyStruct, ::rs::alloc::string::ToString>::to_string(
    ::stdlib::MyStruct const& self) {
  crubit::Slot<::rs::alloc::string::String> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  stdlib::__crubit_internal::
      __crubit_thunk_ToString_uto_ustring_ustdlib_ugolden_x0000003a_x0000003aMyStruct(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace stdlib {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aNonCloneableIterator(
    ::stdlib::NonCloneableIterator&,
    rs_std::Option<::stdlib::NonCloneableValue>* __ret_ptr);
}
}  // namespace stdlib
inline rs_std::Option<::stdlib::NonCloneableValue>
rs_std::impl<::stdlib::NonCloneableIterator, ::rs::core::iter::Iterator>::next(
    ::stdlib::NonCloneableIterator& self) {
  crubit::Slot<rs_std::Option<::stdlib::NonCloneableValue>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  stdlib::__crubit_internal::
      __crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aNonCloneableIterator(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace stdlib {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aRefIterator_x0000003c_x00000027a_x0000003e(
    ::stdlib::RefIterator&,
    rs_std::Option<::std::int32_t const * $static crubit_nonnull>* __ret_ptr);
}
}  // namespace stdlib
inline rs_std::Option<::std::int32_t const * $static crubit_nonnull>
rs_std::impl<::stdlib::RefIterator, ::rs::core::iter::Iterator>::next(
    ::stdlib::RefIterator& self) {
  crubit::Slot<rs_std::Option<::std::int32_t const * $static crubit_nonnull>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  stdlib::__crubit_internal::
      __crubit_thunk_Iterator_unext_ustdlib_ugolden_x0000003a_x0000003aRefIterator_x0000003c_x00000027a_x0000003e(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020_d_x00000020static_x00000020crubit_unonnull_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020_d_x00000020static_x00000020crubit_unonnull_x00000020_x0000003e
static_assert(::std::is_trivially_copy_constructible_v<
              rs_std::Option<::std::int32_t const * $static crubit_nonnull>>);
static_assert(::std::is_trivially_copy_assignable_v<
              rs_std::Option<::std::int32_t const * $static crubit_nonnull>>);
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Option<::std::int32_t const * $static crubit_nonnull>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Option<::std::int32_t const * $static crubit_nonnull>>);
inline rs_std::Option<::std::int32_t const * $static crubit_nonnull>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Option<::std::int32_t const * $static crubit_nonnull>>);
inline constexpr ::std::uint64_t rs_std::Option<
    ::std::int32_t const * $static crubit_nonnull>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint64_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint64_t>(__bytes);
}
inline constexpr void
rs_std::Option<::std::int32_t const * $static crubit_nonnull>::set_tag(
    ::std::uint64_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint64_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint64_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::std::int32_t const * $static crubit_nonnull>::
    Option(::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::std::int32_t const * $static crubit_nonnull>&
rs_std::Option<::std::int32_t const * $static crubit_nonnull>::operator=(
    ::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::std::int32_t const * $static crubit_nonnull>,
           ::std::int32_t const * $static crubit_nonnull, U>)
inline rs_std::Option<::std::int32_t const * $static crubit_nonnull>::Option(
    U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::std::int32_t const * $static crubit_nonnull>,
           ::std::int32_t const * $static crubit_nonnull, U>)
inline rs_std::Option<::std::int32_t const * $static crubit_nonnull>&
rs_std::Option<::std::int32_t const * $static crubit_nonnull>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<
           ::std::int32_t const * $static crubit_nonnull, Opt>)
inline rs_std::Option<::std::int32_t const * $static crubit_nonnull>::Option(
    Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<
           ::std::int32_t const * $static crubit_nonnull, Opt>)
inline rs_std::Option<::std::int32_t const * $static crubit_nonnull>&
rs_std::Option<::std::int32_t const * $static crubit_nonnull>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::std::int32_t const * $static crubit_nonnull>::Option(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

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

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020stdlib_x00000020_x0000003a_x0000003a_x00000020NonCloneableValue_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020stdlib_x00000020_x0000003a_x0000003a_x00000020NonCloneableValue_x00000020_x0000003e
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Option<::stdlib::NonCloneableValue>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Option<::stdlib::NonCloneableValue>>);
inline rs_std::Option<::stdlib::NonCloneableValue>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Option<::stdlib::NonCloneableValue>>);
inline constexpr ::std::uint32_t
rs_std::Option<::stdlib::NonCloneableValue>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void rs_std::Option<::stdlib::NonCloneableValue>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::stdlib::NonCloneableValue>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::stdlib::NonCloneableValue>& rs_std::Option<
    ::stdlib::NonCloneableValue>::operator=(::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::stdlib::NonCloneableValue>,
           ::stdlib::NonCloneableValue, U>)
inline rs_std::Option<::stdlib::NonCloneableValue>::Option(U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::stdlib::NonCloneableValue>,
           ::stdlib::NonCloneableValue, U>)
inline rs_std::Option<::stdlib::NonCloneableValue>&
rs_std::Option<::stdlib::NonCloneableValue>::operator=(U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::stdlib::NonCloneableValue, Opt>)
inline rs_std::Option<::stdlib::NonCloneableValue>::Option(Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::stdlib::NonCloneableValue, Opt>)
inline rs_std::Option<::stdlib::NonCloneableValue>&
rs_std::Option<::stdlib::NonCloneableValue>::operator=(Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::stdlib::NonCloneableValue>::Option(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_STDLIB_STDLIB_GOLDEN
