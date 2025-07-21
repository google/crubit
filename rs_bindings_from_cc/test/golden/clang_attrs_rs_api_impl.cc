// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:clang_attrs_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/clang_attrs.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct HasCustomAlignment) == 64);
static_assert(alignof(struct HasCustomAlignment) == 64);

static_assert(CRUBIT_SIZEOF(struct HasFieldWithCustomAlignment) == 64);
static_assert(alignof(struct HasFieldWithCustomAlignment) == 64);
static_assert(CRUBIT_OFFSET_OF(field, struct HasFieldWithCustomAlignment) == 0);

static_assert(CRUBIT_SIZEOF(struct InheritsFromBaseWithCustomAlignment) == 64);
static_assert(alignof(struct InheritsFromBaseWithCustomAlignment) == 64);

static_assert(CRUBIT_SIZEOF(struct HasCustomAlignmentWithGnuAttr) == 64);
static_assert(alignof(struct HasCustomAlignmentWithGnuAttr) == 64);

#pragma clang diagnostic pop
