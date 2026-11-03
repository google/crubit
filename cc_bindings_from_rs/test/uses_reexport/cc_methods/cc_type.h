// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_REEXPORT_CC_METHODS_CC_TYPE_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_REEXPORT_CC_METHODS_CC_TYPE_H_

namespace cc_ns {

struct CcType {
  int value;
  int GetValue() const { return value; }
  void SetValue(int new_value) { value = new_value; }
  static CcType Make(int value) { return CcType{value}; }
};

}  // namespace cc_ns

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_USES_REEXPORT_CC_METHODS_CC_TYPE_H_
