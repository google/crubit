// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// enums_golden

// clang-format off
#ifndef CC_BINDINGS_FROM_RS_TEST_ENUMS_ENUMS_GOLDEN
#define CC_BINDINGS_FROM_RS_TEST_ENUMS_ENUMS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/rs_std/enum.h"
#include "support/rs_std/traits.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

#include "support/rs_std/rs_alloc.h"
#include "support/rs_std/rs_core.h"

namespace enums::param_name_collisions {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: param_name_collisions :: FooBar") alignas(1)
    [[clang::trivial_abi]] FooBar final {
 public:
  // `enums_golden::param_name_collisions::FooBar` doesn't implement the
  // `Default` trait
  FooBar() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~FooBar() = default;
  FooBar(FooBar&&) = default;
  FooBar& operator=(FooBar&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  FooBar(const FooBar&) = default;
  FooBar& operator=(const FooBar&) = default;
  FooBar(::crubit::UnsafeRelocateTag, FooBar&& value);

  bool operator==(::enums::param_name_collisions::FooBar const& other) const;

 public:
  using Discriminant = ::std::uint8_t;

  struct Foo : public ::crubit::internal::Variant<
                   Foo, ::enums::param_name_collisions::FooBar, 0> {
   public:
    constexpr explicit Foo(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Foo(const Foo&) = default;
    Foo(Foo&&) = default;
    Foo& operator=(const Foo&) = default;
    Foo& operator=(Foo&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 0;
  };

  struct Bar : public ::crubit::internal::Variant<
                   Bar, ::enums::param_name_collisions::FooBar, 1> {
   public:
    constexpr explicit Bar(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Bar(const Bar&) = default;
    Bar(Bar&&) = default;
    Bar& operator=(const Bar&) = default;
    Bar& operator=(Bar&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 1;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::enums::param_name_collisions::FooBar, __Args...>
  constexpr explicit FooBar(::std::in_place_type_t<__Variant> __variant,
                            __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<::std::uint8_t, 0>, Foo, Bar>
      storage_;
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: param_name_collisions :: KeywordEnum") alignas(1)
    [[clang::trivial_abi]] KeywordEnum final {
 public:
  // `enums_golden::param_name_collisions::KeywordEnum` doesn't implement the
  // `Default` trait
  KeywordEnum() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~KeywordEnum() = default;
  KeywordEnum(KeywordEnum&&) = default;
  KeywordEnum& operator=(KeywordEnum&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  KeywordEnum(const KeywordEnum&) = default;
  KeywordEnum& operator=(const KeywordEnum&) = default;
  KeywordEnum(::crubit::UnsafeRelocateTag, KeywordEnum&& value);

  bool operator==(
      ::enums::param_name_collisions::KeywordEnum const& other) const;

 public:
  using Discriminant = ::std::uint8_t;

  struct match : public ::crubit::internal::Variant<
                     match, ::enums::param_name_collisions::KeywordEnum, 0> {
   public:
    constexpr explicit match(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    match(const match&) = default;
    match(match&&) = default;
    match& operator=(const match&) = default;
    match& operator=(match&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 0;
  };

  struct other : public ::crubit::internal::Variant<
                     other, ::enums::param_name_collisions::KeywordEnum, 1> {
   public:
    constexpr explicit other(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    other(const other&) = default;
    other(other&&) = default;
    other& operator=(const other&) = default;
    other& operator=(other&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 1;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::enums::param_name_collisions::KeywordEnum, __Args...>
  constexpr explicit KeywordEnum(::std::in_place_type_t<__Variant> __variant,
                                 __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<::std::uint8_t, 0>, match, other>
      storage_;
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: param_name_collisions :: OptionLike") alignas(1)
    [[clang::trivial_abi]] OptionLike final {
 public:
  // `enums_golden::param_name_collisions::OptionLike` doesn't implement the
  // `Default` trait
  OptionLike() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~OptionLike() = default;
  OptionLike(OptionLike&&) = default;
  OptionLike& operator=(OptionLike&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  OptionLike(const OptionLike&) = default;
  OptionLike& operator=(const OptionLike&) = default;
  OptionLike(::crubit::UnsafeRelocateTag, OptionLike&& value);

  bool matches_variant(::enums::param_name_collisions::OptionLike some) const;

  bool operator==(
      ::enums::param_name_collisions::OptionLike const& other) const;

 public:
  using Discriminant = ::std::uint8_t;

  struct some : public ::crubit::internal::Variant<
                    some, ::enums::param_name_collisions::OptionLike, 0> {
   public:
    constexpr explicit some(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    some(const some&) = default;
    some(some&&) = default;
    some& operator=(const some&) = default;
    some& operator=(some&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 0;
  };

  struct none : public ::crubit::internal::Variant<
                    none, ::enums::param_name_collisions::OptionLike, 1> {
   public:
    constexpr explicit none(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    none(const none&) = default;
    none(none&&) = default;
    none& operator=(const none&) = default;
    none& operator=(none&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 1;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::enums::param_name_collisions::OptionLike, __Args...>
  constexpr explicit OptionLike(::std::in_place_type_t<__Variant> __variant,
                                __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<::std::uint8_t, 0>, some, none>
      storage_;
};

bool check_both(::enums::param_name_collisions::OptionLike some,
                ::enums::param_name_collisions::OptionLike none);

bool is_bar(::enums::param_name_collisions::FooBar const& Bar);

bool is_foo(::enums::param_name_collisions::FooBar Foo);

bool is_match(::enums::param_name_collisions::KeywordEnum match);

bool is_none(::enums::param_name_collisions::OptionLike const& none);

bool is_some(::enums::param_name_collisions::OptionLike some);

}  // namespace enums::param_name_collisions

namespace enums::qr_error {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: qr_error :: StructuredQrError") alignas(8)
    [[clang::trivial_abi]] StructuredQrError final {
 public:
  // `enums_golden::qr_error::StructuredQrError` doesn't implement the `Default`
  // trait
  StructuredQrError() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~StructuredQrError() = default;
  StructuredQrError(StructuredQrError&&) = default;
  StructuredQrError& operator=(StructuredQrError&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  StructuredQrError(const StructuredQrError&) = default;
  StructuredQrError& operator=(const StructuredQrError&) = default;
  StructuredQrError(::crubit::UnsafeRelocateTag, StructuredQrError&& value);

  bool operator==(::enums::qr_error::StructuredQrError const& other) const;

 public:
  using Discriminant = ::std::uint8_t;

  struct AtLeast2Pieces
      : public ::crubit::internal::Variant<
            AtLeast2Pieces, ::enums::qr_error::StructuredQrError, 0> {
   public:
    constexpr explicit AtLeast2Pieces(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    AtLeast2Pieces(const AtLeast2Pieces&) = default;
    AtLeast2Pieces(AtLeast2Pieces&&) = default;
    AtLeast2Pieces& operator=(const AtLeast2Pieces&) = default;
    AtLeast2Pieces& operator=(AtLeast2Pieces&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 0;
  };

  struct TotalMismatch
      : public ::crubit::internal::Variant<
            TotalMismatch, ::enums::qr_error::StructuredQrError, 1> {
   public:
    constexpr explicit TotalMismatch(::crubit::internal::VariantKey,
                                     ::std::uintptr_t __field0)
        : __field0(::std::move(__field0)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    TotalMismatch(const TotalMismatch&) = default;
    TotalMismatch(TotalMismatch&&) = default;
    TotalMismatch& operator=(const TotalMismatch&) = default;
    TotalMismatch& operator=(TotalMismatch&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 1;
    [[maybe_unused]] unsigned char __crubit_padding0[7] = {};

   public:
    ::std::uintptr_t __field0;
  };

  struct MissingParts
      : public ::crubit::internal::Variant<
            MissingParts, ::enums::qr_error::StructuredQrError, 2> {
   public:
    constexpr explicit MissingParts(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    MissingParts(const MissingParts&) = default;
    MissingParts(MissingParts&&) = default;
    MissingParts& operator=(const MissingParts&) = default;
    MissingParts& operator=(MissingParts&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 2;
  };

  struct Parity : public ::crubit::internal::Variant<
                      Parity, ::enums::qr_error::StructuredQrError, 3> {
   public:
    constexpr explicit Parity(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Parity(const Parity&) = default;
    Parity(Parity&&) = default;
    Parity& operator=(const Parity&) = default;
    Parity& operator=(Parity&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 3;
  };

  struct TooShort : public ::crubit::internal::Variant<
                        TooShort, ::enums::qr_error::StructuredQrError, 4> {
   public:
    constexpr explicit TooShort(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    TooShort(const TooShort&) = default;
    TooShort(TooShort&&) = default;
    TooShort& operator=(const TooShort&) = default;
    TooShort& operator=(TooShort&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 4;
  };

  struct StructuredWrongMode
      : public ::crubit::internal::Variant<
            StructuredWrongMode, ::enums::qr_error::StructuredQrError, 5> {
   public:
    constexpr explicit StructuredWrongMode(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    StructuredWrongMode(const StructuredWrongMode&) = default;
    StructuredWrongMode(StructuredWrongMode&&) = default;
    StructuredWrongMode& operator=(const StructuredWrongMode&) = default;
    StructuredWrongMode& operator=(StructuredWrongMode&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 5;
  };

  struct StructuredWrongEnc
      : public ::crubit::internal::Variant<
            StructuredWrongEnc, ::enums::qr_error::StructuredQrError, 6> {
   public:
    constexpr explicit StructuredWrongEnc(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    StructuredWrongEnc(const StructuredWrongEnc&) = default;
    StructuredWrongEnc(StructuredWrongEnc&&) = default;
    StructuredWrongEnc& operator=(const StructuredWrongEnc&) = default;
    StructuredWrongEnc& operator=(StructuredWrongEnc&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 6;
  };

  struct SeqGreaterThanTotal
      : public ::crubit::internal::Variant<
            SeqGreaterThanTotal, ::enums::qr_error::StructuredQrError, 7> {
   public:
    constexpr explicit SeqGreaterThanTotal(::crubit::internal::VariantKey,
                                           ::std::uint8_t __field0,
                                           ::std::uint8_t __field1)
        : __field0(::std::move(__field0)), __field1(::std::move(__field1)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    SeqGreaterThanTotal(const SeqGreaterThanTotal&) = default;
    SeqGreaterThanTotal(SeqGreaterThanTotal&&) = default;
    SeqGreaterThanTotal& operator=(const SeqGreaterThanTotal&) = default;
    SeqGreaterThanTotal& operator=(SeqGreaterThanTotal&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 7;

   public:
    ::std::uint8_t __field0;
    ::std::uint8_t __field1;
  };

  struct LengthMismatch
      : public ::crubit::internal::Variant<
            LengthMismatch, ::enums::qr_error::StructuredQrError, 8> {
   public:
    constexpr explicit LengthMismatch(::crubit::internal::VariantKey,
                                      ::std::uintptr_t __field0,
                                      ::std::uintptr_t __field1)
        : __field0(::std::move(__field0)), __field1(::std::move(__field1)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    LengthMismatch(const LengthMismatch&) = default;
    LengthMismatch(LengthMismatch&&) = default;
    LengthMismatch& operator=(const LengthMismatch&) = default;
    LengthMismatch& operator=(LengthMismatch&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 8;
    [[maybe_unused]] unsigned char __crubit_padding0[7] = {};

   public:
    ::std::uintptr_t __field0;
    ::std::uintptr_t __field1;
  };

  struct UnsupportedVersion
      : public ::crubit::internal::Variant<
            UnsupportedVersion, ::enums::qr_error::StructuredQrError, 9> {
   public:
    constexpr explicit UnsupportedVersion(::crubit::internal::VariantKey,
                                          ::std::int16_t __field0)
        : __field0(::std::move(__field0)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    UnsupportedVersion(const UnsupportedVersion&) = default;
    UnsupportedVersion(UnsupportedVersion&&) = default;
    UnsupportedVersion& operator=(const UnsupportedVersion&) = default;
    UnsupportedVersion& operator=(UnsupportedVersion&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 9;
    [[maybe_unused]] unsigned char __crubit_padding0[1] = {};

   public:
    ::std::int16_t __field0;
  };

  struct SplitMax16
      : public ::crubit::internal::Variant<
            SplitMax16, ::enums::qr_error::StructuredQrError, 10> {
   public:
    constexpr explicit SplitMax16(::crubit::internal::VariantKey,
                                  ::std::uintptr_t __field0)
        : __field0(::std::move(__field0)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    SplitMax16(const SplitMax16&) = default;
    SplitMax16(SplitMax16&&) = default;
    SplitMax16& operator=(const SplitMax16&) = default;
    SplitMax16& operator=(SplitMax16&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 10;
    [[maybe_unused]] unsigned char __crubit_padding0[7] = {};

   public:
    ::std::uintptr_t __field0;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::enums::qr_error::StructuredQrError, __Args...>
  constexpr explicit StructuredQrError(
      ::std::in_place_type_t<__Variant> __variant, __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<::std::uint8_t, 0>,
      AtLeast2Pieces, TotalMismatch, MissingParts, Parity, TooShort,
      StructuredWrongMode, StructuredWrongEnc, SeqGreaterThanTotal,
      LengthMismatch, UnsupportedVersion, SplitMax16>
      storage_;

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: qr_error :: QrError") alignas(8) [[clang::trivial_abi]]
QrError final {
 public:
  // `enums_golden::qr_error::QrError` doesn't implement the `Default` trait
  QrError() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~QrError() = default;
  QrError(QrError&&) = default;
  QrError& operator=(QrError&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  QrError(const QrError&) = default;
  QrError& operator=(const QrError&) = default;
  QrError(::crubit::UnsafeRelocateTag, QrError&& value);

  bool is_data_too_long() const;

  bool operator==(::enums::qr_error::QrError const& other) const;

 public:
  using Discriminant = ::std::uint8_t;

  struct DataTooLong
      : public ::crubit::internal::Variant<DataTooLong,
                                           ::enums::qr_error::QrError, 0> {
   public:
    constexpr explicit DataTooLong(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    DataTooLong(const DataTooLong&) = default;
    DataTooLong(DataTooLong&&) = default;
    DataTooLong& operator=(const DataTooLong&) = default;
    DataTooLong& operator=(DataTooLong&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 11;
  };

  struct InvalidVersion
      : public ::crubit::internal::Variant<InvalidVersion,
                                           ::enums::qr_error::QrError, 1> {
   public:
    constexpr explicit InvalidVersion(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    InvalidVersion(const InvalidVersion&) = default;
    InvalidVersion(InvalidVersion&&) = default;
    InvalidVersion& operator=(const InvalidVersion&) = default;
    InvalidVersion& operator=(InvalidVersion&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 12;
  };

  struct UnsupportedCharacterSet
      : public ::crubit::internal::Variant<UnsupportedCharacterSet,
                                           ::enums::qr_error::QrError, 2> {
   public:
    constexpr explicit UnsupportedCharacterSet(::crubit::internal::VariantKey) {
    }

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    UnsupportedCharacterSet(const UnsupportedCharacterSet&) = default;
    UnsupportedCharacterSet(UnsupportedCharacterSet&&) = default;
    UnsupportedCharacterSet& operator=(const UnsupportedCharacterSet&) =
        default;
    UnsupportedCharacterSet& operator=(UnsupportedCharacterSet&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 13;
  };

  struct InvalidEciDesignator
      : public ::crubit::internal::Variant<InvalidEciDesignator,
                                           ::enums::qr_error::QrError, 3> {
   public:
    constexpr explicit InvalidEciDesignator(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    InvalidEciDesignator(const InvalidEciDesignator&) = default;
    InvalidEciDesignator(InvalidEciDesignator&&) = default;
    InvalidEciDesignator& operator=(const InvalidEciDesignator&) = default;
    InvalidEciDesignator& operator=(InvalidEciDesignator&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 14;
  };

  struct InvalidCharacter
      : public ::crubit::internal::Variant<InvalidCharacter,
                                           ::enums::qr_error::QrError, 4> {
   public:
    constexpr explicit InvalidCharacter(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    InvalidCharacter(const InvalidCharacter&) = default;
    InvalidCharacter(InvalidCharacter&&) = default;
    InvalidCharacter& operator=(const InvalidCharacter&) = default;
    InvalidCharacter& operator=(InvalidCharacter&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 15;
  };

  struct Structured
      : public ::crubit::internal::Variant<Structured,
                                           ::enums::qr_error::QrError, 5> {
   public:
    constexpr explicit Structured(::crubit::internal::VariantKey,
                                  ::enums::qr_error::StructuredQrError __field0)
        : __field0(::std::move(__field0)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Structured(const Structured&) = default;
    Structured(Structured&&) = default;
    Structured& operator=(const Structured&) = default;
    Structured& operator=(Structured&&) = default;

   public:
    ::enums::qr_error::StructuredQrError __field0;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::enums::qr_error::QrError, __Args...>
  constexpr explicit QrError(::std::in_place_type_t<__Variant> __variant,
                             __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Niche<::std::uint8_t, 0, 5, 0, 4, 11>,
      DataTooLong, InvalidVersion, UnsupportedCharacterSet,
      InvalidEciDesignator, InvalidCharacter, Structured>
      storage_;

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::qr_error

namespace enums::repr_128 {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_128 :: ReprI128") alignas(16)
    [[clang::trivial_abi]] ReprI128 final {
 public:
  // `enums_golden::repr_128::ReprI128` doesn't implement the `Default` trait
  ReprI128() = delete;

  static constexpr ReprI128 MakeZero();

  static constexpr ReprI128 MakeMinI128();

  static constexpr ReprI128 MakeMaxI128();

  // No custom `Drop` impl and no custom "drop glue" required
  ~ReprI128() = default;
  ReprI128(ReprI128&&) = default;
  ReprI128& operator=(ReprI128&&) = default;

  // `enums_golden::repr_128::ReprI128` doesn't implement the `Clone` trait
  ReprI128(const ReprI128&) = delete;
  ReprI128& operator=(const ReprI128&) = delete;
  ReprI128(::crubit::UnsafeRelocateTag, ReprI128&& value);

  bool is_min_i128() const;

  bool is_max_i128() const;

  // Not using the `enum_api_v2` bindings for this enum: 128-bit discriminants
  // are not supported (b/254094650)
 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 16> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr ReprI128(PrivateBytesTag, ::std::array<unsigned char, 16> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_128 :: ReprU128") alignas(16)
    [[clang::trivial_abi]] ReprU128 final {
 public:
  // `enums_golden::repr_128::ReprU128` doesn't implement the `Default` trait
  ReprU128() = delete;

  static constexpr ReprU128 MakeZero();

  static constexpr ReprU128 MakeMaxU128();

  // No custom `Drop` impl and no custom "drop glue" required
  ~ReprU128() = default;
  ReprU128(ReprU128&&) = default;
  ReprU128& operator=(ReprU128&&) = default;

  // `enums_golden::repr_128::ReprU128` doesn't implement the `Clone` trait
  ReprU128(const ReprU128&) = delete;
  ReprU128& operator=(const ReprU128&) = delete;
  ReprU128(::crubit::UnsafeRelocateTag, ReprU128&& value);

  bool is_max_u128() const;

  // Not using the `enum_api_v2` bindings for this enum: 128-bit discriminants
  // are not supported (b/254094650)
 private:
  // Field type has been replaced with a blob of bytes: No support for bindings
  // of individual non-repr(C) `enum`s
  ::std::array<unsigned char, 16> __opaque_blob_of_bytes;

 private:
  struct PrivateBytesTag {};
  constexpr ReprU128(PrivateBytesTag, ::std::array<unsigned char, 16> bytes)
      : __opaque_blob_of_bytes(bytes) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_128

namespace enums::repr_c {

struct CRUBIT_INTERNAL_RUST_TYPE(":: enums_golden :: repr_c :: MyEnum") alignas(
    8) [[clang::trivial_abi]] MyEnum final {
 public:
  // Default::default
  MyEnum();

  static ::enums::repr_c::MyEnum MakeE(::rs::alloc::string::String __param_0,
                                       ::std::int32_t __param_1);

  static ::enums::repr_c::MyEnum MakeA(::std::int32_t __param_0,
                                       ::std::int64_t __param_1);

  static MyEnum MakeF();

  // Error generating bindings for variant `enums_golden::repr_c::MyEnum::Z`:
  // Tuple type `()` is not supported in this context

  static MyEnum MakeG();

  // Error generating bindings for variant `enums_golden::repr_c::MyEnum::B`:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  // Error generating bindings for variant `enums_golden::repr_c::MyEnum::C`:
  // Constructing non-tuple, struct-like enum variants is not supported:
  // b/487357254

  static MyEnum MakeD();

  // Drop::drop
  ~MyEnum();

  MyEnum(MyEnum&&) noexcept;
  ::enums::repr_c::MyEnum& operator=(MyEnum&&) noexcept;

  // `enums_golden::repr_c::MyEnum` doesn't implement the `Clone` trait
  MyEnum(const MyEnum&) = delete;
  MyEnum& operator=(const MyEnum&) = delete;
  MyEnum(::crubit::UnsafeRelocateTag, MyEnum&& value);

  struct alignas(8) __crubit_E_struct {
   public:
    ::rs::alloc::string::String __field0;
    ::std::int32_t __field1;
  };
  struct alignas(8) __crubit_A_struct {
    ::std::int32_t __field0;
    ::std::int64_t __field1;
  };
  // Variant F has no size, so no struct is generated.

  // Variant Z has no size, so no struct is generated.

  // Variant G has no size, so no struct is generated.

  struct alignas(8) __crubit_B_struct {
    bool h;
    bool i;
  };
  struct alignas(8) __crubit_C_struct {
    ::std::int32_t a;
    ::std::int32_t b;
    ::std::int32_t c;
  };
  // Variant D has no size, so no struct is generated.

  enum class Tag : ::std::int64_t {
    E = INT64_C(0),
    A = INT64_C(1),
    F = INT64_C(2),
    Z = INT64_C(3),
    G = INT64_C(4),
    B = INT64_C(10000),
    C = INT64_C(10001),
    D = INT64_C(10002),
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_E_struct E;
    __crubit_A_struct A;
    __crubit_B_struct B;
    __crubit_C_struct C;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr MyEnum(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c :: ReprCWithExtremeDiscriminants") alignas(4)
    [[clang::trivial_abi]] ReprCWithExtremeDiscriminants final {
 public:
  // `enums_golden::repr_c::ReprCWithExtremeDiscriminants` doesn't implement the
  // `Default` trait
  ReprCWithExtremeDiscriminants() = delete;

  //  `MinusOne` is a regression test against bindings that used to result
  //  in a C++ compilation error:
  //
  //  ```
  //  .../test/enums/enums.h:480:16: error: integer literal is too large
  //  to be represented in a signed integer type, interpreting as unsigned
  //  [-Werror,-Wimplicitly-unsigned-literal]
  //   480 |     MinusOne = 18446744073709551615,
  //       |                ^
  //  .../test/enums/enums.h:480:16: error: enumerator value evaluates to
  //  18446744073709551615, which cannot be narrowed to type
  //  '::std::int32_t' (aka 'int') [-Wc++11-narrowing]
  //  ```
  static constexpr ReprCWithExtremeDiscriminants MakeMinusOne();

  static constexpr ReprCWithExtremeDiscriminants MakeMinusTwo();

  //  Based on https://github.com/rust-lang/rust/issues/124403:
  //  * Historically, Rust allowed `#[repr(C)]` enums to have discriminants of
  //  arbitrary
  //    size.  However, in C, the default enum size is typically `int` (32-bit
  //    signed). This mismatch creates non-portable layout differences between
  //    Rust and C/C++.
  //  * Rust has introduced the `repr_c_enums_larger_than_int` lint (which is
  //  part of the
  //    `future_incompatible` lint group). It warns when a `#[repr(C)]` enum's
  //    discriminant does not fit into a C `int` or `unsigned int` (essentially
  //    limiting portably supported values to the signed 32-bit range:
  //    `[i32::MIN, i32::MAX]`). This warning is planned to become a hard
  //    compiler error in a future Rust release.
  static constexpr ReprCWithExtremeDiscriminants MakeMinI32();

  static constexpr ReprCWithExtremeDiscriminants MakeMaxI32();

  // No custom `Drop` impl and no custom "drop glue" required
  ~ReprCWithExtremeDiscriminants() = default;
  ReprCWithExtremeDiscriminants(ReprCWithExtremeDiscriminants&&) = default;
  ReprCWithExtremeDiscriminants& operator=(ReprCWithExtremeDiscriminants&&) =
      default;

  // `enums_golden::repr_c::ReprCWithExtremeDiscriminants` doesn't implement the
  // `Clone` trait
  ReprCWithExtremeDiscriminants(const ReprCWithExtremeDiscriminants&) = delete;
  ReprCWithExtremeDiscriminants& operator=(
      const ReprCWithExtremeDiscriminants&) = delete;
  ReprCWithExtremeDiscriminants(::crubit::UnsafeRelocateTag,
                                ReprCWithExtremeDiscriminants&& value);

  bool is_minus_one() const;

  bool is_minus_two() const;

  bool is_min_i32() const;

  bool is_max_i32() const;

  // Variant MinusOne has no size, so no struct is generated.

  // Variant MinusTwo has no size, so no struct is generated.

  // Variant MinI32 has no size, so no struct is generated.

  // Variant MaxI32 has no size, so no struct is generated.

  enum class Tag : ::std::int32_t {
    MinusOne = INT32_C(-1),
    MinusTwo = INT32_C(-2),
    MinI32 = INT32_C(-2147483648),
    MaxI32 = INT32_C(2147483647),
  };

 public:
  Tag tag;

 private:
  struct PrivateTagCtorTag {};
  constexpr ReprCWithExtremeDiscriminants(PrivateTagCtorTag, Tag tag)
      : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

//  This enum is **not** a "ZST" (Zero-Sized Type), because of the C
//  representation (even though it has only a single variant with no payload).
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c :: ReprCWithSingleNoPayloadVariant") alignas(4)
    [[clang::trivial_abi]] ReprCWithSingleNoPayloadVariant final {
 public:
  // `enums_golden::repr_c::ReprCWithSingleNoPayloadVariant` doesn't implement
  // the `Default` trait
  ReprCWithSingleNoPayloadVariant() = delete;

  static constexpr ReprCWithSingleNoPayloadVariant MakeSingleVariant();

  // No custom `Drop` impl and no custom "drop glue" required
  ~ReprCWithSingleNoPayloadVariant() = default;
  ReprCWithSingleNoPayloadVariant(ReprCWithSingleNoPayloadVariant&&) = default;
  ReprCWithSingleNoPayloadVariant& operator=(
      ReprCWithSingleNoPayloadVariant&&) = default;

  // `enums_golden::repr_c::ReprCWithSingleNoPayloadVariant` doesn't implement
  // the `Clone` trait
  ReprCWithSingleNoPayloadVariant(const ReprCWithSingleNoPayloadVariant&) =
      delete;
  ReprCWithSingleNoPayloadVariant& operator=(
      const ReprCWithSingleNoPayloadVariant&) = delete;
  ReprCWithSingleNoPayloadVariant(::crubit::UnsafeRelocateTag,
                                  ReprCWithSingleNoPayloadVariant&& value);

  bool is_single_variant() const;

  // Variant SingleVariant has no size, so no struct is generated.

  enum class Tag : ::std::uint32_t {
    SingleVariant = 0,
  };

 public:
  Tag tag;

 private:
  struct PrivateTagCtorTag {};
  constexpr ReprCWithSingleNoPayloadVariant(PrivateTagCtorTag, Tag tag)
      : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_c

namespace enums::repr_c_clone_active_variant {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_clone_active_variant :: "
    "CloneActiveVariant") alignas(4) [[clang::trivial_abi]]
CloneActiveVariant final {
 public:
  // Default::default
  CloneActiveVariant();

  static ::enums::repr_c_clone_active_variant::CloneActiveVariant MakeA(
      ::std::int32_t __param_0);

  static ::enums::repr_c_clone_active_variant::CloneActiveVariant MakeB(
      ::std::int32_t __param_0);

  static ::enums::repr_c_clone_active_variant::CloneActiveVariant MakeC(
      ::std::int32_t __param_0);

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneActiveVariant() = default;
  CloneActiveVariant(CloneActiveVariant&&) = default;
  CloneActiveVariant& operator=(CloneActiveVariant&&) = default;

  // Clone::clone
  CloneActiveVariant(const CloneActiveVariant&) noexcept;

  // Clone::clone_from
  ::enums::repr_c_clone_active_variant::CloneActiveVariant& operator=(
      const CloneActiveVariant&) noexcept;

  CloneActiveVariant(::crubit::UnsafeRelocateTag, CloneActiveVariant&& value);

  struct alignas(4) __crubit_A_struct {
   public:
    ::std::int32_t __field0;
  };
  struct alignas(4) __crubit_B_struct {
    ::std::int32_t __field0;
  };
  struct alignas(4) __crubit_C_struct {
    ::std::int32_t __field0;
  };

  enum class Tag : ::std::int8_t {
    A = 0,
    B = 1,
    C = 2,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
    __crubit_B_struct B;
    __crubit_C_struct C;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr CloneActiveVariant(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

bool is_a(::enums::repr_c_clone_active_variant::CloneActiveVariant const& e);

bool is_b(::enums::repr_c_clone_active_variant::CloneActiveVariant const& e);

bool is_c(::enums::repr_c_clone_active_variant::CloneActiveVariant const& e);

}  // namespace enums::repr_c_clone_active_variant

namespace enums::repr_c_clone_counter {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_clone_counter :: CloneCount") alignas(8)
    [[clang::trivial_abi]] CloneCount final {
 public:
  // Default::default
  CloneCount();

  // Error generating bindings for variant
  // `enums_golden::repr_c_clone_counter::CloneCount::A`: Constructing
  // non-tuple, struct-like enum variants is not supported: b/487357254

  // No custom `Drop` impl and no custom "drop glue" required
  ~CloneCount() = default;
  CloneCount(CloneCount&&) = default;
  CloneCount& operator=(CloneCount&&) = default;

  // Clone::clone
  CloneCount(const CloneCount&) noexcept;

  // Clone::clone_from
  ::enums::repr_c_clone_counter::CloneCount& operator=(
      const CloneCount&) noexcept;

  CloneCount(::crubit::UnsafeRelocateTag, CloneCount&& value);

  struct alignas(8) __crubit_A_struct {
   public:
    ::std::int32_t* p;
  };

  enum class Tag : ::std::int8_t {
    A = 0,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr CloneCount(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_c_clone_counter

namespace enums::repr_c_drop {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_drop :: DropMe") alignas(8)
    [[clang::trivial_abi]] DropMe final {
 public:
  // Default::default
  DropMe();

  static ::enums::repr_c_drop::DropMe MakeA(::std::int32_t __param_0);

  static ::enums::repr_c_drop::DropMe MakeB(::std::int64_t __param_0);

  static DropMe MakeQ();

  // Error generating bindings for variant
  // `enums_golden::repr_c_drop::DropMe::C`: Constructing non-tuple, struct-like
  // enum variants is not supported: b/487357254

  // Drop::drop
  ~DropMe();

  DropMe(DropMe&&) noexcept;
  ::enums::repr_c_drop::DropMe& operator=(DropMe&&) noexcept;

  // `enums_golden::repr_c_drop::DropMe` doesn't implement the `Clone` trait
  DropMe(const DropMe&) = delete;
  DropMe& operator=(const DropMe&) = delete;
  DropMe(::crubit::UnsafeRelocateTag, DropMe&& value);

  struct alignas(8) __crubit_A_struct {
   public:
    ::std::int32_t __field0;
  };
  struct alignas(8) __crubit_B_struct {
    ::std::int64_t __field0;
  };
  // Variant Q has no size, so no struct is generated.

  struct alignas(8) __crubit_C_struct {
    ::std::int32_t* p;
  };

  enum class Tag : ::std::uint32_t {
    A = 0,
    B = 1,
    Q = 2,
    C = 3,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
    __crubit_B_struct B;
    __crubit_C_struct C;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr DropMe(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_c_drop

namespace enums::repr_c_variant_alignment {

//  `#[repr(align(16))]` raises the enum's alignment to 16 while the payload
//  still sits at offset 4. `enum_alignment - payload_offset` yields 12 here,
//  and `alignas(12)` is a hard "requested alignment is not a positive power
//  of 2" C++ error.
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_variant_alignment :: "
    "OverAlignedNarrowPayload") alignas(16) [[clang::trivial_abi]]
OverAlignedNarrowPayload final {
 public:
  // `enums_golden::repr_c_variant_alignment::OverAlignedNarrowPayload` doesn't
  // implement the `Default` trait
  OverAlignedNarrowPayload() = delete;

  static ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload MakeA(
      ::std::int32_t __param_0);

  static ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload MakeB(
      ::std::uint8_t __param_0);

  static constexpr OverAlignedNarrowPayload MakeC();

  // No custom `Drop` impl and no custom "drop glue" required
  ~OverAlignedNarrowPayload() = default;
  OverAlignedNarrowPayload(OverAlignedNarrowPayload&&) = default;
  OverAlignedNarrowPayload& operator=(OverAlignedNarrowPayload&&) = default;

  // `enums_golden::repr_c_variant_alignment::OverAlignedNarrowPayload` doesn't
  // implement the `Clone` trait
  OverAlignedNarrowPayload(const OverAlignedNarrowPayload&) = delete;
  OverAlignedNarrowPayload& operator=(const OverAlignedNarrowPayload&) = delete;
  OverAlignedNarrowPayload(::crubit::UnsafeRelocateTag,
                           OverAlignedNarrowPayload&& value);

  ::std::int32_t value() const;

  struct alignas(4) __crubit_A_struct {
   public:
    ::std::int32_t __field0;
  };
  struct alignas(4) __crubit_B_struct {
    ::std::uint8_t __field0;
  };
  // Variant C has no size, so no struct is generated.

  enum class Tag : ::std::uint32_t {
    A = 0,
    B = 1,
    C = 2,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
    __crubit_B_struct B;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr OverAlignedNarrowPayload(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

//  Same shape as above, but with a payload wide enough that its offset
//  changes per target: 8 where `u64` is 8-byte aligned, 4 on x86-32. Either
//  way the enum's alignment stays 16, so using that alignment for the
//  payload struct would push the union past Rust's payload offset. Unlike
//  the three shapes above this one is a "don't regress" case: the old
//  `enum_alignment - payload_offset` happened to produce a correct value
//  for it.
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_variant_alignment :: "
    "OverAlignedWidePayload") alignas(16) [[clang::trivial_abi]]
OverAlignedWidePayload final {
 public:
  // `enums_golden::repr_c_variant_alignment::OverAlignedWidePayload` doesn't
  // implement the `Default` trait
  OverAlignedWidePayload() = delete;

  static ::enums::repr_c_variant_alignment::OverAlignedWidePayload MakeA(
      ::std::uint64_t __param_0);

  static ::enums::repr_c_variant_alignment::OverAlignedWidePayload MakeB(
      ::std::uint8_t __param_0);

  static constexpr OverAlignedWidePayload MakeC();

  // No custom `Drop` impl and no custom "drop glue" required
  ~OverAlignedWidePayload() = default;
  OverAlignedWidePayload(OverAlignedWidePayload&&) = default;
  OverAlignedWidePayload& operator=(OverAlignedWidePayload&&) = default;

  // `enums_golden::repr_c_variant_alignment::OverAlignedWidePayload` doesn't
  // implement the `Clone` trait
  OverAlignedWidePayload(const OverAlignedWidePayload&) = delete;
  OverAlignedWidePayload& operator=(const OverAlignedWidePayload&) = delete;
  OverAlignedWidePayload(::crubit::UnsafeRelocateTag,
                         OverAlignedWidePayload&& value);

  ::std::uint64_t value() const;

  struct alignas(8) __crubit_A_struct {
   public:
    ::std::uint64_t __field0;
  };
  struct alignas(8) __crubit_B_struct {
    ::std::uint8_t __field0;
  };
  // Variant C has no size, so no struct is generated.

  enum class Tag : ::std::uint32_t {
    A = 0,
    B = 1,
    C = 2,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
    __crubit_B_struct B;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr OverAlignedWidePayload(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

//  `AtomicU64` has no C++ binding, so the field is replaced by an opaque
//  `std::array<unsigned char, 8>`, which has alignment 1 and therefore
//  throws away the Rust field's alignment. Without a correct `alignas` on the
//  payload struct the union lands at offset 4 instead of Rust's 8.
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_variant_alignment :: "
    "UnbindableFieldPayload") alignas(8) [[clang::trivial_abi]]
UnbindableFieldPayload final {
 public:
  // `enums_golden::repr_c_variant_alignment::UnbindableFieldPayload` doesn't
  // implement the `Default` trait
  UnbindableFieldPayload() = delete;

  // Error generating bindings for variant
  // `enums_golden::repr_c_variant_alignment::UnbindableFieldPayload::A`:
  // Generic types are not supported yet (b/259749095)

  static ::enums::repr_c_variant_alignment::UnbindableFieldPayload MakeB(
      ::std::uint8_t __param_0);

  // No custom `Drop` impl and no custom "drop glue" required
  ~UnbindableFieldPayload() = default;
  UnbindableFieldPayload(UnbindableFieldPayload&&) = default;
  UnbindableFieldPayload& operator=(UnbindableFieldPayload&&) = default;

  // `enums_golden::repr_c_variant_alignment::UnbindableFieldPayload` doesn't
  // implement the `Clone` trait
  UnbindableFieldPayload(const UnbindableFieldPayload&) = delete;
  UnbindableFieldPayload& operator=(const UnbindableFieldPayload&) = delete;
  UnbindableFieldPayload(::crubit::UnsafeRelocateTag,
                         UnbindableFieldPayload&& value);

  static ::enums::repr_c_variant_alignment::UnbindableFieldPayload make_a(
      ::std::uint64_t value);

  ::std::uint64_t value() const;

  struct alignas(8) __crubit_A_struct {
   private:
    // Field type has been replaced with a blob of bytes: Generic types are not
    // supported yet (b/259749095)
    ::std::array<unsigned char, 8> __field0;
  };
  struct alignas(8) __crubit_B_struct {
   public:
    ::std::uint8_t __field0;
  };

  enum class Tag : ::std::uint32_t {
    A = 0,
    B = 1,
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
    __crubit_B_struct B;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr UnbindableFieldPayload(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

//  On x86-32 `i64` has size 8 but alignment 4, so the payload offset (8)
//  exceeds the enum's own alignment (4). Deriving the payload struct's
//  alignment as `enum_alignment - payload_offset` underflowed `u64` here and
//  panicked while generating bindings.
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_c_variant_alignment :: "
    "WideTagNarrowAlignment") alignas(8) [[clang::trivial_abi]]
WideTagNarrowAlignment final {
 public:
  // `enums_golden::repr_c_variant_alignment::WideTagNarrowAlignment` doesn't
  // implement the `Default` trait
  WideTagNarrowAlignment() = delete;

  static ::enums::repr_c_variant_alignment::WideTagNarrowAlignment MakeA(
      ::std::int32_t __param_0, ::std::int64_t __param_1);

  static ::enums::repr_c_variant_alignment::WideTagNarrowAlignment MakeB(
      ::std::uint8_t __param_0);

  // No custom `Drop` impl and no custom "drop glue" required
  ~WideTagNarrowAlignment() = default;
  WideTagNarrowAlignment(WideTagNarrowAlignment&&) = default;
  WideTagNarrowAlignment& operator=(WideTagNarrowAlignment&&) = default;

  // `enums_golden::repr_c_variant_alignment::WideTagNarrowAlignment` doesn't
  // implement the `Clone` trait
  WideTagNarrowAlignment(const WideTagNarrowAlignment&) = delete;
  WideTagNarrowAlignment& operator=(const WideTagNarrowAlignment&) = delete;
  WideTagNarrowAlignment(::crubit::UnsafeRelocateTag,
                         WideTagNarrowAlignment&& value);

  ::std::int64_t sum() const;

  struct alignas(8) __crubit_A_struct {
   public:
    ::std::int32_t __field0;
    ::std::int64_t __field1;
  };
  struct alignas(8) __crubit_B_struct {
    ::std::uint8_t __field0;
  };

  enum class Tag : ::std::int64_t {
    A = INT64_C(0),
    B = INT64_C(1),
  };

 public:
  Tag tag;

 public:
  union {
    __crubit_A_struct A;
    __crubit_B_struct B;
  };

 private:
  struct PrivateTagCtorTag {};
  constexpr WideTagNarrowAlignment(PrivateTagCtorTag, Tag tag) : tag(tag) {}

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_c_variant_alignment

namespace enums::repr_int {

//  Two `NoPayloadX` variants to test that the tag is correctly set
//  (`NoPayload1` should have a tag of 0 and therefore `NoPayload2` is a
//  slightly better test for things like encoding the tag value with the
//  proper endianness, especially given that the tag is 4 bytes wide).
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_int :: IntReprEnumWithNoPayload") alignas(4)
    [[clang::trivial_abi]] IntReprEnumWithNoPayload final {
 public:
  // `enums_golden::repr_int::IntReprEnumWithNoPayload` doesn't implement the
  // `Default` trait
  IntReprEnumWithNoPayload() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~IntReprEnumWithNoPayload() = default;
  IntReprEnumWithNoPayload(IntReprEnumWithNoPayload&&) = default;
  IntReprEnumWithNoPayload& operator=(IntReprEnumWithNoPayload&&) = default;

  // `enums_golden::repr_int::IntReprEnumWithNoPayload` doesn't implement the
  // `Clone` trait
  IntReprEnumWithNoPayload(const IntReprEnumWithNoPayload&) = delete;
  IntReprEnumWithNoPayload& operator=(const IntReprEnumWithNoPayload&) = delete;
  IntReprEnumWithNoPayload(::crubit::UnsafeRelocateTag,
                           IntReprEnumWithNoPayload&& value);

  bool is_no_payload1() const;

  bool is_no_payload2() const;

 public:
  using Discriminant = ::std::uint32_t;

  struct NoPayload1
      : public ::crubit::internal::Variant<
            NoPayload1, ::enums::repr_int::IntReprEnumWithNoPayload, 0> {
   public:
    constexpr explicit NoPayload1(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    NoPayload1(const NoPayload1&) = default;
    NoPayload1(NoPayload1&&) = default;
    NoPayload1& operator=(const NoPayload1&) = default;
    NoPayload1& operator=(NoPayload1&&) = default;
    [[maybe_unused]] ::std::uint32_t __crubit_tag = 0;
  };

  struct NoPayload2
      : public ::crubit::internal::Variant<
            NoPayload2, ::enums::repr_int::IntReprEnumWithNoPayload, 1234> {
   public:
    constexpr explicit NoPayload2(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    NoPayload2(const NoPayload2&) = default;
    NoPayload2(NoPayload2&&) = default;
    NoPayload2& operator=(const NoPayload2&) = default;
    NoPayload2& operator=(NoPayload2&&) = default;
    [[maybe_unused]] ::std::uint32_t __crubit_tag = 1234;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::enums::repr_int::IntReprEnumWithNoPayload, __Args...>
  constexpr explicit IntReprEnumWithNoPayload(
      ::std::in_place_type_t<__Variant> __variant, __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<::std::uint32_t, 0>, NoPayload1,
      NoPayload2>
      storage_;
};

//  This enum is **not** a "ZST" (Zero-Sized Type), because of `#[repr(u32)]`
//  (even though it has only a single variant with no payload).
struct
    CRUBIT_INTERNAL_RUST_TYPE(
        ":: enums_golden :: repr_int :: "
        "IntReprWithSingleNoPayloadVariant") alignas(4) [[clang::trivial_abi]]
    IntReprWithSingleNoPayloadVariant final {
 public:
  // `enums_golden::repr_int::IntReprWithSingleNoPayloadVariant` doesn't
  // implement the `Default` trait
  IntReprWithSingleNoPayloadVariant() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~IntReprWithSingleNoPayloadVariant() = default;
  IntReprWithSingleNoPayloadVariant(IntReprWithSingleNoPayloadVariant&&) =
      default;
  IntReprWithSingleNoPayloadVariant& operator=(
      IntReprWithSingleNoPayloadVariant&&) = default;

  // `enums_golden::repr_int::IntReprWithSingleNoPayloadVariant` doesn't
  // implement the `Clone` trait
  IntReprWithSingleNoPayloadVariant(const IntReprWithSingleNoPayloadVariant&) =
      delete;
  IntReprWithSingleNoPayloadVariant& operator=(
      const IntReprWithSingleNoPayloadVariant&) = delete;
  IntReprWithSingleNoPayloadVariant(::crubit::UnsafeRelocateTag,
                                    IntReprWithSingleNoPayloadVariant&& value);

  bool is_single_variant() const;

 public:
  using Discriminant = ::std::uint32_t;

  struct SingleVariant
      : public ::crubit::internal::Variant<
            SingleVariant, ::enums::repr_int::IntReprWithSingleNoPayloadVariant,
            0> {
   public:
    constexpr explicit SingleVariant(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    SingleVariant(const SingleVariant&) = default;
    SingleVariant(SingleVariant&&) = default;
    SingleVariant& operator=(const SingleVariant&) = default;
    SingleVariant& operator=(SingleVariant&&) = default;
    [[maybe_unused]] ::std::uint32_t __crubit_tag = 0;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::enums::repr_int::IntReprWithSingleNoPayloadVariant,
        __Args...>
  constexpr explicit IntReprWithSingleNoPayloadVariant(
      ::std::in_place_type_t<__Variant> __variant, __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<::std::uint32_t, 0>,
      SingleVariant>
      storage_;
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_int :: NegReprIntEnum") alignas(1)
    [[clang::trivial_abi]] NegReprIntEnum final {
 public:
  // `enums_golden::repr_int::NegReprIntEnum` doesn't implement the `Default`
  // trait
  NegReprIntEnum() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NegReprIntEnum() = default;
  NegReprIntEnum(NegReprIntEnum&&) = default;
  NegReprIntEnum& operator=(NegReprIntEnum&&) = default;

  // `enums_golden::repr_int::NegReprIntEnum` doesn't implement the `Clone`
  // trait
  NegReprIntEnum(const NegReprIntEnum&) = delete;
  NegReprIntEnum& operator=(const NegReprIntEnum&) = delete;
  NegReprIntEnum(::crubit::UnsafeRelocateTag, NegReprIntEnum&& value);

  bool is_minus_one() const;

  bool is_minus_two() const;

 public:
  using Discriminant = ::std::int8_t;

  struct MinusOne : public ::crubit::internal::Variant<
                        MinusOne, ::enums::repr_int::NegReprIntEnum, -1> {
   public:
    constexpr explicit MinusOne(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    MinusOne(const MinusOne&) = default;
    MinusOne(MinusOne&&) = default;
    MinusOne& operator=(const MinusOne&) = default;
    MinusOne& operator=(MinusOne&&) = default;
    [[maybe_unused]] ::std::int8_t __crubit_tag = -1;
  };

  struct MinusTwo : public ::crubit::internal::Variant<
                        MinusTwo, ::enums::repr_int::NegReprIntEnum, -2> {
   public:
    constexpr explicit MinusTwo(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    MinusTwo(const MinusTwo&) = default;
    MinusTwo(MinusTwo&&) = default;
    MinusTwo& operator=(const MinusTwo&) = default;
    MinusTwo& operator=(MinusTwo&&) = default;
    [[maybe_unused]] ::std::int8_t __crubit_tag = -2;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::enums::repr_int::NegReprIntEnum, __Args...>
  constexpr explicit NegReprIntEnum(::std::in_place_type_t<__Variant> __variant,
                                    __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<::std::int8_t, 0>, MinusOne,
      MinusTwo>
      storage_;
};

}  // namespace enums::repr_int

namespace enums::repr_rust {

//  Doc comment of RustReprEnum.
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_rust :: RustReprEnum") alignas(4)
    [[clang::trivial_abi]] RustReprEnum final {
 public:
  // `enums_golden::repr_rust::RustReprEnum` doesn't implement the `Default`
  // trait
  RustReprEnum() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~RustReprEnum() = default;
  RustReprEnum(RustReprEnum&&) = default;
  RustReprEnum& operator=(RustReprEnum&&) = default;

  // `enums_golden::repr_rust::RustReprEnum` doesn't implement the `Clone` trait
  RustReprEnum(const RustReprEnum&) = delete;
  RustReprEnum& operator=(const RustReprEnum&) = delete;
  RustReprEnum(::crubit::UnsafeRelocateTag, RustReprEnum&& value);

  ::std::int32_t get_variant_number() const;

  bool is_tuple_payload_variant() const;

  ::std::int32_t get_first_item_from_tuple_payload() const;

 public:
  using Discriminant = ::std::uint32_t;

  //  Doc comment of Variant1.
  struct Variant1 : public ::crubit::internal::Variant<
                        Variant1, ::enums::repr_rust::RustReprEnum, 0> {
   public:
    constexpr explicit Variant1(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Variant1(const Variant1&) = default;
    Variant1(Variant1&&) = default;
    Variant1& operator=(const Variant1&) = default;
    Variant1& operator=(Variant1&&) = default;
    [[maybe_unused]] ::std::uint32_t __crubit_tag = 0;
  };

  struct Variant2 : public ::crubit::internal::Variant<
                        Variant2, ::enums::repr_rust::RustReprEnum, 1> {
   public:
    constexpr explicit Variant2(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Variant2(const Variant2&) = default;
    Variant2(Variant2&&) = default;
    Variant2& operator=(const Variant2&) = default;
    Variant2& operator=(Variant2&&) = default;
    [[maybe_unused]] ::std::uint32_t __crubit_tag = 1;
  };

  struct Variant3 : public ::crubit::internal::Variant<
                        Variant3, ::enums::repr_rust::RustReprEnum, 2> {
   public:
    constexpr explicit Variant3(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    Variant3(const Variant3&) = default;
    Variant3(Variant3&&) = default;
    Variant3& operator=(const Variant3&) = default;
    Variant3& operator=(Variant3&&) = default;
    [[maybe_unused]] ::std::uint32_t __crubit_tag = 2;
  };

  struct TuplePayloadVariant
      : public ::crubit::internal::Variant<
            TuplePayloadVariant, ::enums::repr_rust::RustReprEnum, 3> {
   public:
    constexpr explicit TuplePayloadVariant(::crubit::internal::VariantKey,
                                           ::std::int32_t __field0,
                                           ::std::int32_t __field1)
        : __field0(::std::move(__field0)), __field1(::std::move(__field1)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    TuplePayloadVariant(const TuplePayloadVariant&) = default;
    TuplePayloadVariant(TuplePayloadVariant&&) = default;
    TuplePayloadVariant& operator=(const TuplePayloadVariant&) = default;
    TuplePayloadVariant& operator=(TuplePayloadVariant&&) = default;
    [[maybe_unused]] ::std::uint32_t __crubit_tag = 3;

   public:
    ::std::int32_t __field0;
    ::std::int32_t __field1;
  };

  struct StructPayloadVariant
      : public ::crubit::internal::Variant<
            StructPayloadVariant, ::enums::repr_rust::RustReprEnum, 4> {
   public:
    constexpr explicit StructPayloadVariant(::crubit::internal::VariantKey,
                                            ::std::int32_t x, ::std::int32_t y)
        : x(::std::move(x)), y(::std::move(y)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    StructPayloadVariant(const StructPayloadVariant&) = default;
    StructPayloadVariant(StructPayloadVariant&&) = default;
    StructPayloadVariant& operator=(const StructPayloadVariant&) = default;
    StructPayloadVariant& operator=(StructPayloadVariant&&) = default;
    [[maybe_unused]] ::std::uint32_t __crubit_tag = 4;

   public:
    ::std::int32_t x;
    ::std::int32_t y;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::enums::repr_rust::RustReprEnum, __Args...>
  constexpr explicit RustReprEnum(::std::in_place_type_t<__Variant> __variant,
                                  __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<::std::uint32_t, 0>, Variant1,
      Variant2, Variant3, TuplePayloadVariant, StructPayloadVariant>
      storage_;

 private:
  static void __crubit_field_offset_assertions();
};

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_rust :: "
    "RustReprWithNamingConflictBetweenCtorsAndMethods") alignas(4)
    [[clang::trivial_abi]]
    RustReprWithNamingConflictBetweenCtorsAndMethods final {
 public:
  // `enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods`
  // doesn't implement the `Default` trait
  RustReprWithNamingConflictBetweenCtorsAndMethods() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~RustReprWithNamingConflictBetweenCtorsAndMethods() = default;
  RustReprWithNamingConflictBetweenCtorsAndMethods(
      RustReprWithNamingConflictBetweenCtorsAndMethods&&) = default;
  RustReprWithNamingConflictBetweenCtorsAndMethods& operator=(
      RustReprWithNamingConflictBetweenCtorsAndMethods&&) = default;

  // `enums_golden::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods`
  // doesn't implement the `Clone` trait
  RustReprWithNamingConflictBetweenCtorsAndMethods(
      const RustReprWithNamingConflictBetweenCtorsAndMethods&) = delete;
  RustReprWithNamingConflictBetweenCtorsAndMethods& operator=(
      const RustReprWithNamingConflictBetweenCtorsAndMethods&) = delete;
  RustReprWithNamingConflictBetweenCtorsAndMethods(
      ::crubit::UnsafeRelocateTag,
      RustReprWithNamingConflictBetweenCtorsAndMethods&& value);

  //  Presence of this function tests the scenario where `MakeNoPayloadVariant`
  //  is a name of:
  //  1. A static method (here/below).
  //  2. An auto-generated factory/constructor static method
  static ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
  MakeNoPayloadVariant();

  static ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
  MakeTuplePayloadVariant(::std::int32_t i);

  static ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods
  MakeStructPayloadVariant(::std::int32_t x);

  ::std::int32_t get_variant_number() const;

  ::std::int32_t get_value() const;

 public:
  using Discriminant = ::std::uint32_t;

  struct NoPayloadVariant
      : public ::crubit::internal::Variant<
            NoPayloadVariant,
            ::enums::repr_rust::
                RustReprWithNamingConflictBetweenCtorsAndMethods,
            0> {
   public:
    constexpr explicit NoPayloadVariant(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    NoPayloadVariant(const NoPayloadVariant&) = default;
    NoPayloadVariant(NoPayloadVariant&&) = default;
    NoPayloadVariant& operator=(const NoPayloadVariant&) = default;
    NoPayloadVariant& operator=(NoPayloadVariant&&) = default;
    [[maybe_unused]] ::std::uint32_t __crubit_tag = 0;
  };

  struct TuplePayloadVariant
      : public ::crubit::internal::Variant<
            TuplePayloadVariant,
            ::enums::repr_rust::
                RustReprWithNamingConflictBetweenCtorsAndMethods,
            1> {
   public:
    constexpr explicit TuplePayloadVariant(::crubit::internal::VariantKey,
                                           ::std::int32_t __field0)
        : __field0(::std::move(__field0)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    TuplePayloadVariant(const TuplePayloadVariant&) = default;
    TuplePayloadVariant(TuplePayloadVariant&&) = default;
    TuplePayloadVariant& operator=(const TuplePayloadVariant&) = default;
    TuplePayloadVariant& operator=(TuplePayloadVariant&&) = default;
    [[maybe_unused]] ::std::uint32_t __crubit_tag = 1;

   public:
    ::std::int32_t __field0;
  };

  struct StructPayloadVariant
      : public ::crubit::internal::Variant<
            StructPayloadVariant,
            ::enums::repr_rust::
                RustReprWithNamingConflictBetweenCtorsAndMethods,
            2> {
   public:
    constexpr explicit StructPayloadVariant(::crubit::internal::VariantKey,
                                            ::std::int32_t x)
        : x(::std::move(x)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    StructPayloadVariant(const StructPayloadVariant&) = default;
    StructPayloadVariant(StructPayloadVariant&&) = default;
    StructPayloadVariant& operator=(const StructPayloadVariant&) = default;
    StructPayloadVariant& operator=(StructPayloadVariant&&) = default;
    [[maybe_unused]] ::std::uint32_t __crubit_tag = 2;

   public:
    ::std::int32_t x;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant,
        ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods,
        __Args...>
  constexpr explicit RustReprWithNamingConflictBetweenCtorsAndMethods(
      ::std::in_place_type_t<__Variant> __variant, __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<::std::uint32_t, 0>,
      NoPayloadVariant, TuplePayloadVariant, StructPayloadVariant>
      storage_;

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for enum
// `enums_golden::repr_rust::RustReprWithSingleNoPayloadVariant`: Zero-sized
// types (ZSTs) are not supported (b/258259459)

//  This enum is not a "ZST" (Zero-Sized Type), because of the payload.
//  There is no tag / discriminant field, because there is only one variant.
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: enums_golden :: repr_rust :: "
    "RustReprWithSingleTuplePayloadVariant") alignas(4) [[clang::trivial_abi]]
RustReprWithSingleTuplePayloadVariant final {
 public:
  // `enums_golden::repr_rust::RustReprWithSingleTuplePayloadVariant` doesn't
  // implement the `Default` trait
  RustReprWithSingleTuplePayloadVariant() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~RustReprWithSingleTuplePayloadVariant() = default;
  RustReprWithSingleTuplePayloadVariant(
      RustReprWithSingleTuplePayloadVariant&&) = default;
  RustReprWithSingleTuplePayloadVariant& operator=(
      RustReprWithSingleTuplePayloadVariant&&) = default;

  // `enums_golden::repr_rust::RustReprWithSingleTuplePayloadVariant` doesn't
  // implement the `Clone` trait
  RustReprWithSingleTuplePayloadVariant(
      const RustReprWithSingleTuplePayloadVariant&) = delete;
  RustReprWithSingleTuplePayloadVariant& operator=(
      const RustReprWithSingleTuplePayloadVariant&) = delete;
  RustReprWithSingleTuplePayloadVariant(
      ::crubit::UnsafeRelocateTag,
      RustReprWithSingleTuplePayloadVariant&& value);

  ::std::int32_t get_single_item_from_tuple_payload() const;

 public:
  using Discriminant = ::std::int64_t;

  struct SingleVariant
      : public ::crubit::internal::Variant<
            SingleVariant,
            ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant,
            INT64_C(0)> {
   public:
    constexpr explicit SingleVariant(::crubit::internal::VariantKey,
                                     ::std::int32_t __field0)
        : __field0(::std::move(__field0)) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    SingleVariant(const SingleVariant&) = default;
    SingleVariant(SingleVariant&&) = default;
    SingleVariant& operator=(const SingleVariant&) = default;
    SingleVariant& operator=(SingleVariant&&) = default;

   public:
    ::std::int32_t __field0;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant,
        __Args...>
  constexpr explicit RustReprWithSingleTuplePayloadVariant(
      ::std::in_place_type_t<__Variant> __variant, __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Single<::std::int64_t, INT64_C(0)>,
      SingleVariant>
      storage_;

 private:
  static void __crubit_field_offset_assertions();
};

}  // namespace enums::repr_rust

template <>
struct ::rs::impl<::enums::param_name_collisions::FooBar, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct ::rs::impl<::enums::param_name_collisions::KeywordEnum,
                  ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct ::rs::impl<::enums::param_name_collisions::OptionLike,
                  ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct ::rs::impl<::enums::qr_error::QrError, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct ::rs::impl<::enums::qr_error::QrError, ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<enums_golden::qr_error::QrError as std::fmt::Debug>::fmt`: Error
  // formatting function return type `std::result::Result<(), std::fmt::Error>`:
  // Failed to format type for the definition of `std::fmt::Error`: Zero-sized
  // types (ZSTs) are not supported (b/258259459)
};

template <>
struct ::rs::impl<::enums::qr_error::StructuredQrError, ::rs::core::cmp::Eq> {
  static constexpr bool kIsImplemented = true;
};

template <>
struct ::rs::impl<::enums::qr_error::StructuredQrError,
                  ::rs::core::fmt::Debug> {
  static constexpr bool kIsImplemented = true;

  // Error generating bindings for associated function
  // `<enums_golden::qr_error::StructuredQrError as std::fmt::Debug>::fmt`:
  // Error formatting function return type `std::result::Result<(),
  // std::fmt::Error>`: Failed to format type for the definition of
  // `std::fmt::Error`: Zero-sized types (ZSTs) are not supported (b/258259459)
};

namespace enums::param_name_collisions {

static_assert(
    sizeof(::enums::param_name_collisions::FooBar) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::param_name_collisions::FooBar) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::enums::param_name_collisions::FooBar>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::param_name_collisions::FooBar>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::param_name_collisions::FooBar>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::enums::param_name_collisions::FooBar>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::enums::param_name_collisions::FooBar>);
inline ::enums::param_name_collisions::FooBar::FooBar(
    ::crubit::UnsafeRelocateTag, FooBar&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aparam_uname_ucollisions_x0000003a_x0000003aFooBar_uenums_ugolden_x0000003a_x0000003aparam_uname_ucollisions_x0000003a_x0000003aFooBar(
    ::enums::param_name_collisions::FooBar const&,
    ::enums::param_name_collisions::FooBar const&);
}
inline bool(FooBar::operator==)(
    ::enums::param_name_collisions::FooBar const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aparam_uname_ucollisions_x0000003a_x0000003aFooBar_uenums_ugolden_x0000003a_x0000003aparam_uname_ucollisions_x0000003a_x0000003aFooBar(
          self, other);
}

static_assert(
    sizeof(::enums::param_name_collisions::KeywordEnum) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::param_name_collisions::KeywordEnum) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::enums::param_name_collisions::KeywordEnum>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::param_name_collisions::KeywordEnum>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::param_name_collisions::KeywordEnum>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::enums::param_name_collisions::KeywordEnum>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::enums::param_name_collisions::KeywordEnum>);
inline ::enums::param_name_collisions::KeywordEnum::KeywordEnum(
    ::crubit::UnsafeRelocateTag, KeywordEnum&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aparam_uname_ucollisions_x0000003a_x0000003aKeywordEnum_uenums_ugolden_x0000003a_x0000003aparam_uname_ucollisions_x0000003a_x0000003aKeywordEnum(
    ::enums::param_name_collisions::KeywordEnum const&,
    ::enums::param_name_collisions::KeywordEnum const&);
}
inline bool(KeywordEnum::operator==)(
    ::enums::param_name_collisions::KeywordEnum const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aparam_uname_ucollisions_x0000003a_x0000003aKeywordEnum_uenums_ugolden_x0000003a_x0000003aparam_uname_ucollisions_x0000003a_x0000003aKeywordEnum(
          self, other);
}

static_assert(
    sizeof(::enums::param_name_collisions::OptionLike) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::param_name_collisions::OptionLike) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::enums::param_name_collisions::OptionLike>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::param_name_collisions::OptionLike>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::param_name_collisions::OptionLike>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::enums::param_name_collisions::OptionLike>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::enums::param_name_collisions::OptionLike>);
inline ::enums::param_name_collisions::OptionLike::OptionLike(
    ::crubit::UnsafeRelocateTag, OptionLike&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_matches_uvariant(
    ::enums::param_name_collisions::OptionLike const&,
    ::enums::param_name_collisions::OptionLike*);
}
inline bool(OptionLike::matches_variant)(
    ::enums::param_name_collisions::OptionLike some) const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_matches_uvariant(self, &some);
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aparam_uname_ucollisions_x0000003a_x0000003aOptionLike_uenums_ugolden_x0000003a_x0000003aparam_uname_ucollisions_x0000003a_x0000003aOptionLike(
    ::enums::param_name_collisions::OptionLike const&,
    ::enums::param_name_collisions::OptionLike const&);
}
inline bool(OptionLike::operator==)(
    ::enums::param_name_collisions::OptionLike const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aparam_uname_ucollisions_x0000003a_x0000003aOptionLike_uenums_ugolden_x0000003a_x0000003aparam_uname_ucollisions_x0000003a_x0000003aOptionLike(
          self, other);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_check_uboth(
    ::enums::param_name_collisions::OptionLike*,
    ::enums::param_name_collisions::OptionLike*);
}
inline bool check_both(::enums::param_name_collisions::OptionLike some,
                       ::enums::param_name_collisions::OptionLike none) {
  return __crubit_internal::__crubit_thunk_check_uboth(&some, &none);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_ubar(
    ::enums::param_name_collisions::FooBar const&);
}
inline bool is_bar(::enums::param_name_collisions::FooBar const& Bar) {
  return __crubit_internal::__crubit_thunk_is_ubar(Bar);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_ufoo(::enums::param_name_collisions::FooBar*);
}
inline bool is_foo(::enums::param_name_collisions::FooBar Foo) {
  return __crubit_internal::__crubit_thunk_is_ufoo(&Foo);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_umatch(
    ::enums::param_name_collisions::KeywordEnum*);
}
inline bool is_match(::enums::param_name_collisions::KeywordEnum match) {
  return __crubit_internal::__crubit_thunk_is_umatch(&match);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_unone(
    ::enums::param_name_collisions::OptionLike const&);
}
inline bool is_none(::enums::param_name_collisions::OptionLike const& none) {
  return __crubit_internal::__crubit_thunk_is_unone(none);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_usome(
    ::enums::param_name_collisions::OptionLike*);
}
inline bool is_some(::enums::param_name_collisions::OptionLike some) {
  return __crubit_internal::__crubit_thunk_is_usome(&some);
}

}  // namespace enums::param_name_collisions

namespace enums::qr_error {

static_assert(
    sizeof(::enums::qr_error::QrError) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::qr_error::QrError) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::enums::qr_error::QrError>);
static_assert(
    ::std::is_trivially_move_constructible_v<::enums::qr_error::QrError>);
static_assert(
    ::std::is_trivially_move_assignable_v<::enums::qr_error::QrError>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::enums::qr_error::QrError>);
static_assert(
    ::std::is_trivially_copy_assignable_v<::enums::qr_error::QrError>);
inline ::enums::qr_error::QrError::QrError(::crubit::UnsafeRelocateTag,
                                           QrError&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_udata_utoo_ulong(
    ::enums::qr_error::QrError const&);
}
inline bool(QrError::is_data_too_long)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_udata_utoo_ulong(self);
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aQrError_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aQrError(
    ::enums::qr_error::QrError const&, ::enums::qr_error::QrError const&);
}
inline bool(QrError::operator==)(
    ::enums::qr_error::QrError const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aQrError_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aQrError(
          self, other);
}
inline void ::enums::qr_error::QrError::__crubit_field_offset_assertions() {
  {
    using __crubit_assert_type = ::enums::qr_error::QrError::Structured;
    static_assert(0 == offsetof(__crubit_assert_type, __field0));
  }
}
static_assert(
    sizeof(::enums::qr_error::StructuredQrError) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::qr_error::StructuredQrError) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::enums::qr_error::StructuredQrError>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::qr_error::StructuredQrError>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::qr_error::StructuredQrError>);
static_assert(::std::is_trivially_copy_constructible_v<
              ::enums::qr_error::StructuredQrError>);
static_assert(::std::is_trivially_copy_assignable_v<
              ::enums::qr_error::StructuredQrError>);
inline ::enums::qr_error::StructuredQrError::StructuredQrError(
    ::crubit::UnsafeRelocateTag, StructuredQrError&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool
__crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aStructuredQrError_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aStructuredQrError(
    ::enums::qr_error::StructuredQrError const&,
    ::enums::qr_error::StructuredQrError const&);
}
inline bool(StructuredQrError::operator==)(
    ::enums::qr_error::StructuredQrError const& other) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_PartialEq_ueq_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aStructuredQrError_uenums_ugolden_x0000003a_x0000003aqr_uerror_x0000003a_x0000003aStructuredQrError(
          self, other);
}
inline void ::enums::qr_error::StructuredQrError::
    __crubit_field_offset_assertions() {
  {
    using __crubit_assert_type =
        ::enums::qr_error::StructuredQrError::TotalMismatch;
    static_assert(8 == offsetof(__crubit_assert_type, __field0));
  }
  {
    using __crubit_assert_type =
        ::enums::qr_error::StructuredQrError::SeqGreaterThanTotal;
    static_assert(1 == offsetof(__crubit_assert_type, __field0));
  }
  {
    using __crubit_assert_type =
        ::enums::qr_error::StructuredQrError::SeqGreaterThanTotal;
    static_assert(2 == offsetof(__crubit_assert_type, __field1));
  }
  {
    using __crubit_assert_type =
        ::enums::qr_error::StructuredQrError::LengthMismatch;
    static_assert(8 == offsetof(__crubit_assert_type, __field0));
  }
  {
    using __crubit_assert_type =
        ::enums::qr_error::StructuredQrError::LengthMismatch;
    static_assert(16 == offsetof(__crubit_assert_type, __field1));
  }
  {
    using __crubit_assert_type =
        ::enums::qr_error::StructuredQrError::UnsupportedVersion;
    static_assert(2 == offsetof(__crubit_assert_type, __field0));
  }
  {
    using __crubit_assert_type =
        ::enums::qr_error::StructuredQrError::SplitMax16;
    static_assert(8 == offsetof(__crubit_assert_type, __field0));
  }
}
}  // namespace enums::qr_error

namespace enums::repr_128 {

static_assert(
    sizeof(::enums::repr_128::ReprI128) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_128::ReprI128) == 16,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr ::enums::repr_128::ReprI128(
    ::enums::repr_128::ReprI128::MakeZero)() {
  return ReprI128(PrivateBytesTag{},
                  {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr ::enums::repr_128::ReprI128(
    ::enums::repr_128::ReprI128::MakeMinI128)() {
  return ReprI128(PrivateBytesTag{},
                  {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128});
}

// `static` constructor
inline constexpr ::enums::repr_128::ReprI128(
    ::enums::repr_128::ReprI128::MakeMaxI128)() {
  return ReprI128(PrivateBytesTag{}, {255, 255, 255, 255, 255, 255, 255, 255,
                                      255, 255, 255, 255, 255, 255, 255, 127});
}
static_assert(::std::is_trivially_destructible_v<::enums::repr_128::ReprI128>);
static_assert(
    ::std::is_trivially_move_constructible_v<::enums::repr_128::ReprI128>);
static_assert(
    ::std::is_trivially_move_assignable_v<::enums::repr_128::ReprI128>);
inline ::enums::repr_128::ReprI128::ReprI128(::crubit::UnsafeRelocateTag,
                                             ReprI128&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_umin_ui128(
    ::enums::repr_128::ReprI128 const&);
}
inline bool(ReprI128::is_min_i128)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_umin_ui128(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_umax_ui128(
    ::enums::repr_128::ReprI128 const&);
}
inline bool(ReprI128::is_max_i128)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_umax_ui128(self);
}
inline void ::enums::repr_128::ReprI128::__crubit_field_offset_assertions() {
  {
    using __crubit_assert_type = ::enums::repr_128::ReprI128;
    static_assert(0 == offsetof(__crubit_assert_type, __opaque_blob_of_bytes));
  }
}
static_assert(
    sizeof(::enums::repr_128::ReprU128) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_128::ReprU128) == 16,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr ::enums::repr_128::ReprU128(
    ::enums::repr_128::ReprU128::MakeZero)() {
  return ReprU128(PrivateBytesTag{},
                  {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0});
}

// `static` constructor
inline constexpr ::enums::repr_128::ReprU128(
    ::enums::repr_128::ReprU128::MakeMaxU128)() {
  return ReprU128(PrivateBytesTag{}, {255, 255, 255, 255, 255, 255, 255, 255,
                                      255, 255, 255, 255, 255, 255, 255, 255});
}
static_assert(::std::is_trivially_destructible_v<::enums::repr_128::ReprU128>);
static_assert(
    ::std::is_trivially_move_constructible_v<::enums::repr_128::ReprU128>);
static_assert(
    ::std::is_trivially_move_assignable_v<::enums::repr_128::ReprU128>);
inline ::enums::repr_128::ReprU128::ReprU128(::crubit::UnsafeRelocateTag,
                                             ReprU128&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_umax_uu128(
    ::enums::repr_128::ReprU128 const&);
}
inline bool(ReprU128::is_max_u128)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_umax_uu128(self);
}
inline void ::enums::repr_128::ReprU128::__crubit_field_offset_assertions() {
  {
    using __crubit_assert_type = ::enums::repr_128::ReprU128;
    static_assert(0 == offsetof(__crubit_assert_type, __opaque_blob_of_bytes));
  }
}
}  // namespace enums::repr_128

namespace enums::repr_c {

static_assert(
    sizeof(::enums::repr_c::MyEnum) == 40,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_c::MyEnum) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uenums_ugolden_x0000003a_x0000003arepr_uc_x0000003a_x0000003aMyEnum(
    ::enums::repr_c::MyEnum* __ret_ptr);
}
inline ::enums::repr_c::MyEnum::MyEnum() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uenums_ugolden_x0000003a_x0000003arepr_uc_x0000003a_x0000003aMyEnum(
          this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_E(::rs::alloc::string::String*, ::std::int32_t,
                                 ::enums::repr_c::MyEnum* __ret_ptr);
}
inline ::enums::repr_c::MyEnum(MyEnum::MakeE)(
    ::rs::alloc::string::String __param_0, ::std::int32_t __param_1) {
  crubit::Slot __param_0_slot((::std::move(__param_0)));
  crubit::Slot<::enums::repr_c::MyEnum> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_E(__param_0_slot.Get(), __param_1,
                                      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_A(::std::int32_t, ::std::int64_t,
                                 ::enums::repr_c::MyEnum* __ret_ptr);
}
inline ::enums::repr_c::MyEnum(MyEnum::MakeA)(::std::int32_t __param_0,
                                              ::std::int64_t __param_1) {
  crubit::Slot<::enums::repr_c::MyEnum> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_A(__param_0, __param_1,
                                      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

// `static` constructor
inline ::enums::repr_c::MyEnum(::enums::repr_c::MyEnum::MakeF)() {
  return MyEnum(PrivateTagCtorTag{}, Tag{INT64_C(2)});
}

// `static` constructor
inline ::enums::repr_c::MyEnum(::enums::repr_c::MyEnum::MakeG)() {
  return MyEnum(PrivateTagCtorTag{}, Tag{INT64_C(4)});
}

// `static` constructor
inline ::enums::repr_c::MyEnum(::enums::repr_c::MyEnum::MakeD)() {
  return MyEnum(PrivateTagCtorTag{}, Tag{INT64_C(10002)});
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_uenums_ugolden_x0000003a_x0000003arepr_uc_x0000003a_x0000003aMyEnum(
    ::enums::repr_c::MyEnum&);
}
inline ::enums::repr_c::MyEnum::~MyEnum() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uenums_ugolden_x0000003a_x0000003arepr_uc_x0000003a_x0000003aMyEnum(
          *this);
}
inline ::enums::repr_c::MyEnum::MyEnum(MyEnum&& other) noexcept : MyEnum() {
  *this = ::std::move(other);
}
inline ::enums::repr_c::MyEnum& ::enums::repr_c::MyEnum::operator=(
    MyEnum&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::enums::repr_c::MyEnum::MyEnum(::crubit::UnsafeRelocateTag,
                                       MyEnum&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::enums::repr_c::MyEnum::__crubit_field_offset_assertions() {
  {
    using __crubit_assert_type = ::enums::repr_c::MyEnum;
    static_assert(8 == offsetof(__crubit_assert_type, E));
  }
  {
    using __crubit_assert_type = ::enums::repr_c::MyEnum;
    static_assert(8 == offsetof(__crubit_assert_type, A));
  }
  {
    using __crubit_assert_type = ::enums::repr_c::MyEnum;
    static_assert(8 == offsetof(__crubit_assert_type, B));
  }
  {
    using __crubit_assert_type = ::enums::repr_c::MyEnum;
    static_assert(8 == offsetof(__crubit_assert_type, C));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c::MyEnum::__crubit_E_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c::MyEnum::__crubit_E_struct;
    static_assert(24 == offsetof(__crubit_assert_variant_type, __field1));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c::MyEnum::__crubit_A_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c::MyEnum::__crubit_A_struct;
    static_assert(8 == offsetof(__crubit_assert_variant_type, __field1));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c::MyEnum::__crubit_B_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, h));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c::MyEnum::__crubit_B_struct;
    static_assert(1 == offsetof(__crubit_assert_variant_type, i));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c::MyEnum::__crubit_C_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, a));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c::MyEnum::__crubit_C_struct;
    static_assert(4 == offsetof(__crubit_assert_variant_type, b));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c::MyEnum::__crubit_C_struct;
    static_assert(8 == offsetof(__crubit_assert_variant_type, c));
  }
}
static_assert(
    sizeof(::enums::repr_c::ReprCWithExtremeDiscriminants) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_c::ReprCWithExtremeDiscriminants) == 4,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr ::enums::repr_c::ReprCWithExtremeDiscriminants(
    ::enums::repr_c::ReprCWithExtremeDiscriminants::MakeMinusOne)() {
  return ReprCWithExtremeDiscriminants(PrivateTagCtorTag{}, Tag{INT64_C(-1)});
}

// `static` constructor
inline constexpr ::enums::repr_c::ReprCWithExtremeDiscriminants(
    ::enums::repr_c::ReprCWithExtremeDiscriminants::MakeMinusTwo)() {
  return ReprCWithExtremeDiscriminants(PrivateTagCtorTag{}, Tag{INT64_C(-2)});
}

// `static` constructor
inline constexpr ::enums::repr_c::ReprCWithExtremeDiscriminants(
    ::enums::repr_c::ReprCWithExtremeDiscriminants::MakeMinI32)() {
  return ReprCWithExtremeDiscriminants(PrivateTagCtorTag{},
                                       Tag{INT64_C(-2147483648)});
}

// `static` constructor
inline constexpr ::enums::repr_c::ReprCWithExtremeDiscriminants(
    ::enums::repr_c::ReprCWithExtremeDiscriminants::MakeMaxI32)() {
  return ReprCWithExtremeDiscriminants(PrivateTagCtorTag{},
                                       Tag{INT64_C(2147483647)});
}
static_assert(::std::is_trivially_destructible_v<
              ::enums::repr_c::ReprCWithExtremeDiscriminants>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_c::ReprCWithExtremeDiscriminants>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_c::ReprCWithExtremeDiscriminants>);
inline ::enums::repr_c::ReprCWithExtremeDiscriminants::
    ReprCWithExtremeDiscriminants(::crubit::UnsafeRelocateTag,
                                  ReprCWithExtremeDiscriminants&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uminus_uone(
    ::enums::repr_c::ReprCWithExtremeDiscriminants const&);
}
inline bool(ReprCWithExtremeDiscriminants::is_minus_one)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uminus_uone(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uminus_utwo(
    ::enums::repr_c::ReprCWithExtremeDiscriminants const&);
}
inline bool(ReprCWithExtremeDiscriminants::is_minus_two)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uminus_utwo(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_umin_ui32(
    ::enums::repr_c::ReprCWithExtremeDiscriminants const&);
}
inline bool(ReprCWithExtremeDiscriminants::is_min_i32)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_umin_ui32(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_umax_ui32(
    ::enums::repr_c::ReprCWithExtremeDiscriminants const&);
}
inline bool(ReprCWithExtremeDiscriminants::is_max_i32)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_umax_ui32(self);
}
inline void ::enums::repr_c::ReprCWithExtremeDiscriminants::
    __crubit_field_offset_assertions() {}
static_assert(
    sizeof(::enums::repr_c::ReprCWithSingleNoPayloadVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_c::ReprCWithSingleNoPayloadVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");

// `static` constructor
inline constexpr ::enums::repr_c::ReprCWithSingleNoPayloadVariant(
    ::enums::repr_c::ReprCWithSingleNoPayloadVariant::MakeSingleVariant)() {
  return ReprCWithSingleNoPayloadVariant(PrivateTagCtorTag{}, Tag{INT64_C(0)});
}
static_assert(::std::is_trivially_destructible_v<
              ::enums::repr_c::ReprCWithSingleNoPayloadVariant>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_c::ReprCWithSingleNoPayloadVariant>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_c::ReprCWithSingleNoPayloadVariant>);
inline ::enums::repr_c::ReprCWithSingleNoPayloadVariant::
    ReprCWithSingleNoPayloadVariant(::crubit::UnsafeRelocateTag,
                                    ReprCWithSingleNoPayloadVariant&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_usingle_uvariant(
    ::enums::repr_c::ReprCWithSingleNoPayloadVariant const&);
}
inline bool(ReprCWithSingleNoPayloadVariant::is_single_variant)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_usingle_uvariant(self);
}
inline void ::enums::repr_c::ReprCWithSingleNoPayloadVariant::
    __crubit_field_offset_assertions() {}
}  // namespace enums::repr_c

namespace enums::repr_c_clone_active_variant {

static_assert(
    sizeof(::enums::repr_c_clone_active_variant::CloneActiveVariant) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_c_clone_active_variant::CloneActiveVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uenums_ugolden_x0000003a_x0000003arepr_uc_uclone_uactive_uvariant_x0000003a_x0000003aCloneActiveVariant(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* __ret_ptr);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant::
    CloneActiveVariant() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uenums_ugolden_x0000003a_x0000003arepr_uc_uclone_uactive_uvariant_x0000003a_x0000003aCloneActiveVariant(
          this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_A(
    ::std::int32_t,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* __ret_ptr);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant(
    CloneActiveVariant::MakeA)(::std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_clone_active_variant::CloneActiveVariant>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_A(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_B(
    ::std::int32_t,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* __ret_ptr);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant(
    CloneActiveVariant::MakeB)(::std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_clone_active_variant::CloneActiveVariant>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_B(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_C(
    ::std::int32_t,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* __ret_ptr);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant(
    CloneActiveVariant::MakeC)(::std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_clone_active_variant::CloneActiveVariant>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_C(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(::std::is_trivially_destructible_v<
              ::enums::repr_c_clone_active_variant::CloneActiveVariant>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_c_clone_active_variant::CloneActiveVariant>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_c_clone_active_variant::CloneActiveVariant>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_uenums_ugolden_x0000003a_x0000003arepr_uc_uclone_uactive_uvariant_x0000003a_x0000003aCloneActiveVariant(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_uenums_ugolden_x0000003a_x0000003arepr_uc_uclone_uactive_uvariant_x0000003a_x0000003aCloneActiveVariant(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant&,
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant::
    CloneActiveVariant(const CloneActiveVariant& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_uenums_ugolden_x0000003a_x0000003arepr_uc_uclone_uactive_uvariant_x0000003a_x0000003aCloneActiveVariant(
          other, this);
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant& ::enums::
    repr_c_clone_active_variant::CloneActiveVariant::operator=(
        const CloneActiveVariant& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_uenums_ugolden_x0000003a_x0000003arepr_uc_uclone_uactive_uvariant_x0000003a_x0000003aCloneActiveVariant(
            *this, other);
  }
  return *this;
}
inline ::enums::repr_c_clone_active_variant::CloneActiveVariant::
    CloneActiveVariant(::crubit::UnsafeRelocateTag,
                       CloneActiveVariant&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::enums::repr_c_clone_active_variant::CloneActiveVariant::
    __crubit_field_offset_assertions() {
  {
    using __crubit_assert_type =
        ::enums::repr_c_clone_active_variant::CloneActiveVariant;
    static_assert(4 == offsetof(__crubit_assert_type, A));
  }
  {
    using __crubit_assert_type =
        ::enums::repr_c_clone_active_variant::CloneActiveVariant;
    static_assert(4 == offsetof(__crubit_assert_type, B));
  }
  {
    using __crubit_assert_type =
        ::enums::repr_c_clone_active_variant::CloneActiveVariant;
    static_assert(4 == offsetof(__crubit_assert_type, C));
  }
  {
    using __crubit_assert_variant_type = ::enums::repr_c_clone_active_variant::
        CloneActiveVariant::__crubit_A_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
  {
    using __crubit_assert_variant_type = ::enums::repr_c_clone_active_variant::
        CloneActiveVariant::__crubit_B_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
  {
    using __crubit_assert_variant_type = ::enums::repr_c_clone_active_variant::
        CloneActiveVariant::__crubit_C_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
}
namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_ua(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&);
}
inline bool is_a(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const& e) {
  return __crubit_internal::__crubit_thunk_is_ua(e);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_ub(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&);
}
inline bool is_b(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const& e) {
  return __crubit_internal::__crubit_thunk_is_ub(e);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uc(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const&);
}
inline bool is_c(
    ::enums::repr_c_clone_active_variant::CloneActiveVariant const& e) {
  return __crubit_internal::__crubit_thunk_is_uc(e);
}

}  // namespace enums::repr_c_clone_active_variant

namespace enums::repr_c_clone_counter {

static_assert(
    sizeof(::enums::repr_c_clone_counter::CloneCount) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_c_clone_counter::CloneCount) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uenums_ugolden_x0000003a_x0000003arepr_uc_uclone_ucounter_x0000003a_x0000003aCloneCount(
    ::enums::repr_c_clone_counter::CloneCount* __ret_ptr);
}
inline ::enums::repr_c_clone_counter::CloneCount::CloneCount() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uenums_ugolden_x0000003a_x0000003arepr_uc_uclone_ucounter_x0000003a_x0000003aCloneCount(
          this);
}
static_assert(::std::is_trivially_destructible_v<
              ::enums::repr_c_clone_counter::CloneCount>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_c_clone_counter::CloneCount>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_c_clone_counter::CloneCount>);
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_uenums_ugolden_x0000003a_x0000003arepr_uc_uclone_ucounter_x0000003a_x0000003aCloneCount(
    ::enums::repr_c_clone_counter::CloneCount const&,
    ::enums::repr_c_clone_counter::CloneCount* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_uenums_ugolden_x0000003a_x0000003arepr_uc_uclone_ucounter_x0000003a_x0000003aCloneCount(
    ::enums::repr_c_clone_counter::CloneCount&,
    ::enums::repr_c_clone_counter::CloneCount const&);
}
inline ::enums::repr_c_clone_counter::CloneCount::CloneCount(
    const CloneCount& other) noexcept {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_uenums_ugolden_x0000003a_x0000003arepr_uc_uclone_ucounter_x0000003a_x0000003aCloneCount(
          other, this);
}
inline ::enums::repr_c_clone_counter::CloneCount& ::enums::
    repr_c_clone_counter::CloneCount::operator=(
        const CloneCount& other) noexcept {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_uenums_ugolden_x0000003a_x0000003arepr_uc_uclone_ucounter_x0000003a_x0000003aCloneCount(
            *this, other);
  }
  return *this;
}
inline ::enums::repr_c_clone_counter::CloneCount::CloneCount(
    ::crubit::UnsafeRelocateTag, CloneCount&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::enums::repr_c_clone_counter::CloneCount::
    __crubit_field_offset_assertions() {
  {
    using __crubit_assert_type = ::enums::repr_c_clone_counter::CloneCount;
    static_assert(8 == offsetof(__crubit_assert_type, A));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c_clone_counter::CloneCount::__crubit_A_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, p));
  }
}
}  // namespace enums::repr_c_clone_counter

namespace enums::repr_c_drop {

static_assert(
    sizeof(::enums::repr_c_drop::DropMe) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_c_drop::DropMe) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uenums_ugolden_x0000003a_x0000003arepr_uc_udrop_x0000003a_x0000003aDropMe(
    ::enums::repr_c_drop::DropMe* __ret_ptr);
}
inline ::enums::repr_c_drop::DropMe::DropMe() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uenums_ugolden_x0000003a_x0000003arepr_uc_udrop_x0000003a_x0000003aDropMe(
          this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_A(::std::int32_t,
                                 ::enums::repr_c_drop::DropMe* __ret_ptr);
}
inline ::enums::repr_c_drop::DropMe(DropMe::MakeA)(::std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_drop::DropMe> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_A(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_B(::std::int64_t,
                                 ::enums::repr_c_drop::DropMe* __ret_ptr);
}
inline ::enums::repr_c_drop::DropMe(DropMe::MakeB)(::std::int64_t __param_0) {
  crubit::Slot<::enums::repr_c_drop::DropMe> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_B(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

// `static` constructor
inline ::enums::repr_c_drop::DropMe(::enums::repr_c_drop::DropMe::MakeQ)() {
  return DropMe(PrivateTagCtorTag{}, Tag{INT64_C(2)});
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_uenums_ugolden_x0000003a_x0000003arepr_uc_udrop_x0000003a_x0000003aDropMe(
    ::enums::repr_c_drop::DropMe&);
}
inline ::enums::repr_c_drop::DropMe::~DropMe() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_uenums_ugolden_x0000003a_x0000003arepr_uc_udrop_x0000003a_x0000003aDropMe(
          *this);
}
inline ::enums::repr_c_drop::DropMe::DropMe(DropMe&& other) noexcept
    : DropMe() {
  *this = ::std::move(other);
}
inline ::enums::repr_c_drop::DropMe& ::enums::repr_c_drop::DropMe::operator=(
    DropMe&& other) noexcept {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::enums::repr_c_drop::DropMe::DropMe(::crubit::UnsafeRelocateTag,
                                            DropMe&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::enums::repr_c_drop::DropMe::__crubit_field_offset_assertions() {
  {
    using __crubit_assert_type = ::enums::repr_c_drop::DropMe;
    static_assert(8 == offsetof(__crubit_assert_type, A));
  }
  {
    using __crubit_assert_type = ::enums::repr_c_drop::DropMe;
    static_assert(8 == offsetof(__crubit_assert_type, B));
  }
  {
    using __crubit_assert_type = ::enums::repr_c_drop::DropMe;
    static_assert(8 == offsetof(__crubit_assert_type, C));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c_drop::DropMe::__crubit_A_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c_drop::DropMe::__crubit_B_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
  {
    using __crubit_assert_variant_type =
        ::enums::repr_c_drop::DropMe::__crubit_C_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, p));
  }
}
}  // namespace enums::repr_c_drop

namespace enums::repr_c_variant_alignment {

static_assert(
    sizeof(::enums::repr_c_variant_alignment::OverAlignedNarrowPayload) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_c_variant_alignment::OverAlignedNarrowPayload) == 16,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_A(
    ::std::int32_t,
    ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload* __ret_ptr);
}
inline ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload(
    OverAlignedNarrowPayload::MakeA)(::std::int32_t __param_0) {
  crubit::Slot<::enums::repr_c_variant_alignment::OverAlignedNarrowPayload>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_A(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_B(
    ::std::uint8_t,
    ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload* __ret_ptr);
}
inline ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload(
    OverAlignedNarrowPayload::MakeB)(::std::uint8_t __param_0) {
  crubit::Slot<::enums::repr_c_variant_alignment::OverAlignedNarrowPayload>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_B(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

// `static` constructor
inline constexpr ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload(
    ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload::MakeC)() {
  return OverAlignedNarrowPayload(PrivateTagCtorTag{}, Tag{INT64_C(2)});
}
static_assert(::std::is_trivially_destructible_v<
              ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload>);
inline ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload::
    OverAlignedNarrowPayload(::crubit::UnsafeRelocateTag,
                             OverAlignedNarrowPayload&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_value(
    ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload const&);
}
inline ::std::int32_t(OverAlignedNarrowPayload::value)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_value(self);
}
inline void ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload::
    __crubit_field_offset_assertions() {
  {
    using __crubit_assert_type =
        ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload;
    static_assert(4 == offsetof(__crubit_assert_type, A));
  }
  {
    using __crubit_assert_type =
        ::enums::repr_c_variant_alignment::OverAlignedNarrowPayload;
    static_assert(4 == offsetof(__crubit_assert_type, B));
  }
  {
    using __crubit_assert_variant_type = ::enums::repr_c_variant_alignment::
        OverAlignedNarrowPayload::__crubit_A_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
  {
    using __crubit_assert_variant_type = ::enums::repr_c_variant_alignment::
        OverAlignedNarrowPayload::__crubit_B_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
}
static_assert(
    sizeof(::enums::repr_c_variant_alignment::OverAlignedWidePayload) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_c_variant_alignment::OverAlignedWidePayload) == 16,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_A(
    ::std::uint64_t,
    ::enums::repr_c_variant_alignment::OverAlignedWidePayload* __ret_ptr);
}
inline ::enums::repr_c_variant_alignment::OverAlignedWidePayload(
    OverAlignedWidePayload::MakeA)(::std::uint64_t __param_0) {
  crubit::Slot<::enums::repr_c_variant_alignment::OverAlignedWidePayload>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_A(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_B(
    ::std::uint8_t,
    ::enums::repr_c_variant_alignment::OverAlignedWidePayload* __ret_ptr);
}
inline ::enums::repr_c_variant_alignment::OverAlignedWidePayload(
    OverAlignedWidePayload::MakeB)(::std::uint8_t __param_0) {
  crubit::Slot<::enums::repr_c_variant_alignment::OverAlignedWidePayload>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_B(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

// `static` constructor
inline constexpr ::enums::repr_c_variant_alignment::OverAlignedWidePayload(
    ::enums::repr_c_variant_alignment::OverAlignedWidePayload::MakeC)() {
  return OverAlignedWidePayload(PrivateTagCtorTag{}, Tag{INT64_C(2)});
}
static_assert(::std::is_trivially_destructible_v<
              ::enums::repr_c_variant_alignment::OverAlignedWidePayload>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_c_variant_alignment::OverAlignedWidePayload>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_c_variant_alignment::OverAlignedWidePayload>);
inline ::enums::repr_c_variant_alignment::OverAlignedWidePayload::
    OverAlignedWidePayload(::crubit::UnsafeRelocateTag,
                           OverAlignedWidePayload&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" ::std::uint64_t __crubit_thunk_value(
    ::enums::repr_c_variant_alignment::OverAlignedWidePayload const&);
}
inline ::std::uint64_t(OverAlignedWidePayload::value)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_value(self);
}
inline void ::enums::repr_c_variant_alignment::OverAlignedWidePayload::
    __crubit_field_offset_assertions() {
  {
    using __crubit_assert_type =
        ::enums::repr_c_variant_alignment::OverAlignedWidePayload;
    static_assert(8 == offsetof(__crubit_assert_type, A));
  }
  {
    using __crubit_assert_type =
        ::enums::repr_c_variant_alignment::OverAlignedWidePayload;
    static_assert(8 == offsetof(__crubit_assert_type, B));
  }
  {
    using __crubit_assert_variant_type = ::enums::repr_c_variant_alignment::
        OverAlignedWidePayload::__crubit_A_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
  {
    using __crubit_assert_variant_type = ::enums::repr_c_variant_alignment::
        OverAlignedWidePayload::__crubit_B_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
}
static_assert(
    sizeof(::enums::repr_c_variant_alignment::UnbindableFieldPayload) == 16,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_c_variant_alignment::UnbindableFieldPayload) == 8,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_B(
    ::std::uint8_t,
    ::enums::repr_c_variant_alignment::UnbindableFieldPayload* __ret_ptr);
}
inline ::enums::repr_c_variant_alignment::UnbindableFieldPayload(
    UnbindableFieldPayload::MakeB)(::std::uint8_t __param_0) {
  crubit::Slot<::enums::repr_c_variant_alignment::UnbindableFieldPayload>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_B(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(::std::is_trivially_destructible_v<
              ::enums::repr_c_variant_alignment::UnbindableFieldPayload>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_c_variant_alignment::UnbindableFieldPayload>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_c_variant_alignment::UnbindableFieldPayload>);
inline ::enums::repr_c_variant_alignment::UnbindableFieldPayload::
    UnbindableFieldPayload(::crubit::UnsafeRelocateTag,
                           UnbindableFieldPayload&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_make_ua(
    ::std::uint64_t,
    ::enums::repr_c_variant_alignment::UnbindableFieldPayload* __ret_ptr);
}
inline ::enums::repr_c_variant_alignment::UnbindableFieldPayload(
    UnbindableFieldPayload::make_a)(::std::uint64_t value) {
  crubit::Slot<::enums::repr_c_variant_alignment::UnbindableFieldPayload>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_make_ua(value, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::uint64_t __crubit_thunk_value(
    ::enums::repr_c_variant_alignment::UnbindableFieldPayload const&);
}
inline ::std::uint64_t(UnbindableFieldPayload::value)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_value(self);
}
inline void ::enums::repr_c_variant_alignment::UnbindableFieldPayload::
    __crubit_field_offset_assertions() {
  {
    using __crubit_assert_type =
        ::enums::repr_c_variant_alignment::UnbindableFieldPayload;
    static_assert(8 == offsetof(__crubit_assert_type, A));
  }
  {
    using __crubit_assert_type =
        ::enums::repr_c_variant_alignment::UnbindableFieldPayload;
    static_assert(8 == offsetof(__crubit_assert_type, B));
  }
  {
    using __crubit_assert_variant_type = ::enums::repr_c_variant_alignment::
        UnbindableFieldPayload::__crubit_B_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
}
static_assert(
    sizeof(::enums::repr_c_variant_alignment::WideTagNarrowAlignment) == 24,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_c_variant_alignment::WideTagNarrowAlignment) == 8,
    "Verify that ADT layout didn't change since this header got generated");

namespace __crubit_internal {
extern "C" void __crubit_thunk_A(
    ::std::int32_t, ::std::int64_t,
    ::enums::repr_c_variant_alignment::WideTagNarrowAlignment* __ret_ptr);
}
inline ::enums::repr_c_variant_alignment::WideTagNarrowAlignment(
    WideTagNarrowAlignment::MakeA)(::std::int32_t __param_0,
                                   ::std::int64_t __param_1) {
  crubit::Slot<::enums::repr_c_variant_alignment::WideTagNarrowAlignment>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_A(__param_0, __param_1,
                                      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_B(
    ::std::uint8_t,
    ::enums::repr_c_variant_alignment::WideTagNarrowAlignment* __ret_ptr);
}
inline ::enums::repr_c_variant_alignment::WideTagNarrowAlignment(
    WideTagNarrowAlignment::MakeB)(::std::uint8_t __param_0) {
  crubit::Slot<::enums::repr_c_variant_alignment::WideTagNarrowAlignment>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_B(__param_0, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
static_assert(::std::is_trivially_destructible_v<
              ::enums::repr_c_variant_alignment::WideTagNarrowAlignment>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_c_variant_alignment::WideTagNarrowAlignment>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_c_variant_alignment::WideTagNarrowAlignment>);
inline ::enums::repr_c_variant_alignment::WideTagNarrowAlignment::
    WideTagNarrowAlignment(::crubit::UnsafeRelocateTag,
                           WideTagNarrowAlignment&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" ::std::int64_t __crubit_thunk_sum(
    ::enums::repr_c_variant_alignment::WideTagNarrowAlignment const&);
}
inline ::std::int64_t(WideTagNarrowAlignment::sum)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_sum(self);
}
inline void ::enums::repr_c_variant_alignment::WideTagNarrowAlignment::
    __crubit_field_offset_assertions() {
  {
    using __crubit_assert_type =
        ::enums::repr_c_variant_alignment::WideTagNarrowAlignment;
    static_assert(8 == offsetof(__crubit_assert_type, A));
  }
  {
    using __crubit_assert_type =
        ::enums::repr_c_variant_alignment::WideTagNarrowAlignment;
    static_assert(8 == offsetof(__crubit_assert_type, B));
  }
  {
    using __crubit_assert_variant_type = ::enums::repr_c_variant_alignment::
        WideTagNarrowAlignment::__crubit_A_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
  {
    using __crubit_assert_variant_type = ::enums::repr_c_variant_alignment::
        WideTagNarrowAlignment::__crubit_A_struct;
    static_assert(8 == offsetof(__crubit_assert_variant_type, __field1));
  }
  {
    using __crubit_assert_variant_type = ::enums::repr_c_variant_alignment::
        WideTagNarrowAlignment::__crubit_B_struct;
    static_assert(0 == offsetof(__crubit_assert_variant_type, __field0));
  }
}
}  // namespace enums::repr_c_variant_alignment

namespace enums::repr_int {

static_assert(
    sizeof(::enums::repr_int::IntReprEnumWithNoPayload) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_int::IntReprEnumWithNoPayload) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::enums::repr_int::IntReprEnumWithNoPayload>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_int::IntReprEnumWithNoPayload>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_int::IntReprEnumWithNoPayload>);
inline ::enums::repr_int::IntReprEnumWithNoPayload::IntReprEnumWithNoPayload(
    ::crubit::UnsafeRelocateTag, IntReprEnumWithNoPayload&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uno_upayload1(
    ::enums::repr_int::IntReprEnumWithNoPayload const&);
}
inline bool(IntReprEnumWithNoPayload::is_no_payload1)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uno_upayload1(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uno_upayload2(
    ::enums::repr_int::IntReprEnumWithNoPayload const&);
}
inline bool(IntReprEnumWithNoPayload::is_no_payload2)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uno_upayload2(self);
}

static_assert(
    sizeof(::enums::repr_int::IntReprWithSingleNoPayloadVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_int::IntReprWithSingleNoPayloadVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::enums::repr_int::IntReprWithSingleNoPayloadVariant>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_int::IntReprWithSingleNoPayloadVariant>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_int::IntReprWithSingleNoPayloadVariant>);
inline ::enums::repr_int::IntReprWithSingleNoPayloadVariant::
    IntReprWithSingleNoPayloadVariant(
        ::crubit::UnsafeRelocateTag,
        IntReprWithSingleNoPayloadVariant&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_usingle_uvariant(
    ::enums::repr_int::IntReprWithSingleNoPayloadVariant const&);
}
inline bool(IntReprWithSingleNoPayloadVariant::is_single_variant)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_usingle_uvariant(self);
}

static_assert(
    sizeof(::enums::repr_int::NegReprIntEnum) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_int::NegReprIntEnum) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::enums::repr_int::NegReprIntEnum>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_int::NegReprIntEnum>);
static_assert(
    ::std::is_trivially_move_assignable_v<::enums::repr_int::NegReprIntEnum>);
inline ::enums::repr_int::NegReprIntEnum::NegReprIntEnum(
    ::crubit::UnsafeRelocateTag, NegReprIntEnum&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uminus_uone(
    ::enums::repr_int::NegReprIntEnum const&);
}
inline bool(NegReprIntEnum::is_minus_one)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uminus_uone(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_uminus_utwo(
    ::enums::repr_int::NegReprIntEnum const&);
}
inline bool(NegReprIntEnum::is_minus_two)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_uminus_utwo(self);
}

}  // namespace enums::repr_int

namespace enums::repr_rust {

static_assert(
    sizeof(::enums::repr_rust::RustReprEnum) == 12,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_rust::RustReprEnum) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<::enums::repr_rust::RustReprEnum>);
static_assert(
    ::std::is_trivially_move_constructible_v<::enums::repr_rust::RustReprEnum>);
static_assert(
    ::std::is_trivially_move_assignable_v<::enums::repr_rust::RustReprEnum>);
inline ::enums::repr_rust::RustReprEnum::RustReprEnum(
    ::crubit::UnsafeRelocateTag, RustReprEnum&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_uvariant_unumber(
    ::enums::repr_rust::RustReprEnum const&);
}
inline ::std::int32_t(RustReprEnum::get_variant_number)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uvariant_unumber(self);
}

namespace __crubit_internal {
extern "C" bool __crubit_thunk_is_utuple_upayload_uvariant(
    ::enums::repr_rust::RustReprEnum const&);
}
inline bool(RustReprEnum::is_tuple_payload_variant)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_is_utuple_upayload_uvariant(self);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_ufirst_uitem_ufrom_utuple_upayload(
    ::enums::repr_rust::RustReprEnum const&);
}
inline ::std::int32_t(RustReprEnum::get_first_item_from_tuple_payload)() const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_get_ufirst_uitem_ufrom_utuple_upayload(self);
}
inline void ::enums::repr_rust::RustReprEnum::
    __crubit_field_offset_assertions() {
  {
    using __crubit_assert_type =
        ::enums::repr_rust::RustReprEnum::TuplePayloadVariant;
    static_assert(4 == offsetof(__crubit_assert_type, __field0));
  }
  {
    using __crubit_assert_type =
        ::enums::repr_rust::RustReprEnum::TuplePayloadVariant;
    static_assert(8 == offsetof(__crubit_assert_type, __field1));
  }
  {
    using __crubit_assert_type =
        ::enums::repr_rust::RustReprEnum::StructPayloadVariant;
    static_assert(4 == offsetof(__crubit_assert_type, x));
  }
  {
    using __crubit_assert_type =
        ::enums::repr_rust::RustReprEnum::StructPayloadVariant;
    static_assert(8 == offsetof(__crubit_assert_type, y));
  }
}
static_assert(
    sizeof(
        ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods) ==
        8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(
        ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods) ==
        4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    ::std::is_trivially_destructible_v<
        ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>);
static_assert(
    ::std::is_trivially_move_constructible_v<
        ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>);
static_assert(
    ::std::is_trivially_move_assignable_v<
        ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>);
inline ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods::
    RustReprWithNamingConflictBetweenCtorsAndMethods(
        ::crubit::UnsafeRelocateTag,
        RustReprWithNamingConflictBetweenCtorsAndMethods&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_MakeNoPayloadVariant(
    ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods*
        __ret_ptr);
}
inline ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods(
    RustReprWithNamingConflictBetweenCtorsAndMethods::MakeNoPayloadVariant)() {
  crubit::Slot<
      ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_MakeNoPayloadVariant(
      __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_MakeTuplePayloadVariant(
    ::std::int32_t,
    ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods*
        __ret_ptr);
}
inline ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods(
    RustReprWithNamingConflictBetweenCtorsAndMethods::MakeTuplePayloadVariant)(
    ::std::int32_t i) {
  crubit::Slot<
      ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_MakeTuplePayloadVariant(
      i, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_MakeStructPayloadVariant(
    ::std::int32_t,
    ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods*
        __ret_ptr);
}
inline ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods(
    RustReprWithNamingConflictBetweenCtorsAndMethods::MakeStructPayloadVariant)(
    ::std::int32_t x) {
  crubit::Slot<
      ::enums::repr_rust::RustReprWithNamingConflictBetweenCtorsAndMethods>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_MakeStructPayloadVariant(
      x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_uvariant_unumber(
    ::enums::repr_rust::
        RustReprWithNamingConflictBetweenCtorsAndMethods const&);
}
inline ::std::int32_t(
    RustReprWithNamingConflictBetweenCtorsAndMethods::get_variant_number)()
    const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uvariant_unumber(self);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_get_uvalue(
    ::enums::repr_rust::
        RustReprWithNamingConflictBetweenCtorsAndMethods const&);
}
inline ::std::int32_t(
    RustReprWithNamingConflictBetweenCtorsAndMethods::get_value)() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_get_uvalue(self);
}
inline void ::enums::repr_rust::
    RustReprWithNamingConflictBetweenCtorsAndMethods::
        __crubit_field_offset_assertions() {
  {
    using __crubit_assert_type = ::enums::repr_rust::
        RustReprWithNamingConflictBetweenCtorsAndMethods::TuplePayloadVariant;
    static_assert(4 == offsetof(__crubit_assert_type, __field0));
  }
  {
    using __crubit_assert_type = ::enums::repr_rust::
        RustReprWithNamingConflictBetweenCtorsAndMethods::StructPayloadVariant;
    static_assert(4 == offsetof(__crubit_assert_type, x));
  }
}
static_assert(
    sizeof(::enums::repr_rust::RustReprWithSingleTuplePayloadVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::enums::repr_rust::RustReprWithSingleTuplePayloadVariant) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<
              ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant>);
static_assert(::std::is_trivially_move_constructible_v<
              ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant>);
static_assert(::std::is_trivially_move_assignable_v<
              ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant>);
inline ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant::
    RustReprWithSingleTuplePayloadVariant(
        ::crubit::UnsafeRelocateTag,
        RustReprWithSingleTuplePayloadVariant&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" ::std::int32_t
__crubit_thunk_get_usingle_uitem_ufrom_utuple_upayload(
    ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant const&);
}
inline ::std::int32_t(
    RustReprWithSingleTuplePayloadVariant::get_single_item_from_tuple_payload)()
    const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_get_usingle_uitem_ufrom_utuple_upayload(self);
}
inline void ::enums::repr_rust::RustReprWithSingleTuplePayloadVariant::
    __crubit_field_offset_assertions() {
  {
    using __crubit_assert_type = ::enums::repr_rust::
        RustReprWithSingleTuplePayloadVariant::SingleVariant;
    static_assert(0 == offsetof(__crubit_assert_type, __field0));
  }
}
}  // namespace enums::repr_rust

#pragma clang diagnostic pop
#endif  // CC_BINDINGS_FROM_RS_TEST_ENUMS_ENUMS_GOLDEN
