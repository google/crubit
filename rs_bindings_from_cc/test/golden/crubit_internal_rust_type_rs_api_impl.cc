// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:crubit_internal_rust_type_cc

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/golden/crubit_internal_rust_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(sizeof(struct ExistingRustTypeFieldTypes) == 5);
static_assert(alignof(struct ExistingRustTypeFieldTypes) == 1);
static_assert(CRUBIT_OFFSET_OF(my_i8_struct,
                               struct ExistingRustTypeFieldTypes) == 0);
static_assert(CRUBIT_OFFSET_OF(my_i8_class,
                               struct ExistingRustTypeFieldTypes) == 1);
static_assert(CRUBIT_OFFSET_OF(my_i8_enum, struct ExistingRustTypeFieldTypes) ==
              2);
static_assert(CRUBIT_OFFSET_OF(my_i8_alias,
                               struct ExistingRustTypeFieldTypes) == 3);
static_assert(CRUBIT_OFFSET_OF(error, struct ExistingRustTypeFieldTypes) == 4);

extern "C" void __rust_thunk___ZN26ExistingRustTypeFieldTypesC1Ev(
    struct ExistingRustTypeFieldTypes* __this) {
  crubit::construct_at(__this);
}

extern "C" void __rust_thunk___Z12AcceptPtrInt3PtrIiE(Ptr<int>* ptr) {
  AcceptPtrInt(std::move(*ptr));
}

static_assert((void (*)(Ptr<int>)) & ::AcceptPtrInt);

extern "C" void
__rust_thunk___Z29AcceptCppTypeWithTemplateArgs23CppTypeWithTemplateArgsIifLb1EE(
    CppTypeWithTemplateArgs<int, float, true>* cpp_type) {
  AcceptCppTypeWithTemplateArgs(std::move(*cpp_type));
}

static_assert((void (*)(CppTypeWithTemplateArgs<int, float, true>)) &
              ::AcceptCppTypeWithTemplateArgs);

extern "C" void __rust_thunk___Z15AcceptReordered11ConvertPtrsIfiE(
    ConvertPtrs<float, int>* x) {
  AcceptReordered(std::move(*x));
}

static_assert((void (*)(ConvertPtrs<float, int>)) & ::AcceptReordered);

extern "C" void __rust_thunk___Z17AcceptWithDefault11WithDefaultIfiE(
    WithDefault<float>* x) {
  AcceptWithDefault(std::move(*x));
}

static_assert((void (*)(WithDefault<float>)) & ::AcceptWithDefault);

extern "C" void __rust_thunk___Z17AcceptSpecialized11MyContainerIiES_IvE(
    MyContainer<int>* a, MyContainer<void>* b) {
  AcceptSpecialized(std::move(*a), std::move(*b));
}

static_assert((void (*)(MyContainer<int>, MyContainer<void>)) &
              ::AcceptSpecialized);

#pragma clang diagnostic pop
