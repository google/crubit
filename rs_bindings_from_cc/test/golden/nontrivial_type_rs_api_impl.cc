// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/nontrivial_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct Nontrivial) == 4);
static_assert(alignof(struct Nontrivial) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct Nontrivial) == 0);

static_assert(CRUBIT_SIZEOF(struct NontrivialInline) == 4);
static_assert(alignof(struct NontrivialInline) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct NontrivialInline) == 0);

extern "C" void __rust_thunk___ZN16NontrivialInlineD1Ev(
    struct NontrivialInline* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct NontrivialMembers) == 4);
static_assert(alignof(struct NontrivialMembers) == 4);
static_assert(CRUBIT_OFFSET_OF(nontrivial_member, struct NontrivialMembers) ==
              0);

extern "C" void __rust_thunk___ZN17NontrivialMembersD1Ev(
    struct NontrivialMembers* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct NontrivialUnpin) == 4);
static_assert(alignof(struct NontrivialUnpin) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct NontrivialUnpin) == 0);

extern "C" void __rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
    struct NontrivialUnpin* __return, struct NontrivialUnpin* nontrivial) {
  new (__return) auto(TakesByValueUnpin(std::move(*nontrivial)));
}

static_assert(
    (struct NontrivialUnpin (*)(struct NontrivialUnpin))&TakesByValueUnpin);

static_assert(sizeof(struct NontrivialByValue) == 1);
static_assert(alignof(struct NontrivialByValue) == 1);

static_assert(sizeof(struct Nonmovable) == 1);
static_assert(alignof(struct Nonmovable) == 1);

#pragma clang diagnostic pop
