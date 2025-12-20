// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:overloads_unsupported_template_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/overloads_unsupported_template.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z8Overloadv() { Overload(); }

static_assert((void (*)()) & ::Overload);

#pragma clang diagnostic pop
