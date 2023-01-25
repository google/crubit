// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:source_location_doc_comments_cc

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/source_location_doc_comments.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct StructFromMacro) == 4);
static_assert(alignof(struct StructFromMacro) == 4);
static_assert(CRUBIT_OFFSET_OF(val, struct StructFromMacro) == 0);

static_assert(sizeof(struct SomeStruct) == 4);
static_assert(alignof(struct SomeStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(some_field, struct SomeStruct) == 0);

static_assert(sizeof(struct SomeStruct3) == 1);
static_assert(alignof(struct SomeStruct3) == 1);

#pragma clang diagnostic pop
