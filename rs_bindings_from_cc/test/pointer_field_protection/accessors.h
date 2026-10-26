// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_POINTER_FIELD_PROTECTION_ACCESSORS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_POINTER_FIELD_PROTECTION_ACCESSORS_H_

// `PointerHolder` is not standard-layout, because its fields have different
// access control. So with `-fexperimental-pointer-field-protection-abi`, `ptr_`
// is a protected pointer field: C++ stores it signed and authenticates it on
// every load.
//
// `ptr()` and `set_ptr()` are simple enough for Crubit's thunkless accessors
// to read and write `ptr_` directly from Rust. That must not happen for a
// protected field, so they have to go through a C++ thunk. The test checks that
// a pointer stored from Rust can be loaded by C++ and vice versa.
class PointerHolder final {
 public:
  PointerHolder() = default;

  int* ptr() const { return ptr_; }
  void set_ptr(int* ptr) { ptr_ = ptr; }

  // Reads the `int` that `ptr_` points to, from C++ code that is not inlined
  // into the bindings.
  int GetFromCpp() const;

  // Makes `ptr_` point to an `int` with value `kValue`, from C++ code that is
  // not inlined into the bindings.
  void SetFromCpp();
  static constexpr int kValue = 42;

 private:
  int* ptr_ = nullptr;

 protected:
  // Only here to make `PointerHolder` not standard-layout.
  int unused_ = 0;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_POINTER_FIELD_PROTECTION_ACCESSORS_H_
