// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:bitfields_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/bitfields.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct WithBitfields) == 32);
static_assert(alignof(struct WithBitfields) == 4);
static_assert(CRUBIT_OFFSET_OF(f2, struct WithBitfields) == 4);
static_assert(CRUBIT_OFFSET_OF(f5, struct WithBitfields) == 20);
static_assert(CRUBIT_OFFSET_OF(f7, struct WithBitfields) == 27);

static_assert(CRUBIT_SIZEOF(struct AlignmentRegressionTest) == 4);
static_assert(alignof(struct AlignmentRegressionTest) == 4);

#pragma clang diagnostic pop
