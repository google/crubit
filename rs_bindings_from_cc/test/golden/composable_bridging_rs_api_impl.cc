// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:composable_bridging_cc

#include "support/bridge.h"
#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"
#include "support/internal/slot.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/composable_bridging.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void __rust_thunk___Z15ReturnCppStructv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(::crubit::CppStructAbi::kSize,
                                     __return_abi_buffer);
  ::crubit::CppStructAbi().Encode(ReturnCppStruct(), __return_encoder);
}

static_assert((struct CppStruct (*)()) & ::ReturnCppStruct);

extern "C" void __rust_thunk___Z13TakeCppStruct9CppStruct(
    const unsigned char* __param_0) {
  ::crubit::Decoder ____param_0_decoder(::crubit::CppStructAbi::kSize,
                                        __param_0);
  TakeCppStruct(::crubit::CppStructAbi().Decode(____param_0_decoder));
}

static_assert((void (*)(struct CppStruct)) & ::TakeCppStruct);

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
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>(
      ::crubit::TransmuteAbi<::Vec3>())
      .Encode(MakeOptionalVec3(x, y, z, is_present), __return_encoder);
}

static_assert((struct MyOption<Vec3> (*)(float, float, float, bool)) &
              ::MakeOptionalVec3);

extern "C" void __rust_thunk___Z11MapMultiply8MyOptionI4Vec3Ef(
    unsigned char* __return_abi_buffer, const unsigned char* v, float factor) {
  ::crubit::Decoder __v_decoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>::kSize, v);
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>(
      ::crubit::TransmuteAbi<::Vec3>())
      .Encode(MapMultiply(::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>(
                              ::crubit::TransmuteAbi<::Vec3>())
                              .Decode(__v_decoder),
                          factor),
              __return_encoder);
}

static_assert((struct MyOption<Vec3> (*)(struct MyOption<Vec3>, float)) &
              ::MapMultiply);

extern "C" void __rust_thunk___Z14MakeMyI8Structv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::MyI8Struct>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::MyI8Struct>>(
      ::crubit::TransmuteAbi<::MyI8Struct>())
      .Encode(MakeMyI8Struct(), __return_encoder);
}

static_assert((struct MyOption<MyI8Struct> (*)()) & ::MakeMyI8Struct);

static_assert(
    (void (*)(::rs_std::SliceRef<
              class std::basic_string_view<char, std::char_traits<char>>>)) &
    ::InspectStringViews);

extern "C" void __rust_thunk___Z12MaybeVoidPtrv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<void*>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<void*>>(
      ::crubit::TransmuteAbi<void*>())
      .Encode(MaybeVoidPtr(), __return_encoder);
}

static_assert((struct MyOption<void*> (*)()) & ::MaybeVoidPtr);

extern "C" void
__rust_thunk___Z40AcceptsSliceAndReturnsStatusErrorIfEmptyN6rs_std8SliceRefIKiEE(
    unsigned char* __return_abi_buffer, ::rs_std::SliceRef<const int> slice) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<
          ::crubit::TransmuteAbi<::rs_std::SliceRef<const int>>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::rs_std::SliceRef<const int>>>(
      ::crubit::TransmuteAbi<::rs_std::SliceRef<const int>>())
      .Encode(AcceptsSliceAndReturnsStatusErrorIfEmpty(slice),
              __return_encoder);
}

static_assert((struct MyOption<rs_std::SliceRef<const int>> (*)(
                  ::rs_std::SliceRef<const int>)) &
              ::AcceptsSliceAndReturnsStatusErrorIfEmpty);

extern "C" void __rust_thunk___Z16ReturnsCStrArrayv(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<char const**>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<char const**>>(
      ::crubit::TransmuteAbi<char const**>())
      .Encode(ReturnsCStrArray(), __return_encoder);
}

static_assert((struct MyOption<const char**> (*)()) & ::ReturnsCStrArray);

extern "C" void __rust_thunk___Z40ReturnsDefaultEnumInComposableBridgeTypev(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::DefaultEnum>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::DefaultEnum>>(
      ::crubit::TransmuteAbi<::DefaultEnum>())
      .Encode(ReturnsDefaultEnumInComposableBridgeType(), __return_encoder);
}

static_assert((struct MyOption<DefaultEnum> (*)()) &
              ::ReturnsDefaultEnumInComposableBridgeType);

extern "C" void __rust_thunk___Z36ReturnsI64EnumInComposableBridgeTypev(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::I64Enum>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::I64Enum>>(
      ::crubit::TransmuteAbi<::I64Enum>())
      .Encode(ReturnsI64EnumInComposableBridgeType(), __return_encoder);
}

static_assert((struct MyOption<I64Enum> (*)()) &
              ::ReturnsI64EnumInComposableBridgeType);

extern "C" void __rust_thunk___Z44ReturnsEnumInNamespaceInComposableBridgeTypev(
    unsigned char* __return_abi_buffer) {
  ::crubit::Encoder __return_encoder(
      ::crubit::MyOptionAbi<
          ::crubit::TransmuteAbi<::some_namespace::EnumInNamespace>>::kSize,
      __return_abi_buffer);
  ::crubit::MyOptionAbi<
      ::crubit::TransmuteAbi<::some_namespace::EnumInNamespace>>(
      ::crubit::TransmuteAbi<::some_namespace::EnumInNamespace>())
      .Encode(ReturnsEnumInNamespaceInComposableBridgeType(), __return_encoder);
}

static_assert((struct MyOption<some_namespace::EnumInNamespace> (*)()) &
              ::ReturnsEnumInNamespaceInComposableBridgeType);

static_assert(
    CRUBIT_SIZEOF(
        class std::basic_string_view<wchar_t, std::char_traits<wchar_t>>) ==
    16);
static_assert(
    alignof(class std::basic_string_view<wchar_t, std::char_traits<wchar_t>>) ==
    8);

#pragma clang diagnostic pop
