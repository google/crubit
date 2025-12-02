// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/annotations:owned_ptr_user
// Features: non_unpin_ctor, std_unique_ptr, std_vector, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/annotations/owned_ptr_user.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert((struct Thing * (*)(int)) & MakeOwnedThing);

static_assert((struct Thing * (*)(int)) & MakeThing);

static_assert((int (*)(struct Thing*))&ThingToValue);

static_assert((int (*)(struct Thing*))&GetThingValue);

#pragma clang diagnostic pop
