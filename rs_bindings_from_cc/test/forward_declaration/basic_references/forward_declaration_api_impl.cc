// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/forward_declaration/basic_references:forward_declaration

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/forward_declaration/basic_references/forward_declaration.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert((::A & (*)()) & ::fwd_source);

static_assert((::A & (*)(::A&)) & ::fwd_ident);

static_assert((::A const& (*)(::A const&)) & ::fwd_ident_const);

#pragma clang diagnostic pop
