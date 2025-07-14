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

extern "C" void __rust_thunk___ZN3FooC1EOS_(struct Foo* __this,
                                            struct Foo* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct Foo* __rust_thunk___ZN3FooaSERKS_(
    struct Foo* __this, const struct Foo* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct Foo* __rust_thunk___ZN3FooaSEOS_(struct Foo* __this,
                                                   struct Foo* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct Foo::Bar) == 4);
static_assert(alignof(struct Foo::Bar) == 4);
static_assert(CRUBIT_OFFSET_OF(bar, struct Foo::Bar) == 0);

extern "C" void __rust_thunk___ZN3Foo3BarC1Ev(struct Foo::Bar* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN3Foo3BarC1EOS0_(struct Foo::Bar* __this,
                                                 struct Foo::Bar* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct Foo::Bar* __rust_thunk___ZN3Foo3BaraSERKS0_(
    struct Foo::Bar* __this, const struct Foo::Bar* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct Foo::Bar* __rust_thunk___ZN3Foo3BaraSEOS0_(
    struct Foo::Bar* __this, struct Foo::Bar* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct Foo::Bar::Baz) == 4);
static_assert(alignof(struct Foo::Bar::Baz) == 4);
static_assert(CRUBIT_OFFSET_OF(baz, struct Foo::Bar::Baz) == 0);

extern "C" void __rust_thunk___ZN3Foo3Bar3BazC1Ev(
    struct Foo::Bar::Baz* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN3Foo3Bar3BazC1EOS1_(
    struct Foo::Bar::Baz* __this, struct Foo::Bar::Baz* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct Foo::Bar::Baz* __rust_thunk___ZN3Foo3Bar3BazaSERKS1_(
    struct Foo::Bar::Baz* __this, const struct Foo::Bar::Baz* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct Foo::Bar::Baz* __rust_thunk___ZN3Foo3Bar3BazaSEOS1_(
    struct Foo::Bar::Baz* __this, struct Foo::Bar::Baz* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct already_snake_case) == 4);
static_assert(alignof(struct already_snake_case) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct already_snake_case) == 0);

extern "C" void __rust_thunk___ZN18already_snake_caseC1Ev(
    struct already_snake_case* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN18already_snake_caseC1EOS_(
    struct already_snake_case* __this, struct already_snake_case* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct already_snake_case*
__rust_thunk___ZN18already_snake_caseaSERKS_(
    struct already_snake_case* __this,
    const struct already_snake_case* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct already_snake_case*
__rust_thunk___ZN18already_snake_caseaSEOS_(
    struct already_snake_case* __this, struct already_snake_case* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct ConflictingSnakeCaseNames) == 4);
static_assert(alignof(struct ConflictingSnakeCaseNames) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct ConflictingSnakeCaseNames) == 0);

extern "C" void __rust_thunk___ZN25ConflictingSnakeCaseNamesC1Ev(
    struct ConflictingSnakeCaseNames* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN25ConflictingSnakeCaseNamesC1EOS_(
    struct ConflictingSnakeCaseNames* __this,
    struct ConflictingSnakeCaseNames* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct ConflictingSnakeCaseNames*
__rust_thunk___ZN25ConflictingSnakeCaseNamesaSERKS_(
    struct ConflictingSnakeCaseNames* __this,
    const struct ConflictingSnakeCaseNames* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct ConflictingSnakeCaseNames*
__rust_thunk___ZN25ConflictingSnakeCaseNamesaSEOS_(
    struct ConflictingSnakeCaseNames* __this,
    struct ConflictingSnakeCaseNames* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct ConflictingSnakeCaseNames_) == 4);
static_assert(alignof(struct ConflictingSnakeCaseNames_) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct ConflictingSnakeCaseNames_) == 0);

extern "C" void __rust_thunk___ZN26ConflictingSnakeCaseNames_C1Ev(
    struct ConflictingSnakeCaseNames_* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN26ConflictingSnakeCaseNames_C1EOS_(
    struct ConflictingSnakeCaseNames_* __this,
    struct ConflictingSnakeCaseNames_* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct ConflictingSnakeCaseNames_*
__rust_thunk___ZN26ConflictingSnakeCaseNames_aSERKS_(
    struct ConflictingSnakeCaseNames_* __this,
    const struct ConflictingSnakeCaseNames_* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct ConflictingSnakeCaseNames_*
__rust_thunk___ZN26ConflictingSnakeCaseNames_aSEOS_(
    struct ConflictingSnakeCaseNames_* __this,
    struct ConflictingSnakeCaseNames_* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct OnlyOneHasNestedItems) == 4);
static_assert(alignof(struct OnlyOneHasNestedItems) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct OnlyOneHasNestedItems) == 0);

extern "C" void __rust_thunk___ZN21OnlyOneHasNestedItemsC1Ev(
    struct OnlyOneHasNestedItems* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN21OnlyOneHasNestedItemsC1EOS_(
    struct OnlyOneHasNestedItems* __this,
    struct OnlyOneHasNestedItems* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct OnlyOneHasNestedItems*
__rust_thunk___ZN21OnlyOneHasNestedItemsaSERKS_(
    struct OnlyOneHasNestedItems* __this,
    const struct OnlyOneHasNestedItems* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct OnlyOneHasNestedItems*
__rust_thunk___ZN21OnlyOneHasNestedItemsaSEOS_(
    struct OnlyOneHasNestedItems* __this,
    struct OnlyOneHasNestedItems* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(struct OnlyOneHasNestedItems_) == 1);
static_assert(alignof(struct OnlyOneHasNestedItems_) == 1);

extern "C" void __rust_thunk___ZN22OnlyOneHasNestedItems_C1Ev(
    struct OnlyOneHasNestedItems_* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN22OnlyOneHasNestedItems_C1EOS_(
    struct OnlyOneHasNestedItems_* __this,
    struct OnlyOneHasNestedItems_* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct OnlyOneHasNestedItems_*
__rust_thunk___ZN22OnlyOneHasNestedItems_aSERKS_(
    struct OnlyOneHasNestedItems_* __this,
    const struct OnlyOneHasNestedItems_* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct OnlyOneHasNestedItems_*
__rust_thunk___ZN22OnlyOneHasNestedItems_aSEOS_(
    struct OnlyOneHasNestedItems_* __this,
    struct OnlyOneHasNestedItems_* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct SameNameAsNamespace) == 4);
static_assert(alignof(struct SameNameAsNamespace) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct SameNameAsNamespace) == 0);

extern "C" void __rust_thunk___ZN19SameNameAsNamespaceC1Ev(
    struct SameNameAsNamespace* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN19SameNameAsNamespaceC1EOS_(
    struct SameNameAsNamespace* __this, struct SameNameAsNamespace* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct SameNameAsNamespace*
__rust_thunk___ZN19SameNameAsNamespaceaSERKS_(
    struct SameNameAsNamespace* __this,
    const struct SameNameAsNamespace* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct SameNameAsNamespace*
__rust_thunk___ZN19SameNameAsNamespaceaSEOS_(
    struct SameNameAsNamespace* __this, struct SameNameAsNamespace* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(struct same_name_as_namespace::Foo) == 1);
static_assert(alignof(struct same_name_as_namespace::Foo) == 1);

extern "C" void __rust_thunk___ZN22same_name_as_namespace3FooC1Ev(
    struct same_name_as_namespace::Foo* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN22same_name_as_namespace3FooC1EOS0_(
    struct same_name_as_namespace::Foo* __this,
    struct same_name_as_namespace::Foo* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct same_name_as_namespace::Foo*
__rust_thunk___ZN22same_name_as_namespace3FooaSERKS0_(
    struct same_name_as_namespace::Foo* __this,
    const struct same_name_as_namespace::Foo* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct same_name_as_namespace::Foo*
__rust_thunk___ZN22same_name_as_namespace3FooaSEOS0_(
    struct same_name_as_namespace::Foo* __this,
    struct same_name_as_namespace::Foo* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(struct same_name_as_namespace::Bar) == 1);
static_assert(alignof(struct same_name_as_namespace::Bar) == 1);

extern "C" void __rust_thunk___ZN22same_name_as_namespace3BarC1Ev(
    struct same_name_as_namespace::Bar* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN22same_name_as_namespace3BarC1EOS0_(
    struct same_name_as_namespace::Bar* __this,
    struct same_name_as_namespace::Bar* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct same_name_as_namespace::Bar*
__rust_thunk___ZN22same_name_as_namespace3BaraSERKS0_(
    struct same_name_as_namespace::Bar* __this,
    const struct same_name_as_namespace::Bar* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct same_name_as_namespace::Bar*
__rust_thunk___ZN22same_name_as_namespace3BaraSEOS0_(
    struct same_name_as_namespace::Bar* __this,
    struct same_name_as_namespace::Bar* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct no_longer_top_level::already_snake_case) ==
              4);
static_assert(alignof(struct no_longer_top_level::already_snake_case) == 4);
static_assert(
    CRUBIT_OFFSET_OF(i, struct no_longer_top_level::already_snake_case) == 0);

extern "C" void __rust_thunk___ZN19no_longer_top_level18already_snake_caseC1Ev(
    struct no_longer_top_level::already_snake_case* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN19no_longer_top_level18already_snake_caseC1EOS0_(
    struct no_longer_top_level::already_snake_case* __this,
    struct no_longer_top_level::already_snake_case* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct no_longer_top_level::already_snake_case*
__rust_thunk___ZN19no_longer_top_level18already_snake_caseaSERKS0_(
    struct no_longer_top_level::already_snake_case* __this,
    const struct no_longer_top_level::already_snake_case* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct no_longer_top_level::already_snake_case*
__rust_thunk___ZN19no_longer_top_level18already_snake_caseaSEOS0_(
    struct no_longer_top_level::already_snake_case* __this,
    struct no_longer_top_level::already_snake_case* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(
    CRUBIT_SIZEOF(struct no_longer_top_level::ConflictingSnakeCaseNames) == 4);
static_assert(alignof(struct no_longer_top_level::ConflictingSnakeCaseNames) ==
              4);
static_assert(CRUBIT_OFFSET_OF(
                  i, struct no_longer_top_level::ConflictingSnakeCaseNames) ==
              0);

extern "C" void
__rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesC1Ev(
    struct no_longer_top_level::ConflictingSnakeCaseNames* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesC1EOS0_(
    struct no_longer_top_level::ConflictingSnakeCaseNames* __this,
    struct no_longer_top_level::ConflictingSnakeCaseNames* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct no_longer_top_level::ConflictingSnakeCaseNames*
__rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesaSERKS0_(
    struct no_longer_top_level::ConflictingSnakeCaseNames* __this,
    const struct no_longer_top_level::ConflictingSnakeCaseNames* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct no_longer_top_level::ConflictingSnakeCaseNames*
__rust_thunk___ZN19no_longer_top_level25ConflictingSnakeCaseNamesaSEOS0_(
    struct no_longer_top_level::ConflictingSnakeCaseNames* __this,
    struct no_longer_top_level::ConflictingSnakeCaseNames* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(
    CRUBIT_SIZEOF(struct no_longer_top_level::ConflictingSnakeCaseNames_) == 4);
static_assert(alignof(struct no_longer_top_level::ConflictingSnakeCaseNames_) ==
              4);
static_assert(CRUBIT_OFFSET_OF(
                  i, struct no_longer_top_level::ConflictingSnakeCaseNames_) ==
              0);

extern "C" void
__rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_C1Ev(
    struct no_longer_top_level::ConflictingSnakeCaseNames_* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_C1EOS0_(
    struct no_longer_top_level::ConflictingSnakeCaseNames_* __this,
    struct no_longer_top_level::ConflictingSnakeCaseNames_* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct no_longer_top_level::ConflictingSnakeCaseNames_*
__rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_aSERKS0_(
    struct no_longer_top_level::ConflictingSnakeCaseNames_* __this,
    const struct no_longer_top_level::ConflictingSnakeCaseNames_* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct no_longer_top_level::ConflictingSnakeCaseNames_*
__rust_thunk___ZN19no_longer_top_level26ConflictingSnakeCaseNames_aSEOS0_(
    struct no_longer_top_level::ConflictingSnakeCaseNames_* __this,
    struct no_longer_top_level::ConflictingSnakeCaseNames_* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(
    CRUBIT_SIZEOF(struct no_longer_top_level::OnlyOneHasNestedItems) == 4);
static_assert(alignof(struct no_longer_top_level::OnlyOneHasNestedItems) == 4);
static_assert(CRUBIT_OFFSET_OF(
                  i, struct no_longer_top_level::OnlyOneHasNestedItems) == 0);

extern "C" void
__rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsC1Ev(
    struct no_longer_top_level::OnlyOneHasNestedItems* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsC1EOS0_(
    struct no_longer_top_level::OnlyOneHasNestedItems* __this,
    struct no_longer_top_level::OnlyOneHasNestedItems* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct no_longer_top_level::OnlyOneHasNestedItems*
__rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsaSERKS0_(
    struct no_longer_top_level::OnlyOneHasNestedItems* __this,
    const struct no_longer_top_level::OnlyOneHasNestedItems* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct no_longer_top_level::OnlyOneHasNestedItems*
__rust_thunk___ZN19no_longer_top_level21OnlyOneHasNestedItemsaSEOS0_(
    struct no_longer_top_level::OnlyOneHasNestedItems* __this,
    struct no_longer_top_level::OnlyOneHasNestedItems* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(sizeof(struct no_longer_top_level::OnlyOneHasNestedItems_) == 1);
static_assert(alignof(struct no_longer_top_level::OnlyOneHasNestedItems_) == 1);

extern "C" void
__rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_C1Ev(
    struct no_longer_top_level::OnlyOneHasNestedItems_* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_C1EOS0_(
    struct no_longer_top_level::OnlyOneHasNestedItems_* __this,
    struct no_longer_top_level::OnlyOneHasNestedItems_* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct no_longer_top_level::OnlyOneHasNestedItems_*
__rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_aSERKS0_(
    struct no_longer_top_level::OnlyOneHasNestedItems_* __this,
    const struct no_longer_top_level::OnlyOneHasNestedItems_* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct no_longer_top_level::OnlyOneHasNestedItems_*
__rust_thunk___ZN19no_longer_top_level22OnlyOneHasNestedItems_aSEOS0_(
    struct no_longer_top_level::OnlyOneHasNestedItems_* __this,
    struct no_longer_top_level::OnlyOneHasNestedItems_* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

static_assert(CRUBIT_SIZEOF(struct no_longer_top_level::SameNameAsNamespace) ==
              4);
static_assert(alignof(struct no_longer_top_level::SameNameAsNamespace) == 4);
static_assert(
    CRUBIT_OFFSET_OF(i, struct no_longer_top_level::SameNameAsNamespace) == 0);

extern "C" void __rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceC1Ev(
    struct no_longer_top_level::SameNameAsNamespace* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceC1EOS0_(
    struct no_longer_top_level::SameNameAsNamespace* __this,
    struct no_longer_top_level::SameNameAsNamespace* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct no_longer_top_level::SameNameAsNamespace*
__rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceaSERKS0_(
    struct no_longer_top_level::SameNameAsNamespace* __this,
    const struct no_longer_top_level::SameNameAsNamespace* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct no_longer_top_level::SameNameAsNamespace*
__rust_thunk___ZN19no_longer_top_level19SameNameAsNamespaceaSEOS0_(
    struct no_longer_top_level::SameNameAsNamespace* __this,
    struct no_longer_top_level::SameNameAsNamespace* __param_0) {
  return &__this->operator=(std::move(*__param_0));
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

extern "C" void
__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooC1EOS1_(
    struct no_longer_top_level::same_name_as_namespace::Foo* __this,
    struct no_longer_top_level::same_name_as_namespace::Foo* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct no_longer_top_level::same_name_as_namespace::Foo*
__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooaSERKS1_(
    struct no_longer_top_level::same_name_as_namespace::Foo* __this,
    const struct no_longer_top_level::same_name_as_namespace::Foo* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct no_longer_top_level::same_name_as_namespace::Foo*
__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3FooaSEOS1_(
    struct no_longer_top_level::same_name_as_namespace::Foo* __this,
    struct no_longer_top_level::same_name_as_namespace::Foo* __param_0) {
  return &__this->operator=(std::move(*__param_0));
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

extern "C" void
__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BarC1EOS1_(
    struct no_longer_top_level::same_name_as_namespace::Bar* __this,
    struct no_longer_top_level::same_name_as_namespace::Bar* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct no_longer_top_level::same_name_as_namespace::Bar*
__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BaraSERKS1_(
    struct no_longer_top_level::same_name_as_namespace::Bar* __this,
    const struct no_longer_top_level::same_name_as_namespace::Bar* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct no_longer_top_level::same_name_as_namespace::Bar*
__rust_thunk___ZN19no_longer_top_level22same_name_as_namespace3BaraSEOS1_(
    struct no_longer_top_level::same_name_as_namespace::Bar* __this,
    struct no_longer_top_level::same_name_as_namespace::Bar* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

#pragma clang diagnostic pop
