// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_

#pragma clang lifetime_elision

namespace ns {
// Implicitly defined special member functions are trivial on a struct with
// only trivial members.
struct Trivial final {
  int trivial_field;

  void Unqualified();
  void ConstQualified() const;
  void LvalueRefQualified() &;
  void ConstLvalueRefQualified() const&;
  void RvalueRefQualified() &&;
  void ConstRvalueRefQualified() const&&;
};

// This struct is trivial, and therefore trivially relocatable etc., but still
// not safe to pass by reference as it is not final.
struct TrivialNonfinal {
  int trivial_field;
};

Trivial TakesByValue(Trivial trivial);
TrivialNonfinal TakesTrivialNonfinalByValue(TrivialNonfinal trivial);

Trivial& TakesByReference(Trivial& trivial);
TrivialNonfinal& TakesTrivialNonfinalByReference(TrivialNonfinal& trivial);

const Trivial& TakesByConstReference(const Trivial& trivial);
const TrivialNonfinal& TakesTrivialNonfinalByConstReference(
    const TrivialNonfinal& trivial);

Trivial&& TakesByRvalueReference(Trivial&& trivial);
TrivialNonfinal&& TakesTrivialNonfinalByRvalueReference(
    TrivialNonfinal&& trivial);

const Trivial&& TakesByConstRvalueReference(const Trivial&& trivial);
const TrivialNonfinal&& TakesTrivialNonfinalByConstRvalueReference(
    const TrivialNonfinal&& trivial);

}  // namespace ns

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_
