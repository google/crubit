// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef FUNCTIONS_H_
#define FUNCTIONS_H_

#ifdef __cplusplus
extern "C" {
#endif

void void_func(void);
int variadic_func(int count, ...);
typedef void (*TypedefFnPtr)(int);
struct FnPtrs {
  void (*field_fn)(int);
  TypedefFnPtr typedef_field;
};
void takes_fn_ptr(void (*cb)(int));
void (*returns_fn_ptr(void))(int);
void array_of_fn_ptrs(void (*cb[2])(int));
int call_cb(int (*cb)(int), int x);
void call_struct_fn_ptr(struct FnPtrs f, int x);

#ifdef __cplusplus
}
#endif

#endif  // FUNCTIONS_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kFnPtrsSize;
extern const size_t kFnPtrsAlign;
extern const size_t kFnPtrs_field_fnOffset;
extern const size_t kFnPtrs_typedef_fieldOffset;
#ifdef __cplusplus
}
#endif
