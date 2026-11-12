// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "pointers.h"
#include <stdalign.h>
#include <stddef.h>

int c_add(int a, int b) {
    return a + b;
}

struct FuncPointers get_func_pointers(void) {
    struct FuncPointers fp;
    fp.add = c_add;
    fp.callback = NULL;
    fp.callback_arg = NULL;
    return fp;
}

int invoke_callback(struct FuncPointers* fp) {
    if (fp->callback) {
        fp->callback(fp->callback_arg);
        return 1;
    }
    return 0;
}

const size_t kPointersSize = sizeof(struct Pointers);
const size_t kPointersAlign = alignof(struct Pointers);
const size_t kPointers_p1Offset = offsetof(struct Pointers, p1);
const size_t kPointers_p2Offset = offsetof(struct Pointers, p2);
const size_t kPointers_p3Offset = offsetof(struct Pointers, p3);
const size_t kPointers_p4Offset = offsetof(struct Pointers, p4);
const size_t kPointers_p5Offset = offsetof(struct Pointers, p5);

const size_t kFuncPointersSize = sizeof(struct FuncPointers);
const size_t kFuncPointersAlign = alignof(struct FuncPointers);
const size_t kFuncPointers_addOffset = offsetof(struct FuncPointers, add);
const size_t kFuncPointers_callbackOffset = offsetof(struct FuncPointers, callback);
const size_t kFuncPointers_callback_argOffset = offsetof(struct FuncPointers, callback_arg);
