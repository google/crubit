// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// into_iterator_rust_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INTO_ITERATOR_INTO_ITERATOR_RUST_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INTO_ITERATOR_INTO_ITERATOR_RUST_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/iterator_adapter.h"
#include "support/rs_std/option.h"
#include "support/rs_std/slice_ref.h"
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

namespace into_iterator_rust {
struct MyContainer;
struct MyIterator;
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: ContainerWithInherentBegin") alignas(4)
    [[clang::trivial_abi]] ContainerWithInherentBegin final {
 public:
  ::std::int32_t begin() const;

  // Error generating bindings for struct
  // `into_iterator_rust_golden::ContainerWithInherentBegin` defined at
  // cc_bindings_from_rs/test/known_traits/into_iterator/into_iterator.rs;l=100:
  // into_iterator_rust_golden::ContainerWithInherentBegin has a method named
  // `begin`, `end`, or `into_iter`, which prevents binding methods for
  // IntoIterator.

  ::std::array<::std::int32_t, 3> data{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: ContainerWithRefIntoIter") alignas(4)
    [[clang::trivial_abi]] ContainerWithRefIntoIter final {
 public:
  // Error generating bindings for struct
  // `into_iterator_rust_golden::ContainerWithRefIntoIter` defined at
  // cc_bindings_from_rs/test/known_traits/into_iterator/into_iterator.rs;l=88:
  // IntoIterator/Iterator impls with generic type or const parameters are not
  // supported yet.

  ::into_iterator_rust::MyIterator* crubit_nonnull iter{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: MoveOnlyIterator") alignas(4)
    [[clang::trivial_abi]] MoveOnlyIterator final {
 public:
  template <typename TAdaptedSelf_ = MoveOnlyIterator>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }
  ::std::int32_t val{};
  ::std::int32_t count{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: MoveOnlyPayload") alignas(4)
    [[clang::trivial_abi]] MoveOnlyPayload final {
 public:
  ::std::int32_t mutating_method();

  ::std::int32_t val{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: MyContainerIntoIter") alignas(4)
    [[clang::trivial_abi]] MyContainerIntoIter final {
 public:
  // `into_iterator_rust_golden::MyContainerIntoIter` doesn't implement the
  // `Default` trait
  MyContainerIntoIter() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyContainerIntoIter() = default;
  MyContainerIntoIter(MyContainerIntoIter&&) = default;
  MyContainerIntoIter& operator=(MyContainerIntoIter&&) = default;

  // `into_iterator_rust_golden::MyContainerIntoIter` doesn't implement the
  // `Clone` trait
  MyContainerIntoIter(const MyContainerIntoIter&) = delete;
  MyContainerIntoIter& operator=(const MyContainerIntoIter&) = delete;
  MyContainerIntoIter(::crubit::UnsafeRelocateTag, MyContainerIntoIter&& value);
  template <typename TAdaptedSelf_ = MyContainerIntoIter>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }

 private:
  union {
    ::std::array<::std::int32_t, 3> data;
  };
  union {
    ::std::uintptr_t index;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: MyContainerIter") alignas(4)
    [[clang::trivial_abi]] MyContainerIter final {
 public:
  // `into_iterator_rust_golden::MyContainerIter` doesn't implement the
  // `Default` trait
  MyContainerIter() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyContainerIter() = default;
  MyContainerIter(MyContainerIter&&) = default;
  MyContainerIter& operator=(MyContainerIter&&) = default;

  // `into_iterator_rust_golden::MyContainerIter` doesn't implement the `Clone`
  // trait
  MyContainerIter(const MyContainerIter&) = delete;
  MyContainerIter& operator=(const MyContainerIter&) = delete;
  MyContainerIter(::crubit::UnsafeRelocateTag, MyContainerIter&& value);
  template <typename TAdaptedSelf_ = MyContainerIter>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }

 private:
  union {
    rs_std::SliceRef<const ::std::int32_t> data;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: MyContainerIterMut") alignas(4)
    [[clang::trivial_abi]] MyContainerIterMut final {
 public:
  // `into_iterator_rust_golden::MyContainerIterMut` doesn't implement the
  // `Default` trait
  MyContainerIterMut() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyContainerIterMut() = default;
  MyContainerIterMut(MyContainerIterMut&&) = default;
  MyContainerIterMut& operator=(MyContainerIterMut&&) = default;

  // `into_iterator_rust_golden::MyContainerIterMut` doesn't implement the
  // `Clone` trait
  MyContainerIterMut(const MyContainerIterMut&) = delete;
  MyContainerIterMut& operator=(const MyContainerIterMut&) = delete;
  MyContainerIterMut(::crubit::UnsafeRelocateTag, MyContainerIterMut&& value);
  template <typename TAdaptedSelf_ = MyContainerIterMut>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }

 private:
  union {
    rs_std::SliceRef<::std::int32_t> data;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: MyIterator") alignas(4)
    [[clang::trivial_abi]] MyIterator final {
 public:
  template <typename TAdaptedSelf_ = MyIterator>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }
  ::std::int32_t value{};

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: SimpleIntoIter") alignas(4)
    [[clang::trivial_abi]] SimpleIntoIter final {
 public:
  template <typename TAdaptedSelf_ = SimpleIntoIter>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }
  ::std::int32_t val{};

 private:
  static void __crubit_field_offset_assertions();
};

::into_iterator_rust::MyContainer make_container(::std::int32_t a,
                                                 ::std::int32_t b,
                                                 ::std::int32_t c);

::into_iterator_rust::ContainerWithInherentBegin make_inherent_container();

::into_iterator_rust::MyIterator make_iterator(::std::int32_t value);

::into_iterator_rust::MoveOnlyIterator make_move_only_iterator(
    ::std::int32_t val, ::std::int32_t count);

::into_iterator_rust::ContainerWithRefIntoIter make_ref_container(
    ::into_iterator_rust::MyIterator* $a crubit_nonnull iter);

}  // namespace into_iterator_rust

template <>
struct rs_std::impl<::into_iterator_rust::ContainerWithInherentBegin,
                    ::rs::core::iter::IntoIterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::ContainerWithInherentBegin as :: core :: "
      "iter :: IntoIterator>::Item") = ::std::int32_t;
  using IntoIter CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::ContainerWithInherentBegin as :: core :: "
      "iter :: IntoIterator>::IntoIter") = ::into_iterator_rust::SimpleIntoIter;

  static ::into_iterator_rust::SimpleIntoIter into_iter(
      ::into_iterator_rust::ContainerWithInherentBegin self);
};

template <>
struct rs_std::impl<::into_iterator_rust::ContainerWithRefIntoIter,
                    ::rs::core::iter::IntoIterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::ContainerWithRefIntoIter<'a> as :: core :: "
      "iter :: IntoIterator>::Item") = ::std::int32_t;
  using IntoIter CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::ContainerWithRefIntoIter<'a> as :: core :: "
      "iter :: IntoIterator>::IntoIter") =
      ::into_iterator_rust::MyIterator* $a crubit_nonnull;

  static ::into_iterator_rust::MyIterator& $a
  into_iter(::into_iterator_rust::ContainerWithRefIntoIter self);
};

template <>
struct rs_std::impl<::into_iterator_rust::MoveOnlyIterator,
                    ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::MoveOnlyIterator as :: core :: iter :: "
      "Iterator>::Item") = ::into_iterator_rust::MoveOnlyPayload;

  static rs_std::Option<::into_iterator_rust::MoveOnlyPayload> next(
      ::into_iterator_rust::MoveOnlyIterator& self);
};

template <>
struct rs_std::impl<::into_iterator_rust::MyContainer,
                    ::rs::core::iter::IntoIterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::MyContainer as :: core :: iter :: "
      "IntoIterator>::Item") = ::std::int32_t;
  using IntoIter CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::MyContainer as :: core :: iter :: "
      "IntoIterator>::IntoIter") = ::into_iterator_rust::MyContainerIntoIter;

  static ::into_iterator_rust::MyContainerIntoIter into_iter(
      ::into_iterator_rust::MyContainer self);
};

