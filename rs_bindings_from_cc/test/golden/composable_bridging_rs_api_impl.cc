// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:composable_bridging_cc

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/composable_bridging.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z15ReturnCppStructv(
    unsigned char* __return_abi_buffer) {
  ::crubit::internal::Encode<::crubit::CppStructAbi>(
      ::crubit::CppStructAbi(), __return_abi_buffer, ReturnCppStruct());
}

static_assert((struct CppStruct (*)())&ReturnCppStruct);

extern "C" void __rust_thunk___Z13TakeCppStruct9CppStruct(
    const unsigned char* __param_0) {
  TakeCppStruct(::crubit::internal::Decode<::crubit::CppStructAbi>(
      ::crubit::CppStructAbi(), __param_0));
}

static_assert((void (*)(struct CppStruct))&TakeCppStruct);

static_assert(CRUBIT_SIZEOF(struct Vec3) == 12);
static_assert(alignof(struct Vec3) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct Vec3) == 0);
static_assert(CRUBIT_OFFSET_OF(y, struct Vec3) == 4);
static_assert(CRUBIT_OFFSET_OF(z, struct Vec3) == 8);

extern "C" void __rust_thunk___ZN4Vec3C1Ev(struct Vec3* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___Z16MakeOptionalVec3fffb(
    unsigned char* __return_abi_buffer, float x, float y, float z,
    bool is_present) {
  ::crubit::internal::Encode<
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>>(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>(
          ::crubit::TransmuteAbi<::Vec3>()),
      __return_abi_buffer, MakeOptionalVec3(x, y, z, is_present));
}

static_assert((struct MyOption<Vec3> (*)(float, float, float,
                                         bool))&MakeOptionalVec3);

extern "C" void __rust_thunk___Z11MapMultiply8MyOptionI4Vec3Ef(
    unsigned char* __return_abi_buffer, const unsigned char* v, float factor) {
  ::crubit::internal::Encode<
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>>(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>(
          ::crubit::TransmuteAbi<::Vec3>()),
      __return_abi_buffer,
      MapMultiply(::crubit::internal::Decode<
                      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>>(
                      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>(
                          ::crubit::TransmuteAbi<::Vec3>()),
                      v),
                  factor));
}

static_assert((struct MyOption<Vec3> (*)(struct MyOption<Vec3>,
                                         float))&MapMultiply);

extern "C" void __rust_thunk___Z14MakeMyI8Structv(
    unsigned char* __return_abi_buffer) {
  ::crubit::internal::Encode<
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::MyI8Struct>>>(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::MyI8Struct>>(
          ::crubit::TransmuteAbi<::MyI8Struct>()),
      __return_abi_buffer, MakeMyI8Struct());
}

static_assert((struct MyOption<MyI8Struct> (*)())&MakeMyI8Struct);

static_assert(
    (void (*)(::rs_std::SliceRef<class std::basic_string_view<
                  char, std::char_traits<char>>>))&InspectStringViews);

extern "C" void __rust_thunk___Z12MaybeVoidPtrv(
    unsigned char* __return_abi_buffer) {
  ::crubit::internal::Encode<
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<void*>>>(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<void*>>(
          ::crubit::TransmuteAbi<void*>()),
      __return_abi_buffer, MaybeVoidPtr());
}

static_assert((struct MyOption<void*> (*)())&MaybeVoidPtr);

extern "C" void
__rust_thunk___Z40AcceptsSliceAndReturnsStatusErrorIfEmptyN6rs_std8SliceRefIKiEE(
    unsigned char* __return_abi_buffer, ::rs_std::SliceRef<const int> slice) {
  ::crubit::internal::Encode<::crubit::MyOptionAbi<
      ::crubit::TransmuteAbi<::rs_std::SliceRef<const int>>>>(
      ::crubit::MyOptionAbi<
          ::crubit::TransmuteAbi<::rs_std::SliceRef<const int>>>(
          ::crubit::TransmuteAbi<::rs_std::SliceRef<const int>>()),
      __return_abi_buffer, AcceptsSliceAndReturnsStatusErrorIfEmpty(slice));
}

static_assert((struct MyOption<rs_std::SliceRef<const int>> (*)(
    ::rs_std::SliceRef<const int>))&AcceptsSliceAndReturnsStatusErrorIfEmpty);

extern "C" void __rust_thunk___Z16ReturnsCStrArrayv(
    unsigned char* __return_abi_buffer) {
  ::crubit::internal::Encode<
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<char const**>>>(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<char const**>>(
          ::crubit::TransmuteAbi<char const**>()),
      __return_abi_buffer, ReturnsCStrArray());
}

static_assert((struct MyOption<const char**> (*)())&ReturnsCStrArray);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<wchar_t, std::char_traits<wchar_t>>) ==
    16);
static_assert(
    alignof(class std::basic_string_view<wchar_t, std::char_traits<wchar_t>>) ==
    8);

#pragma clang diagnostic pop
