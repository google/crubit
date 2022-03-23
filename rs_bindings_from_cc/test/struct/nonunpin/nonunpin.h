// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NONUNPIN_NONUNPIN_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NONUNPIN_NONUNPIN_H_

#include <cstddef>
#pragma clang lifetime_elision

// A deliberately !Unpin class.
class Nonunpin {
 public:
  explicit Nonunpin(int value)
      : value_(value), addr_(reinterpret_cast<size_t>(this)) {}
  Nonunpin(const Nonunpin& other) : Nonunpin(other.value_) {}
  ~Nonunpin() {}
  size_t addr() const { return addr_; }
  int value() const { return value_; }
  void set_value(int new_value) { value_ = new_value; }

 private:
  int value_;
  size_t addr_;
};

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_NONUNPIN_NONUNPIN_H_
