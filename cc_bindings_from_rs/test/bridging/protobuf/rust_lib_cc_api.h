// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// rust_lib_golden
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_PROTOBUF_RUST_LIB_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_PROTOBUF_RUST_LIB_GOLDEN

#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <utility>

#include "cc_bindings_from_rs/test/bridging/protobuf/foo.proto.h"

namespace rust_lib {

// Generated from:
// cc_bindings_from_rs/test/bridging/protobuf/rust_lib.rs;l=12
struct CRUBIT_INTERNAL_RUST_TYPE(":: rust_lib_golden :: FooService") alignas(8)
    [[clang::trivial_abi]] FooService final {
 public:
  // Default::default
  FooService();

  // Drop::drop
  ~FooService();

  FooService(FooService&&);
  FooService& operator=(FooService&&);

  // `FooService` doesn't implement the `Clone` trait
  FooService(const FooService&) = delete;
  FooService& operator=(const FooService&) = delete;
  FooService(::crubit::UnsafeRelocateTag, FooService&& value) {
    memcpy(this, &value, sizeof(value));
  }

  // Generated from:
  // cc_bindings_from_rs/test/bridging/protobuf/rust_lib.rs;l=17
  bool handle_request(const ::foo_service::FooRequest* req,
                      ::foo_service::FooResponse* rsp);

  // Generated from:
  // cc_bindings_from_rs/test/bridging/protobuf/rust_lib.rs;l=24
  const ::foo_service::FooRequestStats* request_stats()
      const& [[clang::annotate_type("lifetime", "__anon1")]];

  // Generated from:
  // cc_bindings_from_rs/test/bridging/protobuf/rust_lib.rs;l=28
  ::foo_service::FooRequestStats clone_request_stats() const;

  // Generated from:
  // cc_bindings_from_rs/test/bridging/protobuf/rust_lib.rs;l=32
  void update_request_stats(::foo_service::FooRequestStats updated_stats);

 private:
  // Field type has been replaced with a blob of bytes: Field is a bridged type
  // and might not be layout-compatible
  //                                     with the C++ type (b/400633609)
  unsigned char stats[8];

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
inline FooService::FooService() {
  __crubit_internal::__crubit_thunk_default(this);
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_drop(::rust_lib::FooService&);
}
inline FooService::~FooService() {
  __crubit_internal::__crubit_thunk_drop(*this);
}
inline FooService::FooService(FooService&& other) : FooService() {
  *this = std::move(other);
}
inline FooService& FooService::operator=(FooService&& other) {
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
    const& [[clang::annotate_type("lifetime", "__anon1")]] {
  auto&& self = *this;
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { std::destroy_at(&this->val); }
    const ::foo_service::FooRequestStats* val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_request_ustats(self,
                                                   __return_value_storage);
  return std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_clone_urequest_ustats(
    ::rust_lib::FooService const&, ::foo_service::FooRequestStats* __ret_ptr);
}
inline ::foo_service::FooRequestStats FooService::clone_request_stats() const {
  auto&& self = *this;
  union __return_value_crubit_return_union {
    constexpr __return_value_crubit_return_union() {}
    ~__return_value_crubit_return_union() { std::destroy_at(&this->val); }
    ::foo_service::FooRequestStats val;
  } __return_value_ret_val_holder;
  auto* __return_value_storage = &__return_value_ret_val_holder.val;
  __crubit_internal::__crubit_thunk_clone_urequest_ustats(
      self, __return_value_storage);
  return std::move(__return_value_ret_val_holder.val);
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_update_urequest_ustats(
    ::rust_lib::FooService&, ::foo_service::FooRequestStats*);
}
inline void FooService::update_request_stats(
    ::foo_service::FooRequestStats updated_stats) {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_update_urequest_ustats(
      self, &updated_stats);
}
inline void FooService::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(FooService, stats));
}
}  // namespace rust_lib
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_PROTOBUF_RUST_LIB_GOLDEN
