// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_STRUCT_WITH_LIFETIMEBOUND_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_STRUCT_WITH_LIFETIMEBOUND_H_

struct PlainStruct {};

struct StructWithLifetimeboundMemberFunction {
  const PlainStruct f() const [[clang::lifetimebound]];
};

struct StructWithLifetimeboundRefMemberFunction {
  const PlainStruct& f() const [[clang::lifetimebound]];
};

class DropClassWithLifetimeboundMemberFunction {
 public:
  const PlainStruct f() const [[clang::lifetimebound]];
  ~DropClassWithLifetimeboundMemberFunction();
};

class DropClassWithLifetimeboundRefMemberFunction {
 public:
  const PlainStruct& f() const [[clang::lifetimebound]];
  ~DropClassWithLifetimeboundRefMemberFunction();
};

struct StructWithLifetimeboundCtor {
  explicit StructWithLifetimeboundCtor(const PlainStruct s
                                       [[clang::lifetimebound]]) {}
};

struct StructWithLifetimeboundRefCtor {
  explicit StructWithLifetimeboundRefCtor(const PlainStruct& s
                                          [[clang::lifetimebound]]) {}
};

struct DropStructWithLifetimeboundCtor {
  explicit DropStructWithLifetimeboundCtor(const PlainStruct s
                                           [[clang::lifetimebound]]) {}
  ~DropStructWithLifetimeboundCtor();
};

struct DropStructWithLifetimeboundRefCtor {
  explicit DropStructWithLifetimeboundRefCtor(const PlainStruct& s
                                              [[clang::lifetimebound]]) {}
  ~DropStructWithLifetimeboundRefCtor();
};

struct DropStructWithRefCtorAndRefMemberFunction {
  explicit DropStructWithRefCtorAndRefMemberFunction(const PlainStruct& s
                                                     [[clang::lifetimebound]]) {
  }
  const PlainStruct& f() const [[clang::lifetimebound]];
  ~DropStructWithRefCtorAndRefMemberFunction();
};

struct DropStructWithCtorAndMemberFunction {
  explicit DropStructWithCtorAndMemberFunction(const PlainStruct s
                                               [[clang::lifetimebound]]) {}
  const PlainStruct f() const [[clang::lifetimebound]];
  ~DropStructWithCtorAndMemberFunction();
};

struct DropStructWithCtorAndRefMemberFunction {
  explicit DropStructWithCtorAndRefMemberFunction(const PlainStruct s
                                                  [[clang::lifetimebound]]) {}
  const PlainStruct& f() const [[clang::lifetimebound]];
  ~DropStructWithCtorAndRefMemberFunction();
};

struct DropStructWithRefCtorAndMemberFunction {
  explicit DropStructWithRefCtorAndMemberFunction(const PlainStruct& s
                                                  [[clang::lifetimebound]]) {}
  // This is a degenerate case, since `PlainStruct` binds no lifetimes.
  const PlainStruct f() const [[clang::lifetimebound]];
  ~DropStructWithRefCtorAndMemberFunction();
};

// We can't figure out the lifetime arity of this struct.
struct Impossible {
  Impossible f() [[clang::lifetimebound]];
  Impossible() = delete;
  ~Impossible() = delete;
  Impossible(const Impossible&) = delete;
  Impossible(Impossible&&) = delete;
  Impossible& operator=(const Impossible&) = delete;
  Impossible& operator=(Impossible&&) = delete;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_STRUCT_WITH_LIFETIMEBOUND_H_
