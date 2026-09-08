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
#include "support/internal/check.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"
#include "support/lifetime_annotations.h"
#include "support/rs_std/vec.h"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <utility>

#include "absl/status/statusor.h"
#include "cc_bindings_from_rs/test/bridging/protobuf/foo.pb.h"
#include "cc_bindings_from_rs/test/bridging/protobuf/foo_cpp_rust_proto.h"
#include "support/protobuf/rust.h"

namespace rust_lib {

// CRUBIT_ANNOTATE: must_bind=
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
  FooService(::crubit::UnsafeRelocateTag, FooService&& value);

  // CRUBIT_ANNOTATE: must_bind=
  bool handle_request(const ::foo_service::FooRequest* req,
                      ::foo_service::FooResponse* rsp);

  // CRUBIT_ANNOTATE: must_bind=
  const ::foo_service::FooRequestStats* request_stats() const& $(__anon1)
      CRUBIT_LIFETIME_BOUND;

  // CRUBIT_ANNOTATE: must_bind=
  ::foo_service::FooRequestStats clone_request_stats() const;

  // CRUBIT_ANNOTATE: must_bind=
  void update_request_stats(::foo_service::FooRequestStats updated_stats);

  // CRUBIT_ANNOTATE: must_bind=
  static void enum_in_signature(::foo_proto::FooEnum _e);

