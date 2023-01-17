// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_unsupported_cc

#include <cstddef>
#include <memory>

#include "support/cxx20_backports.h"
#include "support/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/user_of_unsupported.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___Z23UseNontrivialCustomType20NontrivialCustomType(
    struct NontrivialCustomType* non_trivial_custom_type) {
  UseNontrivialCustomType(std::move(*non_trivial_custom_type));
}

#pragma clang diagnostic pop
