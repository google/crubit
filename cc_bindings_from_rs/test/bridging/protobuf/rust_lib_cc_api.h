// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rust_lib_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_PROTOBUF_RUST_LIB_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_PROTOBUF_RUST_LIB_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/offsetof.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"

#include <array>
#include <cstddef>
#include <cstring>
#include <utility>

#include "cc_bindings_from_rs/test/bridging/protobuf/foo.proto.h"
#include "cc_bindings_from_rs/test/bridging/protobuf/foo_cpp_rust_proto.h"

namespace rust_lib {

struct CRUBIT_INTERNAL_RUST_TYPE(":: rust_lib_golden :: FooService") alignas(8)
    [[clang::trivial_abi]] FooService final {
 public:
  // Default::default
  FooService();

  // Drop::drop
  ~FooService();

  FooService(FooService&&);
  ::rust_lib::FooService& operator=(FooService&&);

  // `rust_lib_golden::FooService` doesn't implement the `Clone` trait
  FooService(const FooService&) = delete;
  FooService& operator=(const FooService&) = delete;
  FooService(::crubit::UnsafeRelocateTag, FooService&& value) {
    ::std::memcpy(this, &value, sizeof(value));
  }

  bool handle_request(const ::foo_service::FooRequest* req,
                      ::foo_service::FooResponse* rsp);

  const ::foo_service::FooRequestStats* request_stats() const& $(__anon1)
      CRUBIT_LIFETIME_BOUND;

  ::foo_service::FooRequestStats clone_request_stats() const;

  void update_request_stats(::foo_service::FooRequestStats updated_stats);

  static void enum_in_signature(::foo_proto::FooEnum _e);

 private:
  // Field type has been replaced with a blob of bytes: Field is a bridged type
  // and might not be layout-compatible
  //                                     with the C++ type (b/400633609)
  ::std::array<unsigned char, 8> stats;

 private:
  static void __crubit_field_offset_assertions();
};

static_assert(
    sizeof(FooService) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(FooService) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void __crubit_thunk_default(::rust_lib::FooService* __ret_ptr);
}
inline ::rust_lib::FooService::FooService() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::rust_lib::FooService&);
}
inline FooService::~FooService() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline ::rust_lib::FooService::FooService(FooService&& other) : FooService() {
  *this = ::std::move(other);
}
inline ::rust_lib::FooService& ::rust_lib::FooService::operator=(
    FooService&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" bool __crubit_thunk_handle_urequest(::rust_lib::FooService&,
                                               const ::foo_service::FooRequest*,
                                               ::foo_service::FooResponse*);
}
inline bool FooService::handle_request(const ::foo_service::FooRequest* req,
                                       ::foo_service::FooResponse* rsp) {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_handle_urequest(self, req, rsp);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_request_ustats(
    ::rust_lib::FooService const&,
    const ::foo_service::FooRequestStats** __ret_ptr);
}
inline const ::foo_service::FooRequestStats* FooService::request_stats()
    const& $(__anon1) CRUBIT_LIFETIME_BOUND {
  auto&& self = *this;
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { ::std::destroy_at(&this->val); }
    const ::foo_service::FooRequestStats* val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_request_ustats(self,
                                                   __return_value_storage);
  return ::std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_urequest_ustats(
    ::rust_lib::FooService const&, ::foo_service::FooRequestStats* __ret_ptr);
}
inline ::foo_service::FooRequestStats FooService::clone_request_stats() const {
  auto&& self = *this;
  crubit::Slot<::foo_service::FooRequestStats> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_clone_urequest_ustats(
      self, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_update_urequest_ustats(
    ::rust_lib::FooService&, ::foo_service::FooRequestStats*);
}
inline void FooService::update_request_stats(
    ::foo_service::FooRequestStats updated_stats) {
  auto&& self = *this;
  crubit::Slot updated_stats_slot((::std::move(updated_stats)));
  return __crubit_internal::__crubit_thunk_update_urequest_ustats(
      self, updated_stats_slot.Get());
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_enum_uin_usignature(::foo_proto::FooEnum);
}
inline void FooService::enum_in_signature(::foo_proto::FooEnum _e) {
  return __crubit_internal::__crubit_thunk_enum_uin_usignature(_e);
}
inline void FooService::__crubit_field_offset_assertions() {
  CRUBIT_WARNING_PUSH("-Wno-invalid-offsetof")
  static_assert(0 == offsetof(FooService, stats));
  CRUBIT_WARNING_POP
}
}  // namespace rust_lib

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_PROTOBUF_RUST_LIB_GOLDEN
