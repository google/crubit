// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:bridge_type_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/lazy_init.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/bridge_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

extern "C" void cpp_to_rust_converter(void* cpp_struct, void* rust_struct);
extern "C" void __rust_thunk___Z15ReturnCppStructv(void* __return) {
  auto __original_cpp_struct = ReturnCppStruct();
  cpp_to_rust_converter(&__original_cpp_struct, __return);
}

static_assert((struct CppStruct (*)()) & ::ReturnCppStruct);

extern "C" void rust_to_cpp_converter(void* rust_struct, void* cpp_struct);
extern "C" void __rust_thunk___Z13TakeCppStruct9CppStruct(void* __param_0) {
  ::crubit::LazyInit<struct CppStruct> __converted___param_0;
  rust_to_cpp_converter(__param_0, &__converted___param_0.val);
  TakeCppStruct(std::move(*&(__converted___param_0.val)));
}

static_assert((void (*)(struct CppStruct)) & ::TakeCppStruct);

#pragma clang diagnostic pop
