// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// tag_decl_annotations_rust_golden
// Features: custom_ffi_types, experimental, fmt, non_unpin_ctor,
// std_unique_ptr, std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_TAG_DECL_ANNOTATIONS_RUST_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_TAG_DECL_ANNOTATIONS_RUST_GOLDEN

#ifdef KYTHE_IS_RUNNING
#pragma kythe_inline_metadata "This file contains Kythe metadata."
#endif
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace tag_decl_annotations_rust {

// CRUBIT_ANNOTATE: cpp_enum=enum class
//
// Generated from:
// cc_bindings_from_rs/test/golden/tag_decl_annotations.rs;l=7
enum class CRUBIT_INTERNAL_RUST_TYPE(
    ":: tag_decl_annotations_rust_golden :: SomeEnum") SomeEnum : std::int32_t {
  // Generated from:
  // cc_bindings_from_rs/test/golden/tag_decl_annotations.rs;l=9
  VARIANT_0 = INT32_C(0),
  // Generated from:
  // cc_bindings_from_rs/test/golden/tag_decl_annotations.rs;l=10
  VARIANT_1 = INT32_C(1),
  // Generated from:
  // cc_bindings_from_rs/test/golden/tag_decl_annotations.rs;l=11
  VARIANT_2 = INT32_C(2),
};

// Generated from:
// cc_bindings_from_rs/test/golden/tag_decl_annotations.rs;l=14
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: tag_decl_annotations_rust_golden :: SomeStruct") alignas(4)
    [[clang::trivial_abi]] SomeStruct final {
 public:
  // `tag_decl_annotations_rust_golden::SomeStruct` doesn't implement the
  // `Default` trait
  SomeStruct() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~SomeStruct() = default;
  SomeStruct(SomeStruct&&) = default;
  SomeStruct& operator=(SomeStruct&&) = default;

  // `tag_decl_annotations_rust_golden::SomeStruct` doesn't implement the
  // `Clone` trait
  SomeStruct(const SomeStruct&) = delete;
  SomeStruct& operator=(const SomeStruct&) = delete;
  SomeStruct(::crubit::UnsafeRelocateTag, SomeStruct&& value) {
    memcpy(this, &value, sizeof(value));
  }
  union {
    // Generated from:
    // cc_bindings_from_rs/test/golden/tag_decl_annotations.rs;l=15
    std::int32_t f;
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
static_assert(std::is_trivially_destructible_v<SomeStruct>);
static_assert(std::is_trivially_move_constructible_v<SomeStruct>);
static_assert(std::is_trivially_move_assignable_v<SomeStruct>);
inline void SomeStruct::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SomeStruct, f));
}
}  // namespace tag_decl_annotations_rust

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_TAG_DECL_ANNOTATIONS_RUST_GOLDEN

