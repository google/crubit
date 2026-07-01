// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// send_sync_types_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_AUTO_TRAITS_SEND_SYNC_TYPES_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_AUTO_TRAITS_SEND_SYNC_TYPES_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <type_traits>

namespace send_sync_types {

//  4. Implements neither Send nor Sync (via raw pointer).
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: send_sync_types_golden :: NeitherSendNorSync") alignas(4)
    [[clang::trivial_abi]] NeitherSendNorSync final {
 public:
  // `send_sync_types_golden::NeitherSendNorSync` doesn't implement the
  // `Default` trait
  NeitherSendNorSync() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~NeitherSendNorSync() = default;
  NeitherSendNorSync(NeitherSendNorSync&&) = default;
  NeitherSendNorSync& operator=(NeitherSendNorSync&&) = default;

  // `send_sync_types_golden::NeitherSendNorSync` doesn't implement the `Clone`
  // trait
  NeitherSendNorSync(const NeitherSendNorSync&) = delete;
  NeitherSendNorSync& operator=(const NeitherSendNorSync&) = delete;
  NeitherSendNorSync(::crubit::UnsafeRelocateTag, NeitherSendNorSync&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t __field0;
  };
  // Field `__field1` omitted: C++ does not support zero-sized types.
 private:
  static void __crubit_field_offset_assertions();
};

//  1. Implements both Send and Sync.
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: send_sync_types_golden :: SendAndSync") alignas(4)
    [[clang::trivial_abi]] SendAndSync final {
 public:
  // `send_sync_types_golden::SendAndSync` doesn't implement the `Default` trait
  SendAndSync() = delete;

  // Synthesized tuple constructor
  explicit SendAndSync(::std::int32_t __field0)
      : __field0(::std::move(__field0)) {}

  // No custom `Drop` impl and no custom "drop glue" required
  ~SendAndSync() = default;
  SendAndSync(SendAndSync&&) = default;
  SendAndSync& operator=(SendAndSync&&) = default;

  // `send_sync_types_golden::SendAndSync` doesn't implement the `Clone` trait
  SendAndSync(const SendAndSync&) = delete;
  SendAndSync& operator=(const SendAndSync&) = delete;
  SendAndSync(::crubit::UnsafeRelocateTag, SendAndSync&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t __field0;
  };

 private:
  static void __crubit_field_offset_assertions();
};

//  2. Implements Send but NOT Sync (via Cell).
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: send_sync_types_golden :: SendButNotSync") alignas(4)
    [[clang::trivial_abi]] SendButNotSync final {
 public:
  // `send_sync_types_golden::SendButNotSync` doesn't implement the `Default`
  // trait
  SendButNotSync() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~SendButNotSync() = default;
  SendButNotSync(SendButNotSync&&) = default;
  SendButNotSync& operator=(SendButNotSync&&) = default;

  // `send_sync_types_golden::SendButNotSync` doesn't implement the `Clone`
  // trait
  SendButNotSync(const SendButNotSync&) = delete;
  SendButNotSync& operator=(const SendButNotSync&) = delete;
  SendButNotSync(::crubit::UnsafeRelocateTag, SendButNotSync&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t __field0;
  };
  // Field `__field1` omitted: C++ does not support zero-sized types.
 private:
  static void __crubit_field_offset_assertions();
};

//  3. Does NOT implement Send, but implements Sync (via MutexGuard).
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: send_sync_types_golden :: SyncButNotSend") alignas(4)
    [[clang::trivial_abi]] SyncButNotSend final {
 public:
  // `send_sync_types_golden::SyncButNotSend` doesn't implement the `Default`
  // trait
  SyncButNotSend() = delete;

  // No custom `Drop` impl and no custom "drop glue" required
  ~SyncButNotSend() = default;
  SyncButNotSend(SyncButNotSend&&) = default;
  SyncButNotSend& operator=(SyncButNotSend&&) = default;

  // `send_sync_types_golden::SyncButNotSend` doesn't implement the `Clone`
  // trait
  SyncButNotSend(const SyncButNotSend&) = delete;
  SyncButNotSend& operator=(const SyncButNotSend&) = delete;
  SyncButNotSend(::crubit::UnsafeRelocateTag, SyncButNotSend&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }
  union {
    ::std::int32_t __field0;
  };
  // Field `__field1` omitted: C++ does not support zero-sized types.
 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(NeitherSendNorSync) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(NeitherSendNorSync) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<NeitherSendNorSync>);
static_assert(::std::is_trivially_move_constructible_v<
              ::send_sync_types::NeitherSendNorSync>);
static_assert(::std::is_trivially_move_assignable_v<
              ::send_sync_types::NeitherSendNorSync>);
inline void NeitherSendNorSync::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(NeitherSendNorSync, __field0));
}
static_assert(
    sizeof(SendAndSync) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SendAndSync) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<SendAndSync>);
static_assert(
    ::std::is_trivially_move_constructible_v<::send_sync_types::SendAndSync>);
static_assert(
    ::std::is_trivially_move_assignable_v<::send_sync_types::SendAndSync>);
inline void SendAndSync::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SendAndSync, __field0));
}
static_assert(
    sizeof(SendButNotSync) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SendButNotSync) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<SendButNotSync>);
static_assert(::std::is_trivially_move_constructible_v<
              ::send_sync_types::SendButNotSync>);
static_assert(
    ::std::is_trivially_move_assignable_v<::send_sync_types::SendButNotSync>);
inline void SendButNotSync::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SendButNotSync, __field0));
}
static_assert(
    sizeof(SyncButNotSend) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(SyncButNotSend) == 4,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(::std::is_trivially_destructible_v<SyncButNotSend>);
static_assert(::std::is_trivially_move_constructible_v<
              ::send_sync_types::SyncButNotSend>);
static_assert(
    ::std::is_trivially_move_assignable_v<::send_sync_types::SyncButNotSend>);
inline void SyncButNotSend::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(SyncButNotSend, __field0));
}
}  // namespace send_sync_types

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_AUTO_TRAITS_SEND_SYNC_TYPES_GOLDEN
