// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/global:global
// Features: supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/global/global.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z6Unusedi(int arg) { Unused(arg); }

static_assert((void (*)(int)) & ::Unused);

static_assert((int (*)()) & ::GetIntVal);

static_assert((int (*)()) & ::GetNamespacedIntVal);

static_assert((int (*)()) & ::GetCNamespacedIntVal);

static_assert((int (*)()) & ::GetInlineIntVal);

#pragma clang diagnostic pop
