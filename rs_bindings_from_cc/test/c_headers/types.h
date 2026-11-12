// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef TYPES_H_
#define TYPES_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
struct Types {
  int8_t i8;
  uint64_t u64;
  size_t sz;
  bool b;
  wchar_t wc;
  long double ld;
  _Complex float cf;
  _Complex double cd;
  _Atomic int ai;
  int vec __attribute__((vector_size(16)));
};

#ifdef __cplusplus
}
#endif

#endif  // TYPES_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kTypesSize;
extern const size_t kTypesAlign;
#ifdef __cplusplus
}
#endif
