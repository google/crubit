// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_DERIVES_ABSTRACT_BASE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_DERIVES_ABSTRACT_BASE_H_

#include <optional>

class AbstractBase {
 public:
  virtual ~AbstractBase() = default;
  virtual int pure_virtual_method() = 0;
};

class DerivesAbstractBase : public AbstractBase {
 public:
  ~DerivesAbstractBase() override = default;
  int pure_virtual_method() override { return 42; }
};

inline std::optional<DerivesAbstractBase> get_optional() {
  return DerivesAbstractBase();
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BRIDGING_DERIVES_ABSTRACT_BASE_H_
