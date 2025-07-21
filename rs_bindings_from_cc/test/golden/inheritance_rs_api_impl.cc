// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:inheritance_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/inheritance.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(class Base0) == 1);
static_assert(alignof(class Base0) == 1);

static_assert(CRUBIT_SIZEOF(class Base1) == 16);
static_assert(alignof(class Base1) == 8);

static_assert(CRUBIT_SIZEOF(class Base2) == 2);
static_assert(alignof(class Base2) == 2);

static_assert(CRUBIT_SIZEOF(struct Derived) == 16);
static_assert(alignof(struct Derived) == 8);
static_assert(CRUBIT_OFFSET_OF(derived_1, struct Derived) == 12);

static_assert(CRUBIT_SIZEOF(class VirtualBase1) == 24);
static_assert(alignof(class VirtualBase1) == 8);

static_assert(CRUBIT_SIZEOF(class VirtualBase2) == 24);
static_assert(alignof(class VirtualBase2) == 8);

static_assert(CRUBIT_SIZEOF(class VirtualDerived) == 32);
static_assert(alignof(class VirtualDerived) == 8);

static_assert(CRUBIT_SIZEOF(class MyAbstractClass) == 8);
static_assert(alignof(class MyAbstractClass) == 8);

static_assert(sizeof(class MethodBase1) == 1);
static_assert(alignof(class MethodBase1) == 1);

static_assert(sizeof(class MethodBase2) == 1);
static_assert(alignof(class MethodBase2) == 1);

static_assert(sizeof(class MethodDerived) == 1);
static_assert(alignof(class MethodDerived) == 1);

#pragma clang diagnostic pop
