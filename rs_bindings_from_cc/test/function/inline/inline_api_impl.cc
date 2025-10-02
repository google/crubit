// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/function/inline:inline
// Features: supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/function/inline/inline.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" int __rust_thunk___Z18hello_world_inlinev() {
  return hello_world_inline();
}

static_assert((int (*)())&hello_world_inline);

static_assert(CRUBIT_SIZEOF(struct SomeStruct) == 4);
static_assert(alignof(struct SomeStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(int_field, struct SomeStruct) == 0);

extern "C" int __rust_thunk___Z24take_struct_by_const_ptrPK10SomeStruct(
    struct SomeStruct const* s) {
  return take_struct_by_const_ptr(s);
}

static_assert((int (*)(struct SomeStruct const*))&take_struct_by_const_ptr);

extern "C" unsigned int __rust_thunk___Z19double_unsigned_intj(unsigned int i) {
  return double_unsigned_int(i);
}

static_assert((unsigned int (*)(unsigned int))&double_unsigned_int);

extern "C" int __rust_thunk___ZN10namespaced24forward_declared_doublerEi(
    int x) {
  return namespaced::forward_declared_doubler(x);
}

static_assert((int (*)(int))&namespaced::forward_declared_doubler);

#pragma clang diagnostic pop
