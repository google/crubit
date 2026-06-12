// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// tag_decl_annotations_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TAG_DECL_ANNOTATIONS_TAG_DECL_ANNOTATIONS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TAG_DECL_ANNOTATIONS_TAG_DECL_ANNOTATIONS_GOLDEN

#ifdef KYTHE_IS_RUNNING
#pragma kythe_inline_metadata "This file contains Kythe metadata."
#endif
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/offsetof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>

namespace tag_decl_annotations {

// CRUBIT_ANNOTATE: cpp_enum=enum class
//
// Generated from:
// cc_bindings_from_rs/test/tag_decl_annotations/tag_decl_annotations.rs;l=7
enum class CRUBIT_INTERNAL_RUST_TYPE(
    ":: tag_decl_annotations_golden :: SomeEnum") SomeEnum : ::std::int32_t {
  // Generated from:
  // cc_bindings_from_rs/test/tag_decl_annotations/tag_decl_annotations.rs;l=9
  VARIANT_0 = INT32_C(0),
  // Generated from:
  // cc_bindings_from_rs/test/tag_decl_annotations/tag_decl_annotations.rs;l=10
  VARIANT_1 = INT32_C(1),
  // Generated from:
  // cc_bindings_from_rs/test/tag_decl_annotations/tag_decl_annotations.rs;l=11
  VARIANT_2 = INT32_C(2),
};

// Generated from:
// cc_bindings_from_rs/test/tag_decl_annotations/tag_decl_annotations.rs;l=14
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tag_decl_annotations_golden :: SomeStruct") alignas(4)
    [[clang::trivial_abi]] SomeStruct final {
 public:
  // `tag_decl_annotations_golden::SomeStruct` doesn't implement the `Default`
  // trait
  SomeStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~SomeStruct() = default;
  SomeStruct(SomeStruct&&) = default;
  SomeStruct& operator=(SomeStruct&&) = default;

  // `tag_decl_annotations_golden::SomeStruct` doesn't implement the `Clone`
  // trait
  SomeStruct(const SomeStruct&) = delete;
  SomeStruct& operator=(const SomeStruct&) = delete;
  SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/tag_decl_annotations/tag_decl_annotations.rs;l=15
    ::std::int32_t f;
  };

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SomeStruct) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<SomeStruct>);
static_assert(::std::is_trivially_move_constructible_v<
              ::tag_decl_annotations::SomeStruct>);
static_assert(
    ::std::is_trivially_move_assignable_v<::tag_decl_annotations::SomeStruct>);
inline void SomeStruct::__crubit_field_offset_assertions() {
  CRUBIT_WARNING_PUSH("-Wno-invalid-offsetof")
  static_assert(0 == offsetof(SomeStruct, f));
  CRUBIT_WARNING_POP
}
}  // namespace tag_decl_annotations

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TAG_DECL_ANNOTATIONS_TAG_DECL_ANNOTATIONS_GOLDEN

