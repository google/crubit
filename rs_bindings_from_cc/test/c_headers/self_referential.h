// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef SELF_REFERENTIAL_H_
#define SELF_REFERENTIAL_H_

#ifdef __cplusplus
extern "C" {
#endif

struct Node {
  int value;
  struct Node* next;
};

#ifdef __cplusplus
}
#endif

#endif  // SELF_REFERENTIAL_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kNodeSize;
extern const size_t kNodeAlign;
extern const size_t kNode_valueOffset;
extern const size_t kNode_nextOffset;
#ifdef __cplusplus
}
#endif