template <>
struct rs_std::impl<::into_iterator_rust::MyContainerIntoIter,
                    ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::MyContainerIntoIter as :: core :: iter :: "
      "Iterator>::Item") = ::std::int32_t;

  static rs_std::Option<::std::int32_t> next(
      ::into_iterator_rust::MyContainerIntoIter& self);
};

template <>
struct rs_std::impl<::into_iterator_rust::MyContainerIter,
                    ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::MyContainerIter<'a> as :: core :: iter :: "
      "Iterator>::Item") = ::std::int32_t const* $a crubit_nonnull;

  static rs_std::Option<::std::int32_t const * $static crubit_nonnull> next(
      ::into_iterator_rust::MyContainerIter& self);
};

template <>
struct rs_std::impl<::into_iterator_rust::MyContainerIterMut,
                    ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::MyContainerIterMut<'a> as :: core :: iter "
      ":: Iterator>::Item") = ::std::int32_t* $a crubit_nonnull;

  static rs_std::Option<::std::int32_t* $static crubit_nonnull> next(
      ::into_iterator_rust::MyContainerIterMut& self);
};

namespace into_iterator_rust {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: MyContainer") alignas(4)
    [[clang::trivial_abi]] MyContainer final {
 public:
  template <typename TAdaptedSelf_ = MyContainer>
  inline ::into_iterator_rust::MyContainerIntoIter into_iter() &&;
  template <typename TAdaptedSelf_ = MyContainer>
  rs::IteratorAdapter<::into_iterator_rust::MyContainerIter> begin() const&;
  template <typename TAdaptedSelf_ = MyContainer>
  rs::IteratorEnd end() const&;
  template <typename TAdaptedSelf_ = MyContainer>
  rs::IteratorAdapter<::into_iterator_rust::MyContainerIterMut> begin() &;
  template <typename TAdaptedSelf_ = MyContainer>
  rs::IteratorEnd end() &;
  ::std::array<::std::int32_t, 3> data{};

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace into_iterator_rust

template <>
struct rs_std::impl<::into_iterator_rust::MyIterator,
                    ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::MyIterator as :: core :: iter :: "
      "Iterator>::Item") = ::std::int32_t;

