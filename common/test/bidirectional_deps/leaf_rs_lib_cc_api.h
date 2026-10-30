// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// leaf_rs_lib_golden

// clang-format off
#ifndef COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_RS_LIB_GOLDEN
#define COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_RS_LIB_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#pragma clang diagnostic ignored "-Wshadow"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"
#include "support/rs_std/enum.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>
#include <utility>

namespace leaf_rs_lib {

struct CRUBIT_INTERNAL_RUST_TYPE(":: leaf_rs_lib_golden :: LeafRsEnum") alignas(
    1) [[clang::trivial_abi]] LeafRsEnum final {
 public:
  // `leaf_rs_lib_golden::LeafRsEnum` doesn't implement the `Default` trait
  LeafRsEnum() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~LeafRsEnum() = default;
  LeafRsEnum(LeafRsEnum&&) = default;
  LeafRsEnum& operator=(LeafRsEnum&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  LeafRsEnum(const LeafRsEnum&) = default;
  LeafRsEnum& operator=(const LeafRsEnum&) = default;
  LeafRsEnum(::crubit::UnsafeRelocateTag, LeafRsEnum&& value);

 public:
  using Discriminant = ::std::uint8_t;

  struct KUnknown
      : public ::crubit::internal::Variant<KUnknown, ::leaf_rs_lib::LeafRsEnum,
                                           0> {
   public:
    constexpr explicit KUnknown(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    KUnknown(const KUnknown&) = default;
    KUnknown(KUnknown&&) = default;
    KUnknown& operator=(const KUnknown&) = default;
    KUnknown& operator=(KUnknown&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 0;
  };

  struct KValue1
      : public ::crubit::internal::Variant<KValue1, ::leaf_rs_lib::LeafRsEnum,
                                           1> {
   public:
    constexpr explicit KValue1(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    KValue1(const KValue1&) = default;
    KValue1(KValue1&&) = default;
    KValue1& operator=(const KValue1&) = default;
    KValue1& operator=(KValue1&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 1;
  };

  struct KValue2
      : public ::crubit::internal::Variant<KValue2, ::leaf_rs_lib::LeafRsEnum,
                                           2> {
   public:
    constexpr explicit KValue2(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    KValue2(const KValue2&) = default;
    KValue2(KValue2&&) = default;
    KValue2& operator=(const KValue2&) = default;
    KValue2& operator=(KValue2&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 2;
  };

  struct KValue3
      : public ::crubit::internal::Variant<KValue3, ::leaf_rs_lib::LeafRsEnum,
                                           3> {
   public:
    constexpr explicit KValue3(::crubit::internal::VariantKey) {}

   private:
    // Variants can't be copied or moved on their own, only as part of the enum.
    // See `::crubit::internal::Variant`.
    template <typename...>
    friend union ::crubit::internal::VariadicUnion;
    KValue3(const KValue3&) = default;
    KValue3(KValue3&&) = default;
    KValue3& operator=(const KValue3&) = default;
    KValue3& operator=(KValue3&&) = default;
    [[maybe_unused]] ::std::uint8_t __crubit_tag = 3;
  };

  // Constructs the enum holding the variant `__Variant`, built in place from
  // `__args`. `__Variant::Make(__args...)` is shorthand for this.
  template <typename __Variant, typename... __Args>
    requires ::crubit::internal::ConstructibleVariantOf<
        __Variant, ::leaf_rs_lib::LeafRsEnum, __Args...>
  constexpr explicit LeafRsEnum(::std::in_place_type_t<__Variant> __variant,
                                __Args&&... __args)
      : storage_(__variant, ::std::forward<__Args>(__args)...) {}

 private:
  friend struct ::crubit::internal::EnumAccess;
  ::crubit::internal::VariadicUnionStorage<
      ::crubit::internal::tag_encoding::Direct<::std::uint8_t, 0>, KUnknown,
      KValue1, KValue2, KValue3>
      storage_;
};

struct CRUBIT_INTERNAL_RUST_TYPE(":: leaf_rs_lib_golden :: LeafRsType") alignas(
    1) [[clang::trivial_abi]] LeafRsType final {
 public:
  // Default::default
  LeafRsType();

  // No custom `Drop` impl and no custom "drop glue" required
  ~LeafRsType() = default;
  LeafRsType(LeafRsType&&) = default;
  LeafRsType& operator=(LeafRsType&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  LeafRsType(const LeafRsType&) = default;
  LeafRsType& operator=(const LeafRsType&) = default;
  LeafRsType(::crubit::UnsafeRelocateTag, LeafRsType&& value);

 private:
  // Field type has been replaced with a blob of bytes: Field is private
  ::std::array<unsigned char, 1> field;

 private:
  static void __crubit_field_offset_assertions();
};

::std::uint8_t unwrap(::leaf_rs_lib::LeafRsType x);

::std::uint8_t unwrap_enum(::leaf_rs_lib::LeafRsEnum x);

::leaf_rs_lib::LeafRsType wrap(::std::uint8_t x);

::leaf_rs_lib::LeafRsEnum wrap_enum(::std::uint8_t x);

static_assert(
    sizeof(::leaf_rs_lib::LeafRsEnum) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::leaf_rs_lib::LeafRsEnum) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<::leaf_rs_lib::LeafRsEnum>);
static_assert(
    ::std::is_trivially_move_constructible_v<::leaf_rs_lib::LeafRsEnum>);
static_assert(::std::is_trivially_move_assignable_v<::leaf_rs_lib::LeafRsEnum>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::leaf_rs_lib::LeafRsEnum>);
static_assert(::std::is_trivially_copy_assignable_v<::leaf_rs_lib::LeafRsEnum>);
inline ::leaf_rs_lib::LeafRsEnum::LeafRsEnum(::crubit::UnsafeRelocateTag,
                                             LeafRsEnum&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

using LeafRsTypeAlias CRUBIT_INTERNAL_RUST_TYPE(
    ":: leaf_rs_lib_golden :: LeafRsType") = ::leaf_rs_lib::LeafRsType;
static_assert(
    sizeof(::leaf_rs_lib::LeafRsType) == 1,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(::leaf_rs_lib::LeafRsType) == 1,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_uleaf_urs_ulib_ugolden_x0000003a_x0000003aLeafRsType(
    ::leaf_rs_lib::LeafRsType* __ret_ptr);
}
inline ::leaf_rs_lib::LeafRsType::LeafRsType() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_uleaf_urs_ulib_ugolden_x0000003a_x0000003aLeafRsType(
          this);
}
static_assert(::std::is_trivially_destructible_v<::leaf_rs_lib::LeafRsType>);
static_assert(
    ::std::is_trivially_move_constructible_v<::leaf_rs_lib::LeafRsType>);
static_assert(::std::is_trivially_move_assignable_v<::leaf_rs_lib::LeafRsType>);
static_assert(
    ::std::is_trivially_copy_constructible_v<::leaf_rs_lib::LeafRsType>);
static_assert(::std::is_trivially_copy_assignable_v<::leaf_rs_lib::LeafRsType>);
inline ::leaf_rs_lib::LeafRsType::LeafRsType(::crubit::UnsafeRelocateTag,
                                             LeafRsType&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void ::leaf_rs_lib::LeafRsType::__crubit_field_offset_assertions() {
  using __crubit_assert_type = ::leaf_rs_lib::LeafRsType;
  static_assert(0 == offsetof(__crubit_assert_type, field));
}
namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_unwrap(::leaf_rs_lib::LeafRsType*);
}
inline ::std::uint8_t unwrap(::leaf_rs_lib::LeafRsType x) {
  return __crubit_internal::__crubit_thunk_unwrap(&x);
}

namespace __crubit_internal {
extern "C" ::std::uint8_t __crubit_thunk_unwrap_uenum(
    ::leaf_rs_lib::LeafRsEnum*);
}
inline ::std::uint8_t unwrap_enum(::leaf_rs_lib::LeafRsEnum x) {
  return __crubit_internal::__crubit_thunk_unwrap_uenum(&x);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_wrap(::std::uint8_t,
                                    ::leaf_rs_lib::LeafRsType* __ret_ptr);
}
inline ::leaf_rs_lib::LeafRsType wrap(::std::uint8_t x) {
  crubit::Slot<::leaf_rs_lib::LeafRsType> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_wrap(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_wrap_uenum(::std::uint8_t,
                                          ::leaf_rs_lib::LeafRsEnum* __ret_ptr);
}
inline ::leaf_rs_lib::LeafRsEnum wrap_enum(::std::uint8_t x) {
  crubit::Slot<::leaf_rs_lib::LeafRsEnum> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_wrap_uenum(x, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

}  // namespace leaf_rs_lib

#pragma clang diagnostic pop
#endif  // COMMON_TEST_BIDIRECTIONAL_DEPS_LEAF_RS_LIB_GOLDEN