 private:
  union {
    ::proto::Rust<::foo_service::FooRequestStats> stats;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for struct `rust_lib_golden::NewStatusOr` defined
// at
// cc_bindings_from_rs/test/bridging/protobuf/rust_lib.rs;l=76:
// Type bindings for rust_lib_golden::NewStatusOr suppressed due to being mapped
// to an existing C++ type (absl::StatusOr<{T}>)

// CRUBIT_ANNOTATE: must_bind=
struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: rust_lib_golden :: StructWithProto") alignas(8) [[clang::trivial_abi]]
StructWithProto final {
 public:
  // Type is not a C++ aggregate: Field `stats` requires drop glue but is not
  // C++-movable

  // Default::default
  StructWithProto();

  // Drop::drop
  ~StructWithProto();

  StructWithProto(StructWithProto&&);
  ::rust_lib::StructWithProto& operator=(StructWithProto&&);

  // `rust_lib_golden::StructWithProto` doesn't implement the `Clone` trait
  StructWithProto(const StructWithProto&) = delete;
  StructWithProto& operator=(const StructWithProto&) = delete;
  StructWithProto(::crubit::UnsafeRelocateTag, StructWithProto&& value);

  union {
    ::proto::Rust<::foo_service::FooRequestStats> stats;
  };

 private:
  static void __crubit_field_offset_assertions();
};

// CRUBIT_ANNOTATE: must_bind=
absl::StatusOr<::proto::Rust<::foo_service::FooRequestStats>>
create_proto_status_or(::std::int32_t num);

// CRUBIT_ANNOTATE: must_bind=
rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>> create_proto_vec(
    ::std::int32_t num);

// CRUBIT_ANNOTATE: must_bind=
::rust_lib::StructWithProto create_struct_with_proto(::std::int32_t num);

// CRUBIT_ANNOTATE: must_bind=
//  # Safety
//
//  `p` must be valid for reads.
::std::int32_t read_proto_pointer(
    ::proto::Rust<::foo_service::FooRequestStats> const* p);

// CRUBIT_ANNOTATE: must_bind=
::std::int32_t read_proto_ref(
    ::proto::Rust<::foo_service::FooRequestStats> const& p);

}  // namespace rust_lib

#ifndef _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020proto_x00000020_x0000003a_x0000003a_x00000020Rust_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020foo_uservice_x00000020_x0000003a_x0000003a_x00000020FooRequestStats_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020proto_x00000020_x0000003a_x0000003a_x00000020Rust_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020foo_uservice_x00000020_x0000003a_x0000003a_x00000020FooRequestStats_x00000020_x0000003e_x00000020_x0000003e
template <>
struct alignas(8) CRUBIT_INTERNAL_RUST_TYPE(
    ":: alloc :: vec :: Vec < :: foo_proto :: FooRequestStats >")
    rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>> {
 public:
  // Default::default
  Vec();

  // Clone::clone
  Vec(const Vec&);

  // Clone::clone_from
  rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>& operator=(
      const Vec&);

  Vec(Vec&&);
  rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>& operator=(Vec&&);
  Vec(::crubit::UnsafeRelocateTag, Vec&& value);

  ~Vec() noexcept;
  ::proto::Rust<::foo_service::FooRequestStats>* data() noexcept;
  ::proto::Rust<::foo_service::FooRequestStats> const* data() const noexcept;
  std::size_t size() const noexcept;
  ::proto::Rust<::foo_service::FooRequestStats>& operator[](
      std::size_t index) noexcept;
  ::proto::Rust<::foo_service::FooRequestStats> const& operator[](
      std::size_t index) const noexcept;
  ::proto::Rust<::foo_service::FooRequestStats>* begin() noexcept;
  ::proto::Rust<::foo_service::FooRequestStats> const* begin() const noexcept;
  ::proto::Rust<::foo_service::FooRequestStats>* end() noexcept;
  ::proto::Rust<::foo_service::FooRequestStats> const* end() const noexcept;

 private:
  unsigned char storage_[24];
};
#endif

namespace rust_lib {

static_assert(
    sizeof(FooService) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(FooService) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_urust_ulib_ugolden_x0000003a_x0000003aFooService(
    ::rust_lib::FooService* __ret_ptr);
}
inline ::rust_lib::FooService::FooService() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_urust_ulib_ugolden_x0000003a_x0000003aFooService(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_urust_ulib_ugolden_x0000003a_x0000003aFooService(
    ::rust_lib::FooService&);
}
inline FooService::~FooService() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_urust_ulib_ugolden_x0000003a_x0000003aFooService(
          *this);
}
inline ::rust_lib::FooService::FooService(FooService&& other) : FooService() {
  *this = ::std::move(other);
}
inline ::rust_lib::FooService& ::rust_lib::FooService::operator=(
    FooService&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::rust_lib::FooService::FooService(::crubit::UnsafeRelocateTag,
                                          FooService&& value) {
  ::std::memcpy(this, &value, sizeof(value));
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
  static_assert(0 == offsetof(FooService, stats));
}
static_assert(
    sizeof(StructWithProto) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(StructWithProto) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_urust_ulib_ugolden_x0000003a_x0000003aStructWithProto(
    ::rust_lib::StructWithProto* __ret_ptr);
}
inline ::rust_lib::StructWithProto::StructWithProto() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_urust_ulib_ugolden_x0000003a_x0000003aStructWithProto(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_urust_ulib_ugolden_x0000003a_x0000003aStructWithProto(
    ::rust_lib::StructWithProto&);
}
inline StructWithProto::~StructWithProto() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_urust_ulib_ugolden_x0000003a_x0000003aStructWithProto(
          *this);
}
inline ::rust_lib::StructWithProto::StructWithProto(StructWithProto&& other)
    : StructWithProto() {
  *this = ::std::move(other);
}
inline ::rust_lib::StructWithProto& ::rust_lib::StructWithProto::operator=(
    StructWithProto&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline ::rust_lib::StructWithProto::StructWithProto(::crubit::UnsafeRelocateTag,
                                                    StructWithProto&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}
inline void StructWithProto::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(StructWithProto, stats));
}
namespace __crubit_internal {
extern "C" void __crubit_thunk_create_uproto_ustatus_uor(
    ::std::int32_t,
    absl::StatusOr<::proto::Rust<::foo_service::FooRequestStats>>* __ret_ptr);
}
inline absl::StatusOr<::proto::Rust<::foo_service::FooRequestStats>>
create_proto_status_or(::std::int32_t num) {
  crubit::Slot<absl::StatusOr<::proto::Rust<::foo_service::FooRequestStats>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create_uproto_ustatus_uor(
      num, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create_uproto_uvec(
    ::std::int32_t,
    rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>* __ret_ptr);
}
inline rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>
create_proto_vec(::std::int32_t num) {
  crubit::Slot<rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>>
      __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create_uproto_uvec(num,
                                                       __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" void __crubit_thunk_create_ustruct_uwith_uproto(
    ::std::int32_t, ::rust_lib::StructWithProto* __ret_ptr);
}
inline ::rust_lib::StructWithProto create_struct_with_proto(
    ::std::int32_t num) {
  crubit::Slot<::rust_lib::StructWithProto> __return_value_ret_val_holder;
  auto* __return_value_storage = __return_value_ret_val_holder.Get();
  __crubit_internal::__crubit_thunk_create_ustruct_uwith_uproto(
      num, __return_value_storage);
  return ::std::move(__return_value_ret_val_holder).AssumeInitAndTakeValue();
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_read_uproto_upointer(
    ::proto::Rust<::foo_service::FooRequestStats> const*);
}
inline ::std::int32_t read_proto_pointer(
    ::proto::Rust<::foo_service::FooRequestStats> const* p) {
  return __crubit_internal::__crubit_thunk_read_uproto_upointer(p);
}

namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_read_uproto_uref(
    ::proto::Rust<::foo_service::FooRequestStats> const&);
}
inline ::std::int32_t read_proto_ref(
    ::proto::Rust<::foo_service::FooRequestStats> const& p) {
  return __crubit_internal::__crubit_thunk_read_uproto_uref(p);
}

}  // namespace rust_lib

