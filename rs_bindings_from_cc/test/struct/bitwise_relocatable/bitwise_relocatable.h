// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_BITWISE_RELOCATABLE_BITWISE_RELOCATABLE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_BITWISE_RELOCATABLE_BITWISE_RELOCATABLE_H_

#include <optional>
#include <string>

#include "support/annotations.h"

struct Trivial {
  int x;
};
struct UserProvidedMoveDtor {
  UserProvidedMoveDtor(UserProvidedMoveDtor&&);
  ~UserProvidedMoveDtor();
  int x;
};
struct CRUBIT_UNSAFE_BITWISE_RELOCATABLE AssertedBR {
  AssertedBR(AssertedBR&&);
  ~AssertedBR();
  int x;
};
template <typename T>
struct CRUBIT_UNSAFE_BITWISE_RELOCATABLE_IF_ELIGIBLE IfEligible {
  IfEligible(IfEligible&&);
  ~IfEligible();
  T t;
};
using IfEligibleTrivial = IfEligible<Trivial>;
using IfEligibleUserMove = IfEligible<UserProvidedMoveDtor>;
using NestedIfEligible = IfEligible<IfEligible<Trivial>>;

struct J {
  J(const J&) = default;
  J(J&&) = delete;
  ~J() = default;
  int x;
};
struct K {
  K(const K&) = delete;
  K(K&&) = delete;
  ~K();
  int fd_;
};
struct CRUBIT_UNSAFE_BITWISE_RELOCATABLE K2 {
  K2(const K2&) = delete;
  K2(K2&&) = delete;
  ~K2();
  int fd_;
};
struct HolderK2 {
  K2 k;
};
struct DeletedMoveAssign {
  DeletedMoveAssign& operator=(DeletedMoveAssign&&) = delete;
  int x;
};

struct A {
  std::string s;
};
struct [[clang::trivial_abi]] B {
  std::string s;
};
struct [[clang::trivial_abi]] TA {
  TA(TA&&);
  ~TA();
  int x;
};

struct OptInt {
  std::optional<int> o;
};
struct OptUserMove {
  std::optional<UserProvidedMoveDtor> o;
};

struct FieldOverrideUserMove {
  CRUBIT_UNSAFE_BITWISE_RELOCATABLE_FIELD UserProvidedMoveDtor m;
};

struct UserCopy {
  UserCopy(const UserCopy&);
  UserCopy(UserCopy&&) = default;
  ~UserCopy() = default;
};

union UnionWithStr {
  int x;
  std::string s;
  UnionWithStr();
  ~UnionWithStr();
};

struct DerivesUserMove : UserProvidedMoveDtor {};
struct CRUBIT_UNSAFE_BITWISE_RELOCATABLE_IF_ELIGIBLE IfEligibleDerivesUserMove
    : UserProvidedMoveDtor {};

struct Polymorphic {
  virtual ~Polymorphic();
};

struct FieldOverride {
  CRUBIT_UNSAFE_BITWISE_RELOCATABLE_FIELD_LIBCXX_ONLY std::string s;
};

#endif
