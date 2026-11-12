// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef BASIC_STRUCTS_H_
#define BASIC_STRUCTS_H_

#ifdef __cplusplus
extern "C" {
#endif

struct Point {
  int x;
  int y;
};
union Data {
  int i;
  float f;
};

#ifdef __cplusplus
}
#endif

#endif  // BASIC_STRUCTS_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kPointSize;
extern const size_t kPointAlign;
extern const size_t kPoint_xOffset;
extern const size_t kPoint_yOffset;
extern const size_t kDataSize;
extern const size_t kDataAlign;
extern const size_t kData_iOffset;
extern const size_t kData_fOffset;
#ifdef __cplusplus
}
#endif
