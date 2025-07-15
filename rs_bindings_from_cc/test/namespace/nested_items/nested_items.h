// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_NAMESPACE_NESTED_ITEMS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_NAMESPACE_NESTED_ITEMS_H_

#include "support/annotations.h"

namespace same {
inline int AFunction() { return 42; }
}  // namespace same

class Same {
 public:
  struct NestedItem {
    int NestedItemFunction() { return 42; }
  };

  enum class NestedEnum {
    kOne,
    kTwo,
  };

  int Method() { return 9001; }
};

namespace foo {
struct Foo {
  struct foo {
    struct Item {};
    static inline int BFunction() { return 41; }
  };
};
}  // namespace foo

struct CRUBIT_RUST_NAME("OuterRustName") OuterCpp {
  struct Inner {};
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_NAMESPACE_NESTED_ITEMS_H_
