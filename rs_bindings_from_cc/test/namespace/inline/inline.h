// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_NAMESPACE_INLINE_INLINE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_NAMESPACE_INLINE_INLINE_H_

#pragma clang lifetime_elision

namespace foo {
inline namespace inline1 {

struct MyStruct final {
  int value;
};

inline int GetStructValue1(const foo::MyStruct& s) { return s.value; }

inline int GetStructValue2(const foo::inline1::MyStruct& s) { return s.value; }

}  // namespace inline1

// Test coverage for the case where additional declarations appear in `inline1`,
// but without `inline namespace ...`, just with `namespace inline1`.
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Winline-namespace-reopened-noninline"
namespace inline1 {
inline int GetStructValue3(const foo::MyStruct& s) { return s.value; }
inline int GetStructValue4(const foo::inline1::MyStruct& s) { return s.value; }
}  // namespace inline1
#pragma clang diagnostic pop
}  // namespace foo

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_NAMESPACE_INLINE_INLINE_H_
