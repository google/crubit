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
#include "support/bridge.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/iterator_adapter.h"
#include "support/rs_std/slice_ref.h"
#include "support/rs_std/traits.h"

#include <array>
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
  // `into_iterator_rust_golden::ContainerWithInherentBegin` doesn't implement
  // the `Default` trait
  ContainerWithInherentBegin() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~ContainerWithInherentBegin() = default;
  ContainerWithInherentBegin(ContainerWithInherentBegin&&) = default;
  ContainerWithInherentBegin& operator=(ContainerWithInherentBegin&&) = default;

  // `into_iterator_rust_golden::ContainerWithInherentBegin` doesn't implement
  // the `Clone` trait
  ContainerWithInherentBegin(const ContainerWithInherentBegin&) = delete;
  ContainerWithInherentBegin& operator=(const ContainerWithInherentBegin&) =
      delete;
  ContainerWithInherentBegin(::crubit::UnsafeRelocateTag,
                             ContainerWithInherentBegin&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  ::std::int32_t begin() const;

  // Error generating bindings for struct
  // `into_iterator_rust_golden::ContainerWithInherentBegin` defined at
  // cc_bindings_from_rs/test/known_traits/into_iterator/into_iterator.rs;l=100:
  // into_iterator_rust_golden::ContainerWithInherentBegin has a method named
  // `begin`, `end`, or `into_iter`, which prevents binding methods for
  // IntoIterator.

  union {
    ::std::array<::std::int32_t, 3> data;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: ContainerWithRefIntoIter") alignas(8)
    [[clang::trivial_abi]] ContainerWithRefIntoIter final {
 public:
  // `into_iterator_rust_golden::ContainerWithRefIntoIter` doesn't implement the
  // `Default` trait
  ContainerWithRefIntoIter() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~ContainerWithRefIntoIter() = default;
  ContainerWithRefIntoIter(ContainerWithRefIntoIter&&) = default;
  ContainerWithRefIntoIter& operator=(ContainerWithRefIntoIter&&) = default;

  // `into_iterator_rust_golden::ContainerWithRefIntoIter` doesn't implement the
  // `Clone` trait
  ContainerWithRefIntoIter(const ContainerWithRefIntoIter&) = delete;
  ContainerWithRefIntoIter& operator=(const ContainerWithRefIntoIter&) = delete;
  ContainerWithRefIntoIter(::crubit::UnsafeRelocateTag,
                           ContainerWithRefIntoIter&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  // Error generating bindings for struct
  // `into_iterator_rust_golden::ContainerWithRefIntoIter` defined at
  // cc_bindings_from_rs/test/known_traits/into_iterator/into_iterator.rs;l=88:
  // IntoIterator/Iterator impls with generic type or const parameters are not
  // supported yet.

  union {
    ::into_iterator_rust::MyIterator* crubit_nonnull iter;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: MoveOnlyIterator") alignas(4)
    [[clang::trivial_abi]] MoveOnlyIterator final {
 public:
  // `into_iterator_rust_golden::MoveOnlyIterator` doesn't implement the
  // `Default` trait
  MoveOnlyIterator() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MoveOnlyIterator() = default;
  MoveOnlyIterator(MoveOnlyIterator&&) = default;
  MoveOnlyIterator& operator=(MoveOnlyIterator&&) = default;

  // `into_iterator_rust_golden::MoveOnlyIterator` doesn't implement the `Clone`
  // trait
  MoveOnlyIterator(const MoveOnlyIterator&) = delete;
  MoveOnlyIterator& operator=(const MoveOnlyIterator&) = delete;
  MoveOnlyIterator(::crubit::UnsafeRelocateTag, MoveOnlyIterator&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  template <typename TAdaptedSelf_ = MoveOnlyIterator>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }
  union {
    ::std::int32_t val;
  };
  union {
    ::std::int32_t count;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: MoveOnlyPayload") alignas(4)
    [[clang::trivial_abi]] MoveOnlyPayload final {
 public:
  // `into_iterator_rust_golden::MoveOnlyPayload` doesn't implement the
  // `Default` trait
  MoveOnlyPayload() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MoveOnlyPayload() = default;
  MoveOnlyPayload(MoveOnlyPayload&&) = default;
  MoveOnlyPayload& operator=(MoveOnlyPayload&&) = default;

  // `into_iterator_rust_golden::MoveOnlyPayload` doesn't implement the `Clone`
  // trait
  MoveOnlyPayload(const MoveOnlyPayload&) = delete;
  MoveOnlyPayload& operator=(const MoveOnlyPayload&) = delete;
  MoveOnlyPayload(::crubit::UnsafeRelocateTag, MoveOnlyPayload&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  ::std::int32_t mutating_method();

  union {
    ::std::int32_t val;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: MyContainerIntoIter") alignas(8)
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
  MyContainerIntoIter(::crubit::UnsafeRelocateTag,
                      MyContainerIntoIter&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  template <typename TAdaptedSelf_ = MyContainerIntoIter>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }

 private:
  union {
    ::std::uintptr_t index;
  };
  union {
    ::std::array<::std::int32_t, 3> data;
  };
  unsigned char __padding0[4];

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: MyContainerIter") alignas(8)
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
  MyContainerIter(::crubit::UnsafeRelocateTag, MyContainerIter&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
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
    ":: into_iterator_rust_golden :: MyContainerIterMut") alignas(8)
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
  MyContainerIterMut(::crubit::UnsafeRelocateTag, MyContainerIterMut&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
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
  // `into_iterator_rust_golden::MyIterator` doesn't implement the `Default`
  // trait
  MyIterator() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyIterator() = default;
  MyIterator(MyIterator&&) = default;
  MyIterator& operator=(MyIterator&&) = default;

  // `into_iterator_rust_golden::MyIterator` doesn't implement the `Clone` trait
  MyIterator(const MyIterator&) = delete;
  MyIterator& operator=(const MyIterator&) = delete;
  MyIterator(::crubit::UnsafeRelocateTag, MyIterator&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  template <typename TAdaptedSelf_ = MyIterator>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }
  union {
    ::std::int32_t value;
  };

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: SimpleIntoIter") alignas(4)
    [[clang::trivial_abi]] SimpleIntoIter final {
 public:
  // `into_iterator_rust_golden::SimpleIntoIter` doesn't implement the `Default`
  // trait
  SimpleIntoIter() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~SimpleIntoIter() = default;
  SimpleIntoIter(SimpleIntoIter&&) = default;
  SimpleIntoIter& operator=(SimpleIntoIter&&) = default;

  // `into_iterator_rust_golden::SimpleIntoIter` doesn't implement the `Clone`
  // trait
  SimpleIntoIter(const SimpleIntoIter&) = delete;
  SimpleIntoIter& operator=(const SimpleIntoIter&) = delete;
  SimpleIntoIter(::crubit::UnsafeRelocateTag, SimpleIntoIter&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  template <typename TAdaptedSelf_ = SimpleIntoIter>
  inline rs::IteratorAdapter<TAdaptedSelf_*> begin() & {
    return rs::IteratorAdapter<TAdaptedSelf_*>(this);
  }
  inline rs::IteratorEnd end() & { return rs::IteratorEnd(); }
  union {
    ::std::int32_t val;
  };

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

  static ::std::optional<::into_iterator_rust::MoveOnlyPayload> next(
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

  static ::std::optional<::std::int32_t> next(
      ::into_iterator_rust::MyContainerIntoIter& self);
};

template <>
struct rs_std::impl<::into_iterator_rust::MyContainerIter,
                    ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::MyContainerIter<'a> as :: core :: iter :: "
      "Iterator>::Item") = ::std::int32_t const* $a crubit_nonnull;

  static ::std::optional<::std::int32_t const * $a crubit_nonnull> next(
      ::into_iterator_rust::MyContainerIter& self);
};

template <>
struct rs_std::impl<::into_iterator_rust::MyContainerIterMut,
                    ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::MyContainerIterMut<'a> as :: core :: iter "
      ":: Iterator>::Item") = ::std::int32_t* $a crubit_nonnull;

  static ::std::optional<::std::int32_t* $a crubit_nonnull> next(
      ::into_iterator_rust::MyContainerIterMut& self);
};

namespace into_iterator_rust {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: into_iterator_rust_golden :: MyContainer") alignas(4)
    [[clang::trivial_abi]] MyContainer final {
 public:
  // `into_iterator_rust_golden::MyContainer` doesn't implement the `Default`
  // trait
  MyContainer() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~MyContainer() = default;
  MyContainer(MyContainer&&) = default;
  MyContainer& operator=(MyContainer&&) = default;

  // `into_iterator_rust_golden::MyContainer` doesn't implement the `Clone`
  // trait
  MyContainer(const MyContainer&) = delete;
  MyContainer& operator=(const MyContainer&) = delete;
  MyContainer(::crubit::UnsafeRelocateTag, MyContainer&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
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
  union {
    ::std::array<::std::int32_t, 3> data;
  };

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

  static ::std::optional<::std::int32_t> next(
      ::into_iterator_rust::MyIterator& self);
};

template <>
struct rs_std::impl<::into_iterator_rust::SimpleIntoIter,
                    ::rs::core::iter::Iterator> {
  static constexpr bool kIsImplemented = true;
  using Item CRUBIT_INTERNAL_RUST_TYPE(
      "<into_iterator_rust_golden::SimpleIntoIter as :: core :: iter :: "
      "Iterator>::Item") = ::std::int32_t;

  static ::std::optional<::std::int32_t> next(
      ::into_iterator_rust::SimpleIntoIter& self);
};

namespace into_iterator_rust {

static_assert(
    sizeof(ContainerWithInherentBegin) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ContainerWithInherentBegin) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<ContainerWithInherentBegin>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::ContainerWithInherentBegin>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::ContainerWithInherentBegin>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_begin(
    ::into_iterator_rust::ContainerWithInherentBegin const&);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t ContainerWithInherentBegin::begin() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_begin(self);
}
inline void ContainerWithInherentBegin::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ContainerWithInherentBegin, data));
}
static_assert(
    sizeof(ContainerWithRefIntoIter) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ContainerWithRefIntoIter) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<ContainerWithRefIntoIter>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::ContainerWithRefIntoIter>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::ContainerWithRefIntoIter>);
inline void ContainerWithRefIntoIter::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ContainerWithRefIntoIter, iter));
}
static_assert(
    sizeof(MoveOnlyIterator) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MoveOnlyIterator) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MoveOnlyIterator>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::MoveOnlyIterator>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::MoveOnlyIterator>);
inline void MoveOnlyIterator::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MoveOnlyIterator, val));
  static_assert(4 == offsetof(MoveOnlyIterator, count));
}
static_assert(
    sizeof(MoveOnlyPayload) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MoveOnlyPayload) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MoveOnlyPayload>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::MoveOnlyPayload>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::MoveOnlyPayload>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::std::int32_t __crubit_thunk_mutating_umethod(
    ::into_iterator_rust::MoveOnlyPayload&);
