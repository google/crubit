// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_ABI_CLASS_ABI_CLASS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_ABI_CLASS_ABI_CLASS_H_

#pragma clang lifetime_elision

// These are regression tests for b/270384825
//
// Notes that apply to all the structs below:
//
// * The structs below have different ABI classification.  We use these structs
//   to verify if Rust bindings will use a compatible ABI.  An incompatible ABI
//   can mean that Rust and C++ will look for a function return value in a
//   different CPU register.  For example, https://stackoverflow.com/a/42413484
//   points out that integers may be returned in `rax` or `rdx`, but floating
//   point values may use `xmm0` or `xmm1`.  One risk is that the ABI
//   classification can change when we change the type of some fields to
//   represent them as an opaque blob of bytes - e.g.:
//   `pub(crate) __unnamed_field0: [MaybeUninit<u8>; 4]`
//
// * To encourage `rs_bindings_from_cc` to represent the fields as an opaque
//   blob of bytes (i.e. to not preserve the actual type of the field) the
//   fields below:
//     - are `private`
//     - use a nested struct (an unsupported type)
//
// * Optimizing compiler can make the disassembly of the `Create` methods quite
//   empty (probably because the input argument uses the same register as the
//   return value.  To make the tests more sensitive to ABI choices, the `Add`
//   method is used (to actually operate on the input arguments and to have to
//   calculate a *new* return value).  For example, this is how the
//   `StructFloat::Create` method's disassembly may look like:
//       __rust_thunk___ZN11StructFloat6CreateEf:
//              push   %rbp
//              mov    %rsp,%rbp
//              pop    %rbp
//              ret

// Expected System V ABI classification of this struct: SSE.
// The return value is expected to be put into the `xmm0` register:
// https://godbolt.org/z/a1aK8Yxdx
//
// This is a regression test for b/270454629.  Before this bug was fixed the
// `test_struct_float` test would fail (with the actual/right value being quite
// random - depending on what happened to be present in the `xmm0` register
// before the test).
struct StructFloat final {
 public:
  static inline StructFloat Create(float f) {
    StructFloat s;
    s.float_var = f;
    return s;
  }

  static inline StructFloat Add(StructFloat x, StructFloat y) {
    StructFloat s;
    s.float_var = x.float_var + y.float_var;
    return s;
  }

  static inline float Inspect(StructFloat s) { return s.float_var; }

 private:
  struct {
    float float_var;
  };
};

// Expected System V ABI classification of this struct: memory:
//     > If the size of an object is larger than eight eightbytes, or it
//     > contains unaligned fields, it has class MEMORY
//     ...
//     > If the class is MEMORY, pass the argument on the stack at an address
//     > respecting the arguments alignment (which might be more than its
//     > natural alignement)
//     ...
//     > If the type has class MEMORY, then the caller provides space for the
//     > return value and passes the address of this storage in %rdi as if it
//     > were the first argument to the function. In effect, this address
//     > becomes a “hidden” first argument. This storage must not overlap any
//     > data visible to the callee through other names than this argument.  On
//     > return %rax will contain the address that has been passed in by the
//     > caller in %rdi.
//     (from "System V Application Binary Interface AMD64 Architecture Processor
//     Supplement (With LP64 and ILP32 Programming Models) Version 1.0")
//
// See also: https://godbolt.org/z/a1aK8Yxdx
//
// This is a regression test for b/270454629.  Before this bug was fixed the
// `test_struct_memory` test would trigger undefined beahvior and would usually
// crash with segmentation fault.
struct StructMemory final {
 public:
  static inline StructMemory Create(int i) {
    StructMemory s;
    s.int_var = i;
    return s;
  }

  static inline StructMemory Add(StructMemory x, StructMemory y) {
    StructMemory s;
    s.int_var = x.int_var + y.int_var;
    return s;
  }

  static inline int Inspect(StructMemory s) { return s.int_var; }

 private:
  // Using `char` and `packed` to misalign `int_var` - this is needed to
  // ensure that the ABI classification of the whole struct is: "memory".
  struct {
    char char_var;
    __attribute__((packed)) int int_var;
  };
};

// Expected System V ABI classification of this struct: integer.
// The return value is expected to be put into the `rax` register:
// https://godbolt.org/z/a1aK8Yxdx
//
// This struct is used for tests for completeness, but it is *not* expected
// to uncover problems caused by ABI differences, because we expect that the
// same ABI classification (integer) will be used by the C++ struct below
// and by the Rust struct (even if the `int_var` field would be replaced
// by `[u8; 4]`).
struct StructInteger final {
 public:
  static inline StructInteger Create(int i) {
    StructInteger s;
    s.int_var = i;
    return s;
  }

  static inline StructInteger Add(StructInteger x, StructInteger y) {
    StructInteger s;
    s.int_var = x.int_var + y.int_var;
    return s;
  }

  static inline int Inspect(StructInteger s) { return s.int_var; }

 private:
  struct {
    int int_var;
  };
};

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_STRUCT_ABI_CLASS_ABI_CLASS_H_
