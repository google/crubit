// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:noescape_pointers_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/noescape_pointers.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"

static_assert((void (*)(int*)) & ::takes_noescape_nonnull);

static_assert((void (*)(int*)) & ::takes_noescape_nullable);

static_assert((void (*)(int*)) & ::takes_nonnull_only);

static_assert((void (*)(void*)) & ::takes_noescape_void);

static_assert((void (*)(Incomplete*)) & ::takes_noescape_incomplete);

static_assert((void (*)(int**)) & ::takes_noescape_pointer_to_pointer);

static_assert((void (*)(int const*)) & ::takes_noescape_const_pointee);

#pragma clang diagnostic pop
