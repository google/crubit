// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_ELIDED_LIFETIMES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_ELIDED_LIFETIMES_H_

// With `#pragma clang lifetime_elision` the `rs_type` representation of
// constructor parameters in the IR will change as follows:
// - s/ *SomeStruct / &mut 'a SomeStruct /  (the `__this` parameter)
// - s/ *const SomeStruct / &'b SomeStruct /  (the copy constructor parameter)
#pragma clang lifetime_elision

struct ElidedLifetimes {
  ElidedLifetimes() : int_field(456) {}
  // TODO(lukasza): Add a copy constructor (to be mapped to Clone?)
  explicit ElidedLifetimes(int i) : int_field(i) {}
  int int_field;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_CONSTRUCTORS_ELIDED_LIFETIMES_H_
