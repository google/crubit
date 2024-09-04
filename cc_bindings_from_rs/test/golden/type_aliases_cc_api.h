// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated C++ bindings for the following Rust crate:
// type_aliases_rust

// clang-format off
#pragma once

#include <cstdint>

namespace type_aliases_rust {

namespace test_type_aliases {
using TypeAlias2 = std::int32_t;
using TypeAlias = std::int32_t;

// Generated from:
// cc_bindings_from_rs/test/golden/type_aliases.rs;l=9
std::int32_t func_using_alias();

}  // namespace test_type_aliases

namespace test_deprecated_type_alias {
using TypeAlias [[deprecated("Use `OtherTypeAlias` instead")]] = std::int32_t;
}

namespace test_type_aliases {

namespace __crubit_internal {
extern "C" std::int32_t __crubit_thunk_func_uusing_ualias();
}
inline std::int32_t func_using_alias() {
  return __crubit_internal::__crubit_thunk_func_uusing_ualias();
}

}  // namespace test_type_aliases

namespace test_deprecated_type_alias {}

}  // namespace type_aliases_rust
