// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef MUTUALLY_REFERENTIAL_H_
#define MUTUALLY_REFERENTIAL_H_

#ifdef __cplusplus
extern "C" {
#endif

struct A;
struct B;
struct A {
  struct B* b;
};
struct B {
  struct A* a;
};

#ifdef __cplusplus
}
#endif

#endif  // MUTUALLY_REFERENTIAL_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kASize;
extern const size_t kAAlign;
extern const size_t kA_bOffset;
extern const size_t kBSize;
extern const size_t kBAlign;
extern const size_t kB_aOffset;
#ifdef __cplusplus
}
#endif
