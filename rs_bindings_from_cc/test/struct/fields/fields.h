// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FIELDS_FIELDS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FIELDS_FIELDS_H_

struct SomeStruct final {
  char char_var;
  int int_var;
};

// Make sure we can deal with the case where a variable declaration shadows the
// struct declaration.
// There's a fairly common case where this happens in real-world code:
// <time.h> defines `extern long timezone`, while <sys/time.h> defines
// `struct timezone`.
extern int SomeStruct;

class SomeClass final {
 public:
  int public_field = 0;

 private:
  int private_field_ = 0;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_FIELDS_FIELDS_H_
