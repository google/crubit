// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/simple:simple
// Features: supported, unsafe_types

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/function/simple/simple.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert((int (*)())&return_value);

static_assert((int* (*)())&return_pointer);

static_assert((int& (*)())&return_reference);

static_assert((void (*)(int*))&take_pointer);

static_assert((void (*)(int&))&take_reference);

static_assert((int const* (*)(int const*)) & forward_pointer);

static_assert((int const& (*)(int const&)) & forward_reference);

static_assert((int (*)(int, int))&multiply);

static_assert((int (*)(int, int))&multiply_with_unnamed_parameters);

static_assert((int (*)(int, int, int))&multiply_with_keyword_named_parameters);

static_assert((int (*)())&llvm_no_mangle_marker);

static_assert((int (*)())&asm_name_with_dollar_sign);

static_assert((crubit::type_identity_t<int(int, int)> * (*)()) &
              get_pointer_to_multiply_function);

static_assert((crubit::type_identity_t<int(int, int)> & (*)()) &
              get_reference_to_multiply_function);

extern "C" crubit::type_identity_t<int(int, int)>*
__rust_thunk___Z39inline_get_pointer_to_multiply_functionv() {
  return inline_get_pointer_to_multiply_function();
}

static_assert((crubit::type_identity_t<int(int, int)> * (*)()) &
              inline_get_pointer_to_multiply_function);

extern "C" int __rust_thunk___Z15apply_binary_opiiPFiiiE(
    int x, int y, crubit::type_identity_t<int(int, int)>* op) {
  return apply_binary_op(x, y, op);
}

static_assert((int (*)(
    int, int, crubit::type_identity_t<int(int, int)>*))&apply_binary_op);

#pragma clang diagnostic pop
