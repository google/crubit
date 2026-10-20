// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_INHERITANCE_VIRTUAL_BASE_CTOR_DTOR_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_INHERITANCE_VIRTUAL_BASE_CTOR_DTOR_H_

#pragma clang lifetime_elision

namespace virtual_base_ctor_dtor {

struct VirtualBase {
  ~VirtualBase();
  static int GetDestructorCallCount();

  int base_field = 123;
};

// A class with a virtual base class, and with a constructor and a destructor
// that are *not* inline (they are defined in the `.cc` file).  In the Microsoft
// C++ ABI, such constructors and destructors need a C++ thunk (see
// `can_skip_cc_thunk` in `generate_function_thunk.rs`).
class Derived : public virtual VirtualBase {
 public:
  explicit Derived(int value);
  ~Derived();

  int value() const;
  int base_field_from_cpp() const;

  static int GetDestructorCallCount();

 private:
  int value_;
};

}  // namespace virtual_base_ctor_dtor

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_INHERITANCE_VIRTUAL_BASE_CTOR_DTOR_H_
