// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// trait_golden
// Features: callables, supported, types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_FOR_CC_TYPE_TRAIT_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_FOR_CC_TYPE_TRAIT_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/rs_std/traits.h"

#include <cstdint>

#include "cc_bindings_from_rs/test/traits/for_cc_type/cc_type.h"

namespace trait {

// Generated from:
// cc_bindings_from_rs/test/traits/for_cc_type/trait.rs;l=5
struct CRUBIT_INTERNAL_RUST_TYPE(":: trait_golden :: Trait") Trait {
  template <typename T>
  using impl = rs_std::impl<T, Trait>;
};

}  // namespace trait

template <>
struct rs_std::impl<::CcType, ::trait::Trait> {
  static constexpr bool kIsImplemented = true;

  // Generated from:
  // cc_bindings_from_rs/test/traits/for_cc_type/trait.rs;l=10
  static ::std::int32_t get_value(::CcType const& self);
};

namespace trait {
namespace __crubit_internal {
extern "C" ::std::int32_t __crubit_thunk_Trait_uget_uvalue(::CcType const&);
}
}  // namespace trait
inline ::std::int32_t rs_std::impl<::CcType, ::trait::Trait>::get_value(
    ::CcType const& self) {
  return trait::__crubit_internal::__crubit_thunk_Trait_uget_uvalue(self);
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_TRAITS_FOR_CC_TYPE_TRAIT_GOLDEN