  static rs_std::Option<::std::int32_t> next(
      ::into_iterator_rust::MyIterator& self);
};

template <>
struct rs_std::impl<::into_iterator_rust::SimpleIntoIter,
                    ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::SimpleIntoIter as :: core :: iter :: "
      "Iterator>::Item") = ::std::int32_t;

  static rs_std::Option<::std::int32_t> next(
      ::into_iterator_rust::SimpleIntoIter& self);
};
#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
template <>
struct alignas(4)
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
  using tag_type = ::std::uint32_t;
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
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char storage_[4];
};
#endif

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
template <>
struct alignas(4)
    CRUBIT_INTERNAL_RUST_TYPE("std :: option :: Option < & 'static mut i32 >")
        rs_std::Option<::std::int32_t* $static crubit_nonnull>
    : public rs_std::OptionBase<
          rs_std::Option<::std::int32_t* $static crubit_nonnull>,
          ::std::int32_t* $static crubit_nonnull> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<::std::int32_t* $static crubit_nonnull>,
                         ::std::int32_t* $static crubit_nonnull>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option, ::std::int32_t* $static crubit_nonnull, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option, ::std::int32_t* $static crubit_nonnull, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             ::std::int32_t* $static crubit_nonnull, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             ::std::int32_t* $static crubit_nonnull, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = 0;
  ::std::int32_t* $static crubit_nonnull* some_ptr() noexcept {
    return reinterpret_cast<::std::int32_t* $static crubit_nonnull*>(storage_);
  }
  ::std::int32_t* $static crubit_nonnull const* some_const_ptr()
      const noexcept {
    return reinterpret_cast<::std::int32_t* $static crubit_nonnull const*>(
        storage_);
  }
  void set_some_tag() noexcept {}
  constexpr void set_none_tag() noexcept { set_tag(kNoneVal); }
  constexpr bool is_none() const noexcept { return tag() == kNoneVal; }
  constexpr ::std::uint32_t tag() const& noexcept;
  constexpr void set_tag(::std::uint32_t tag) noexcept;

 private:
  unsigned char storage_[4];
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

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020into_uiterator_urust_x00000020_x0000003a_x0000003a_x00000020MoveOnlyPayload_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020into_uiterator_urust_x00000020_x0000003a_x0000003a_x00000020MoveOnlyPayload_x00000020_x0000003e
template <>
struct alignas(4) CRUBIT_INTERNAL_RUST_TYPE(
    "std :: option :: Option < :: into_iterator_rust_golden :: MoveOnlyPayload "
    ">") rs_std::Option<::into_iterator_rust::MoveOnlyPayload>
    : public rs_std::OptionBase<
          rs_std::Option<::into_iterator_rust::MoveOnlyPayload>,
          ::into_iterator_rust::MoveOnlyPayload> {
 public:
  // `core::option::Option` doesn't implement the `Clone` trait
  Option(const Option&) = delete;
  Option& operator=(const Option&) = delete;
  Option(Option&&) = default;
  Option& operator=(Option&&) = default;

  Option(::crubit::UnsafeRelocateTag, Option&& value);
  using base_type =
      rs_std::OptionBase<rs_std::Option<::into_iterator_rust::MoveOnlyPayload>,
                         ::into_iterator_rust::MoveOnlyPayload>;
  constexpr Option() = default;
  constexpr Option(::std::nullopt_t) noexcept;
  constexpr Option& operator=(::std::nullopt_t) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option, ::into_iterator_rust::MoveOnlyPayload, U>)
  Option(U&& value) noexcept;
  template <typename U>
    requires(rs_std::OptionForwardConstructible<
             Option, ::into_iterator_rust::MoveOnlyPayload, U>)
  Option& operator=(U&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             ::into_iterator_rust::MoveOnlyPayload, Opt>)
  Option(Opt&& value) noexcept;
  template <typename Opt>
    requires(rs_std::OptionFromStdOptional<
             ::into_iterator_rust::MoveOnlyPayload, Opt>)
  Option& operator=(Opt&& value) noexcept;
  template <typename... Args>
  explicit Option(::std::in_place_t ip, Args&&... args) noexcept;
  ~Option() noexcept = default;

 private:
  friend base_type;
  using tag_type = ::std::uint32_t;
  static constexpr tag_type kNoneVal = 0;
  ::into_iterator_rust::MoveOnlyPayload* some_ptr() noexcept {
    return reinterpret_cast<::into_iterator_rust::MoveOnlyPayload*>(storage_ +
                                                                    4);
  }
  ::into_iterator_rust::MoveOnlyPayload const* some_const_ptr() const noexcept {
    return reinterpret_cast<::into_iterator_rust::MoveOnlyPayload const*>(
        storage_ + 4);
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

namespace into_iterator_rust {

static_assert(
    sizeof(::into_iterator_rust::ContainerWithInherentBegin) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into_iterator_rust::ContainerWithInherentBegin) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<ContainerWithInherentBegin>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::ContainerWithInherentBegin>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::ContainerWithInherentBegin>);
namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_begin(
    ::into_iterator_rust::ContainerWithInherentBegin const&);
}
inline ::std::int32_t(ContainerWithInherentBegin::begin)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_begin(self);
}
inline void ::into_iterator_rust::ContainerWithInherentBegin::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into_iterator_rust::ContainerWithInherentBegin;
  static_assert(0 == offsetof(__crubit_assert_type, data));
}
static_assert(
    sizeof(::into_iterator_rust::ContainerWithRefIntoIter) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into_iterator_rust::ContainerWithRefIntoIter) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<ContainerWithRefIntoIter>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::ContainerWithRefIntoIter>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::ContainerWithRefIntoIter>);
