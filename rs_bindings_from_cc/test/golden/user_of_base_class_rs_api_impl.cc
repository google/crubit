// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/user_of_base_class.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct Derived2) == 24);
static_assert(alignof(struct Derived2) == 8);
static_assert(CRUBIT_OFFSET_OF(derived_1, struct Derived2) == 20);

static_assert(CRUBIT_SIZEOF(class VirtualDerived2) == 32);
static_assert(alignof(class VirtualDerived2) == 8);

#pragma clang diagnostic pop
