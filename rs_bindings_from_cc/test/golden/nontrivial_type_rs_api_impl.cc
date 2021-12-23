// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/nontrivial_type.h"

extern "C" void __rust_thunk___ZN16NontrivialInlineD1Ev(
    NontrivialInline* __this) {
  std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN17NontrivialMembersD1Ev(
    NontrivialMembers* __this) {
  std ::destroy_at(__this);
}

static_assert(sizeof(Nontrivial) == 4);
static_assert(alignof(Nontrivial) == 4);
static_assert(offsetof(Nontrivial, field) * 8 == 0);

static_assert(sizeof(NontrivialInline) == 4);
static_assert(alignof(NontrivialInline) == 4);
static_assert(offsetof(NontrivialInline, field) * 8 == 0);

static_assert(sizeof(NontrivialMembers) == 4);
static_assert(alignof(NontrivialMembers) == 4);
static_assert(offsetof(NontrivialMembers, nontrivial_member) * 8 == 0);