inline void ::into_iterator_rust::ContainerWithRefIntoIter::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into_iterator_rust::ContainerWithRefIntoIter;
  static_assert(0 == offsetof(__crubit_assert_type, iter));
}
static_assert(
    sizeof(::into_iterator_rust::MoveOnlyIterator) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into_iterator_rust::MoveOnlyIterator) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MoveOnlyIterator>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::MoveOnlyIterator>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::MoveOnlyIterator>);
inline void ::into_iterator_rust::MoveOnlyIterator::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into_iterator_rust::MoveOnlyIterator;
  static_assert(0 == offsetof(__crubit_assert_type, val));
  static_assert(4 == offsetof(__crubit_assert_type, count));
}
static_assert(
    sizeof(::into_iterator_rust::MoveOnlyPayload) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into_iterator_rust::MoveOnlyPayload) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MoveOnlyPayload>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::MoveOnlyPayload>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::MoveOnlyPayload>);
namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_mutating_umethod(
    ::into_iterator_rust::MoveOnlyPayload&);
}
inline ::std::int32_t(MoveOnlyPayload::mutating_method)() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_mutating_umethod(self);
}
inline void ::into_iterator_rust::MoveOnlyPayload::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into_iterator_rust::MoveOnlyPayload;
  static_assert(0 == offsetof(__crubit_assert_type, val));
}
static_assert(
    sizeof(::into_iterator_rust::MyContainer) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into_iterator_rust::MyContainer) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyContainer>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::MyContainer>);
