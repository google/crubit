// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef POINTERS_H_
#define POINTERS_H_
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

struct Pointers {
  int* p1;
  const int* p2;
  int** p3;
  const int** p4;
  int* const* p5;
};

struct FuncPointers {
  int (*add)(int, int);
  void (*callback)(void*);
  void* callback_arg;
};

int invoke_callback(struct FuncPointers* fp);
struct FuncPointers get_func_pointers(void);

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kPointersSize;
extern const size_t kPointersAlign;
extern const size_t kPointers_p1Offset;
extern const size_t kPointers_p2Offset;
extern const size_t kPointers_p3Offset;
extern const size_t kPointers_p4Offset;
extern const size_t kPointers_p5Offset;

extern const size_t kFuncPointersSize;
extern const size_t kFuncPointersAlign;
extern const size_t kFuncPointers_addOffset;
extern const size_t kFuncPointers_callbackOffset;
extern const size_t kFuncPointers_callback_argOffset;
#ifdef __cplusplus
}
#endif

#endif  // POINTERS_H_
