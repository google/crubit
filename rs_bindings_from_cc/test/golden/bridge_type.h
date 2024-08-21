// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BRIDGE_TYPE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BRIDGE_TYPE_H_

struct [[clang::annotate("crubit_bridge_type", "RustStruct"),
         clang::annotate("crubit_bridge_type_rust_to_cpp_converter",
                         "rust_to_cpp_converter"),
         clang::annotate("crubit_bridge_type_cpp_to_rust_converter",
                         "cpp_to_rust_converter")]] CppStruct {
  ~CppStruct();
};

CppStruct ReturnCppStruct();

void TakeCppStruct(CppStruct);

void TakeCppStructByPtr(CppStruct*);

CppStruct* ReturnCppStructByPtr();

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BRIDGE_TYPE_H_
