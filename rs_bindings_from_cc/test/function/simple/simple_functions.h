// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_

#pragma clang lifetime_elision

int return_value();
int* return_pointer();
int& return_reference();
void take_pointer(int* i);
void take_reference(int& i);
const int* forward_pointer(const int* i);
const int& forward_reference(const int& i);
int multiply(int x, int y);
int multiply_with_unnamed_parameters(int, int);
int multiply_with_keyword_named_parameters(int self, int crate, int super);

// LLVM identifiers use the `\01` prefix to suppress mangling:
// https://llvm.org/docs/LangRef.html#identifiers
// Test that we can import functions that have such names.
// If `__USER_LABEL_PREFIX__` is non-empty, the Clang mangler adds the `\01`
// prefix; otherwise, we add it here ourselves.
#define IS_EMPTY_HELPER 1
#define IS_EMPTY(x) IS_EMPTY2(x)
#define IS_EMPTY2(x) IS_EMPTY_HELPER##x
#if IS_EMPTY(__USER_LABEL_PREFIX__)
int llvm_no_mangle_marker() __asm("\01_llvm_no_mangle_marker");
#else
int llvm_no_mangle_marker() __asm("_llvm_no_mangle_marker");
#endif

// Test that we can import functions whose `__asm` name contains a dollar sign.
// For example, the Apple SDKs use dollar signs in their symbol versioning
// macros (e.g. `__DARWIN_EXTSN()`).
int asm_name_with_dollar_sign() __asm("asm$name$with$dollar$sign");

// https://cdecl.org/?q=int+%28*get_multiply_function%28%29%29%28int%2C+int%29:
// declare foo as function returning pointer to function (int, int) returning
// int
int (*get_pointer_to_multiply_function())(int, int);

// Same as above, but returning a *reference* to a function.
int (&get_reference_to_multiply_function())(int, int);

inline int (*inline_get_pointer_to_multiply_function())(int, int) {
  return multiply;
}

inline int apply_binary_op(int x, int y, int (*op)(int, int)) {
  return op(x, y);
}

// TODO(b/217419782): Add testcases for pointers to functions that take or
// return takes/returns non-trivially-movable types by value. In particular,
// some function signatures might require going through a C++ thunk - such
// function pointers can't work without a thunk. See also
// <internal link>

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_FUNCTION_SIMPLE_SIMPLE_FUNCTIONS_H_
