// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unsafe_types_transitive_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/unsafe_types_transitive.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct PublicPointer) == 8);
static_assert(alignof(struct PublicPointer) == 8);
static_assert(CRUBIT_OFFSET_OF(p, struct PublicPointer) == 0);

static_assert(CRUBIT_SIZEOF(class PrivatePointer) == 8);
static_assert(alignof(class PrivatePointer) == 8);

extern "C" int __rust_thunk___Z19DerefPrivatePointer14PrivatePointer(
    class PrivatePointer* p) {
  return DerefPrivatePointer(std::move(*p));
}

static_assert(CRUBIT_SIZEOF(struct TransitivePublicPointer) == 16);
static_assert(alignof(struct TransitivePublicPointer) == 8);
static_assert(CRUBIT_OFFSET_OF(pub, struct TransitivePublicPointer) == 0);
static_assert(CRUBIT_OFFSET_OF(priv, struct TransitivePublicPointer) == 8);

static_assert(CRUBIT_SIZEOF(union Union) == 4);
static_assert(alignof(union Union) == 4);
static_assert(CRUBIT_OFFSET_OF(i, union Union) == 0);
static_assert(CRUBIT_OFFSET_OF(f, union Union) == 0);

extern "C" int __rust_thunk___Z18DerefPublicPointer13PublicPointer(
    struct PublicPointer* p) {
  return DerefPublicPointer(std::move(*p));
}

extern "C" int
__rust_thunk___Z28DerefTransitivePublicPointer23TransitivePublicPointer(
    struct TransitivePublicPointer* p) {
  return DerefTransitivePublicPointer(std::move(*p));
}

extern "C" int __rust_thunk___Z9ReadUnion5Union(union Union* u) {
  return ReadUnion(std::move(*u));
}

#pragma clang diagnostic pop