// This file contains Kythe metadata. eyJ0eXBlIjoia3l0aGUwIiwibWV0YSI6W3sidHlwZSI6ImFuY2hvcl9hbmNob3IiLCJzb3VyY2VfYmVnaW4iOjI2NCwic291cmNlX2VuZCI6MjcyLCJ0YXJnZXRfYmVnaW4iOjExOTUsInRhcmdldF9lbmQiOjEyMDMsImVkZ2UiOiIva3l0aGUvZWRnZS9pbXB1dGVzIiwic291cmNlX3ZuYW1lIjp7ImNvcnB1cyI6ImNvcnB1cyIsInBhdGgiOiJ0aGlyZF9wYXJ0eS9jcnViaXQvY2NfYmluZGluZ3NfZnJvbV9ycy90ZXN0L3RhZ19kZWNsX2Fubm90YXRpb25zL3RhZ19kZWNsX2Fubm90YXRpb25zLnJzIiwibGFuZ3VhZ2UiOiJydXN0In19LHsidHlwZSI6ImFuY2hvcl9hbmNob3IiLCJzb3VyY2VfYmVnaW4iOjMwOSwic291cmNlX2VuZCI6MzE4LCJ0YXJnZXRfYmVnaW4iOjEzNTIsInRhcmdldF9lbmQiOjEzNjEsImVkZ2UiOiIva3l0aGUvZWRnZS9pbXB1dGVzIiwic291cmNlX3ZuYW1lIjp7ImNvcnB1cyI6ImNvcnB1cyIsInBhdGgiOiJ0aGlyZF9wYXJ0eS9jcnViaXQvY2NfYmluZGluZ3NfZnJvbV9ycy90ZXN0L3RhZ19kZWNsX2Fubm90YXRpb25zL3RhZ19kZWNsX2Fubm90YXRpb25zLnJzIiwibGFuZ3VhZ2UiOiJydXN0In19LHsidHlwZSI6ImFuY2hvcl9hbmNob3IiLCJzb3VyY2VfYmVnaW4iOjM1OCwic291cmNlX2VuZCI6MzY3LCJ0YXJnZXRfYmVnaW4iOjE1MDYsInRhcmdldF9lbmQiOjE1MTUsImVkZ2UiOiIva3l0aGUvZWRnZS9pbXB1dGVzIiwic291cmNlX3ZuYW1lIjp7ImNvcnB1cyI6ImNvcnB1cyIsInBhdGgiOiJ0aGlyZF9wYXJ0eS9jcnViaXQvY2NfYmluZGluZ3NfZnJvbV9ycy90ZXN0L3RhZ19kZWNsX2Fubm90YXRpb25zL3RhZ19kZWNsX2Fubm90YXRpb25zLnJzIiwibGFuZ3VhZ2UiOiJydXN0In19LHsidHlwZSI6ImFuY2hvcl9hbmNob3IiLCJzb3VyY2VfYmVnaW4iOjQwNywic291cmNlX2VuZCI6NDE2LCJ0YXJnZXRfYmVnaW4iOjE2NjAsInRhcmdldF9lbmQiOjE2NjksImVkZ2UiOiIva3l0aGUvZWRnZS9pbXB1dGVzIiwic291cmNlX3ZuYW1lIjp7ImNvcnB1cyI6ImNvcnB1cyIsInBhdGgiOiJ0aGlyZF9wYXJ0eS9jcnViaXQvY2NfYmluZGluZ3NfZnJvbV9ycy90ZXN0L3RhZ19kZWNsX2Fubm90YXRpb25zL3RhZ19kZWNsX2Fubm90YXRpb25zLnJzIiwibGFuZ3VhZ2UiOiJydXN0In19LHsidHlwZSI6ImFuY2hvcl9hbmNob3IiLCJzb3VyY2VfYmVnaW4iOjQ1Niwic291cmNlX2VuZCI6NDY2LCJ0YXJnZXRfYmVnaW4iOjE5MzYsInRhcmdldF9lbmQiOjE5NDYsImVkZ2UiOiIva3l0aGUvZWRnZS9pbXB1dGVzIiwic291cmNlX3ZuYW1lIjp7ImNvcnB1cyI6ImNvcnB1cyIsInBhdGgiOiJ0aGlyZF9wYXJ0eS9jcnViaXQvY2NfYmluZGluZ3NfZnJvbV9ycy90ZXN0L3RhZ19kZWNsX2Fubm90YXRpb25zL3RhZ19kZWNsX2Fubm90YXRpb25zLnJzIiwibGFuZ3VhZ2UiOiJydXN0In19LHsidHlwZSI6ImFuY2hvcl9hbmNob3IiLCJzb3VyY2VfYmVnaW4iOjQ3Nywic291cmNlX2VuZCI6NDc4LCJ0YXJnZXRfYmVnaW4iOjI3MTcsInRhcmdldF9lbmQiOjI3MTgsImVkZ2UiOiIva3l0aGUvZWRnZS9pbXB1dGVzIiwic291cmNlX3ZuYW1lIjp7ImNvcnB1cyI6ImNvcnB1cyIsInBhdGgiOiJ0aGlyZF9wYXJ0eS9jcnViaXQvY2NfYmluZGluZ3NfZnJvbV9ycy90ZXN0L3RhZ19kZWNsX2Fubm90YXRpb25zL3RhZ19kZWNsX2Fubm90YXRpb25zLnJzIiwibGFuZ3VhZ2UiOiJydXN0In19XX0=
