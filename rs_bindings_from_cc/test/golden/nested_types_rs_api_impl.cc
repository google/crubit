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

#pragma clang diagnostic pop
