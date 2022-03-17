// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_NO_ELIDED_LIFETIMES_NO_ELIDED_LIFETIMES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_NO_ELIDED_LIFETIMES_NO_ELIDED_LIFETIMES_H_

// The two functions below help test interactions between safe Rust code
// and C++ code without lifetime annotations.  Here, `StorePointer` is not
// annotated with lifetimes (and therefore can stash a pointer to `int_ref`
// in a global variable, asking callers of `ReadStoredPointer` to "be careful").
//
// The function-under-test below takes `int_ref` as `const int&`, because:
// 1) In presence of lifetimes (no lifetimes below) references would
//    become references in the generated Rust bindings and we care
//    mostly about safety of using Rust references.
// 2) We want to test the simplest possible scenario that shows the unsafety
//    problem.  Therefore we test with `int` rather than with a struct.
//    NOLINTNEXTLINE(google3-readability-pass-trivial-by-value)
void StorePointer(const int& int_ref);
int ReadStoredPointer();

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_NO_ELIDED_LIFETIMES_NO_ELIDED_LIFETIMES_H_
