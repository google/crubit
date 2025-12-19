// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsafe_attrs_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/unsafe_attrs.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" bool __rust_thunk___ZL11ReturnsTruev() { return ReturnsTrue(); }

static_assert((bool (*)()) & ::ReturnsTrue);

extern "C" bool __rust_thunk___ZL12ReturnsFalsev() { return ReturnsFalse(); }

static_assert((bool (*)()) & ::ReturnsFalse);

static_assert((void (*)()) & ::TotallySafe);

static_assert((void (*)(void*)) & ::TotallyUnsafe);

static_assert((void (*)()) & ::SafeSignatureButAnnotatedUnsafe);

static_assert((void (*)()) & ::SafeSignatureButAnnotatedSafe);

static_assert((void (*)(void*)) & ::UnsafeSignatureButAnnotatedUnsafe);

static_assert((void (*)(void*)) & ::UnsafeSignatureButAnnotatedSafe);

#pragma clang diagnostic pop
