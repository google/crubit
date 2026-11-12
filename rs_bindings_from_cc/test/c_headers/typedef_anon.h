// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef TYPEDEF_ANON_H_
#define TYPEDEF_ANON_H_

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  int x;
  int y;
} PointTypedef;
typedef enum { RED, GREEN, BLUE } ColorTypedef;
typedef PointTypedef PointAlias;
typedef PointAlias PointChainAlias;

#ifdef __cplusplus
}
#endif

#endif  // TYPEDEF_ANON_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kPointTypedefSize;
extern const size_t kPointTypedefAlign;
extern const size_t kPointTypedef_xOffset;
extern const size_t kPointTypedef_yOffset;
#ifdef __cplusplus
}
#endif
