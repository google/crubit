// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/test/golden/trivial_type.h"

extern "C" void __rust_destructor_thunk__Trivial(Trivial* __this) {
  return std ::destroy_at(__this);
}
extern "C" void __rust_destructor_thunk__TrivialWithDefaulted(
    TrivialWithDefaulted* __this) {
  return std ::destroy_at(__this);
}

static_assert(sizeof(Trivial) == 4);
static_assert(alignof(Trivial) == 4);
static_assert(offsetof(Trivial, trivial_field) * 8 == 0);

static_assert(sizeof(TrivialWithDefaulted) == 4);
static_assert(alignof(TrivialWithDefaulted) == 4);
static_assert(offsetof(TrivialWithDefaulted, trivial_field) * 8 == 0);
