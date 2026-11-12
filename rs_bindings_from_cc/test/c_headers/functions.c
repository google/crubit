// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "functions.h"
void void_func(void) {}
int variadic_func(int count, ...) { return count; }
void takes_fn_ptr(void (*cb)(int)) {}
void (*returns_fn_ptr(void))(int) { return 0; }
void array_of_fn_ptrs(void (*cb[2])(int)) {}
int call_cb(int (*cb)(int), int x) { return cb ? cb(x) : 0; }
void call_struct_fn_ptr(struct FnPtrs f, int x) { if (f.field_fn) f.field_fn(x); }

#include <stddef.h>

#include <stdalign.h>
const size_t kFnPtrsSize = sizeof(struct FnPtrs);
const size_t kFnPtrsAlign = alignof(struct FnPtrs);
const size_t kFnPtrs_field_fnOffset = offsetof(struct FnPtrs, field_fn);
const size_t kFnPtrs_typedef_fieldOffset = offsetof(struct FnPtrs, typedef_field);
