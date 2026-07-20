// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/bazel_unit_tests/cycle_test/external_dependency.h"
#include "rs_bindings_from_cc/test/bazel_unit_tests/cycle_test/library_to_bind.h"

int MyTypeAdder(int a, int b) {
  MyType m;
  return m.Add(a, b) + other_func();
}

// TODO(b/497927944): Rust APIs which use this type won't get bindings back into
// C++, as this type has no C++ target / header, only the .cc definition inside
// Crubit. (See `types_in_extra_cpp_srcs` golden test for details).
struct MyStruct {
  int a;
  int b;
};

int MyStructAdder(MyStruct x) { return x.a + x.b; }
