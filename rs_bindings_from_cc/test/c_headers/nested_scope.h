// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef NESTED_SCOPE_H_
#define NESTED_SCOPE_H_

#ifdef __cplusplus
extern "C" {
#endif

struct OuterScope {
  struct InnerScope {
    int x;
  } inner;
};
void use_inner_scope(struct InnerScope i);

#ifdef __cplusplus
}
#endif

#endif  // NESTED_SCOPE_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kOuterScopeSize;
extern const size_t kOuterScopeAlign;
extern const size_t kOuterScope_innerOffset;
#ifdef __cplusplus
}
#endif
