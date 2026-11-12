// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef ZERO_LENGTH_ARRAYS_H_
#define ZERO_LENGTH_ARRAYS_H_

#ifdef __cplusplus
extern "C" {
#endif

struct ZeroLength {
  int length;
  int data[0];
};
struct MultiDim {
  int data[2][3];
};

#ifdef __cplusplus
}
#endif

#endif  // ZERO_LENGTH_ARRAYS_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kZeroLengthSize;
extern const size_t kZeroLengthAlign;
extern const size_t kZeroLength_lengthOffset;
extern const size_t kZeroLength_dataOffset;
extern const size_t kMultiDimSize;
extern const size_t kMultiDimAlign;
extern const size_t kMultiDim_dataOffset;
#ifdef __cplusplus
}
#endif
