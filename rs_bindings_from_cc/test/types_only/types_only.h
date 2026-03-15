// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_ONLY_TYPES_ONLY_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_ONLY_TYPES_ONLY_H_

// Should not be imported.
inline void FreeFunction() {}

struct Copyable {
  // Should not be imported.
  void Method() {}
  int field;
};

class Cloneable {
 public:
  explicit Cloneable(int field) : field_(field) {}
  Cloneable(const Cloneable&) = default;
  Cloneable(Cloneable&&) = default;
  Cloneable& operator=(const Cloneable&) = default;
  Cloneable& operator=(Cloneable&&) = default;
  ~Cloneable() { field_ = 0; }
  // Should not be imported.
  void Method() {}

 private:
  int field_;
};

class Movable {
 public:
  explicit Movable(int field) : field_(field) {}
  Movable(Movable&& other) : field_(other.field_) { other.field_ = 0; }
  Movable& operator=(Movable&& other) {
    field_ = other.field_;
    other.field_ = 0;
    return *this;
  }
  ~Movable() { field_ = 0; }
  // Should not be imported.
  void Method() {}

 private:
  int field_;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TYPES_ONLY_TYPES_ONLY_H_
