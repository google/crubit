// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

#pragma clang lifetime_elision

// Nontrivial due to (declared, but not yet defined) user-specified constructor
// and destructor.
//
// This makes it nontrivial for calls (so not trivially relocatable), as well
// as specifically giving it a nontrivial move constructor and destructor.
struct Nontrivial final {
  Nontrivial(Nontrivial&&);
  ~Nontrivial();

  void MemberFunction();

  int field;
};

// Nontrivial due to (inline) user-specified constructor and destructor.
//
// This makes it nontrivial for calls (so not trivially relocatable), as well
// as specifically giving it a nontrivial move constructor and destructor.
struct NontrivialInline final {
  NontrivialInline(NontrivialInline&&) {}
  ~NontrivialInline() {}

  void MemberFunction(){};

  int field;
};

// Nontrivial due to member variables.
//
// This changes how the destructor / drop impl work -- instead of calling
// the destructor for NontrivialMembers, it just calls the destructors for
// each field.
struct NontrivialMembers final {
  Nontrivial nontrivial_member;
};

void TakesByValue(Nontrivial nontrivial);
void TakesByValueInline(NontrivialInline nontrivial);

Nontrivial ReturnsByValue();

const Nontrivial& TakesByConstReference(const Nontrivial& nontrivial);
Nontrivial& TakesByReference(Nontrivial& nontrivial);

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_