static_assert(
    ::std::is_trivially_move_assignable_v<::into_iterator_rust::MyContainer>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainer(
    ::into_iterator_rust::MyContainer*,
    ::into_iterator_rust::MyContainerIntoIter* __ret_ptr);
}
template <typename TAdaptedSelf_>
inline ::into_iterator_rust::MyContainerIntoIter(
    ::into_iterator_rust::MyContainer::into_iter)() && {
  MyContainer&& self_ = ::std::move(*this);
  auto call_into_iter = [&]() -> decltype(auto) {
    crubit::Slot<::into_iterator_rust::MyContainerIntoIter>
        __return_value_ret_val_holder;
    auto* __return_value_storage = __return_value_ret_val_holder.Get();
    __crubit_internal::
        __crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainer(
            &self_, __return_value_storage);
    return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
  };
  return call_into_iter();
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_IntoIterator_uinto_uiter_u_x00000026into_uiterator_urust_ugolden_x0000003a_x0000003aMyContainer(
    ::into_iterator_rust::MyContainer const&,
    ::into_iterator_rust::MyContainerIter* __ret_ptr);
}
template <typename TAdaptedSelf_>
inline rs::IteratorAdapter<::into_iterator_rust::MyContainerIter>(
    ::into_iterator_rust::MyContainer::begin)() const& {
  const MyContainer& self_ = *this;
  auto call_into_iter = [&]() -> decltype(auto) {
    crubit::Slot<::into_iterator_rust::MyContainerIter>
        __return_value_ret_val_holder;
    auto* __return_value_storage = __return_value_ret_val_holder.Get();
    __crubit_internal::
        __crubit_thunk_IntoIterator_uinto_uiter_u_x00000026into_uiterator_urust_ugolden_x0000003a_x0000003aMyContainer(
            self_, __return_value_storage);
    return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
  };
  return rs::IteratorAdapter<::into_iterator_rust::MyContainerIter>(
      call_into_iter());
}
template <typename TAdaptedSelf_>
inline rs::IteratorEnd(::into_iterator_rust::MyContainer::end)() const& {
  return rs::IteratorEnd();
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_IntoIterator_uinto_uiter_u_x00000026mut_x00000020into_uiterator_urust_ugolden_x0000003a_x0000003aMyContainer(
    ::into_iterator_rust::MyContainer&,
    ::into_iterator_rust::MyContainerIterMut* __ret_ptr);
}
template <typename TAdaptedSelf_>
inline rs::IteratorAdapter<::into_iterator_rust::MyContainerIterMut>(
    ::into_iterator_rust::MyContainer::begin)() & {
  MyContainer& self_ = *this;
  auto call_into_iter = [&]() -> decltype(auto) {
    crubit::Slot<::into_iterator_rust::MyContainerIterMut>
        __return_value_ret_val_holder;
    auto* __return_value_storage = __return_value_ret_val_holder.Get();
    __crubit_internal::
        __crubit_thunk_IntoIterator_uinto_uiter_u_x00000026mut_x00000020into_uiterator_urust_ugolden_x0000003a_x0000003aMyContainer(
            self_, __return_value_storage);
    return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
  };
  return rs::IteratorAdapter<::into_iterator_rust::MyContainerIterMut>(
      call_into_iter());
}
template <typename TAdaptedSelf_>
inline rs::IteratorEnd(::into_iterator_rust::MyContainer::end)() & {
  return rs::IteratorEnd();
}
inline void ::into_iterator_rust::MyContainer::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into_iterator_rust::MyContainer;
  static_assert(0 == offsetof(__crubit_assert_type, data));
}
static_assert(
    sizeof(::into_iterator_rust::MyContainerIntoIter) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into_iterator_rust::MyContainerIntoIter) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::into_iterator_rust::MyContainerIntoIter>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::MyContainerIntoIter>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::MyContainerIntoIter>);
inline ::into_iterator_rust::MyContainerIntoIter::MyContainerIntoIter(
    ::crubit::UnsafeRelocateTag, MyContainerIntoIter&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::into_iterator_rust::MyContainerIntoIter::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into_iterator_rust::MyContainerIntoIter;
  static_assert(0 == offsetof(__crubit_assert_type, data));
  static_assert(12 == offsetof(__crubit_assert_type, index));
}
static_assert(
    sizeof(::into_iterator_rust::MyContainerIter) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into_iterator_rust::MyContainerIter) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::into_iterator_rust::MyContainerIter>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::MyContainerIter>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::MyContainerIter>);
inline ::into_iterator_rust::MyContainerIter::MyContainerIter(
    ::crubit::UnsafeRelocateTag, MyContainerIter&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::into_iterator_rust::MyContainerIter::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into_iterator_rust::MyContainerIter;
  static_assert(0 == offsetof(__crubit_assert_type, data));
}
static_assert(
    sizeof(::into_iterator_rust::MyContainerIterMut) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into_iterator_rust::MyContainerIterMut) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::into_iterator_rust::MyContainerIterMut>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::MyContainerIterMut>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::MyContainerIterMut>);
inline ::into_iterator_rust::MyContainerIterMut::MyContainerIterMut(
    ::crubit::UnsafeRelocateTag, MyContainerIterMut&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::into_iterator_rust::MyContainerIterMut::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into_iterator_rust::MyContainerIterMut;
  static_assert(0 == offsetof(__crubit_assert_type, data));
}
static_assert(
    sizeof(::into_iterator_rust::MyIterator) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into_iterator_rust::MyIterator) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyIterator>);
static_assert(
    ::std::is_trivially_move_constructible_v<::into_iterator_rust::MyIterator>);
static_assert(
    ::std::is_trivially_move_assignable_v<::into_iterator_rust::MyIterator>);
inline void ::into_iterator_rust::MyIterator::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into_iterator_rust::MyIterator;
  static_assert(0 == offsetof(__crubit_assert_type, value));
}
static_assert(
    sizeof(::into_iterator_rust::SimpleIntoIter) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::into_iterator_rust::SimpleIntoIter) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<SimpleIntoIter>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::SimpleIntoIter>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::SimpleIntoIter>);
