// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// type_aliases_rust_golden
// Features: custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr,
// std_vector, supported, wrapper

// clang-format off
#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_TYPE_ALIASES_RUST_GOLDEN
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_TYPE_ALIASES_RUST_GOLDEN

#include "support/annotations_internal.h"

#include <cstdint>

namespace type_aliases_rust {

namespace test_type_aliases {
using TypeAlias CRUBIT_INTERNAL_RUST_TYPE(
    ":: type_aliases_rust_golden :: test_type_aliases :: TypeAlias") =
    std::int32_t;
}

namespace test_deprecated_type_alias {
using TypeAlias CRUBIT_INTERNAL_RUST_TYPE(
    ":: type_aliases_rust_golden :: test_deprecated_type_alias :: TypeAlias")
    [[deprecated("Use `OtherTypeAlias` instead")]] = std::int32_t;
}

namespace test_type_aliases {

// Generated from:
// cc_bindings_from_rs/test/golden/type_aliases.rs;l=9
std::int32_t func_using_alias();
using TypeAlias2 CRUBIT_INTERNAL_RUST_TYPE(
    ":: type_aliases_rust_golden :: test_type_aliases :: TypeAlias2") =
    std::int32_t;
}  // namespace test_type_aliases

namespace test_deprecated_type_alias {}

namespace test_type_aliases {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_func_uusing_ualias();
}
inline std::int32_t func_using_alias() {
  return __crubit_internal::__crubit_thunk_func_uusing_ualias();
}

}  // namespace test_type_aliases

}  // namespace type_aliases_rust
#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_GOLDEN_TYPE_ALIASES_RUST_GOLDEN
