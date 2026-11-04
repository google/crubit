// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_BITWISE_RELOCATABLE_BITWISE_RELOCATABLE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_BITWISE_RELOCATABLE_BITWISE_RELOCATABLE_H_

#include <expected>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "support/annotations.h"

struct Trivial {
  int x;
};
struct UserProvidedMoveDtor {
  UserProvidedMoveDtor();
  UserProvidedMoveDtor(UserProvidedMoveDtor&&);
  ~UserProvidedMoveDtor();
  int x;
};
struct CRUBIT_UNSAFE_BITWISE_RELOCATABLE AssertedBR {
  AssertedBR();
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
template <typename T>
IfEligible<T>::IfEligible(IfEligible&&) = default;
template <typename T>
IfEligible<T>::~IfEligible() = default;
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
  int fd;
};
struct CRUBIT_UNSAFE_BITWISE_RELOCATABLE K2 {
  K2(const K2&) = delete;
  K2(K2&&) = delete;
  ~K2();
  int fd;
};

struct A {
  std::string s;
};
template <typename T>
struct [[clang::trivial_abi]] TrivialAbiTemplate {
  TrivialAbiTemplate(TrivialAbiTemplate&&);
  ~TrivialAbiTemplate();
  T s;
};
template <typename T>
TrivialAbiTemplate<T>::TrivialAbiTemplate(TrivialAbiTemplate&&) = default;
template <typename T>
TrivialAbiTemplate<T>::~TrivialAbiTemplate() = default;
using B = TrivialAbiTemplate<std::string>;
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

struct StdStringOnLibcxx {
  CRUBIT_BITWISE_RELOCATABLE_STD_STRING_ONLY_ON_LIBCXX std::string s;
};

AssertedBR MakeAssertedBR(int x);
int GetAssertedBR(const AssertedBR& a);

StdStringOnLibcxx MakeStdStringOnLibcxx();
size_t GetStdStringOnLibcxxLength(const StdStringOnLibcxx& f);

struct OptAsserted {
  std::optional<AssertedBR> o;
};

struct UniquePtrInt {
  std::unique_ptr<int> u;
};
struct UniquePtrUserMove {
  std::unique_ptr<UserProvidedMoveDtor> u;
};

struct SharedPtrUserMove {
  std::shared_ptr<UserProvidedMoveDtor> s;
};

struct VecStr {
  std::vector<std::string> v;
};
struct VecInt {
  std::vector<int> v;
};

struct VariantIntStr {
  std::variant<int, std::string> v;
};
struct FuncVoid {
  std::function<void()> f;
};
struct MapIntInt {
  std::map<int, int> m;
};

struct RowCursorShapeFoo {
  RowCursorShapeFoo(RowCursorShapeFoo&&);
  ~RowCursorShapeFoo();
};
struct [[clang::trivial_abi]] RowCursorShape {
  std::unique_ptr<RowCursorShapeFoo> ptr;
};

struct StructWithVecAndUniquePtr {
  std::vector<std::string> v;
  std::unique_ptr<int> u;
};
StructWithVecAndUniquePtr MakeStructWithVecAndUniquePtr();
size_t GetStructWithVecAndUniquePtrLen(const StructWithVecAndUniquePtr& s);

#endif
