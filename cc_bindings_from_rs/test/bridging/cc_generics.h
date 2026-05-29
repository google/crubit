// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_CC_GENERICS_H_
#define THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_CC_GENERICS_H_

namespace crubit {
namespace test {

template <typename T>
struct MyOptional {
  bool has_value;
  T value;
};

struct MyStatus {
  bool ok;
};

template <typename T>
struct MyStatusOr {
  bool has_value;
  T value;
};

template <typename T1, typename T2>
struct MyPair {
  T1 first;
  T2 second;
};

struct MyIntBoolPair {
  int first;
  bool second;
};

}  // namespace test
}  // namespace crubit

#endif  // THIRD_PARTY_CRUBIT_CC_BINDINGS_FROM_RS_TEST_BRIDGING_CC_GENERICS_H_
