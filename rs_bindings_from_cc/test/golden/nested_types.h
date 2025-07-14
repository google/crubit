// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NESTED_TYPES_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NESTED_TYPES_H_

struct Foo {
  int foo;
  struct Bar {
    int bar;
    struct Baz {
      int baz;
    };
  };
};

struct already_snake_case {
  using Inner = int;
  Inner i;
};

struct ConflictingSnakeCaseNames {
  using Inner = int;
  Inner i;
};

struct ConflictingSnakeCaseNames_ {
  using Inner = int;
  Inner i;
};

struct OnlyOneHasNestedItems {
  using Inner = int;
  Inner i;
};

struct OnlyOneHasNestedItems_ {
  // no nested items
};

struct SameNameAsNamespace {
  using Inner = int;
  Inner i;
};

namespace same_name_as_namespace {
struct Foo {};
}  // namespace same_name_as_namespace
namespace same_name_as_namespace {
struct Bar {};
}  // namespace same_name_as_namespace

namespace no_longer_top_level {

struct already_snake_case {
  using Inner = int;
  Inner i;
};

struct ConflictingSnakeCaseNames {
  using Inner = int;
  Inner i;
};

struct ConflictingSnakeCaseNames_ {
  using Inner = int;
  Inner i;
};

struct OnlyOneHasNestedItems {
  using Inner = int;
  Inner i;
};

struct OnlyOneHasNestedItems_ {
  // no nested items
};

struct SameNameAsNamespace {
  using Inner = int;
  Inner i;
};

namespace same_name_as_namespace {
struct Foo {};
}  // namespace same_name_as_namespace
namespace same_name_as_namespace {
struct Bar {};
}  // namespace same_name_as_namespace

}  // namespace no_longer_top_level

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NESTED_TYPES_H_
