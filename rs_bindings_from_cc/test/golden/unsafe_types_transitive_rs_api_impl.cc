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

extern "C" void __rust_thunk___ZN13PublicPointerC1Ev(
    struct PublicPointer* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN13PublicPointerC1EOS_(
    struct PublicPointer* __this, struct PublicPointer* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct PublicPointer* __rust_thunk___ZN13PublicPointeraSERKS_(
    struct PublicPointer* __this, const struct PublicPointer* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct PublicPointer* __rust_thunk___ZN13PublicPointeraSEOS_(
    struct PublicPointer* __this, struct PublicPointer* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(class PrivatePointer) == 8);
static_assert(alignof(class PrivatePointer) == 8);

extern "C" void __rust_thunk___ZN14PrivatePointerC1Ev(
    class PrivatePointer* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN14PrivatePointerC1EOS_(
    class PrivatePointer* __this, class PrivatePointer* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class PrivatePointer* __rust_thunk___ZN14PrivatePointeraSERKS_(
    class PrivatePointer* __this, const class PrivatePointer* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" class PrivatePointer* __rust_thunk___ZN14PrivatePointeraSEOS_(
    class PrivatePointer* __this, class PrivatePointer* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" int __rust_thunk___Z19DerefPrivatePointer14PrivatePointer(
    class PrivatePointer* p) {
  return DerefPrivatePointer(std::move(*p));
}

static_assert(CRUBIT_SIZEOF(struct TransitivePublicPointer) == 16);
static_assert(alignof(struct TransitivePublicPointer) == 8);
static_assert(CRUBIT_OFFSET_OF(pub, struct TransitivePublicPointer) == 0);
static_assert(CRUBIT_OFFSET_OF(priv, struct TransitivePublicPointer) == 8);

extern "C" void __rust_thunk___ZN23TransitivePublicPointerC1Ev(
    struct TransitivePublicPointer* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN23TransitivePublicPointerC1EOS_(
    struct TransitivePublicPointer* __this,
    struct TransitivePublicPointer* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct TransitivePublicPointer*
__rust_thunk___ZN23TransitivePublicPointeraSERKS_(
    struct TransitivePublicPointer* __this,
    const struct TransitivePublicPointer* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct TransitivePublicPointer*
__rust_thunk___ZN23TransitivePublicPointeraSEOS_(
    struct TransitivePublicPointer* __this,
    struct TransitivePublicPointer* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(union Union) == 4);
static_assert(alignof(union Union) == 4);
static_assert(CRUBIT_OFFSET_OF(i, union Union) == 0);
static_assert(CRUBIT_OFFSET_OF(f, union Union) == 0);

extern "C" void __rust_thunk___ZN5UnionC1Ev(union Union* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN5UnionC1EOS_(union Union* __this,
                                              union Union* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" union Union* __rust_thunk___ZN5UnionaSERKS_(
    union Union* __this, const union Union* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" union Union* __rust_thunk___ZN5UnionaSEOS_(union Union* __this,
                                                      union Union* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

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
