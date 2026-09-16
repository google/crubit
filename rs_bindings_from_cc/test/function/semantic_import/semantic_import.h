// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SEMANTIC_IMPORT_SEMANTIC_IMPORT_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SEMANTIC_IMPORT_SEMANTIC_IMPORT_H_

#include "support/rs_std/slice_ref.h"

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

class Pointers {
 public:
  const int* p() const { return p_; }
  void set_p(const int* p) { p_ = p; }
  int* mut_p() const { return mut_p_; }
  void set_mut_p(int* mut_p) { mut_p_ = mut_p; }

 private:
  const int* p_ = nullptr;
  int* mut_p_ = nullptr;
};

class NonTrivial {
 public:
  NonTrivial() = default;
  // A user-provided destructor is what makes this type non-trivial (and so
  // `!Unpin` in Rust).
  // `= default` would defeat the purpose.
  // NOLINTNEXTLINE(modernize-use-equals-default)
  ~NonTrivial() {}
  const NonTrivial* p() const { return p_; }
  void set_p(const NonTrivial* p) { p_ = p; }
  NonTrivial* mut_p() { return mut_p_; }
  void set_mut_p(NonTrivial* mut_p) { mut_p_ = mut_p; }

 private:
  const NonTrivial* p_ = nullptr;
  NonTrivial* mut_p_ = nullptr;
};

// A type which is never defined, so that `Incomplete*` below is a pointer to an
// incomplete type.
class Incomplete;

class MorePointers {
 public:
  void* v() const { return v_; }
  void set_v(void* v) { v_ = v; }
  Incomplete* i() const { return i_; }
  void set_i(Incomplete* i) { i_ = i; }

 private:
  void* v_ = nullptr;
  Incomplete* i_ = nullptr;
};

// `rs_std::SliceRef` is not a raw pointer, so these accessors keep using a
// thunk.
class Slices {
 public:
  rs_std::SliceRef<const int> s() const { return s_; }
  void set_s(rs_std::SliceRef<const int> s) { s_ = s; }

 private:
  rs_std::SliceRef<const int> s_;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SEMANTIC_IMPORT_SEMANTIC_IMPORT_H_
