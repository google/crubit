// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:no_unique_address_cc
// Features: experimental, supported

#include <cstddef>
#include <memory>

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/no_unique_address.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct Struct) == 8);
static_assert(alignof(struct Struct) == 4);
static_assert(CRUBIT_OFFSET_OF(field1, struct Struct) == 0);
static_assert(CRUBIT_OFFSET_OF(field2, struct Struct) == 4);

extern "C" void __rust_thunk___ZN6StructC1Ev(struct Struct* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN6StructC1EOS_(struct Struct* __this,
                                               struct Struct* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct Struct* __rust_thunk___ZN6StructaSERKS_(
    struct Struct* __this, const struct Struct* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct Struct* __rust_thunk___ZN6StructaSEOS_(
    struct Struct* __this, struct Struct* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void __rust_thunk___ZN6Struct4MakeEic(struct Struct* __return,
                                                 int f1, char f2) {
  new (__return) auto(Struct::Make(f1, f2));
}

static_assert(sizeof(struct PaddingBetweenFields) == 8);
static_assert(alignof(struct PaddingBetweenFields) == 4);
static_assert(CRUBIT_OFFSET_OF(field1, struct PaddingBetweenFields) == 0);
static_assert(CRUBIT_OFFSET_OF(field2, struct PaddingBetweenFields) == 4);

extern "C" void __rust_thunk___ZN20PaddingBetweenFieldsC1Ev(
    struct PaddingBetweenFields* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN20PaddingBetweenFieldsC1EOS_(
    struct PaddingBetweenFields* __this,
    struct PaddingBetweenFields* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct PaddingBetweenFields*
__rust_thunk___ZN20PaddingBetweenFieldsaSERKS_(
    struct PaddingBetweenFields* __this,
    const struct PaddingBetweenFields* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct PaddingBetweenFields*
__rust_thunk___ZN20PaddingBetweenFieldsaSEOS_(
    struct PaddingBetweenFields* __this,
    struct PaddingBetweenFields* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void __rust_thunk___ZN20PaddingBetweenFields4MakeEci(
    struct PaddingBetweenFields* __return, char f1, int f2) {
  new (__return) auto(PaddingBetweenFields::Make(f1, f2));
}

static_assert(sizeof(struct FieldInTailPadding_InnerStruct) == 8);
static_assert(alignof(struct FieldInTailPadding_InnerStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(inner_int_field,
                               struct FieldInTailPadding_InnerStruct) == 0);
static_assert(CRUBIT_OFFSET_OF(inner_char_field,
                               struct FieldInTailPadding_InnerStruct) == 4);

extern "C" void __rust_thunk___ZN30FieldInTailPadding_InnerStructC1Ev(
    struct FieldInTailPadding_InnerStruct* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN30FieldInTailPadding_InnerStructC1ERKS_(
    struct FieldInTailPadding_InnerStruct* __this,
    const struct FieldInTailPadding_InnerStruct* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" struct FieldInTailPadding_InnerStruct*
__rust_thunk___ZN30FieldInTailPadding_InnerStructaSERKS_(
    struct FieldInTailPadding_InnerStruct* __this,
    const struct FieldInTailPadding_InnerStruct* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" void __rust_thunk___ZN30FieldInTailPadding_InnerStructD1Ev(
    struct FieldInTailPadding_InnerStruct* __this) {
  std::destroy_at(__this);
}

static_assert(sizeof(struct FieldInTailPadding) == 8);
static_assert(alignof(struct FieldInTailPadding) == 4);
static_assert(CRUBIT_OFFSET_OF(inner_struct, struct FieldInTailPadding) == 0);
static_assert(CRUBIT_OFFSET_OF(char_in_tail_padding_of_prev_field,
                               struct FieldInTailPadding) == 5);

extern "C" void __rust_thunk___ZN18FieldInTailPaddingC1ERKS_(
    struct FieldInTailPadding* __this,
    const struct FieldInTailPadding* __param_0) {
  crubit::construct_at(__this, *__param_0);
}

extern "C" void __rust_thunk___ZN18FieldInTailPaddingC1EOS_(
    struct FieldInTailPadding* __this, struct FieldInTailPadding* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" void __rust_thunk___ZN18FieldInTailPaddingD1Ev(
    struct FieldInTailPadding* __this) {
  std::destroy_at(__this);
}

extern "C" struct FieldInTailPadding*
__rust_thunk___ZN18FieldInTailPaddingaSERKS_(
    struct FieldInTailPadding* __this,
    const struct FieldInTailPadding* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct FieldInTailPadding*
__rust_thunk___ZN18FieldInTailPaddingaSEOS_(
    struct FieldInTailPadding* __this, struct FieldInTailPadding* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void __rust_thunk___ZN18FieldInTailPaddingC1Eicc(
    struct FieldInTailPadding* __this, int inner_int, char inner_char,
    char outer_char) {
  crubit::construct_at(__this, inner_int, inner_char, outer_char);
}

#pragma clang diagnostic pop
