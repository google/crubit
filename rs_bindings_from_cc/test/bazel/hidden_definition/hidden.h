// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HIDDEN_DEFINITION_HIDDEN_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HIDDEN_DEFINITION_HIDDEN_H_

#include <memory>

namespace hidden_ns {

class Payload {
 public:
  virtual ~Payload() = default;
};

struct Holder {
  explicit Holder(std::unique_ptr<Payload> payload);
  std::unique_ptr<Payload> TakePayload();

  std::unique_ptr<Payload> payload;
  int value = 0;
};

}  // namespace hidden_ns

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_HIDDEN_DEFINITION_HIDDEN_H_
