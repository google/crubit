// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// File comment

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMMENT_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMMENT_H_

#pragma clang lifetime_elision

// TODO(b/202933018): Re-enable once namespaces are supported
// namespace ns {
// a

/// Foo
struct Foo final {
  // TODO(rosica): This comment appears near fields of a struct, and
  // is currently generated below the struct definiton on the Rust side.

  /// A field
  int i;

  // TODO(rosica): This comment appears between fields of a struct, and
  // is currently generated below the struct definiton on the Rust side.

  /// Another field
  int j;

  // TODO(rosica): This comment appears near fields of a struct, and
  // is currently generated below the struct definiton on the Rust side.
};

// b

// }  // namespace ns

// c

/// foo
inline void foo() {
  // ignore
}

/// Bar
struct Bar final {
  int i;
};

// d

struct HasNoComments final {
  int i;
};

// e

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_COMMENT_H_
