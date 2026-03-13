// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/types_only:types_only
// Features: types

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/types_only/types_only.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct Copyable) == 4);
static_assert(alignof(struct Copyable) == 4);
static_assert(CRUBIT_OFFSET_OF(field, struct Copyable) == 0);

extern "C" void __rust_thunk___ZN8CopyableC1Ev(struct Copyable* __this) {
  crubit::construct_at(__this);
}

static_assert(CRUBIT_SIZEOF(class Cloneable) == 4);
static_assert(alignof(class Cloneable) == 4);

extern "C" void __rust_thunk___ZN9CloneableC1Ei(class Cloneable* __this,
                                                int field) {
  crubit::construct_at(__this, field);
}

extern "C" void __rust_thunk___ZN9CloneableC1ERKS_(
    class Cloneable* __this, class Cloneable const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN9CloneableC1EOS_(class Cloneable* __this,
                                                  class Cloneable* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class Cloneable* __rust_thunk___ZN9CloneableaSERKS_(
    class Cloneable* __this, class Cloneable const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

extern "C" class Cloneable* __rust_thunk___ZN9CloneableaSEOS_(
    class Cloneable* __this, class Cloneable* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

extern "C" void __rust_thunk___ZN9CloneableD1Ev(class Cloneable* __this) {
  std::destroy_at(__this);
}

static_assert(CRUBIT_SIZEOF(class Movable) == 4);
static_assert(alignof(class Movable) == 4);

extern "C" void __rust_thunk___ZN7MovableC1Ei(class Movable* __this,
                                              int field) {
  crubit::construct_at(__this, field);
}

extern "C" void __rust_thunk___ZN7MovableC1EOS_(class Movable* __this,
                                                class Movable* other) {
  crubit::construct_at(__this, std::move(*other));
}

extern "C" class Movable* __rust_thunk___ZN7MovableaSEOS_(
    class Movable* __this, class Movable* other) {
  return std::addressof(__this->operator=(std::move(*other)));
}

extern "C" void __rust_thunk___ZN7MovableD1Ev(class Movable* __this) {
  std::destroy_at(__this);
}

#pragma clang diagnostic pop