/// \endcond
}  // namespace __crubit_internal
inline ::std::int32_t MoveOnlyPayload::mutating_method() {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_mutating_umethod(self);
}
inline void MoveOnlyPayload::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MoveOnlyPayload, val));
}
static_assert(
    sizeof(MyContainer) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyContainer) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyContainer>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::MyContainer>);
static_assert(
    ::std::is_trivially_move_assignable_v<::into_iterator_rust::MyContainer>);
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_into_uiter(
    ::into_iterator_rust::MyContainer*,
    ::into_iterator_rust::MyContainerIntoIter* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
template <typename TAdaptedSelf_>
inline ::into_iterator_rust::MyContainerIntoIter MyContainer::into_iter() && {
  MyContainer&& self_ = ::std::move(*this);
  auto call_into_iter = [&]() -> decltype(auto) {
    crubit::Slot<::into_iterator_rust::MyContainerIntoIter>
        __return_value_ret_val_holder;
    auto* __return_value_storage = __return_value_ret_val_holder.Get();
    __crubit_internal::__crubit_thunk_into_uiter(&self_,
                                                 __return_value_storage);
    return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
  };
  return call_into_iter();
}
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_into_uiter(
    ::into_iterator_rust::MyContainer const&,
    ::into_iterator_rust::MyContainerIter* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
template <typename TAdaptedSelf_>
inline rs::IteratorAdapter<::into_iterator_rust::MyContainerIter>
MyContainer::begin() const& {
  const MyContainer& self_ = *this;
  auto call_into_iter = [&]() -> decltype(auto) {
    crubit::Slot<::into_iterator_rust::MyContainerIter>
        __return_value_ret_val_holder;
    auto* __return_value_storage = __return_value_ret_val_holder.Get();
    __crubit_internal::__crubit_thunk_into_uiter(self_, __return_value_storage);
    return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
  };
  return rs::IteratorAdapter<::into_iterator_rust::MyContainerIter>(
      call_into_iter());
}
template <typename TAdaptedSelf_>
inline rs::IteratorEnd MyContainer::end() const& {
  return rs::IteratorEnd();
}
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_into_uiter(
    ::into_iterator_rust::MyContainer&,
    ::into_iterator_rust::MyContainerIterMut* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
template <typename TAdaptedSelf_>
inline rs::IteratorAdapter<::into_iterator_rust::MyContainerIterMut>
MyContainer::begin() & {
  MyContainer& self_ = *this;
  auto call_into_iter = [&]() -> decltype(auto) {
    crubit::Slot<::into_iterator_rust::MyContainerIterMut>
        __return_value_ret_val_holder;
    auto* __return_value_storage = __return_value_ret_val_holder.Get();
    __crubit_internal::__crubit_thunk_into_uiter(self_, __return_value_storage);
    return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
  };
  return rs::IteratorAdapter<::into_iterator_rust::MyContainerIterMut>(
      call_into_iter());
}
template <typename TAdaptedSelf_>
inline rs::IteratorEnd MyContainer::end() & {
  return rs::IteratorEnd();
}
inline void MyContainer::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyContainer, data));
}
static_assert(
    sizeof(MyContainerIntoIter) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyContainerIntoIter) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyContainerIntoIter>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::MyContainerIntoIter>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::MyContainerIntoIter>);
