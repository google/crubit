// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef BITFIELDS_H_
#define BITFIELDS_H_

#ifdef __cplusplus
extern "C" {
#endif

struct Bitfields {
  int a : 3;
  unsigned b : 5;
  int : 0;
  int d : 2;
  int span_unit : 31;
  signed int signed_bf : 2;
};

#ifdef __cplusplus
}
#endif

#endif  // BITFIELDS_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kBitfieldsSize;
extern const size_t kBitfieldsAlign;
#ifdef __cplusplus
}
#endif
