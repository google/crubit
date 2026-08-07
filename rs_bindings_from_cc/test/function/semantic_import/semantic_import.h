// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SEMANTIC_IMPORT_SEMANTIC_IMPORT_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SEMANTIC_IMPORT_SEMANTIC_IMPORT_H_

class S {
 public:
  explicit S(int x) : x_(x) {}
  int x() const { return x_; }
  int get_x() { return x_; }
  void set_x(int x) { x_ = x; }

 private:
  int x_;
};

class T : public S {
 public:
  T(int x, float y) : S(x), y_(y) {}
  float y() const { return y_; }

 private:
  float y_;
};

class Chars {
 public:
  char c() { return c_; }
  signed char sc() { return sc_; }
  unsigned char uc() { return uc_; }
  void set_c(char c) { c_ = c; }
  void set_sc(signed char sc) { sc_ = sc; }
  void set_uc(unsigned char uc) { uc_ = uc; }

 private:
  char c_ = 'c';
  signed char sc_ = 's';
  unsigned char uc_ = 'u';
};

class Bools {
 public:
  bool b() { return b_; }
  void set_b(bool b) { b_ = b; }

 private:
  bool b_ = true;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SEMANTIC_IMPORT_SEMANTIC_IMPORT_H_