inline void MyContainerIntoIter::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyContainerIntoIter, index));
  static_assert(8 == offsetof(MyContainerIntoIter, data));
}
static_assert(
    sizeof(MyContainerIter) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyContainerIter) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyContainerIter>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::MyContainerIter>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::MyContainerIter>);
inline void MyContainerIter::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyContainerIter, data));
}
static_assert(
    sizeof(MyContainerIterMut) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyContainerIterMut) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyContainerIterMut>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::MyContainerIterMut>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::MyContainerIterMut>);
inline void MyContainerIterMut::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyContainerIterMut, data));
}
static_assert(
    sizeof(MyIterator) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(MyIterator) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<MyIterator>);
static_assert(
    ::std::is_trivially_move_constructible_v<::into_iterator_rust::MyIterator>);
static_assert(
    ::std::is_trivially_move_assignable_v<::into_iterator_rust::MyIterator>);
inline void MyIterator::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(MyIterator, value));
}
static_assert(
    sizeof(SimpleIntoIter) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SimpleIntoIter) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<SimpleIntoIter>);
static_assert(::std::is_trivially_move_constructible_v<
              ::into_iterator_rust::SimpleIntoIter>);
static_assert(::std::is_trivially_move_assignable_v<
              ::into_iterator_rust::SimpleIntoIter>);
