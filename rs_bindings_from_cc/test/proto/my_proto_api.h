// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_PROTO_TEST_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_PROTO_TEST_H_

#include <cstdint>

#include "rs_bindings_from_cc/test/proto/my.proto.h"

namespace test {

inline my_package::MyMessage ReturnValue() {
  my_package::MyMessage msg;
  msg.set_my_num(123);
  return msg;
}

inline int64_t ExtractFromValue(my_package::MyMessage msg) {
  return msg.my_num();
}

inline int64_t ExtractFromConstPtr(const my_package::MyMessage* msg) {
  return msg->my_num();
}

inline int64_t ExtractFromConstRef(const my_package::MyMessage& msg) {
  return msg.my_num();
}

inline int64_t ExtractFromMutablePtr(my_package::MyMessage* msg) {
  return msg->my_num();
}
inline int64_t ExtractFromMutableRef(my_package::MyMessage& msg) {
  return msg.my_num();
};

}  // namespace test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_PROTO_TEST_H_
