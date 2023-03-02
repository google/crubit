// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:types_cc
// Features: experimental, supported

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/types.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN10SomeStructC1Ev(struct SomeStruct* __this) {
  crubit::construct_at(__this);
}
extern "C" void __rust_thunk___ZN10SomeStructC1EOS_(
    struct SomeStruct* __this, struct SomeStruct* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" struct SomeStruct* __rust_thunk___ZN10SomeStructaSERKS_(
    struct SomeStruct* __this, const struct SomeStruct* __param_0) {
  return &__this->operator=(*__param_0);
}
extern "C" struct SomeStruct* __rust_thunk___ZN10SomeStructaSEOS_(
    struct SomeStruct* __this, struct SomeStruct* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}
extern "C" void __rust_thunk___ZN19FieldTypeTestStructC1EOS_(
    struct FieldTypeTestStruct* __this, struct FieldTypeTestStruct* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}
extern "C" void __rust_thunk___Z21VoidReturningFunctionv() {
  VoidReturningFunction();
}
extern "C" crubit::type_identity_t<int&(int const&, int*)>*
__rust_thunk___Z32FunctionPointerReturningFunctionv() {
  return FunctionPointerReturningFunction();
}
extern "C" void* __rust_thunk___Z24FunctionWithVoidPointersPvPKv(
    void* __param_0, void const* __param_1) {
  return FunctionWithVoidPointers(__param_0, __param_1);
}

static_assert(sizeof(struct SomeStruct) == 1);
static_assert(alignof(struct SomeStruct) == 1);

static_assert(sizeof(struct FieldTypeTestStruct) == 200);
static_assert(alignof(struct FieldTypeTestStruct) == 8);
static_assert(CRUBIT_OFFSET_OF(bool_field, struct FieldTypeTestStruct) == 0);
static_assert(CRUBIT_OFFSET_OF(char_field, struct FieldTypeTestStruct) == 1);
static_assert(CRUBIT_OFFSET_OF(unsigned_char_field,
                               struct FieldTypeTestStruct) == 2);
static_assert(CRUBIT_OFFSET_OF(signed_char_field, struct FieldTypeTestStruct) ==
              3);
static_assert(CRUBIT_OFFSET_OF(char16_t_field, struct FieldTypeTestStruct) ==
              4);
static_assert(CRUBIT_OFFSET_OF(char32_t_field, struct FieldTypeTestStruct) ==
              8);
static_assert(CRUBIT_OFFSET_OF(wchar_t_field, struct FieldTypeTestStruct) ==
              12);
static_assert(CRUBIT_OFFSET_OF(short_field, struct FieldTypeTestStruct) == 16);
static_assert(CRUBIT_OFFSET_OF(int_field, struct FieldTypeTestStruct) == 20);
static_assert(CRUBIT_OFFSET_OF(long_field, struct FieldTypeTestStruct) == 24);
static_assert(CRUBIT_OFFSET_OF(long_long_field, struct FieldTypeTestStruct) ==
              32);
static_assert(CRUBIT_OFFSET_OF(unsigned_short_field,
                               struct FieldTypeTestStruct) == 40);
static_assert(CRUBIT_OFFSET_OF(unsigned_int_field,
                               struct FieldTypeTestStruct) == 44);
static_assert(CRUBIT_OFFSET_OF(unsigned_long_field,
                               struct FieldTypeTestStruct) == 48);
static_assert(CRUBIT_OFFSET_OF(unsigned_long_long_field,
                               struct FieldTypeTestStruct) == 56);
static_assert(CRUBIT_OFFSET_OF(signed_short_field,
                               struct FieldTypeTestStruct) == 64);
static_assert(CRUBIT_OFFSET_OF(signed_int_field, struct FieldTypeTestStruct) ==
              68);
static_assert(CRUBIT_OFFSET_OF(signed_long_field, struct FieldTypeTestStruct) ==
              72);
static_assert(CRUBIT_OFFSET_OF(signed_long_long_field,
                               struct FieldTypeTestStruct) == 80);
static_assert(CRUBIT_OFFSET_OF(ptrdiff_t_field, struct FieldTypeTestStruct) ==
              88);
static_assert(CRUBIT_OFFSET_OF(size_t_field, struct FieldTypeTestStruct) == 96);
static_assert(CRUBIT_OFFSET_OF(float_field, struct FieldTypeTestStruct) == 104);
static_assert(CRUBIT_OFFSET_OF(double_field, struct FieldTypeTestStruct) ==
              112);
static_assert(CRUBIT_OFFSET_OF(ptr_field, struct FieldTypeTestStruct) == 120);
static_assert(CRUBIT_OFFSET_OF(void_ptr_field, struct FieldTypeTestStruct) ==
              128);
static_assert(CRUBIT_OFFSET_OF(const_void_ptr_field,
                               struct FieldTypeTestStruct) == 136);
static_assert(CRUBIT_OFFSET_OF(void_double_ptr_field,
                               struct FieldTypeTestStruct) == 144);
static_assert(CRUBIT_OFFSET_OF(struct_field, struct FieldTypeTestStruct) ==
              152);
static_assert(CRUBIT_OFFSET_OF(struct_ptr_field, struct FieldTypeTestStruct) ==
              160);
static_assert(CRUBIT_OFFSET_OF(const_struct_ptr_field,
                               struct FieldTypeTestStruct) == 168);
static_assert(CRUBIT_OFFSET_OF(struct_ref_field, struct FieldTypeTestStruct) ==
              176);
static_assert(CRUBIT_OFFSET_OF(const_struct_ref_field,
                               struct FieldTypeTestStruct) == 184);
static_assert(CRUBIT_OFFSET_OF(forward_declared_ptr_field,
                               struct FieldTypeTestStruct) == 192);

#pragma clang diagnostic pop
