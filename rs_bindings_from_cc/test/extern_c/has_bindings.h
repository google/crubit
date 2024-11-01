// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_ALLOWED_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_ALLOWED_H_

#include "absl/base/attributes.h"
namespace crubit::has_bindings {

inline void crubit_void_function_non_extern_c() {}

extern "C" {

struct Struct {
  int* x;
  char y;
  Struct* z;
};

using StructAlias = Struct;

struct ABSL_ATTRIBUTE_TRIVIAL_ABI NontrivialStruct {
  int* x;
  ~NontrivialStruct() {
    // this can do anything, but we'll do something silly for the sake of
    // example.
    if (x != nullptr) {
      *x = 42;
    }
  }
};

enum Enum {
  kEnumerator = 0,
  // This doesn't receive bindings, because the enumerator has an unrecognized
  // attribute.
  kUnkownAttrEnumerator [[deprecated]] = 1,
};

union Union {
  int x;
  int y;
};

inline void crubit_void_function() {}
void crubit_non_inline_function();
inline const void* crubit_void_ptr_identity(const void* x) { return x; }
inline void crubit_nullability_annotated_function(const void* _Nullable x) {}
inline int crubit_add(int x, char y) { return x + y; }
inline Struct crubit_anystruct(Struct x, const StructAlias*) { return x; }
inline Enum crubit_enum_function(Enum x) { return x; }
inline Union crubit_union_function(Union x) { return x; }

// Note the use of references, rather than pointers. A C++ function reference
// corresponds to a Rust function pointer, more or less.
typedef void (&Callback)(int* x);
inline void crubit_invoke_callback(void (&f)(int* x), int* x) { f(x); }

// Whereas a C++ function pointer, being nullable, corresponds to a Rust
// Option<function pointer>.
typedef void (*NullableCallback)(int* x);
inline void crubit_invoke_nullable_callback(void (*f)(int* x), int* x) { f(x); }
}  // extern "C"

extern "C" void crubit_extern_c_directly_function();

struct MyDerivedStruct : Struct {
  int derived_x;
};

struct Nontrivial {
  ~Nontrivial() {}  // NOLINT(modernize-use-equals-default)
};

using NontrivialAlias = Nontrivial;

inline void crubit_accepts_nontrivial_ptr(Nontrivial*) {}
inline Nontrivial* crubit_returns_nontrivial_ptr() { return nullptr; }

}  // namespace crubit::has_bindings
#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_EXTERN_C_ALLOWED_H_
