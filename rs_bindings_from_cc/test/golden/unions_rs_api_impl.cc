// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:unions_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/unions.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(union EmptyUnion) == 1);
static_assert(alignof(union EmptyUnion) == 1);

static_assert(CRUBIT_SIZEOF(struct Nontrivial) == 4);
static_assert(alignof(struct Nontrivial) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct Nontrivial) == 0);

static_assert(sizeof(union UnionToRename) == 1);
static_assert(alignof(union UnionToRename) == 1);

static_assert(sizeof(struct TriviallyCopyableButNontriviallyDestructible) == 1);
static_assert(alignof(struct TriviallyCopyableButNontriviallyDestructible) ==
              1);

extern "C" void
__rust_thunk___ZN44TriviallyCopyableButNontriviallyDestructibleD1Ev(
    struct TriviallyCopyableButNontriviallyDestructible* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(union NonEmptyUnion) == 8);
static_assert(alignof(union NonEmptyUnion) == 8);
static_assert(CRUBIT_OFFSET_OF(bool_field, union NonEmptyUnion) == 0);
static_assert(CRUBIT_OFFSET_OF(char_field, union NonEmptyUnion) == 0);
static_assert(CRUBIT_OFFSET_OF(int_field, union NonEmptyUnion) == 0);
static_assert(CRUBIT_OFFSET_OF(long_long_field, union NonEmptyUnion) == 0);

static_assert(CRUBIT_SIZEOF(union NonCopyUnion) == 4);
static_assert(alignof(union NonCopyUnion) == 4);
static_assert(CRUBIT_OFFSET_OF(trivial_member, union NonCopyUnion) == 0);
static_assert(CRUBIT_OFFSET_OF(nontrivial_member, union NonCopyUnion) == 0);

static_assert(sizeof(union NonCopyUnion2) == 1);
static_assert(alignof(union NonCopyUnion2) == 1);
static_assert(CRUBIT_OFFSET_OF(trivial_member, union NonCopyUnion2) == 0);
static_assert(CRUBIT_OFFSET_OF(nontrivial_member, union NonCopyUnion2) == 0);

static_assert(sizeof(union UnionWithOpaqueField) == 42);
static_assert(alignof(union UnionWithOpaqueField) == 1);
static_assert(CRUBIT_OFFSET_OF(constant_array_field_not_yet_supported,
                               union UnionWithOpaqueField) == 0);

static_assert(CRUBIT_SIZEOF(struct TrivialButInheritable) == 4);
static_assert(alignof(struct TrivialButInheritable) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct TrivialButInheritable) == 0);

static_assert(CRUBIT_SIZEOF(union UnionWithInheritable) == 4);
static_assert(alignof(union UnionWithInheritable) == 4);
static_assert(CRUBIT_OFFSET_OF(t, union UnionWithInheritable) == 0);

static_assert(sizeof(TypedefUnion) == 1);
static_assert(alignof(TypedefUnion) == 1);
static_assert(CRUBIT_OFFSET_OF(trivial_member, TypedefUnion) == 0);

static_assert(CRUBIT_SIZEOF(TypedefUnionWithInheritable) == 4);
static_assert(alignof(TypedefUnionWithInheritable) == 4);
static_assert(CRUBIT_OFFSET_OF(t, TypedefUnionWithInheritable) == 0);

#pragma clang diagnostic pop
