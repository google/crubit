// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:typedefs_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/typedefs.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct SomeStruct) == 1);
static_assert(alignof(struct SomeStruct) == 1);

static_assert(sizeof(SomeOtherStruct) == 1);
static_assert(alignof(SomeOtherStruct) == 1);

static_assert(sizeof(union SomeUnion) == 1);
static_assert(alignof(union SomeUnion) == 1);

static_assert(sizeof(SomeOtherUnion) == 1);
static_assert(alignof(SomeOtherUnion) == 1);

#pragma clang diagnostic pop
