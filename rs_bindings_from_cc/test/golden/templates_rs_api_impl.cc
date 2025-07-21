// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:templates_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/templates.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct DifferentScope) == 1);
static_assert(alignof(struct DifferentScope) == 1);

static_assert(sizeof(struct test_namespace_bindings::TemplateParam) == 1);
static_assert(alignof(struct test_namespace_bindings::TemplateParam) == 1);

static_assert(sizeof(class private_classes::HasPrivateType) == 1);
static_assert(alignof(class private_classes::HasPrivateType) == 1);

#pragma clang diagnostic pop
