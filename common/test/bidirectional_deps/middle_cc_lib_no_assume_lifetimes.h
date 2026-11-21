// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_COMMON_TEST_MIDDLE_CC_LIB_NO_ASSUME_LIFETIMES_H_
#define CRUBIT_COMMON_TEST_MIDDLE_CC_LIB_NO_ASSUME_LIFETIMES_H_

#include <utility>

#include "common/test/bidirectional_deps/leaf_rs_lib.h"

using NoAssumeAlias = leaf_rs_lib::LeafRsLifetime;

struct NoAssumeHolder {
  leaf_rs_lib::LeafRsLifetime field;
};

inline leaf_rs_lib::LeafRsLifetime MakeNoAssumeLifetime() {
  return leaf_rs_lib::static_lifetime();
}

inline unsigned char TakeNoAssumeLifetime(leaf_rs_lib::LeafRsLifetime x) {
  return leaf_rs_lib::unwrap_lifetime(std::move(x));
}

inline NoAssumeAlias MakeNoAssumeAlias() {
  return leaf_rs_lib::static_lifetime();
}

inline unsigned char TakeNoAssumeAlias(NoAssumeAlias x) {
  return leaf_rs_lib::unwrap_lifetime(std::move(x));
}

inline NoAssumeHolder MakeNoAssumeHolder() {
  return NoAssumeHolder{leaf_rs_lib::static_lifetime()};
}

inline leaf_rs_lib::LeafRsBounded MakeNoAssumeBounded() {
  static const unsigned char kValue = 5;
  return leaf_rs_lib::wrap_bounded(&kValue, &kValue);
}

inline leaf_rs_lib::LeafRsStaticBound MakeNoAssumeStaticBound() {
  static const unsigned char kValue = 6;
  return leaf_rs_lib::wrap_static_bound(&kValue);
}

#endif  // CRUBIT_COMMON_TEST_MIDDLE_CC_LIB_NO_ASSUME_LIFETIMES_H_
