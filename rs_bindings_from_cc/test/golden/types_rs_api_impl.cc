// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/support/offsetof.h"
#include "rs_bindings_from_cc/test/golden/types.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN10SomeStructC1Ev(class SomeStruct* __this) {
  crubit::construct_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN10SomeStructC1ERKS_(
    class SomeStruct* __this, const class SomeStruct& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN10SomeStructC1EOS_(
    class SomeStruct* __this, class SomeStruct&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN10SomeStructD1Ev(class SomeStruct* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" class SomeStruct& __rust_thunk___ZN10SomeStructaSERKS_(
    class SomeStruct* __this, const class SomeStruct& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" class SomeStruct& __rust_thunk___ZN10SomeStructaSEOS_(
    class SomeStruct* __this, class SomeStruct&& __param_0) {
  return __this->operator=(std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN19FieldTypeTestStructC1ERKS_(
    class FieldTypeTestStruct* __this,
    const class FieldTypeTestStruct& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN19FieldTypeTestStructC1EOS_(
    class FieldTypeTestStruct* __this, class FieldTypeTestStruct&& __param_0) {
  crubit::construct_at(std::forward<decltype(__this)>(__this),
                       std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN19FieldTypeTestStructD1Ev(
    class FieldTypeTestStruct* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___Z21VoidReturningFunctionv() {
  VoidReturningFunction();
}

static_assert(sizeof(class SomeStruct) == 1);
static_assert(alignof(class SomeStruct) == 1);

static_assert(sizeof(class FieldTypeTestStruct) == 288);
static_assert(alignof(class FieldTypeTestStruct) == 8);
static_assert(CRUBIT_OFFSET_OF(bool_field, class FieldTypeTestStruct) == 0);
static_assert(CRUBIT_OFFSET_OF(char_field, class FieldTypeTestStruct) == 1);
static_assert(CRUBIT_OFFSET_OF(unsigned_char_field,
                               class FieldTypeTestStruct) == 2);
static_assert(CRUBIT_OFFSET_OF(signed_char_field, class FieldTypeTestStruct) ==
              3);
static_assert(CRUBIT_OFFSET_OF(char16_t_field, class FieldTypeTestStruct) == 4);
static_assert(CRUBIT_OFFSET_OF(char32_t_field, class FieldTypeTestStruct) == 8);
static_assert(CRUBIT_OFFSET_OF(wchar_t_field, class FieldTypeTestStruct) == 12);
static_assert(CRUBIT_OFFSET_OF(short_field, class FieldTypeTestStruct) == 16);
static_assert(CRUBIT_OFFSET_OF(int_field, class FieldTypeTestStruct) == 20);
static_assert(CRUBIT_OFFSET_OF(long_field, class FieldTypeTestStruct) == 24);
static_assert(CRUBIT_OFFSET_OF(long_long_field, class FieldTypeTestStruct) ==
              32);
static_assert(CRUBIT_OFFSET_OF(unsigned_short_field,
                               class FieldTypeTestStruct) == 40);
static_assert(CRUBIT_OFFSET_OF(unsigned_int_field, class FieldTypeTestStruct) ==
              44);
static_assert(CRUBIT_OFFSET_OF(unsigned_long_field,
                               class FieldTypeTestStruct) == 48);
static_assert(CRUBIT_OFFSET_OF(unsigned_long_long_field,
                               class FieldTypeTestStruct) == 56);
static_assert(CRUBIT_OFFSET_OF(signed_short_field, class FieldTypeTestStruct) ==
              64);
static_assert(CRUBIT_OFFSET_OF(signed_int_field, class FieldTypeTestStruct) ==
              68);
static_assert(CRUBIT_OFFSET_OF(signed_long_field, class FieldTypeTestStruct) ==
              72);
static_assert(CRUBIT_OFFSET_OF(signed_long_long_field,
                               class FieldTypeTestStruct) == 80);
static_assert(CRUBIT_OFFSET_OF(int8_t_field, class FieldTypeTestStruct) == 88);
static_assert(CRUBIT_OFFSET_OF(int16_t_field, class FieldTypeTestStruct) == 90);
static_assert(CRUBIT_OFFSET_OF(int32_t_field, class FieldTypeTestStruct) == 92);
static_assert(CRUBIT_OFFSET_OF(int64_t_field, class FieldTypeTestStruct) == 96);
static_assert(CRUBIT_OFFSET_OF(std_int8_t_field, class FieldTypeTestStruct) ==
              104);
static_assert(CRUBIT_OFFSET_OF(std_int16_t_field, class FieldTypeTestStruct) ==
              106);
static_assert(CRUBIT_OFFSET_OF(std_int32_t_field, class FieldTypeTestStruct) ==
              108);
static_assert(CRUBIT_OFFSET_OF(std_int64_t_field, class FieldTypeTestStruct) ==
              112);
static_assert(CRUBIT_OFFSET_OF(uint8_t_field, class FieldTypeTestStruct) ==
              120);
static_assert(CRUBIT_OFFSET_OF(uint16_t_field, class FieldTypeTestStruct) ==
              122);
static_assert(CRUBIT_OFFSET_OF(uint32_t_field, class FieldTypeTestStruct) ==
              124);
static_assert(CRUBIT_OFFSET_OF(uint64_t_field, class FieldTypeTestStruct) ==
              128);
static_assert(CRUBIT_OFFSET_OF(std_uint8_t_field, class FieldTypeTestStruct) ==
              136);
static_assert(CRUBIT_OFFSET_OF(std_uint16_t_field, class FieldTypeTestStruct) ==
              138);
static_assert(CRUBIT_OFFSET_OF(std_uint32_t_field, class FieldTypeTestStruct) ==
              140);
static_assert(CRUBIT_OFFSET_OF(std_uint64_t_field, class FieldTypeTestStruct) ==
              144);
static_assert(CRUBIT_OFFSET_OF(ptrdiff_t_field, class FieldTypeTestStruct) ==
              152);
static_assert(CRUBIT_OFFSET_OF(size_t_field, class FieldTypeTestStruct) == 160);
static_assert(CRUBIT_OFFSET_OF(intptr_t_field, class FieldTypeTestStruct) ==
              168);
static_assert(CRUBIT_OFFSET_OF(uintptr_t_field, class FieldTypeTestStruct) ==
              176);
static_assert(CRUBIT_OFFSET_OF(std_ptrdiff_t_field,
                               class FieldTypeTestStruct) == 184);
static_assert(CRUBIT_OFFSET_OF(std_size_t_field, class FieldTypeTestStruct) ==
              192);
static_assert(CRUBIT_OFFSET_OF(std_intptr_t_field, class FieldTypeTestStruct) ==
              200);
static_assert(CRUBIT_OFFSET_OF(std_uintptr_t_field,
                               class FieldTypeTestStruct) == 208);
static_assert(CRUBIT_OFFSET_OF(float_field, class FieldTypeTestStruct) == 216);
static_assert(CRUBIT_OFFSET_OF(double_field, class FieldTypeTestStruct) == 224);
static_assert(CRUBIT_OFFSET_OF(ptr_field, class FieldTypeTestStruct) == 232);
static_assert(CRUBIT_OFFSET_OF(struct_field, class FieldTypeTestStruct) == 240);
static_assert(CRUBIT_OFFSET_OF(struct_ptr_field, class FieldTypeTestStruct) ==
              248);
static_assert(CRUBIT_OFFSET_OF(const_struct_ptr_field,
                               class FieldTypeTestStruct) == 256);
static_assert(CRUBIT_OFFSET_OF(struct_ref_field, class FieldTypeTestStruct) ==
              264);
static_assert(CRUBIT_OFFSET_OF(const_struct_ref_field,
                               class FieldTypeTestStruct) == 272);
static_assert(CRUBIT_OFFSET_OF(forward_declared_ptr_field,
                               class FieldTypeTestStruct) == 280);

#pragma clang diagnostic pop
