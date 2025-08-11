// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:lifetimes_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/lifetimes.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert((void (*)(crubit::type_identity_t<void()>*))&AddHook);

static_assert((void (*)(crubit::type_identity_t<void()>*))&AddHookWithTypedef);

static_assert((void (*)(crubit::type_identity_t<void()>&))&AddAnotherHook);

static_assert(
    (void (*)(crubit::type_identity_t<void()>&))&AddAnotherHookWithTypedef);

static_assert((void (*)(int*))&ConsumeArray);

static_assert((void (*)(int*))&ConsumeArrayWithTypedef);

#pragma clang diagnostic pop
