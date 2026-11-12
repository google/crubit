// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef FLEXIBLE_ARRAYS_H_
#define FLEXIBLE_ARRAYS_H_

#ifdef __cplusplus
extern "C" {
#endif

struct Flex {
  int length;
  int data[];
};

#ifdef __cplusplus
}
#endif

#endif  // FLEXIBLE_ARRAYS_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kFlexSize;
extern const size_t kFlexAlign;
extern const size_t kFlex_lengthOffset;
extern const size_t kFlex_dataOffset;
#ifdef __cplusplus
}
#endif
