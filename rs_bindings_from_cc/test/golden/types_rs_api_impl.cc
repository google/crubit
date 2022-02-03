// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/types.h"

extern "C" void __rust_thunk___ZN10SomeStructC1Ev(class SomeStruct* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN10SomeStructC1ERKS_(
    class SomeStruct* __this, const class SomeStruct& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN10SomeStructD1Ev(class SomeStruct* __this) {
  std ::destroy_at(__this);
}
extern "C" class SomeStruct& __rust_thunk___ZN10SomeStructaSERKS_(
    class SomeStruct* __this, const class SomeStruct& __param_0) {
  return __this->operator=(__param_0);
}
extern "C" void __rust_thunk___ZN19FieldTypeTestStructC1ERKS_(
    class FieldTypeTestStruct* __this,
    const class FieldTypeTestStruct& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN19FieldTypeTestStructD1Ev(
    class FieldTypeTestStruct* __this) {
  std ::destroy_at(__this);
}
extern "C" void __rust_thunk___Z21VoidReturningFunctionv() {
  VoidReturningFunction();
}

static_assert(sizeof(class SomeStruct) == 1);
static_assert(alignof(class SomeStruct) == 1);

static_assert(sizeof(class FieldTypeTestStruct) == 280);
static_assert(alignof(class FieldTypeTestStruct) == 8);
static_assert(offsetof(class FieldTypeTestStruct, bool_field) * 8 == 0);
static_assert(offsetof(class FieldTypeTestStruct, char_field) * 8 == 8);
static_assert(offsetof(class FieldTypeTestStruct, unsigned_char_field) * 8 ==
              16);
static_assert(offsetof(class FieldTypeTestStruct, signed_char_field) * 8 == 24);
static_assert(offsetof(class FieldTypeTestStruct, char16_t_field) * 8 == 32);
static_assert(offsetof(class FieldTypeTestStruct, char32_t_field) * 8 == 64);
static_assert(offsetof(class FieldTypeTestStruct, wchar_t_field) * 8 == 96);
static_assert(offsetof(class FieldTypeTestStruct, short_field) * 8 == 128);
static_assert(offsetof(class FieldTypeTestStruct, int_field) * 8 == 160);
static_assert(offsetof(class FieldTypeTestStruct, long_field) * 8 == 192);
static_assert(offsetof(class FieldTypeTestStruct, long_long_field) * 8 == 256);
static_assert(offsetof(class FieldTypeTestStruct, unsigned_short_field) * 8 ==
              320);
static_assert(offsetof(class FieldTypeTestStruct, unsigned_int_field) * 8 ==
              352);
static_assert(offsetof(class FieldTypeTestStruct, unsigned_long_field) * 8 ==
              384);
static_assert(offsetof(class FieldTypeTestStruct, unsigned_long_long_field) *
                  8 ==
              448);
static_assert(offsetof(class FieldTypeTestStruct, signed_short_field) * 8 ==
              512);
static_assert(offsetof(class FieldTypeTestStruct, signed_int_field) * 8 == 544);
static_assert(offsetof(class FieldTypeTestStruct, signed_long_field) * 8 ==
              576);
static_assert(offsetof(class FieldTypeTestStruct, signed_long_long_field) * 8 ==
              640);
static_assert(offsetof(class FieldTypeTestStruct, int8_t_field) * 8 == 704);
static_assert(offsetof(class FieldTypeTestStruct, int16_t_field) * 8 == 720);
static_assert(offsetof(class FieldTypeTestStruct, int32_t_field) * 8 == 736);
static_assert(offsetof(class FieldTypeTestStruct, int64_t_field) * 8 == 768);
static_assert(offsetof(class FieldTypeTestStruct, std_int8_t_field) * 8 == 832);
static_assert(offsetof(class FieldTypeTestStruct, std_int16_t_field) * 8 ==
              848);
static_assert(offsetof(class FieldTypeTestStruct, std_int32_t_field) * 8 ==
              864);
static_assert(offsetof(class FieldTypeTestStruct, std_int64_t_field) * 8 ==
              896);
static_assert(offsetof(class FieldTypeTestStruct, uint8_t_field) * 8 == 960);
static_assert(offsetof(class FieldTypeTestStruct, uint16_t_field) * 8 == 976);
static_assert(offsetof(class FieldTypeTestStruct, uint32_t_field) * 8 == 992);
static_assert(offsetof(class FieldTypeTestStruct, uint64_t_field) * 8 == 1024);
static_assert(offsetof(class FieldTypeTestStruct, std_uint8_t_field) * 8 ==
              1088);
static_assert(offsetof(class FieldTypeTestStruct, std_uint16_t_field) * 8 ==
              1104);
static_assert(offsetof(class FieldTypeTestStruct, std_uint32_t_field) * 8 ==
              1120);
static_assert(offsetof(class FieldTypeTestStruct, std_uint64_t_field) * 8 ==
              1152);
static_assert(offsetof(class FieldTypeTestStruct, ptrdiff_t_field) * 8 == 1216);
static_assert(offsetof(class FieldTypeTestStruct, size_t_field) * 8 == 1280);
static_assert(offsetof(class FieldTypeTestStruct, intptr_t_field) * 8 == 1344);
static_assert(offsetof(class FieldTypeTestStruct, uintptr_t_field) * 8 == 1408);
static_assert(offsetof(class FieldTypeTestStruct, std_ptrdiff_t_field) * 8 ==
              1472);
static_assert(offsetof(class FieldTypeTestStruct, std_size_t_field) * 8 ==
              1536);
static_assert(offsetof(class FieldTypeTestStruct, std_intptr_t_field) * 8 ==
              1600);
static_assert(offsetof(class FieldTypeTestStruct, std_uintptr_t_field) * 8 ==
              1664);
static_assert(offsetof(class FieldTypeTestStruct, float_field) * 8 == 1728);
static_assert(offsetof(class FieldTypeTestStruct, double_field) * 8 == 1792);
static_assert(offsetof(class FieldTypeTestStruct, ptr_field) * 8 == 1856);
static_assert(offsetof(class FieldTypeTestStruct, struct_field) * 8 == 1920);
static_assert(offsetof(class FieldTypeTestStruct, struct_ptr_field) * 8 ==
              1984);
static_assert(offsetof(class FieldTypeTestStruct, const_struct_ptr_field) * 8 ==
              2048);
static_assert(offsetof(class FieldTypeTestStruct, struct_ref_field) * 8 ==
              2112);
static_assert(offsetof(class FieldTypeTestStruct, const_struct_ref_field) * 8 ==
              2176);
