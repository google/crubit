// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ESCAPING_KEYWORDS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ESCAPING_KEYWORDS_H_

#pragma clang lifetime_elision

struct type final {
  int dyn;
};

void impl(int match);

template <typename async>
struct await {
  async crate;
};

template <typename Self>
Self become(Self where);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_ESCAPING_KEYWORDS_H_
