// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// example_crate_golden
// Features: callables, supported, types

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_EXAMPLES_RUST_CPP_ENUM_EXAMPLE_CRATE_GOLDEN
#define THIRD_PARTY_CRUBIT_EXAMPLES_RUST_CPP_ENUM_EXAMPLE_CRATE_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"
#include "support/rs_std/traits.h"

#include <cstdint>

#include "support/rs_std/rs_core.h"

namespace example_crate {

// CRUBIT_ANNOTATE: cpp_enum=enum class
//
// Generated from:
// examples/rust/cpp_enum/example.rs;l=13
enum class CRUBIT_INTERNAL_RUST_TYPE(
    ":: example_crate_golden :: Color") Color : ::std::int32_t {
  Red = INT32_C(0),
  Blue = INT32_C(1),
  Green = INT32_C(5),
  Gray = INT32_C(5),
  Magenta = INT32_C(7),
};

}  // namespace example_crate

template <>
struct rs_std::impl<::example_crate::Color,
                    ::rs::core::cmp::PartialEq<::example_crate::Color>> {
  static constexpr bool kIsImplemented = true;

  // Generated from:
  // examples/rust/cpp_enum/example.rs;l=13
  static bool eq(::example_crate::Color const& self,
                 ::example_crate::Color const& other);
};

namespace example_crate {
namespace __crubit_internal {
extern "C" bool __crubit_thunk_PartialEq_ueq(::example_crate::Color const&,
                                             ::example_crate::Color const&);
}
}  // namespace example_crate
inline bool rs_std::impl<::example_crate::Color,
                         ::rs::core::cmp::PartialEq<::example_crate::Color>>::
    eq(::example_crate::Color const& self,
       ::example_crate::Color const& other) {
  return example_crate::__crubit_internal::__crubit_thunk_PartialEq_ueq(self,
                                                                        other);
}

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_EXAMPLES_RUST_CPP_ENUM_EXAMPLE_CRATE_GOLDEN
