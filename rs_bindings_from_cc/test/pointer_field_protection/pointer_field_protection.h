// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_POINTER_FIELD_PROTECTION_POINTER_FIELD_PROTECTION_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_POINTER_FIELD_PROTECTION_POINTER_FIELD_PROTECTION_H_

// `TrivialAbiPointer` is trivial for calls (because of
// `[[clang::trivial_abi]]`), so Crubit treats it as `Unpin` and lets Rust move
// it with `memcpy`.
//
// `-fexperimental-pointer-field-protection-abi` enables pointer field
// protection (PFP) for all types that are not standard-layout.
// `TrivialAbiPointer` is not standard-layout, because its fields have
// different access control.  There is no attribute in the source code, so
// Crubit cannot see that PFP applies to this type.
//
// `TrivialAbiPointer` also has a user-provided destructor, so it is not
// trivially copyable.  With `-fexperimental-pointer-field-protection-tagged`
// on AArch64, Clang encodes `ptr_` with the address of the `TrivialAbiPointer`
// object.  C++ copy and move operations re-encode `ptr_` for the new address,
// but a `memcpy` does not.  See also `clang/test/CodeGenCXX/pfp-coerce.cpp` and
// `clang/docs/StructureProtection.md` in LLVM.
class [[clang::trivial_abi]] TrivialAbiPointer final {
 public:
  // Returns an object that points to an `int` with value `kValue`.
  static TrivialAbiPointer Create();
  static constexpr int kValue = 42;

  TrivialAbiPointer(const TrivialAbiPointer&) = default;
  TrivialAbiPointer& operator=(const TrivialAbiPointer&) = default;

  // The custom destructor means that `TrivialAbiPointer` is not trivially
  // copyable.  This is intentionally custom and not `= default`.
  ~TrivialAbiPointer() {}  // NOLINT(modernize-use-equals-default)

  // Reads the `int` that `ptr_` points to.
  int Get() const;

  // Does a C++ copy of an object from `Create()` to the heap, and then
  // returns `Get()` of the copy.
  static int GetAfterCppCopy();

 private:
  explicit TrivialAbiPointer(int* ptr) : ptr_(ptr) {}

  int* ptr_;

 protected:
  // Only here to make `TrivialAbiPointer` not standard-layout.
  int unused_ = 0;
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_POINTER_FIELD_PROTECTION_POINTER_FIELD_PROTECTION_H_
