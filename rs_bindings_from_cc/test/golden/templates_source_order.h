// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_SOURCE_ORDER_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_SOURCE_ORDER_H_

template <typename T>
class MyTemplate final {
  T t;

 public:
  void processT(T t);
};

struct TopLevel final {};

using Alias1 = MyTemplate<int>;
using Alias2 = MyTemplate<float>;
using Alias3 = MyTemplate<TopLevel>;
using Alias4 = MyTemplate<double>;
using Alias5 = MyTemplate<bool>;
using Alias6 = MyTemplate<MyTemplate<TopLevel>>;

namespace test_namespace_bindings {
struct Inner final {};
using Alias7 = MyTemplate<char>;
using Alias8 = MyTemplate<Inner>;
using Alias9 = MyTemplate<MyTemplate<Inner>>;
}  // namespace test_namespace_bindings

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TEMPLATES_SOURCE_ORDER_H_
