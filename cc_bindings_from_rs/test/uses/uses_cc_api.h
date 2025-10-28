// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// uses_golden
// Features: do_not_hardcode_status_bridge, non_unpin_ctor, std_unique_ptr,
// std_vector, supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_USES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_USES_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <utility>

#include "cc_bindings_from_rs/test/uses/extern_crate.h"

namespace uses {

namespace test_mod {

// Generated from:
// cc_bindings_from_rs/test/uses/uses.rs;l=10
std::int32_t f();

}  // namespace test_mod

using ::uses::test_mod::f;

// Generated from:
// cc_bindings_from_rs/test/uses/uses.rs;l=16
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: uses_golden :: AliasOfExportedStruct") alignas(4) [[clang::trivial_abi]]
AliasOfExportedStruct final {
 public:
  // `private_mod::ReexportedStruct` doesn't implement the `Default` trait
  AliasOfExportedStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~AliasOfExportedStruct() = default;
  AliasOfExportedStruct(AliasOfExportedStruct&&) = default;
  AliasOfExportedStruct& operator=(AliasOfExportedStruct&&) = default;

  // `private_mod::ReexportedStruct` doesn't implement the `Clone` trait
  AliasOfExportedStruct(const AliasOfExportedStruct&) = delete;
  AliasOfExportedStruct& operator=(const AliasOfExportedStruct&) = delete;
  AliasOfExportedStruct(::crubit::UnsafeRelocateTag,
                        AliasOfExportedStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/uses/uses.rs;l=21
  static ::uses::AliasOfExportedStruct create(std::int32_t field);

  union {
    // Generated from:
    // cc_bindings_from_rs/test/uses/uses.rs;l=17
    std::int32_t field;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Generated from:
// cc_bindings_from_rs/test/uses/uses.rs;l=26
std::int32_t private_fn();

using ::uses::private_fn;
using ExportedStruct CRUBIT_INTERNAL_RUST_TYPE(
    ":: uses_golden :: AliasOfExportedStruct") = ::uses::AliasOfExportedStruct;
using AliasOfExportedStruct CRUBIT_INTERNAL_RUST_TYPE(
    ":: uses_golden :: AliasOfExportedStruct") = ::uses::AliasOfExportedStruct;
using X CRUBIT_INTERNAL_RUST_TYPE(":: extern_crate :: X") = ::extern_crate::X;

// Generated from:
// cc_bindings_from_rs/test/uses/uses.rs;l=41
::extern_crate::X return_x();

// CRUBIT_ANNOTATE: must_bind=
//
// Generated from:
// cc_bindings_from_rs/test/uses/uses.rs;l=46
::extern_crate::Y return_y();

// Generated from:
// cc_bindings_from_rs/test/uses/uses.rs;l=50
struct CRUBIT_INTERNAL_RUST_TYPE(":: uses_golden :: Original") alignas(4)
    [[clang::trivial_abi]] Original final {
 public:
  // `Original` doesn't implement the `Default` trait
  Original() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~Original() = default;
  Original(Original&&) = default;
  Original& operator=(Original&&) = default;

  // `Original` doesn't implement the `Clone` trait
  Original(const Original&) = delete;
  Original& operator=(const Original&) = delete;
  Original(::crubit::UnsafeRelocateTag, Original&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/uses/uses.rs;l=51
    std::int32_t field;
  };

 private:
  static void __crubit_field_offset_assertions();
};
using Alias CRUBIT_INTERNAL_RUST_TYPE(":: uses_golden :: Alias") =
    ::uses::Original;
using Alias2 CRUBIT_INTERNAL_RUST_TYPE(":: uses_golden :: Alias") =
    ::uses::Original;

namespace doc_hidden_test::visible {

// Generated from:
// cc_bindings_from_rs/test/uses/uses.rs;l=60
std::int32_t private_fn();

}  // namespace doc_hidden_test::visible

namespace doc_hidden_test::hidden {
using ::uses::doc_hidden_test::visible::private_fn;
}

namespace doc_hidden_test::visible {
using ::uses::doc_hidden_test::visible::private_fn;
}

namespace a::c {

// Generated from:
// cc_bindings_from_rs/test/uses/uses.rs;l=78
std::int32_t private_middle_path();

}  // namespace a::c

namespace a {}

namespace test_mod {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_f();
}
inline std::int32_t f() { return __crubit_internal::__crubit_thunk_f(); }

}  // namespace test_mod

static_assert(
    sizeof(AliasOfExportedStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(AliasOfExportedStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<AliasOfExportedStruct>);
static_assert(std::is_trivially_move_constructible_v<AliasOfExportedStruct>);
static_assert(std::is_trivially_move_assignable_v<AliasOfExportedStruct>);
namespace __crubit_internal {
extern "C" void __crubit_thunk_create(std::int32_t,
                                      ::uses::AliasOfExportedStruct* __ret_ptr);
}
inline ::uses::AliasOfExportedStruct AliasOfExportedStruct::create(
    std::int32_t field) {
  crubit::Slot<::uses::AliasOfExportedStruct> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create(field, __return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}
inline void AliasOfExportedStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(AliasOfExportedStruct, field));
}
namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_private_ufn();
}
inline std::int32_t private_fn() {
  return __crubit_internal::__crubit_thunk_private_ufn();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_ux(::extern_crate::X* __ret_ptr);
}
inline ::extern_crate::X return_x() {
  crubit::Slot<::extern_crate::X> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_ux(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_return_uy(::extern_crate::Y* __ret_ptr);
}
inline ::extern_crate::Y return_y() {
  crubit::Slot<::extern_crate::Y> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_return_uy(__return_value_storage);
  return std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

static_assert(
    sizeof(Original) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(Original) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(std::is_trivially_destructible_v<Original>);
static_assert(std::is_trivially_move_constructible_v<Original>);
static_assert(std::is_trivially_move_assignable_v<Original>);
inline void Original::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(Original, field));
}

namespace doc_hidden_test::visible {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_private_ufn();
}
inline std::int32_t private_fn() {
  return __crubit_internal::__crubit_thunk_private_ufn();
}

}  // namespace doc_hidden_test::visible

namespace doc_hidden_test::hidden {}

namespace doc_hidden_test::visible {}

namespace a::c {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_private_umiddle_upath();
}
inline std::int32_t private_middle_path() {
  return __crubit_internal::__crubit_thunk_private_umiddle_upath();
}

}  // namespace a::c

namespace a {}

}  // namespace uses
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_USES_GOLDEN
