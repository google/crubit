// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNIONS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNIONS_H_

#pragma clang lifetime_elision

union EmptyUnion {};

struct Nontrivial final {
  explicit Nontrivial();
  Nontrivial(Nontrivial&&);

  int field;
};

union [[clang::annotate("crubit_rust_name", "RenamedUnion")]] UnionToRename {};

struct TriviallyCopyableButNontriviallyDestructible {
  TriviallyCopyableButNontriviallyDestructible(
      const TriviallyCopyableButNontriviallyDestructible&) = default;
  ~TriviallyCopyableButNontriviallyDestructible() {}
};

union NonEmptyUnion {
  bool bool_field;
  char char_field;
  int int_field;
  long long long_long_field;
};

union NonCopyUnion {
  bool trivial_member;
  Nontrivial nontrivial_member;
};

union NonCopyUnion2 {
  bool trivial_member;
  TriviallyCopyableButNontriviallyDestructible nontrivial_member;
};

union UnionWithOpaqueField {
  char constant_array_field_not_yet_supported[42];
};

struct TrivialButInheritable {
  int x;
};

union UnionWithInheritable {
  TrivialButInheritable t;
};

typedef union {
  bool trivial_member;
} TypedefUnion;

typedef union {
  TrivialButInheritable t;
} TypedefUnionWithInheritable;

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_UNIONS_H_
