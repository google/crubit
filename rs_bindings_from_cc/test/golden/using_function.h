// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USING_FUNCTION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USING_FUNCTION_H_

namespace A {
void Foo();
int Add(int a, int b);
void Overloaded(int x);
void Overloaded(float x);
}  // namespace A

namespace B {
using ::A::Foo;
/// A doc comment on the using-declaration.
using ::A::Add;
// Both overloads are brought in; the alias is ambiguous, just like the
// original overload set.
using ::A::Overloaded;
}  // namespace B

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_USING_FUNCTION_H_
