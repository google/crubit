// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/struct/bitwise_relocatable/bitwise_relocatable.h"

#include <cstddef>
#include <memory>

UserProvidedMoveDtor::UserProvidedMoveDtor() = default;
UserProvidedMoveDtor::UserProvidedMoveDtor(UserProvidedMoveDtor&&) = default;
UserProvidedMoveDtor::~UserProvidedMoveDtor() = default;

AssertedBR::AssertedBR() = default;
AssertedBR::AssertedBR(AssertedBR&&) = default;
AssertedBR::~AssertedBR() = default;

K::~K() = default;
K2::~K2() = default;

TA::TA(TA&&) = default;
TA::~TA() = default;

UserCopy::UserCopy(const UserCopy&) = default;

UnionWithStr::UnionWithStr() : x(0) {}
UnionWithStr::~UnionWithStr() {}

Polymorphic::~Polymorphic() = default;

AssertedBR MakeAssertedBR(int x) {
  AssertedBR a;
  a.x = x;
  return a;
}

int GetAssertedBR(const AssertedBR& a) { return a.x; }

StdStringOnLibcxx MakeStdStringOnLibcxx() {
  StdStringOnLibcxx f;
  f.s = "hello";
  return f;
}

size_t GetStdStringOnLibcxxLength(const StdStringOnLibcxx& f) {
  return f.s.length();
}

RowCursorShapeFoo::RowCursorShapeFoo(RowCursorShapeFoo&&) = default;
RowCursorShapeFoo::~RowCursorShapeFoo() = default;

StructWithVecAndUniquePtr MakeStructWithVecAndUniquePtr() {
  StructWithVecAndUniquePtr s;
  s.v.push_back("hello");
  s.u = std::make_unique<int>(42);
  return s;
}
size_t GetStructWithVecAndUniquePtrLen(const StructWithVecAndUniquePtr& s) {
  return s.v.size() + static_cast<size_t>(*s.u);
}
