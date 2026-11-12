// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef SAME_NAME_TAG_H_
#define SAME_NAME_TAG_H_

#ifdef __cplusplus
extern "C" {
#endif

typedef struct TagSameName {
  int x;
} TagSameName;

#ifdef __cplusplus
}
#endif

#endif  // SAME_NAME_TAG_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kTagSameNameSize;
extern const size_t kTagSameNameAlign;
extern const size_t kTagSameName_xOffset;
#ifdef __cplusplus
}
#endif
