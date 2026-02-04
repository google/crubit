// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/assume_lifetimes:free_function
// Features: assume_lifetimes, custom_ffi_types, non_unpin_ctor, std_unique_ptr, std_vector, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/assume_lifetimes/free_function.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert((int& (*)(int&)) & ::increment_int_ref);

#pragma clang diagnostic pop
