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
  ::crubit::internal::Encode<::crubit::CppStructAbi>(__return_abi_buffer,
                                                     ReturnCppStruct());
}

extern "C" void __rust_thunk___Z13TakeCppStruct9CppStruct(
    const unsigned char* __param_0) {
  TakeCppStruct(::crubit::internal::Decode<::crubit::CppStructAbi>(__param_0));
}

static_assert(CRUBIT_SIZEOF(struct Vec3) == 12);
static_assert(alignof(struct Vec3) == 4);
static_assert(CRUBIT_OFFSET_OF(x, struct Vec3) == 0);
static_assert(CRUBIT_OFFSET_OF(y, struct Vec3) == 4);
static_assert(CRUBIT_OFFSET_OF(z, struct Vec3) == 8);

extern "C" void __rust_thunk___ZN4Vec3C1Ev(struct Vec3* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___ZN4Vec3C1EOS_(struct Vec3* __this,
                                             struct Vec3* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" struct Vec3* __rust_thunk___ZN4Vec3aSERKS_(
    struct Vec3* __this, const struct Vec3* __param_0) {
  return &__this->operator=(*__param_0);
}

extern "C" struct Vec3* __rust_thunk___ZN4Vec3aSEOS_(struct Vec3* __this,
                                                     struct Vec3* __param_0) {
  return &__this->operator=(std::move(*__param_0));
}

extern "C" void __rust_thunk___Z16MakeOptionalVec3fffb(
    unsigned char* __return_abi_buffer, float x, float y, float z,
    bool is_present) {
  ::crubit::internal::Encode<
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>>(
      __return_abi_buffer, MakeOptionalVec3(x, y, z, is_present));
}

extern "C" void __rust_thunk___Z11MapMultiply8MyOptionI4Vec3Ef(
    unsigned char* __return_abi_buffer, const unsigned char* v, float factor) {
  ::crubit::internal::Encode<
      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>>(
      __return_abi_buffer,
      MapMultiply(::crubit::internal::Decode<
                      ::crubit::MyOptionAbi<::crubit::TransmuteAbi<::Vec3>>>(v),
                  factor));
}

#pragma clang diagnostic pop
