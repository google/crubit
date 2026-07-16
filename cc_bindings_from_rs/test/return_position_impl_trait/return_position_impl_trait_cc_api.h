// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// return_position_impl_trait_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_RETURN_POSITION_IMPL_TRAIT_RETURN_POSITION_IMPL_TRAIT_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_RETURN_POSITION_IMPL_TRAIT_RETURN_POSITION_IMPL_TRAIT_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/internal/memswap.h"
#include "support/internal/slot.h"

#include <array>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <utility>

namespace return_position_impl_trait {

struct CRUBIT_INTERNAL_RUST_TYPE(
    ":: return_position_impl_trait_golden :: ArcWrapper") alignas(8)
    [[clang::trivial_abi]] ArcWrapper final {
 public:
  // Default::default
  ArcWrapper();

  // Drop::drop
  ~ArcWrapper();

  ArcWrapper(ArcWrapper&&);
  ::return_position_impl_trait::ArcWrapper& operator=(ArcWrapper&&);

  // Clone::clone
  ArcWrapper(const ArcWrapper&);

  // Clone::clone_from
  ::return_position_impl_trait::ArcWrapper& operator=(const ArcWrapper&);

  ArcWrapper(::crubit::UnsafeRelocateTag, ArcWrapper&& value);

  ::std::uintptr_t refcount() const;

 private:
  // Field type has been replaced with a blob of bytes: Generic types are not
  // supported yet (b/259749095)
  ::std::array<unsigned char, 8> arc;

 private:
  static void __crubit_field_offset_assertions();
};

// Error generating bindings for function
// `return_position_impl_trait_golden::return_impl_drop` defined at
// cc_bindings_from_rs/test/return_position_impl_trait/return_position_impl_trait.rs;l=25:
// Attempted to write out unknown type from Rust to C

// Error generating bindings for function
// `return_position_impl_trait_golden::return_impl_future_trivial` defined at
// cc_bindings_from_rs/test/return_position_impl_trait/return_position_impl_trait.rs;l=40:
// Error formatting function return type `impl std::future::Future<Output = i32>
// + 'static`: Generic types are not supported yet (b/259749095)

// Error generating bindings for function
// `return_position_impl_trait_golden::return_impl_future_with_drop` defined at
// cc_bindings_from_rs/test/return_position_impl_trait/return_position_impl_trait.rs;l=30:
// Error formatting function return type `impl std::future::Future<Output = ()>
// + 'static`: The following Rust type is not supported yet: {async
// block@return_position_impl_trait_golden::return_impl_future_with_drop::{closure#0}}

// Error generating bindings for function
// `return_position_impl_trait_golden::return_impl_iterator_trivial` defined at
// cc_bindings_from_rs/test/return_position_impl_trait/return_position_impl_trait.rs;l=55:
// Error formatting function return type `impl std::iter::Iterator<Item = i32> +
// 'static`: Generic types are not supported yet (b/259749095)

// Error generating bindings for function
// `return_position_impl_trait_golden::return_impl_iterator_with_drop` defined
// at
// cc_bindings_from_rs/test/return_position_impl_trait/return_position_impl_trait.rs;l=46:
// Error formatting function return type `impl std::iter::Iterator<Item = ()> +
// 'static`: Generic types are not supported yet (b/259749095)

static_assert(
    sizeof(ArcWrapper) == 8,
    "Verify that ADT layout didn't change since this header got generated");
static_assert(
    alignof(ArcWrapper) == 8,
    "Verify that ADT layout didn't change since this header got generated");
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Default_udefault_ureturn_uposition_uimpl_utrait_ugolden_x0000003a_x0000003aArcWrapper(
    ::return_position_impl_trait::ArcWrapper* __ret_ptr);
}
inline ::return_position_impl_trait::ArcWrapper::ArcWrapper() {
  __crubit_internal::
      __crubit_thunk_Default_udefault_ureturn_uposition_uimpl_utrait_ugolden_x0000003a_x0000003aArcWrapper(
          this);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Drop_udrop_ureturn_uposition_uimpl_utrait_ugolden_x0000003a_x0000003aArcWrapper(
    ::return_position_impl_trait::ArcWrapper&);
}
inline ArcWrapper::~ArcWrapper() {
  __crubit_internal::
      __crubit_thunk_Drop_udrop_ureturn_uposition_uimpl_utrait_ugolden_x0000003a_x0000003aArcWrapper(
          *this);
}
inline ::return_position_impl_trait::ArcWrapper::ArcWrapper(ArcWrapper&& other)
    : ArcWrapper() {
  *this = ::std::move(other);
}
inline ::return_position_impl_trait::ArcWrapper& ::return_position_impl_trait::
    ArcWrapper::operator=(ArcWrapper&& other) {
  crubit::MemSwap(*this, other);
  return *this;
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ureturn_uposition_uimpl_utrait_ugolden_x0000003a_x0000003aArcWrapper(
    ::return_position_impl_trait::ArcWrapper const&,
    ::return_position_impl_trait::ArcWrapper* __ret_ptr);
}
namespace __crubit_internal {
extern "C" void
__crubit_thunk_Clone_uclone_ufrom_ureturn_uposition_uimpl_utrait_ugolden_x0000003a_x0000003aArcWrapper(
    ::return_position_impl_trait::ArcWrapper&,
    ::return_position_impl_trait::ArcWrapper const&);
}
inline ::return_position_impl_trait::ArcWrapper::ArcWrapper(
    const ArcWrapper& other) {
  __crubit_internal::
      __crubit_thunk_Clone_uclone_ureturn_uposition_uimpl_utrait_ugolden_x0000003a_x0000003aArcWrapper(
          other, this);
}
inline ::return_position_impl_trait::ArcWrapper& ::return_position_impl_trait::
    ArcWrapper::operator=(const ArcWrapper& other) {
  if (this != &other) {
    __crubit_internal::
        __crubit_thunk_Clone_uclone_ufrom_ureturn_uposition_uimpl_utrait_ugolden_x0000003a_x0000003aArcWrapper(
            *this, other);
  }
  return *this;
}
inline ::return_position_impl_trait::ArcWrapper::ArcWrapper(
    ::crubit::UnsafeRelocateTag, ArcWrapper&& value) {
  ::std::memcpy(this, &value, sizeof(value));
}

namespace __crubit_internal {
extern "C" ::std::uintptr_t __crubit_thunk_refcount(
    ::return_position_impl_trait::ArcWrapper const&);
}
inline ::std::uintptr_t ArcWrapper::refcount() const {
  auto&& self = *this;
  return __crubit_internal::__crubit_thunk_refcount(self);
}
inline void ArcWrapper::__crubit_field_offset_assertions() {
  static_assert(0 == offsetof(ArcWrapper, arc));
}
}  // namespace return_position_impl_trait

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_RETURN_POSITION_IMPL_TRAIT_RETURN_POSITION_IMPL_TRAIT_GOLDEN
