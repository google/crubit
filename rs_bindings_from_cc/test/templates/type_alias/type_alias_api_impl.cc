// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/templates/type_alias:type_alias
// Features: custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr, std_vector, supported, wrapper

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/templates/type_alias/type_alias.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(class MyTemplate<int>) == 4);
static_assert(alignof(class MyTemplate<int>) == 4);

extern "C" void
__rust_thunk___ZN10MyTemplateIiEC1Ev__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2ftype_5falias_3atype_5falias(
    class MyTemplate<int>* __this) {
  crubit::construct_at(__this);
}

extern "C" void
__rust_thunk___ZN10MyTemplateIiE6CreateEi__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2ftype_5falias_3atype_5falias(
    class MyTemplate<int>* __return, int value) {
  new (__return) auto(MyTemplate<int>::Create(value));
}

static_assert((class MyTemplate<int> (*)(int)) & ::MyTemplate<int>::Create);

extern "C" int const*
__rust_thunk___ZNK10MyTemplateIiE5valueEv__2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2ftemplates_2ftype_5falias_3atype_5falias(
    class MyTemplate<int> const* __this) {
  return std::addressof(__this->value());
}

static_assert((int const& (MyTemplate<int>::*)() const) &
              ::MyTemplate<int>::value);

#pragma clang diagnostic pop
