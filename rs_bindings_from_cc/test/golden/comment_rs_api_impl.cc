// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:comment_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/comment.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct Foo) == 8);
static_assert(alignof(struct Foo) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct Foo) == 0);
static_assert(CRUBIT_OFFSET_OF(j, struct Foo) == 4);

extern "C" void __rust_thunk___Z3foov() { foo(); }

static_assert((void (*)())&foo);

static_assert(CRUBIT_SIZEOF(struct Bar) == 4);
static_assert(alignof(struct Bar) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct Bar) == 0);

static_assert(CRUBIT_SIZEOF(struct HasNoComments) == 4);
static_assert(alignof(struct HasNoComments) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct HasNoComments) == 0);

#pragma clang diagnostic pop
