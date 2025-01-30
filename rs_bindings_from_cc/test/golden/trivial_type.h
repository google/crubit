// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_

#pragma clang lifetime_elision

namespace ns {
// Implicitly defined special member functions are trivial on a struct with
// only trivial members.
struct Trivial {
  int trivial_field;

  void Unqualified();
  void ConstQualified() const;
  void LvalueRefQualified() &;
  void ConstLvalueRefQualified() const&;
  void RvalueRefQualified() &&;
  void ConstRvalueRefQualified() const&&;
};

Trivial TakesByValue(Trivial trivial);
Trivial& TakesByReference(Trivial& trivial);
const Trivial& TakesByConstReference(const Trivial& trivial);
Trivial&& TakesByRvalueReference(Trivial&& trivial);
const Trivial&& TakesByConstRvalueReference(const Trivial&& trivial);

}  // namespace ns

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_