// This file contains Kythe metadata. eyJ0eXBlIjoia3l0aGUwIiwibWV0YSI6W3sidHlwZSI6ImFuY2hvcl9hbmNob3IiLCJzb3VyY2VfYmVnaW4iOjI0NzY5OSwic291cmNlX2VuZCI6MjQ3NzA3LCJ0YXJnZXRfYmVnaW4iOjEwNTQsInRhcmdldF9lbmQiOjEwNjIsImVkZ2UiOiIva3l0aGUvZWRnZS9pbXB1dGVzIiwic291cmNlX3ZuYW1lIjp7ImNvcnB1cyI6ImNvcnB1cyIsInBhdGgiOiJ0aGlyZF9wYXJ0eS9jcnViaXQvY2NfYmluZGluZ3NfZnJvbV9ycy90ZXN0L2dvbGRlbi90YWdfZGVjbF9hbm5vdGF0aW9ucy5ycyIsImxhbmd1YWdlIjoicnVzdCJ9fSx7InR5cGUiOiJhbmNob3JfYW5jaG9yIiwic291cmNlX2JlZ2luIjoyNDc3NDQsInNvdXJjZV9lbmQiOjI0Nzc1MywidGFyZ2V0X2JlZ2luIjoxMTk1LCJ0YXJnZXRfZW5kIjoxMjA0LCJlZGdlIjoiL2t5dGhlL2VkZ2UvaW1wdXRlcyIsInNvdXJjZV92bmFtZSI6eyJjb3JwdXMiOiJjb3JwdXMiLCJwYXRoIjoidGhpcmRfcGFydHkvY3J1Yml0L2NjX2JpbmRpbmdzX2Zyb21fcnMvdGVzdC9nb2xkZW4vdGFnX2RlY2xfYW5ub3RhdGlvbnMucnMiLCJsYW5ndWFnZSI6InJ1c3QifX0seyJ0eXBlIjoiYW5jaG9yX2FuY2hvciIsInNvdXJjZV9iZWdpbiI6MjQ3NzkzLCJzb3VyY2VfZW5kIjoyNDc4MDIsInRhcmdldF9iZWdpbiI6MTMzNSwidGFyZ2V0X2VuZCI6MTM0NCwiZWRnZSI6Ii9reXRoZS9lZGdlL2ltcHV0ZXMiLCJzb3VyY2Vfdm5hbWUiOnsiY29ycHVzIjoiY29ycHVzIiwicGF0aCI6InRoaXJkX3BhcnR5L2NydWJpdC9jY19iaW5kaW5nc19mcm9tX3JzL3Rlc3QvZ29sZGVuL3RhZ19kZWNsX2Fubm90YXRpb25zLnJzIiwibGFuZ3VhZ2UiOiJydXN0In19LHsidHlwZSI6ImFuY2hvcl9hbmNob3IiLCJzb3VyY2VfYmVnaW4iOjI0Nzg0Miwic291cmNlX2VuZCI6MjQ3ODUxLCJ0YXJnZXRfYmVnaW4iOjE0NzUsInRhcmdldF9lbmQiOjE0ODQsImVkZ2UiOiIva3l0aGUvZWRnZS9pbXB1dGVzIiwic291cmNlX3ZuYW1lIjp7ImNvcnB1cyI6ImNvcnB1cyIsInBhdGgiOiJ0aGlyZF9wYXJ0eS9jcnViaXQvY2NfYmluZGluZ3NfZnJvbV9ycy90ZXN0L2dvbGRlbi90YWdfZGVjbF9hbm5vdGF0aW9ucy5ycyIsImxhbmd1YWdlIjoicnVzdCJ9fSx7InR5cGUiOiJhbmNob3JfYW5jaG9yIiwic291cmNlX2JlZ2luIjoyNDc4OTEsInNvdXJjZV9lbmQiOjI0NzkwMSwidGFyZ2V0X2JlZ2luIjoxNzQyLCJ0YXJnZXRfZW5kIjoxNzUyLCJlZGdlIjoiL2t5dGhlL2VkZ2UvaW1wdXRlcyIsInNvdXJjZV92bmFtZSI6eyJjb3JwdXMiOiJjb3JwdXMiLCJwYXRoIjoidGhpcmRfcGFydHkvY3J1Yml0L2NjX2JpbmRpbmdzX2Zyb21fcnMvdGVzdC9nb2xkZW4vdGFnX2RlY2xfYW5ub3RhdGlvbnMucnMiLCJsYW5ndWFnZSI6InJ1c3QifX0seyJ0eXBlIjoiYW5jaG9yX2FuY2hvciIsInNvdXJjZV9iZWdpbiI6MjQ3OTEyLCJzb3VyY2VfZW5kIjoyNDc5MTMsInRhcmdldF9iZWdpbiI6MjUxMCwidGFyZ2V0X2VuZCI6MjUxMSwiZWRnZSI6Ii9reXRoZS9lZGdlL2ltcHV0ZXMiLCJzb3VyY2Vfdm5hbWUiOnsiY29ycHVzIjoiY29ycHVzIiwicGF0aCI6InRoaXJkX3BhcnR5L2NydWJpdC9jY19iaW5kaW5nc19mcm9tX3JzL3Rlc3QvZ29sZGVuL3RhZ19kZWNsX2Fubm90YXRpb25zLnJzIiwibGFuZ3VhZ2UiOiJydXN0In19XX0=
