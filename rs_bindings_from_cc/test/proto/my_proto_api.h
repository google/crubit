// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_PROTO_TEST_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_PROTO_TEST_H_

#include <cstdint>

#include "rs_bindings_from_cc/test/proto/my.proto.h"

namespace test {
inline void SetStringOnMyMessage(my_package::MyMessage* msg, int64_t num) {
  msg->set_my_num(num);
}
}  // namespace test

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_PROTO_TEST_H_
