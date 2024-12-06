// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsafe_attrs_cc
// Features: experimental, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/unsafe_attrs.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" bool __rust_thunk___ZL11ReturnsTruev() { return ReturnsTrue(); }

extern "C" bool __rust_thunk___ZL12ReturnsFalsev() { return ReturnsFalse(); }

#pragma clang diagnostic pop