#ifndef _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020proto_x00000020_x0000003a_x0000003a_x00000020Rust_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020foo_uservice_x00000020_x0000003a_x0000003a_x00000020FooRequestStats_x00000020_x0000003e_x00000020_x0000003e
#define _CRUBIT_BINDINGS_FOR_IMPL_rs_ustd_x00000020_x0000003a_x0000003a_x00000020Vec_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020proto_x00000020_x0000003a_x0000003a_x00000020Rust_x00000020_x0000003c_x00000020_x0000003a_x0000003a_x00000020foo_uservice_x00000020_x0000003a_x0000003a_x00000020FooRequestStats_x00000020_x0000003e_x00000020_x0000003e
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cfoo_uproto_x0000003a_x0000003athird_uparty_ucrubit_ucc_ubindings_ufrom_urs_utest_ubridging_uprotobuf_ufoo_uproto_x0000003a_x0000003aFooRequestStats_x0000003e(
    rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>* __ret_ptr);
}
inline rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>::Vec() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cfoo_uproto_x0000003a_x0000003athird_uparty_ucrubit_ucc_ubindings_ufrom_urs_utest_ubridging_uprotobuf_ufoo_uproto_x0000003a_x0000003aFooRequestStats_x0000003e(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cfoo_uproto_x0000003a_x0000003athird_uparty_ucrubit_ucc_ubindings_ufrom_urs_utest_ubridging_uprotobuf_ufoo_uproto_x0000003a_x0000003aFooRequestStats_x0000003e(
    rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>> const&,
    rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cfoo_uproto_x0000003a_x0000003athird_uparty_ucrubit_ucc_ubindings_ufrom_urs_utest_ubridging_uprotobuf_ufoo_uproto_x0000003a_x0000003aFooRequestStats_x0000003e(
    rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>&,
    rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>> const&);
}
inline rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>::Vec(
    const Vec& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cfoo_uproto_x0000003a_x0000003athird_uparty_ucrubit_ucc_ubindings_ufrom_urs_utest_ubridging_uprotobuf_ufoo_uproto_x0000003a_x0000003aFooRequestStats_x0000003e(
          other, this);
}
inline rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>&
rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>::operator=(
    const Vec& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cfoo_uproto_x0000003a_x0000003athird_uparty_ucrubit_ucc_ubindings_ufrom_urs_utest_ubridging_uprotobuf_ufoo_uproto_x0000003a_x0000003aFooRequestStats_x0000003e(
            *this, other);
  }
  return *this;
}
inline rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>::Vec(
    Vec&& other)
    : Vec() {
  *this = ::std::move(other);
}
inline rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>& rs_std::Vec<
    ::proto::Rust<::foo_service::FooRequestStats>>::operator=(Vec&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
inline rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>::Vec(
    ::crubit::UnsafeRelocateTag, Vec&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

extern "C" void
__crubit_thunk_Drop_udrop_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cfoo_uproto_x0000003a_x0000003athird_uparty_ucrubit_ucc_ubindings_ufrom_urs_utest_ubridging_uprotobuf_ufoo_uproto_x0000003a_x0000003aFooRequestStats_x0000003e(
    void* vec) noexcept;
inline rs_std::Vec<
    ::proto::Rust<::foo_service::FooRequestStats>>::~Vec() noexcept {
  __crubit_thunk_Drop_udrop_ustd_x0000003a_x0000003avec_x0000003a_x0000003aVec_x0000003cfoo_uproto_x0000003a_x0000003athird_uparty_ucrubit_ucc_ubindings_ufrom_urs_utest_ubridging_uprotobuf_ufoo_uproto_x0000003a_x0000003aFooRequestStats_x0000003e(
      this);
}
inline ::proto::Rust<::foo_service::FooRequestStats>*
rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>::data() noexcept {
  return std::bit_cast<::proto::Rust<::foo_service::FooRequestStats>*>(
      *reinterpret_cast<const std::uintptr_t*>(&storage_[8]));
}
inline ::proto::Rust<::foo_service::FooRequestStats> const* rs_std::Vec<
    ::proto::Rust<::foo_service::FooRequestStats>>::data() const noexcept {
  return std::bit_cast<::proto::Rust<::foo_service::FooRequestStats>*>(
      *reinterpret_cast<const std::uintptr_t*>(&storage_[8]));
}
inline std::size_t rs_std::Vec<
    ::proto::Rust<::foo_service::FooRequestStats>>::size() const noexcept {
  return std::bit_cast<std::size_t>(
      *reinterpret_cast<const std::size_t*>(&storage_[16]));
}
inline ::proto::Rust<::foo_service::FooRequestStats>&
rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>::operator[](
    std::size_t index) noexcept {
  CRUBIT_CHECK(index < size());
  return data()[index];
}
inline ::proto::Rust<::foo_service::FooRequestStats> const&
rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>::operator[](
    std::size_t index) const noexcept {
  CRUBIT_CHECK(index < size());
  return data()[index];
}
inline ::proto::Rust<::foo_service::FooRequestStats>*
rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>::begin() noexcept {
  return data();
}
inline ::proto::Rust<::foo_service::FooRequestStats> const* rs_std::Vec<
    ::proto::Rust<::foo_service::FooRequestStats>>::begin() const noexcept {
  return data();
}
inline ::proto::Rust<::foo_service::FooRequestStats>*
rs_std::Vec<::proto::Rust<::foo_service::FooRequestStats>>::end() noexcept {
  return data() + size();
}
inline ::proto::Rust<::foo_service::FooRequestStats> const* rs_std::Vec<
    ::proto::Rust<::foo_service::FooRequestStats>>::end() const noexcept {
  return data() + size();
}
#endif

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_PROTOBUF_RUST_LIB_GOLDEN
