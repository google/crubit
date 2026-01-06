// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// cpp_enums_golden
// Features: custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector,
// supported

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_CPP_ENUMS_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_CPP_ENUMS_GOLDEN

#include "support/annotations_internal.h"

#include <cstdint>

namespace cpp_enums {

namespace forward_declared_enum {
enum class B : std::uint8_t;
}

namespace classless_enum {

// CRUBIT_ANNOTATE: cpp_enum=enum
//
// Generated from:
// cc_bindings_from_rs/test/enums/cpp_enums.rs;l=11
enum CRUBIT_INTERNAL_RUST_TYPE(
    ":: cpp_enums_golden :: classless_enum :: Color") Color : std::int32_t {
  RED = INT32_C(0),
  BLUE = INT32_C(2),
};

}  // namespace classless_enum

namespace cpp_enum {

// CRUBIT_ANNOTATE: cpp_enum=enum class
//
// Generated from:
// cc_bindings_from_rs/test/enums/cpp_enums.rs;l=22
enum class CRUBIT_INTERNAL_RUST_TYPE(
    ":: cpp_enums_golden :: cpp_enum :: Color") Color : std::int32_t {
  RED = INT32_C(0),
  BLUE = INT32_C(2),
};

}  // namespace cpp_enum

namespace deprecated_enum {

// CRUBIT_ANNOTATE: cpp_enum=enum class
//
// Generated from:
// cc_bindings_from_rs/test/enums/cpp_enums.rs;l=38
enum class CRUBIT_INTERNAL_RUST_TYPE(
    ":: cpp_enums_golden :: deprecated_enum :: Color")
    [[nodiscard]] [[deprecated("Use NewColor")]] Color : std::int32_t{
        RED = INT32_C(0),
        BLUE = INT32_C(2),
    };

}  // namespace deprecated_enum

namespace forward_declared_enum {

// Generated from:
// cc_bindings_from_rs/test/enums/cpp_enums.rs;l=50
::cpp_enums::forward_declared_enum::B AFunction();

// CRUBIT_ANNOTATE: cpp_enum=enum class
//
// Generated from:
// cc_bindings_from_rs/test/enums/cpp_enums.rs;l=56
enum class CRUBIT_INTERNAL_RUST_TYPE(
    ":: cpp_enums_golden :: forward_declared_enum :: B") B : std::uint8_t {
  ONE = 1,
  TWO = 2,
};

}  // namespace forward_declared_enum

namespace classless_enum {}

namespace cpp_enum {}

namespace deprecated_enum {}

namespace forward_declared_enum {

namespace __crubit_internal {
extern "C" ::cpp_enums::forward_declared_enum::B __crubit_thunk_AFunction();
}
inline ::cpp_enums::forward_declared_enum::B AFunction() {
  return __crubit_internal::__crubit_thunk_AFunction();
}

}  // namespace forward_declared_enum

}  // namespace cpp_enums
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_ENUMS_CPP_ENUMS_GOLDEN
