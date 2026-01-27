// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/methods_qualifiers:methods_qualifiers
// Features: assume_lifetimes, custom_ffi_types, experimental, non_unpin_ctor, std_unique_ptr, std_vector, supported, unhardcode_c9_co, wrapper

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/struct/methods_qualifiers/methods_qualifiers.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(struct UnpinStructWithRefQualifiedMethods) == 4);
static_assert(alignof(struct UnpinStructWithRefQualifiedMethods) == 4);
static_assert(CRUBIT_OFFSET_OF(i, struct UnpinStructWithRefQualifiedMethods) ==
              0);

extern "C" void __rust_thunk___ZN34UnpinStructWithRefQualifiedMethodsC1Ev(
    struct UnpinStructWithRefQualifiedMethods* __this) {
  crubit::construct_at(__this);
}

static_assert((void (UnpinStructWithRefQualifiedMethods::*)()) &
              ::UnpinStructWithRefQualifiedMethods::increment_i);

static_assert((int (UnpinStructWithRefQualifiedMethods::*)()) &
              ::UnpinStructWithRefQualifiedMethods::unqualified_get_i);

static_assert((int (UnpinStructWithRefQualifiedMethods::*)() const) &
              ::UnpinStructWithRefQualifiedMethods::const_qualified_get_i);

static_assert(
    (int (UnpinStructWithRefQualifiedMethods::*)() &)&::
        UnpinStructWithRefQualifiedMethods::lvalue_ref_qualified_get_i);

static_assert((int (UnpinStructWithRefQualifiedMethods::*)()
                   const&)&::UnpinStructWithRefQualifiedMethods::
                  const_lvalue_ref_qualified_get_i);

static_assert(
    (int (UnpinStructWithRefQualifiedMethods::*)() &&)&::
        UnpinStructWithRefQualifiedMethods::rvalue_ref_qualified_get_i);

static_assert((int (UnpinStructWithRefQualifiedMethods::*)()
                   const&&)&::UnpinStructWithRefQualifiedMethods::
                  const_rvalue_ref_qualified_get_i);

#pragma clang diagnostic pop
