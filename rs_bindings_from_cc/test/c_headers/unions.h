// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef UNIONS_H_
#define UNIONS_H_

#ifdef __cplusplus
extern "C" {
#endif

union UnionOfStructs {
  struct {
    int a;
    int b;
  } s1;
  struct {
    float c;
    float d;
  } s2;
};
union UnionWithArrays {
  int arr1[4];
  float arr2[2];
};

#ifdef __cplusplus
}
#endif

#endif  // UNIONS_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kUnionOfStructsSize;
extern const size_t kUnionOfStructsAlign;
extern const size_t kUnionOfStructs_s1Offset;
extern const size_t kUnionOfStructs_s2Offset;
extern const size_t kUnionWithArraysSize;
extern const size_t kUnionWithArraysAlign;
extern const size_t kUnionWithArrays_arr1Offset;
extern const size_t kUnionWithArrays_arr2Offset;
#ifdef __cplusplus
}
#endif
