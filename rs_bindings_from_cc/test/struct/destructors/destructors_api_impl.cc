// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/struct/destructors:destructors
// Features: do_not_hardcode_status_bridge, infer_operator_lifetimes, std_unique_ptr, std_vector, supported

#include "support/internal/cxx20_backports.h"
#include "support/internal/offsetof.h"
#include "support/internal/sizeof.h"

#include <cstddef>
#include <memory>

// Public headers of the C++ library being wrapped.
#include "rs_bindings_from_cc/test/struct/destructors/destructors.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"

static_assert(CRUBIT_SIZEOF(class DestructionOrderRecorder) == 4);
static_assert(alignof(class DestructionOrderRecorder) == 4);

extern "C" void __rust_thunk___ZN24DestructionOrderRecorderC1Ei(
    class DestructionOrderRecorder* __this, int int_field) {
  crubit::construct_at(__this, int_field);
}

extern "C" void __rust_thunk___ZN24DestructionOrderRecorderC1EOS_(
    class DestructionOrderRecorder* __this,
    class DestructionOrderRecorder* other) {
  crubit::construct_at(__this, std::move(*other));
}

extern "C" class DestructionOrderRecorder*
__rust_thunk___ZN24DestructionOrderRecorderaSEOS_(
    class DestructionOrderRecorder* __this,
    class DestructionOrderRecorder* other) {
  return std::addressof(__this->operator=(std::move(*other)));
}

extern "C" void __rust_thunk___ZN24DestructionOrderRecorderD1Ev(
    class DestructionOrderRecorder* __this) {
  std::destroy_at(__this);
}

static_assert((void (*)(int))&DestructionOrderRecorder::RecordDestruction);

static_assert((int (*)())&DestructionOrderRecorder::GetDestructionRecord);

static_assert((void (*)())&DestructionOrderRecorder::ClearDestructionRecord);

static_assert(CRUBIT_SIZEOF(class FieldDestructionOrderTester) == 12);
static_assert(alignof(class FieldDestructionOrderTester) == 4);

extern "C" void __rust_thunk___ZN27FieldDestructionOrderTesterC1EOS_(
    class FieldDestructionOrderTester* __this,
    class FieldDestructionOrderTester* __param_0) {
  crubit::construct_at(__this, std::move(*__param_0));
}

extern "C" void __rust_thunk___ZN27FieldDestructionOrderTesterD1Ev(
    class FieldDestructionOrderTester* __this) {
  std::destroy_at(__this);
}

extern "C" class FieldDestructionOrderTester*
__rust_thunk___ZN27FieldDestructionOrderTesteraSEOS_(
    class FieldDestructionOrderTester* __this,
    class FieldDestructionOrderTester* __param_0) {
  return std::addressof(__this->operator=(std::move(*__param_0)));
}

extern "C" void
__rust_thunk___ZN27FieldDestructionOrderTester6CreateE24DestructionOrderRecorderS0_S0_(
    class FieldDestructionOrderTester* __return,
    class DestructionOrderRecorder* field1,
    class DestructionOrderRecorder* field2,
    class DestructionOrderRecorder* field3) {
  new (__return) auto(FieldDestructionOrderTester::Create(
      std::move(*field1), std::move(*field2), std::move(*field3)));
}

static_assert((class FieldDestructionOrderTester (*)(
    class DestructionOrderRecorder, class DestructionOrderRecorder,
    class DestructionOrderRecorder))&FieldDestructionOrderTester::Create);

extern "C" void
__rust_thunk___ZN27FieldDestructionOrderTester15DestructFromCppEiii(
    int field1, int field2, int field3) {
  FieldDestructionOrderTester::DestructFromCpp(field1, field2, field3);
}

static_assert((void (*)(int, int,
                        int))&FieldDestructionOrderTester::DestructFromCpp);

#pragma clang diagnostic pop
