// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMPOSABLE_BRIDGING_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMPOSABLE_BRIDGING_H_

// Note: a real example would require that Crubit implements CrubitAbiTrait in
// order for the generated code to properly compile. This example just serves to
// illustrate what the generated code will look like.
struct
    // clang-format off
    [[clang::annotate("crubit_bridge_rust_name", "RustStruct")]]
    [[clang::annotate("crubit_bridge_abi_rust", "RustStructAbi")]]
    [[clang::annotate("crubit_bridge_abi_cpp", "::crubit::CppStructAbi")]]
    // clang-format on
    CppStruct {};

CppStruct ReturnCppStruct();

void TakeCppStruct(CppStruct);

template <typename T>
// clang-format off
struct
    [[clang::annotate("crubit_bridge_rust_name", "MyOption")]]
    [[clang::annotate("crubit_bridge_abi_rust", "MyOptionAbi")]]
    [[clang::annotate("crubit_bridge_abi_cpp", "::crubit::MyOptionAbi")]]
// clang-format on
MyOption {
  // std::optional<T> value;
};

struct Vec3 {
  float x;
  float y;
  float z;
};

MyOption<Vec3> MakeOptionalVec3(float x, float y, float z, bool is_present);

MyOption<Vec3> MapMultiply(MyOption<Vec3> v, float factor);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMPOSABLE_BRIDGING_H_
