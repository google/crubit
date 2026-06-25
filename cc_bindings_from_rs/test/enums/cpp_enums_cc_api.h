// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// cpp_enums_golden

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_CPP_ENUMS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_CPP_ENUMS_GOLDEN

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#pragma clang diagnostic ignored "-Wunused-private-field"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#pragma clang diagnostic ignored "-Wignored-attributes"
#include "support/annotations_internal.h"

#include <cstdint>

namespace cpp_enums::forward_declared_enum {
enum class B : ::std::uint8_t;
}

namespace cpp_enums::classless_enum {

// CRUBIT_ANNOTATE: cpp_enum=enum
enum CRUBIT_INTERNAL_RUST_TYPE(
    ":: cpp_enums_golden :: classless_enum :: Color") Color : ::std::int32_t {
  RED = INT32_C(0),
  BLUE = INT32_C(2),
};

}  // namespace cpp_enums::classless_enum

namespace cpp_enums::cpp_enum {

// CRUBIT_ANNOTATE: cpp_enum=enum class
enum class CRUBIT_INTERNAL_RUST_TYPE(
    ":: cpp_enums_golden :: cpp_enum :: Color") Color : ::std::int32_t {
  RED = INT32_C(0),
  BLUE = INT32_C(2),
};

}  // namespace cpp_enums::cpp_enum

namespace cpp_enums::deprecated_enum {

// CRUBIT_ANNOTATE: cpp_enum=enum class
enum class CRUBIT_INTERNAL_RUST_TYPE(
    ":: cpp_enums_golden :: deprecated_enum :: Color")
    [[nodiscard]] [[deprecated("Use NewColor")]] Color : ::std::int32_t{
        RED = INT32_C(0),
        BLUE = INT32_C(2),
    };

}  // namespace cpp_enums::deprecated_enum

namespace cpp_enums::forward_declared_enum {

::cpp_enums::forward_declared_enum::B AFunction();

// CRUBIT_ANNOTATE: cpp_enum=enum class
enum class CRUBIT_INTERNAL_RUST_TYPE(
    ":: cpp_enums_golden :: forward_declared_enum :: B") B : ::std::uint8_t {
  ONE = 1,
  TWO = 2,
};

}  // namespace cpp_enums::forward_declared_enum

namespace cpp_enums::forward_declared_enum {

namespace __crubit_internal {
/// \cond CRUBIT_INTERNAL
extern "C" ::cpp_enums::forward_declared_enum::B __crubit_thunk_AFunction();
/// \endcond
}  // namespace __crubit_internal
inline ::cpp_enums::forward_declared_enum::B AFunction() {
  return __crubit_internal::__crubit_thunk_AFunction();
}

}  // namespace cpp_enums::forward_declared_enum

#pragma clang diagnostic pop
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_CPP_ENUMS_GOLDEN
