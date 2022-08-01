// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

#pragma clang lifetime_elision

// Nontrivial due to (declared, but not yet defined) user-specified constructor
// and destructor.
//
// This makes it nontrivial for calls (so not trivially relocatable), as well
// as specifically giving it a nontrivial move constructor and destructor.
struct Nontrivial final {
  explicit Nontrivial();
  explicit Nontrivial(int field);
  explicit Nontrivial(int field, int unused);
  Nontrivial(const Nontrivial&);
  Nontrivial(Nontrivial&&);
  Nontrivial& operator=(const Nontrivial&);
  Nontrivial& operator=(Nontrivial&&);
  Nontrivial& operator=(int);
  // NOLINTNEXTLINE(misc-unconventional-assign-operator)
  Nontrivial operator=(float);
  ~Nontrivial();

  void MemberFunction();

  int field;
};

// Nontrivial due to (inline) user-specified constructor and destructor.
//
// This makes it nontrivial for calls (so not trivially relocatable), as well
// as specifically giving it a nontrivial move constructor and destructor.
struct NontrivialInline final {
  explicit NontrivialInline() : NontrivialInline(0) {}
  explicit NontrivialInline(int field) : field(field) {}
  explicit NontrivialInline(int field, int unused) : NontrivialInline(field) {}
  NontrivialInline(const NontrivialInline&) {}
  NontrivialInline(NontrivialInline&&) {}
  NontrivialInline& operator=(const NontrivialInline&) { return *this; }
  NontrivialInline& operator=(NontrivialInline&&) { return *this; }
  NontrivialInline& operator=(int) { return *this; }
  ~NontrivialInline() {}

  void MemberFunction() {}

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

// Nontrivial, but trivially relocatable and final (and therefore Unpin).
struct [[clang::trivial_abi]] NontrivialUnpin final {
  explicit NontrivialUnpin();
  explicit NontrivialUnpin(int field);
  explicit NontrivialUnpin(int field, int unused);
  NontrivialUnpin(const NontrivialUnpin&);
  NontrivialUnpin(NontrivialUnpin&&);
  NontrivialUnpin(Nontrivial&&);
  NontrivialUnpin& operator=(const NontrivialUnpin&);
  NontrivialUnpin& operator=(NontrivialUnpin&&);
  NontrivialUnpin& operator=(int);
  ~NontrivialUnpin();

  void MemberFunction();

  int field;
};

void TakesByValue(Nontrivial nontrivial);
void TakesByValueInline(NontrivialInline nontrivial);
void TakesByValueUnpin(NontrivialUnpin nontrivial);

Nontrivial ReturnsByValue();
NontrivialUnpin ReturnsByValueUnpin();

const Nontrivial& TakesByConstReference(const Nontrivial& nontrivial);
Nontrivial& TakesByReference(Nontrivial& nontrivial);

const NontrivialUnpin& TakesByConstReferenceUnpin(
    const NontrivialUnpin& nontrivial);
NontrivialUnpin& TakesByReferenceUnpin(NontrivialUnpin& nontrivial);

// Finally, testing for strange by-value APIs.
struct NontrivialByValue {
  NontrivialByValue(const NontrivialByValue& other) = default;
  NontrivialByValue(NontrivialByValue&& other) = default;
  NontrivialByValue& operator=(const NontrivialByValue& other) = default;
  NontrivialByValue& operator=(NontrivialByValue&& other) = default;
  // // NOLINTNEXTLINE(misc-unconventional-assign-operator)
  NontrivialByValue operator=(Nontrivial other);
  NontrivialByValue operator==(NontrivialByValue other);
};

struct Nonmovable final {
  explicit Nonmovable();
  Nonmovable(const Nonmovable&) = delete;
  Nonmovable(Nonmovable&&) = delete;
  ~Nonmovable();

  void MemberFunction();
};

void TakesNonmovableByValue(Nonmovable nonmovable);
Nonmovable ReturnsNonmovableByValue();

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_
