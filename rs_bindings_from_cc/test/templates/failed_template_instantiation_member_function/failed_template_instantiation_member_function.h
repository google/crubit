// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_H_

#pragma clang lifetime_elision

struct NoMethod final {};

struct HasMethodReturningVoid final {
  void Method() {}
};

struct HasMethodReturningInt final {
  int Method() { return 1; };
};

template <typename T>
struct A final {
  void NoOp(){};

  // It's important that the return type is `auto`, to exercise the return type
  // deduction logic in Crubit's importers/function.cc, which used to cause
  // Crubit to crash because the return type cannot be deduced.
  auto Call_MethodReturnAuto(T t) { return t.Method(); }

  int Call_MethodReturnInt(T t) { return t.Method(); }

  // TODO:(b/248542210): Currently, Crubit still imports the following method,
  // since `FailMethod` passes type checking (but would fail instantiation), and
  // Crubit doesn't check function templates recursively.
  // Uncomment the following block when the issue is addressed.
  // void Call_FailMethod(T t) {
  //   FailMethod();  // Similarly, this also fails for `FailStaticMethod();`.
  // }

  void Call_FailMethod_StaticAssert() {
    FailMethod();
    static_assert(false);
  }

  void Call_FailStaticMethod_StaticAssert() {
    FailStaticMethod();
    static_assert(false);
  }

  static void FailStaticMethod() { static_assert(false); }
  void FailMethod() { static_assert(false); }
};

using AForNoMethod = A<NoMethod>;
void inline InvokeNoOp(AForNoMethod a) { a.NoOp(); }

using AForHasMethodReturningVoid = A<HasMethodReturningVoid>;
auto inline InvokeMethodReturnAuto(AForHasMethodReturningVoid a) {
  a.Call_MethodReturnAuto(HasMethodReturningVoid{});
}

using AForHasMethodReturningInt = A<HasMethodReturningInt>;
int inline InvokeMethodReturnAutoAndInt(AForHasMethodReturningInt a) {
  auto t = HasMethodReturningInt{};
  a.Call_MethodReturnAuto(t);
  return a.Call_MethodReturnInt(t);
}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TEMPLATES_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_FAILED_TEMPLATE_INSTANTIATION_MEMBER_FUNCTION_H_
