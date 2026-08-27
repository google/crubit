// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:user_of_base_class_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/user_of_base_class.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct Derived2) == 24);
static_assert(alignof(struct Derived2) == 8);
static_assert(CRUBIT_OFFSET_OF(derived_1, struct Derived2) == 20);

extern "C" void __rust_thunk___ZN8Derived2C1Ev(struct Derived2* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN8Derived2C1ERKS_(
    struct Derived2* __this, struct Derived2 const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN8Derived2C1EOS_(struct Derived2* __this,
                                                 struct Derived2* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct Derived2* __rust_thunk___ZN8Derived2aSERKS_(
    struct Derived2* __this, struct Derived2 const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((struct Derived2 & (::Derived2::*)(struct Derived2 const&)) &
              ::Derived2::operator=);

extern "C" struct Derived2* __rust_thunk___ZN8Derived2aSEOS_(
    struct Derived2* __this, struct Derived2* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((struct Derived2 & (::Derived2::*)(struct Derived2&&)) &
              ::Derived2::operator=);

static_assert(CRUBIT_SIZEOF(class VirtualDerived2) == 32);
static_assert(alignof(class VirtualDerived2) == 8);

extern "C" void __rust_thunk___ZN15VirtualDerived2C1Ev(
    class VirtualDerived2* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN15VirtualDerived2C1ERKS_(
    class VirtualDerived2* __this, class VirtualDerived2 const* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN15VirtualDerived2C1EOS_(
    class VirtualDerived2* __this, class VirtualDerived2* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" class VirtualDerived2* __rust_thunk___ZN15VirtualDerived2aSERKS_(
    class VirtualDerived2* __this, class VirtualDerived2 const* __param_0) {
  return std::addressof(__this->operator=(*__param_0));
}

static_assert((class VirtualDerived2 &
               (::VirtualDerived2::*)(class VirtualDerived2 const&)) &
              ::VirtualDerived2::operator=);

extern "C" class VirtualDerived2* __rust_thunk___ZN15VirtualDerived2aSEOS_(
    class VirtualDerived2* __this, class VirtualDerived2* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

static_assert((class VirtualDerived2 &
               (::VirtualDerived2::*)(class VirtualDerived2&&)) &
              ::VirtualDerived2::operator=);

#pragma clang diagnostic pop