inline void ::into_iterator_rust::SimpleIntoIter::
    __crubit_field_offset_assertions() {
  using __crubit_assert_type = ::into_iterator_rust::SimpleIntoIter;
  static_assert(0 == offsetof(__crubit_assert_type, val));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_make_ucontainer(
    ::std::int32_t, ::std::int32_t, ::std::int32_t,
    ::into_iterator_rust::MyContainer* __ret_ptr);
}
inline ::into_iterator_rust::MyContainer make_container(::std::int32_t a,
                                                        ::std::int32_t b,
                                                        ::std::int32_t c) {
  crubit::Slot<::into_iterator_rust::MyContainer> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_ucontainer(a, b, c,
                                                    __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_uinherent_ucontainer(
    ::into_iterator_rust::ContainerWithInherentBegin* __ret_ptr);
}
inline ::into_iterator_rust::ContainerWithInherentBegin
make_inherent_container() {
  crubit::Slot<::into_iterator_rust::ContainerWithInherentBegin>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uinherent_ucontainer(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_uiterator(
    ::std::int32_t, ::into_iterator_rust::MyIterator* __ret_ptr);
}
inline ::into_iterator_rust::MyIterator make_iterator(::std::int32_t value) {
  crubit::Slot<::into_iterator_rust::MyIterator> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uiterator(value,
                                                   __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_umove_uonly_uiterator(
    ::std::int32_t, ::std::int32_t,
    ::into_iterator_rust::MoveOnlyIterator* __ret_ptr);
}
inline ::into_iterator_rust::MoveOnlyIterator make_move_only_iterator(
    ::std::int32_t val, ::std::int32_t count) {
  crubit::Slot<::into_iterator_rust::MoveOnlyIterator>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_umove_uonly_uiterator(
      val, count, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_uref_ucontainer(
    ::into_iterator_rust::MyIterator* $a crubit_nonnull,
    ::into_iterator_rust::ContainerWithRefIntoIter* __ret_ptr);
}
inline ::into_iterator_rust::ContainerWithRefIntoIter make_ref_container(
    ::into_iterator_rust::MyIterator* $a crubit_nonnull iter) {
  crubit::Slot<::into_iterator_rust::ContainerWithRefIntoIter>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uref_ucontainer(
      iter, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace into_iterator_rust

namespace into_iterator_rust {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aContainerWithInherentBegin(
    ::into_iterator_rust::ContainerWithInherentBegin*,
    ::into_iterator_rust::SimpleIntoIter* __ret_ptr);
}
}  // namespace into_iterator_rust
inline ::into_iterator_rust::SimpleIntoIter
rs_std::impl<::into_iterator_rust::ContainerWithInherentBegin,
             ::rs::core::iter::IntoIterator>::
    into_iter(::into_iterator_rust::ContainerWithInherentBegin self) {
  crubit::Slot<::into_iterator_rust::SimpleIntoIter>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aContainerWithInherentBegin(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace into_iterator_rust {
namespace __crubit_internal {
extern "C" ::into_iterator_rust::MyIterator& $a
__crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aContainerWithRefIntoIter_x0000003c_x00000027a_x0000003e(
    ::into_iterator_rust::ContainerWithRefIntoIter*);
}
}  // namespace into_iterator_rust
inline ::into_iterator_rust::MyIterator& $a
rs_std::impl<::into_iterator_rust::ContainerWithRefIntoIter,
             ::rs::core::iter::IntoIterator>::
    into_iter(::into_iterator_rust::ContainerWithRefIntoIter self) {
  return into_iterator_rust::__crubit_internal::
      __crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aContainerWithRefIntoIter_x0000003c_x00000027a_x0000003e(
          &self);
}

namespace into_iterator_rust {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMoveOnlyIterator(
    ::into_iterator_rust::MoveOnlyIterator&,
    rs_std::Option<::into_iterator_rust::MoveOnlyPayload>* __ret_ptr);
}
}  // namespace into_iterator_rust
inline rs_std::Option<::into_iterator_rust::MoveOnlyPayload> rs_std::impl<
    ::into_iterator_rust::MoveOnlyIterator,
    ::rs::core::iter::Iterator>::next(::into_iterator_rust::MoveOnlyIterator&
                                          self) {
  crubit::Slot<rs_std::Option<::into_iterator_rust::MoveOnlyPayload>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMoveOnlyIterator(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace into_iterator_rust {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainer(
    ::into_iterator_rust::MyContainer*,
    ::into_iterator_rust::MyContainerIntoIter* __ret_ptr);
}
}  // namespace into_iterator_rust
inline ::into_iterator_rust::MyContainerIntoIter rs_std::impl<
    ::into_iterator_rust::MyContainer,
    ::rs::core::iter::IntoIterator>::into_iter(::into_iterator_rust::MyContainer
                                                   self) {
  crubit::Slot<::into_iterator_rust::MyContainerIntoIter>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainer(
          &self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace into_iterator_rust {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIntoIter(
    ::into_iterator_rust::MyContainerIntoIter&,
    rs_std::Option<::std::int32_t>* __ret_ptr);
}
}  // namespace into_iterator_rust
inline rs_std::Option<::std::int32_t> rs_std::impl<
    ::into_iterator_rust::MyContainerIntoIter,
    ::rs::core::iter::Iterator>::next(::into_iterator_rust::MyContainerIntoIter&
                                          self) {
  crubit::Slot<rs_std::Option<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIntoIter(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace into_iterator_rust {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIter_x0000003c_x00000027a_x0000003e(
    ::into_iterator_rust::MyContainerIter&,
    rs_std::Option<::std::int32_t const * $static crubit_nonnull>* __ret_ptr);
}
}  // namespace into_iterator_rust
inline rs_std::Option<::std::int32_t const * $static crubit_nonnull>
rs_std::impl<::into_iterator_rust::MyContainerIter,
             ::rs::core::iter::Iterator>::
    next(::into_iterator_rust::MyContainerIter& self) {
  crubit::Slot<rs_std::Option<::std::int32_t const * $static crubit_nonnull>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIter_x0000003c_x00000027a_x0000003e(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace into_iterator_rust {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIterMut_x0000003c_x00000027a_x0000003e(
    ::into_iterator_rust::MyContainerIterMut&,
    rs_std::Option<::std::int32_t* $static crubit_nonnull>* __ret_ptr);
}
}  // namespace into_iterator_rust
inline rs_std::Option<::std::int32_t* $static crubit_nonnull> rs_std::impl<
    ::into_iterator_rust::MyContainerIterMut,
    ::rs::core::iter::Iterator>::next(::into_iterator_rust::MyContainerIterMut&
                                          self) {
  crubit::Slot<rs_std::Option<::std::int32_t* $static crubit_nonnull>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIterMut_x0000003c_x00000027a_x0000003e(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace into_iterator_rust {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyIterator(
    ::into_iterator_rust::MyIterator&,
    rs_std::Option<::std::int32_t>* __ret_ptr);
}
}  // namespace into_iterator_rust
inline rs_std::Option<::std::int32_t> rs_std::impl<
    ::into_iterator_rust::MyIterator,
    ::rs::core::iter::Iterator>::next(::into_iterator_rust::MyIterator& self) {
  crubit::Slot<rs_std::Option<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyIterator(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace into_iterator_rust {
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aSimpleIntoIter(
    ::into_iterator_rust::SimpleIntoIter&,
    rs_std::Option<::std::int32_t>* __ret_ptr);
}
}  // namespace into_iterator_rust
inline rs_std::Option<::std::int32_t>
rs_std::impl<::into_iterator_rust::SimpleIntoIter, ::rs::core::iter::Iterator>::
    next(::into_iterator_rust::SimpleIntoIter& self) {
  crubit::Slot<rs_std::Option<::std::int32_t>> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aSimpleIntoIter(
          self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020const_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
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
inline constexpr ::std::uint32_t rs_std::Option<
    ::std::int32_t const * $static crubit_nonnull>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void
rs_std::Option<::std::int32_t const * $static crubit_nonnull>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
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

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020std_x00000020_x0000003a_x0000003a_x00000020int32_ut_x00000020_x0000002a_x00000020crubit_unonnull_x00000020_x0000003e
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Option<::std::int32_t* $static crubit_nonnull>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Option<::std::int32_t* $static crubit_nonnull>>);
inline rs_std::Option<::std::int32_t* $static crubit_nonnull>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Option<::std::int32_t* $static crubit_nonnull>>);
inline constexpr ::std::uint32_t
rs_std::Option<::std::int32_t* $static crubit_nonnull>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void
rs_std::Option<::std::int32_t* $static crubit_nonnull>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::std::int32_t* $static crubit_nonnull>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::std::int32_t* $static crubit_nonnull>&
rs_std::Option<::std::int32_t* $static crubit_nonnull>::operator=(
    ::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::std::int32_t* $static crubit_nonnull>,
           ::std::int32_t* $static crubit_nonnull, U>)
inline rs_std::Option<::std::int32_t* $static crubit_nonnull>::Option(
    U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::std::int32_t* $static crubit_nonnull>,
           ::std::int32_t* $static crubit_nonnull, U>)
inline rs_std::Option<::std::int32_t* $static crubit_nonnull>&
rs_std::Option<::std::int32_t* $static crubit_nonnull>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::int32_t* $static crubit_nonnull,
                                         Opt>)
inline rs_std::Option<::std::int32_t* $static crubit_nonnull>::Option(
    Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(rs_std::OptionFromStdOptional<::std::int32_t* $static crubit_nonnull,
                                         Opt>)
inline rs_std::Option<::std::int32_t* $static crubit_nonnull>&
rs_std::Option<::std::int32_t* $static crubit_nonnull>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::std::int32_t* $static crubit_nonnull>::Option(
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

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020into_uiterator_urust_x00000020_x0000003a_x0000003a_x00000020MoveOnlyPayload_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Option_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020into_uiterator_urust_x00000020_x0000003a_x0000003a_x00000020MoveOnlyPayload_x00000020_x0000003e
static_assert(::std::is_trivially_move_constructible_v<
              rs_std::Option<::into_iterator_rust::MoveOnlyPayload>>);
static_assert(::std::is_trivially_move_assignable_v<
              rs_std::Option<::into_iterator_rust::MoveOnlyPayload>>);
inline rs_std::Option<::into_iterator_rust::MoveOnlyPayload>::Option(
    ::crubit::UnsafeRelocateTag, Option&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
static_assert(::std::is_trivially_destructible_v<
              rs_std::Option<::into_iterator_rust::MoveOnlyPayload>>);
inline constexpr ::std::uint32_t
rs_std::Option<::into_iterator_rust::MoveOnlyPayload>::tag() const& noexcept {
  ::std::array<unsigned char, sizeof(::std::uint32_t)> __bytes = {};
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    __bytes[i] = storage_[0 + i];
  }
  return ::std::bit_cast<::std::uint32_t>(__bytes);
}
inline constexpr void
rs_std::Option<::into_iterator_rust::MoveOnlyPayload>::set_tag(
    ::std::uint32_t tag) noexcept {
  auto __bytes =
      ::std::bit_cast<::std::array<unsigned char, sizeof(::std::uint32_t)>>(
          tag);
  for (::std::size_t i = 0; i < sizeof(::std::uint32_t); ++i) {
    storage_[0 + i] = __bytes[i];
  }
}

inline constexpr rs_std::Option<::into_iterator_rust::MoveOnlyPayload>::Option(
    ::std::nullopt_t) noexcept
    : base_type(::std::nullopt) {}
inline constexpr rs_std::Option<::into_iterator_rust::MoveOnlyPayload>&
rs_std::Option<::into_iterator_rust::MoveOnlyPayload>::operator=(
    ::std::nullopt_t) noexcept {
  base_type::operator=(::std::nullopt);
  return *this;
}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::into_iterator_rust::MoveOnlyPayload>,
           ::into_iterator_rust::MoveOnlyPayload, U>)
inline rs_std::Option<::into_iterator_rust::MoveOnlyPayload>::Option(
    U&& value) noexcept
    : base_type(::std::forward<U>(value)) {}
template <typename U>
  requires(rs_std::OptionForwardConstructible<
           rs_std::Option<::into_iterator_rust::MoveOnlyPayload>,
           ::into_iterator_rust::MoveOnlyPayload, U>)
inline rs_std::Option<::into_iterator_rust::MoveOnlyPayload>&
rs_std::Option<::into_iterator_rust::MoveOnlyPayload>::operator=(
    U&& value) noexcept {
  base_type::operator=(::std::forward<U>(value));
  return *this;
}
template <typename Opt>
  requires(
      rs_std::OptionFromStdOptional<::into_iterator_rust::MoveOnlyPayload, Opt>)
inline rs_std::Option<::into_iterator_rust::MoveOnlyPayload>::Option(
    Opt&& value) noexcept
    : base_type(::std::forward<Opt>(value)) {}
template <typename Opt>
  requires(
      rs_std::OptionFromStdOptional<::into_iterator_rust::MoveOnlyPayload, Opt>)
inline rs_std::Option<::into_iterator_rust::MoveOnlyPayload>&
rs_std::Option<::into_iterator_rust::MoveOnlyPayload>::operator=(
    Opt&& value) noexcept {
  base_type::operator=(::std::forward<Opt>(value));
  return *this;
}
template <typename... Args>
inline rs_std::Option<::into_iterator_rust::MoveOnlyPayload>::Option(
    ::std::in_place_t ip, Args&&... args) noexcept
    : base_type(ip, ::std::forward<Args>(args)...) {}

#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INTO_ITERATOR_INTO_ITERATOR_RUST_GOLDEN
