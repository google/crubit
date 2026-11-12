// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef NESTED_ANONYMOUS_H_
#define NESTED_ANONYMOUS_H_

#ifdef __cplusplus
extern "C" {
#endif

struct Nested {
  struct {
    int a;
    int b;
  } anon_struct;
  union {
    int c;
    float d;
  } anon_union;
};

#ifdef __cplusplus
}
#endif

#endif  // NESTED_ANONYMOUS_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kNestedSize;
extern const size_t kNestedAlign;
#ifdef __cplusplus
}
#endif
