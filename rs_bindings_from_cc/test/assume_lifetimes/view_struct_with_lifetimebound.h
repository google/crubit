// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_VIEW_STRUCT_WITH_LIFETIMEBOUND_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_VIEW_STRUCT_WITH_LIFETIMEBOUND_H_

struct PlainStruct {};

struct StructWithLifetimeboundMemberFunction {
  const PlainStruct f() const [[clang::lifetimebound]];

 private:
  int* p;
};

struct StructWithLifetimeboundRefMemberFunction {
  const PlainStruct& f() const [[clang::lifetimebound]];

 private:
  int* p;
};

class DropClassWithLifetimeboundMemberFunction {
  int* p;

 public:
  const PlainStruct f() const [[clang::lifetimebound]];
  ~DropClassWithLifetimeboundMemberFunction();
};

class DropClassWithLifetimeboundRefMemberFunction {
  int* p;

 public:
  const PlainStruct& f() const [[clang::lifetimebound]];
  ~DropClassWithLifetimeboundRefMemberFunction();
};

struct StructWithLifetimeboundCtor {
  explicit StructWithLifetimeboundCtor(const PlainStruct s
                                       [[clang::lifetimebound]]) {}

 private:
  int* p;
};

struct StructWithLifetimeboundRefCtor {
  explicit StructWithLifetimeboundRefCtor(const PlainStruct& s
                                          [[clang::lifetimebound]]) {}

 private:
  int* p;
};

struct DropStructWithLifetimeboundCtor {
  explicit DropStructWithLifetimeboundCtor(const PlainStruct s
                                           [[clang::lifetimebound]]) {}
  ~DropStructWithLifetimeboundCtor();

 private:
  int* p;
};

struct DropStructWithLifetimeboundRefCtor {
  explicit DropStructWithLifetimeboundRefCtor(const PlainStruct& s
                                              [[clang::lifetimebound]]) {}
  ~DropStructWithLifetimeboundRefCtor();

 private:
  int* p;
};

struct DropStructWithRefCtorAndRefMemberFunction {
  explicit DropStructWithRefCtorAndRefMemberFunction(const PlainStruct& s
                                                     [[clang::lifetimebound]]) {
  }
  const PlainStruct& f() const [[clang::lifetimebound]];
  ~DropStructWithRefCtorAndRefMemberFunction();

 private:
  int* p;
};

struct DropStructWithCtorAndMemberFunction {
  explicit DropStructWithCtorAndMemberFunction(const PlainStruct s
                                               [[clang::lifetimebound]]) {}
  const PlainStruct f() const [[clang::lifetimebound]];
  ~DropStructWithCtorAndMemberFunction();

 private:
  int* p;
};

struct DropStructWithCtorAndRefMemberFunction {
  explicit DropStructWithCtorAndRefMemberFunction(const PlainStruct s
                                                  [[clang::lifetimebound]]) {}
  const PlainStruct& f() const [[clang::lifetimebound]];
  ~DropStructWithCtorAndRefMemberFunction();

 private:
  int* p;
};

struct DropStructWithRefCtorAndMemberFunction {
  explicit DropStructWithRefCtorAndMemberFunction(const PlainStruct& s
                                                  [[clang::lifetimebound]]) {}
  // This is a degenerate case, since `PlainStruct` binds no lifetimes.
  const PlainStruct f() const [[clang::lifetimebound]];
  ~DropStructWithRefCtorAndMemberFunction();

 private:
  int* p;
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

 private:
  int* p;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_ASSUME_LIFETIMES_VIEW_STRUCT_WITH_LIFETIMEBOUND_H_
