// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nested_types_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/nested_types.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct Foo) == 4);
static_assert(alignof(struct Foo) == 4);
static_assert(CRUBIT_OFFSET_OF(foo, struct Foo) == 0);

extern "C" void __rust_thunk___ZN3FooC1Ev(struct Foo* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct Foo::Bar) == 4);
static_assert(alignof(struct Foo::Bar) == 4);
static_assert(CRUBIT_OFFSET_OF(bar, struct Foo::Bar) == 0);

extern "C" void __rust_thunk___ZN3Foo3BarC1Ev(struct Foo::Bar* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(struct Foo::Bar::Baz) == 4);
static_assert(alignof(struct Foo::Bar::Baz) == 4);
static_assert(CRUBIT_OFFSET_OF(baz, struct Foo::Bar::Baz) == 0);

extern "C" void __rust_thunk___ZN3Foo3Bar3BazC1Ev(
    struct Foo::Bar::Baz* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct already_snake_case) == 1);
static_assert(alignof(struct already_snake_case) == 1);
static_assert(CRUBIT_OFFSET_OF(i, struct already_snake_case) == 0);

extern "C" void __rust_thunk___ZN18already_snake_caseC1Ev(
    struct already_snake_case* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct ConflictingSnakeCaseNames) == 1);
static_assert(alignof(struct ConflictingSnakeCaseNames) == 1);
static_assert(CRUBIT_OFFSET_OF(i, struct ConflictingSnakeCaseNames) == 0);

extern "C" void __rust_thunk___ZN25ConflictingSnakeCaseNamesC1Ev(
    struct ConflictingSnakeCaseNames* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct ConflictingSnakeCaseNames_) == 1);
static_assert(alignof(struct ConflictingSnakeCaseNames_) == 1);
static_assert(CRUBIT_OFFSET_OF(i, struct ConflictingSnakeCaseNames_) == 0);

extern "C" void __rust_thunk___ZN26ConflictingSnakeCaseNames_C1Ev(
    struct ConflictingSnakeCaseNames_* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct OnlyOneHasNestedItems) == 1);
static_assert(alignof(struct OnlyOneHasNestedItems) == 1);
static_assert(CRUBIT_OFFSET_OF(i, struct OnlyOneHasNestedItems) == 0);

extern "C" void __rust_thunk___ZN21OnlyOneHasNestedItemsC1Ev(
    struct OnlyOneHasNestedItems* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct OnlyOneHasNestedItems::Inner) == 1);
static_assert(alignof(struct OnlyOneHasNestedItems::Inner) == 1);

extern "C" void __rust_thunk___ZN21OnlyOneHasNestedItems5InnerC1Ev(
    struct OnlyOneHasNestedItems::Inner* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct OnlyOneHasNestedItems_) == 1);
static_assert(alignof(struct OnlyOneHasNestedItems_) == 1);

extern "C" void __rust_thunk___ZN22OnlyOneHasNestedItems_C1Ev(
    struct OnlyOneHasNestedItems_* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct SameNameAsNamespace) == 1);
static_assert(alignof(struct SameNameAsNamespace) == 1);
static_assert(CRUBIT_OFFSET_OF(i, struct SameNameAsNamespace) == 0);

extern "C" void __rust_thunk___ZN19SameNameAsNamespaceC1Ev(
    struct SameNameAsNamespace* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct same_name_as_namespace::Foo) == 1);
static_assert(alignof(struct same_name_as_namespace::Foo) == 1);

extern "C" void __rust_thunk___ZN22same_name_as_namespace3FooC1Ev(
    struct same_name_as_namespace::Foo* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct same_name_as_namespace::Bar) == 1);
static_assert(alignof(struct same_name_as_namespace::Bar) == 1);

extern "C" void __rust_thunk___ZN22same_name_as_namespace3BarC1Ev(
    struct same_name_as_namespace::Bar* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct no_longer_top_level::already_snake_case) == 1);
static_assert(alignof(struct no_longer_top_level::already_snake_case) == 1);
static_assert(
    CRUBIT_OFFSET_OF(i, struct no_longer_top_level::already_snake_case) == 0);

extern "C" void __rust_thunk___ZN19no_longer_top_level18already_snake_caseC1Ev(
    struct no_longer_top_level::already_snake_case* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct no_longer_top_level::ConflictingSnakeCaseNames) ==
              1);
static_assert(alignof(struct no_longer_top_level::ConflictingSnakeCaseNames) ==
              1);
static_assert(CRUBIT_OFFSET_OF(
                  i, struct no_longer_top_level::ConflictingSnakeCaseNames) ==
              0);

extern "C" void
__rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesC1Ev(
    struct no_longer_top_level::ConflictingSnakeCaseNames* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct no_longer_top_level::ConflictingSnakeCaseNames_) ==
              1);
static_assert(alignof(struct no_longer_top_level::ConflictingSnakeCaseNames_) ==
              1);
static_assert(CRUBIT_OFFSET_OF(
                  i, struct no_longer_top_level::ConflictingSnakeCaseNames_) ==
              0);

extern "C" void
__rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_C1Ev(
    struct no_longer_top_level::ConflictingSnakeCaseNames_* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct no_longer_top_level::OnlyOneHasNestedItems) == 1);
static_assert(alignof(struct no_longer_top_level::OnlyOneHasNestedItems) == 1);
static_assert(CRUBIT_OFFSET_OF(
                  i, struct no_longer_top_level::OnlyOneHasNestedItems) == 0);

extern "C" void
__rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsC1Ev(
    struct no_longer_top_level::OnlyOneHasNestedItems* __this) {
  crubit::construct_at(__this);
}

static_assert(
    sizeof(struct no_longer_top_level::OnlyOneHasNestedItems::Inner) == 1);
static_assert(
    alignof(struct no_longer_top_level::OnlyOneHasNestedItems::Inner) == 1);

extern "C" void
__rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItems5InnerC1Ev(
    struct no_longer_top_level::OnlyOneHasNestedItems::Inner* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct no_longer_top_level::OnlyOneHasNestedItems_) == 1);
static_assert(alignof(struct no_longer_top_level::OnlyOneHasNestedItems_) == 1);

extern "C" void
__rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_C1Ev(
    struct no_longer_top_level::OnlyOneHasNestedItems_* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct no_longer_top_level::SameNameAsNamespace) == 1);
static_assert(alignof(struct no_longer_top_level::SameNameAsNamespace) == 1);
static_assert(
    CRUBIT_OFFSET_OF(i, struct no_longer_top_level::SameNameAsNamespace) == 0);

extern "C" void __rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceC1Ev(
    struct no_longer_top_level::SameNameAsNamespace* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct no_longer_top_level::same_name_as_namespace::Foo) ==
              1);
static_assert(
    alignof(struct no_longer_top_level::same_name_as_namespace::Foo) == 1);

extern "C" void
__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooC1Ev(
    struct no_longer_top_level::same_name_as_namespace::Foo* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct no_longer_top_level::same_name_as_namespace::Bar) ==
              1);
static_assert(
    alignof(struct no_longer_top_level::same_name_as_namespace::Bar) == 1);

extern "C" void
__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BarC1Ev(
    struct no_longer_top_level::same_name_as_namespace::Bar* __this) {
  crubit::construct_at(__this);
}

static_assert(sizeof(struct ContainsForwardDeclared) == 1);
static_assert(alignof(struct ContainsForwardDeclared) == 1);

extern "C" void __rust_thunk___ZN23ContainsForwardDeclaredC1Ev(
    struct ContainsForwardDeclared* __this) {
  crubit::construct_at(__this);
}

#pragma clang diagnostic pop
