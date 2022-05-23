// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/no_unique_address.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN6StructC1Ev(class Struct* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN6StructC1ERKS_(class Struct* __this,
                                                const class Struct& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN6StructC1EOS_(class Struct* __this,
                                               class Struct&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN6StructD1Ev(class Struct* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class Struct& __rust_thunk___ZN6StructaSERKS_(
    class Struct* __this, const class Struct& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class Struct& __rust_thunk___ZN6StructaSEOS_(
    class Struct* __this, class Struct&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class Struct __rust_thunk___ZN6Struct4MakeEic(int f1, char f2) {
  return Struct::Make(std::forward<decltype(f1)>(f1),
                      std::forward<decltype(f2)>(f2));
}
extern "C" void __rust_thunk___ZN20PaddingBetweenFieldsC1Ev(
    class PaddingBetweenFields* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN20PaddingBetweenFieldsC1ERKS_(
    class PaddingBetweenFields* __this,
    const class PaddingBetweenFields& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN20PaddingBetweenFieldsC1EOS_(
    class PaddingBetweenFields* __this,
    class PaddingBetweenFields&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN20PaddingBetweenFieldsD1Ev(
    class PaddingBetweenFields* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class PaddingBetweenFields&
__rust_thunk___ZN20PaddingBetweenFieldsaSERKS_(
    class PaddingBetweenFields* __this,
    const class PaddingBetweenFields& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class PaddingBetweenFields&
__rust_thunk___ZN20PaddingBetweenFieldsaSEOS_(
    class PaddingBetweenFields* __this,
    class PaddingBetweenFields&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class PaddingBetweenFields
__rust_thunk___ZN20PaddingBetweenFields4MakeEci(char f1, int f2) {
  return PaddingBetweenFields::Make(std::forward<decltype(f1)>(f1),
                                    std::forward<decltype(f2)>(f2));
}
extern "C" void __rust_thunk___ZN30FieldInTailPadding_InnerStructC1Ev(
    class FieldInTailPadding_InnerStruct* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN30FieldInTailPadding_InnerStructC1ERKS_(
    class FieldInTailPadding_InnerStruct* __this,
    const class FieldInTailPadding_InnerStruct& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class FieldInTailPadding_InnerStruct&
__rust_thunk___ZN30FieldInTailPadding_InnerStructaSERKS_(
    class FieldInTailPadding_InnerStruct* __this,
    const class FieldInTailPadding_InnerStruct& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN30FieldInTailPadding_InnerStructD1Ev(
    class FieldInTailPadding_InnerStruct* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN18FieldInTailPaddingC1ERKS_(
    class FieldInTailPadding* __this,
    const class FieldInTailPadding& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN18FieldInTailPaddingC1EOS_(
    class FieldInTailPadding* __this, class FieldInTailPadding&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN18FieldInTailPaddingD1Ev(
    class FieldInTailPadding* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class FieldInTailPadding&
__rust_thunk___ZN18FieldInTailPaddingaSERKS_(
    class FieldInTailPadding* __this,
    const class FieldInTailPadding& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class FieldInTailPadding&
__rust_thunk___ZN18FieldInTailPaddingaSEOS_(
    class FieldInTailPadding* __this, class FieldInTailPadding&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN18FieldInTailPaddingC1Eicc(
    class FieldInTailPadding* __this, int inner_int, char inner_char,
    char outer_char) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(inner_int)>(inner_int),
                       std::forward<decltype(inner_char)>(inner_char),
                       std::forward<decltype(outer_char)>(outer_char));
}

static_assert(sizeof(class Struct) == 8);
static_assert(alignof(class Struct) == 4);
static_assert(CRUBIT_OFFSET_OF(field1, class Struct) == 0);
static_assert(CRUBIT_OFFSET_OF(field2, class Struct) == 4);

static_assert(sizeof(class PaddingBetweenFields) == 8);
static_assert(alignof(class PaddingBetweenFields) == 4);
static_assert(CRUBIT_OFFSET_OF(field1, class PaddingBetweenFields) == 0);
static_assert(CRUBIT_OFFSET_OF(field2, class PaddingBetweenFields) == 4);

static_assert(sizeof(class FieldInTailPadding_InnerStruct) == 8);
static_assert(alignof(class FieldInTailPadding_InnerStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(inner_int_field,
                               class FieldInTailPadding_InnerStruct) == 0);
static_assert(CRUBIT_OFFSET_OF(inner_char_field,
                               class FieldInTailPadding_InnerStruct) == 4);

static_assert(sizeof(class FieldInTailPadding) == 8);
static_assert(alignof(class FieldInTailPadding) == 4);
static_assert(CRUBIT_OFFSET_OF(inner_struct, class FieldInTailPadding) == 0);
static_assert(CRUBIT_OFFSET_OF(char_in_tail_padding_of_prev_field,
                               class FieldInTailPadding) == 5);

#pragma clang diagnostic pop
