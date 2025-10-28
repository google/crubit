// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// aliasing_references_golden
// Features: do_not_hardcode_status_bridge, non_unpin_ctor, std_unique_ptr,
// std_vector, supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ALIASING_REFERENCES_ALIASING_REFERENCES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ALIASING_REFERENCES_ALIASING_REFERENCES_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/check_no_mutable_aliasing.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace aliasing_references {

// Generated from:
// cc_bindings_from_rs/test/aliasing_references/aliasing_references.rs;l=8
void mut_refs(std::int32_t& __param_0, std::int32_t& __param_1);

// Generated from:
// cc_bindings_from_rs/test/aliasing_references/aliasing_references.rs;l=9
void mut_ref_and_shared_refs(std::int32_t& __param_0,
                             std::int32_t const& __param_1,
                             std::int32_t const& __param_2);

// Generated from:
// cc_bindings_from_rs/test/aliasing_references/aliasing_references.rs;l=12
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: aliasing_references_golden :: SomeStruct") alignas(4)
    [[clang::trivial_abi]] SomeStruct final {
 public:
  // Default::default
  SomeStruct();

  // No custom `Drop` impl and no custom "drop glue" required
  ~SomeStruct() = default;
  SomeStruct(SomeStruct&&) = default;
  SomeStruct& operator=(SomeStruct&&) = default;

  // Rust types that are `Copy` get trivial, `default` C++ copy constructor and
  // assignment operator.
  SomeStruct(const SomeStruct&) = default;
  SomeStruct& operator=(const SomeStruct&) = default;
  SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/aliasing_references/aliasing_references.rs;l=17
  void mut_self_and_mut_ref(std::int32_t& __param_1);

  // Generated from:
  // cc_bindings_from_rs/test/aliasing_references/aliasing_references.rs;l=18
  void mut_self_and_shared_ref(std::int32_t const& __param_1);

  // Generated from:
  // cc_bindings_from_rs/test/aliasing_references/aliasing_references.rs;l=19
  void shared_self_and_mut_ref(std::int32_t& __param_1) const;

  // Generated from:
  // cc_bindings_from_rs/test/aliasing_references/aliasing_references.rs;l=20
  void shared_self_and_shared_ref_allows_alias(
      std::int32_t const& __param_1) const;

  union {
    // Generated from:
    // cc_bindings_from_rs/test/aliasing_references/aliasing_references.rs;l=13
    std::int32_t field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/aliasing_references/aliasing_references.rs;l=24
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: aliasing_references_golden :: NonFreezeType") alignas(4)
    [[clang::trivial_abi]] NonFreezeType final {
 public:
  // Default::default
  NonFreezeType();

  // No custom `Drop` impl and no custom "drop glue" required
  ~NonFreezeType() = default;
  NonFreezeType(NonFreezeType&&) = default;
  NonFreezeType& operator=(NonFreezeType&&) = default;

  // `NonFreezeType` doesn't implement the `Clone` trait
  NonFreezeType(const NonFreezeType&) = delete;
  NonFreezeType& operator=(const NonFreezeType&) = delete;
  NonFreezeType(::crubit::UnsafeRelocateTag, NonFreezeType&& value) {
    memcpy(this, &value, sizeof(value));
  }

  //  # Safety
  //
  //
  //
  //  This function must not be called while an outstanding reference to the
  //  underlying
  //
  //  `i32` is held.
  //
  // Generated from:
  // cc_bindings_from_rs/test/aliasing_references/aliasing_references.rs;l=32
  std::int32_t& [[clang::annotate_type("lifetime",
                                       "__anon1")]] as_mut_unchecked()
      const& [[clang::annotate_type("lifetime", "__anon1")]];

  // Generated from:
  // cc_bindings_from_rs/test/aliasing_references/aliasing_references.rs;l=35
  void shared_self_mut_ref_allows_alias(std::int32_t& __param_1) const;

 private:
  // Field type has been replaced with a blob of bytes: Generic types are not
  // supported yet (b/259749095)
  unsigned char __field0[4];

 private:
  static void __crubit_field_offset_assertions();
};

namespace __crubit_internal {
extern "C" void __crubit_thunk_mut_urefs(std::int32_t&, std::int32_t&);
}
inline void mut_refs(std::int32_t& __param_0, std::int32_t& __param_1) {
  crubit::internal::CheckNoMutableAliasing(
      crubit::internal::AsMutPtrDatas<std::int32_t&, std::int32_t&>(__param_0,
                                                                    __param_1),
      crubit::internal::AsPtrDatas<>());
  return __crubit_internal::__crubit_thunk_mut_urefs(__param_0, __param_1);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_mut_uref_uand_ushared_urefs(std::int32_t&,
                                                           std::int32_t const&,
                                                           std::int32_t const&);
}
inline void mut_ref_and_shared_refs(std::int32_t& __param_0,
                                    std::int32_t const& __param_1,
                                    std::int32_t const& __param_2) {
  crubit::internal::CheckNoMutableAliasing(
      crubit::internal::AsMutPtrDatas<std::int32_t&>(__param_0),
      crubit::internal::AsPtrDatas<std::int32_t const&, std::int32_t const&>(
          __param_1, __param_2));
  return __crubit_internal::__crubit_thunk_mut_uref_uand_ushared_urefs(
      __param_0, __param_1, __param_2);
}

static_assert(
    sizeof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::aliasing_references::SomeStruct* __ret_ptr);
}
inline SomeStruct::SomeStruct() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<SomeStruct>);
static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
static_assert(std::is_trivially_copy_constructible_v<SomeStruct>);
static_assert(std::is_trivially_copy_assignable_v<SomeStruct>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_mut_uself_uand_umut_uref(
    ::aliasing_references::SomeStruct&, std::int32_t&);
}
inline void SomeStruct::mut_self_and_mut_ref(std::int32_t& __param_1) {
  auto&& self = *this;
  crubit::internal::CheckNoMutableAliasing(
      crubit::internal::AsMutPtrDatas<::aliasing_references::SomeStruct&,
                                      std::int32_t&>(self, __param_1),
      crubit::internal::AsPtrDatas<>());
  return __crubit_internal::__crubit_thunk_mut_uself_uand_umut_uref(self,
                                                                    __param_1);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_mut_uself_uand_ushared_uref(
    ::aliasing_references::SomeStruct&, std::int32_t const&);
}
inline void SomeStruct::mut_self_and_shared_ref(std::int32_t const& __param_1) {
  auto&& self = *this;
  crubit::internal::CheckNoMutableAliasing(
      crubit::internal::AsMutPtrDatas<::aliasing_references::SomeStruct&>(self),
      crubit::internal::AsPtrDatas<std::int32_t const&>(__param_1));
  return __crubit_internal::__crubit_thunk_mut_uself_uand_ushared_uref(
      self, __param_1);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_shared_uself_uand_umut_uref(
    ::aliasing_references::SomeStruct const&, std::int32_t&);
}
inline void SomeStruct::shared_self_and_mut_ref(std::int32_t& __param_1) const {
  auto&& self = *this;
  crubit::internal::CheckNoMutableAliasing(
      crubit::internal::AsMutPtrDatas<std::int32_t&>(__param_1),
      crubit::internal::AsPtrDatas<::aliasing_references::SomeStruct const&>(
          self));
  return __crubit_internal::__crubit_thunk_shared_uself_uand_umut_uref(
      self, __param_1);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_shared_uself_uand_ushared_uref_uallows_ualias(
    ::aliasing_references::SomeStruct const&, std::int32_t const&);
}
inline void SomeStruct::shared_self_and_shared_ref_allows_alias(
    std::int32_t const& __param_1) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_shared_uself_uand_ushared_uref_uallows_ualias(self,
                                                                   __param_1);
}
inline void SomeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SomeStruct, field));
}
static_assert(
    sizeof(NonFreezeType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NonFreezeType) == 4,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(
    ::aliasing_references::NonFreezeType* __ret_ptr);
}
inline NonFreezeType::NonFreezeType() {
  __crubit_internal::__crubit_thunk_default(this);
}
static_assert(std::is_trivially_destructible_v<NonFreezeType>);
static_assert(std::is_trivially_move_constructible_v<NonFreezeType>);
static_assert(std::is_trivially_move_assignable_v<NonFreezeType>);
namespace __crubit_internal {
extern "C" std::int32_t& [[clang::annotate_type(
    "lifetime",
    "__anon1")]] __crubit_thunk_as_umut_uunchecked(::aliasing_references::
                                                       NonFreezeType const&);
}
inline std::int32_t& [[clang::annotate_type(
    "lifetime", "__anon1")]] NonFreezeType::as_mut_unchecked()
    const& [[clang::annotate_type("lifetime", "__anon1")]] {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_as_umut_uunchecked(self);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_shared_uself_umut_uref_uallows_ualias(
    ::aliasing_references::NonFreezeType const&, std::int32_t&);
}
inline void NonFreezeType::shared_self_mut_ref_allows_alias(
    std::int32_t& __param_1) const {
  auto&& self = *this;
  return __crubit_internal::
      __crubit_thunk_shared_uself_umut_uref_uallows_ualias(self, __param_1);
}
inline void NonFreezeType::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NonFreezeType, __field0));
}
}  // namespace aliasing_references
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ALIASING_REFERENCES_ALIASING_REFERENCES_GOLDEN