inline void SimpleIntoIter::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SimpleIntoIter, val));
}
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_make_ucontainer(
    ::std::int32_t, ::std::int32_t, ::std::int32_t,
    ::into_iterator_rust::MyContainer* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
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
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_make_uinherent_ucontainer(
    ::into_iterator_rust::ContainerWithInherentBegin* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
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
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_make_uiterator(
    ::std::int32_t, ::into_iterator_rust::MyIterator* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
inline ::into_iterator_rust::MyIterator make_iterator(::std::int32_t value) {
  crubit::Slot<::into_iterator_rust::MyIterator> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_uiterator(value,
                                                   __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_make_umove_uonly_uiterator(
    ::std::int32_t, ::std::int32_t,
    ::into_iterator_rust::MoveOnlyIterator* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
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
/// \cond CRUBIT_INTERNAL
extern "C" void __crubit_thunk_make_uref_ucontainer(
    ::into_iterator_rust::MyIterator* $a crubit_nonnull,
    ::into_iterator_rust::ContainerWithRefIntoIter* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
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
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aContainerWithInherentBegin(
    ::into_iterator_rust::ContainerWithInherentBegin*,
    ::into_iterator_rust::SimpleIntoIter* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
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
/// \cond CRUBIT_INTERNAL
extern "C" ::into_iterator_rust::MyIterator& $a
__crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aContainerWithRefIntoIter_x0000003c_x00000027a_x0000003e(
    ::into_iterator_rust::ContainerWithRefIntoIter*);
/// \endcond
}  // namespace __crubit_internal
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
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMoveOnlyIterator(
    ::into_iterator_rust::MoveOnlyIterator&, unsigned char* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
}  // namespace into_iterator_rust
inline ::std::optional<::into_iterator_rust::MoveOnlyPayload> rs_std::impl<
    ::into_iterator_rust::MoveOnlyIterator,
    ::rs::core::iter::Iterator>::next(::into_iterator_rust::MoveOnlyIterator&
                                          self) {
  unsigned char __return_value_storage[::crubit::OptionAbi<
      ::crubit::TransmuteAbi<::into_iterator_rust::MoveOnlyPayload>>::kSize];
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMoveOnlyIterator(
          self, __return_value_storage);
  return ::crubit::internal::Decode<::crubit::OptionAbi<
      ::crubit::TransmuteAbi<::into_iterator_rust::MoveOnlyPayload>>>(
      ::crubit::OptionAbi<
          ::crubit::TransmuteAbi<::into_iterator_rust::MoveOnlyPayload>>(
          ::crubit::TransmuteAbi<::into_iterator_rust::MoveOnlyPayload>()),
      __return_value_storage);
}

namespace into_iterator_rust {
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_IntoIterator_uinto_uiter_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainer(
    ::into_iterator_rust::MyContainer*,
    ::into_iterator_rust::MyContainerIntoIter* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
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
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIntoIter(
    ::into_iterator_rust::MyContainerIntoIter&, unsigned char* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
}  // namespace into_iterator_rust
inline ::std::optional<::std::int32_t> rs_std::impl<
    ::into_iterator_rust::MyContainerIntoIter,
    ::rs::core::iter::Iterator>::next(::into_iterator_rust::MyContainerIntoIter&
                                          self) {
  unsigned char __return_value_storage
      [::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>::kSize];
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIntoIter(
          self, __return_value_storage);
  return ::crubit::internal::Decode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>(
          ::crubit::TransmuteAbi<::std::int32_t>()),
      __return_value_storage);
}

namespace into_iterator_rust {
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIter_x0000003c_x00000027a_x0000003e(
    ::into_iterator_rust::MyContainerIter&, unsigned char* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
}  // namespace into_iterator_rust
inline ::std::optional<::std::int32_t const * $a crubit_nonnull> rs_std::impl<
    ::into_iterator_rust::MyContainerIter,
    ::rs::core::iter::Iterator>::next(::into_iterator_rust::MyContainerIter&
                                          self) {
  unsigned char
      __return_value_storage[::crubit::OptionAbi<::crubit::TransmuteAbi<
          ::std::int32_t const * $static crubit_nonnull>>::kSize];
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIter_x0000003c_x00000027a_x0000003e(
          self, __return_value_storage);
  return ::crubit::internal::Decode<::crubit::OptionAbi<
      ::crubit::TransmuteAbi<::std::int32_t const * $static crubit_nonnull>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t const *
                                                 $static crubit_nonnull>>(
          ::crubit::TransmuteAbi<::std::int32_t const *
                                 $static crubit_nonnull>()),
      __return_value_storage);
}

namespace into_iterator_rust {
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIterMut_x0000003c_x00000027a_x0000003e(
    ::into_iterator_rust::MyContainerIterMut&, unsigned char* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
}  // namespace into_iterator_rust
inline ::std::optional<::std::int32_t* $a crubit_nonnull> rs_std::impl<
    ::into_iterator_rust::MyContainerIterMut,
    ::rs::core::iter::Iterator>::next(::into_iterator_rust::MyContainerIterMut&
                                          self) {
  unsigned char __return_value_storage[::crubit::OptionAbi<
      ::crubit::TransmuteAbi<::std::int32_t* $static crubit_nonnull>>::kSize];
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyContainerIterMut_x0000003c_x00000027a_x0000003e(
          self, __return_value_storage);
  return ::crubit::internal::Decode<::crubit::OptionAbi<
      ::crubit::TransmuteAbi<::std::int32_t* $static crubit_nonnull>>>(
      ::crubit::OptionAbi<
          ::crubit::TransmuteAbi<::std::int32_t* $static crubit_nonnull>>(
          ::crubit::TransmuteAbi<::std::int32_t* $static crubit_nonnull>()),
      __return_value_storage);
}

namespace into_iterator_rust {
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyIterator(
    ::into_iterator_rust::MyIterator&, unsigned char* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
}  // namespace into_iterator_rust
inline ::std::optional<::std::int32_t> rs_std::impl<
    ::into_iterator_rust::MyIterator,
    ::rs::core::iter::Iterator>::next(::into_iterator_rust::MyIterator& self) {
  unsigned char __return_value_storage
      [::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>::kSize];
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aMyIterator(
          self, __return_value_storage);
  return ::crubit::internal::Decode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>(
          ::crubit::TransmuteAbi<::std::int32_t>()),
      __return_value_storage);
}

namespace into_iterator_rust {
namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" void
__crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aSimpleIntoIter(
    ::into_iterator_rust::SimpleIntoIter&, unsigned char* __ret_ptr);
/// \endcond
}  // namespace __crubit_internal
}  // namespace into_iterator_rust
inline ::std::optional<::std::int32_t>
rs_std::impl<::into_iterator_rust::SimpleIntoIter, ::rs::core::iter::Iterator>::
    next(::into_iterator_rust::SimpleIntoIter& self) {
  unsigned char __return_value_storage
      [::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>::kSize];
  into_iterator_rust::__crubit_internal::
      __crubit_thunk_Iterator_unext_uinto_uiterator_urust_ugolden_x0000003a_x0000003aSimpleIntoIter(
          self, __return_value_storage);
  return ::crubit::internal::Decode<
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>>(
      ::crubit::OptionAbi<::crubit::TransmuteAbi<::std::int32_t>>(
          ::crubit::TransmuteAbi<::std::int32_t>()),
      __return_value_storage);
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_KNOWN_TRAITS_INTO_ITERATOR_INTO_ITERATOR_RUST_GOLDEN
