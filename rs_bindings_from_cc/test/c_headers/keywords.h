// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef KEYWORDS_H_
#define KEYWORDS_H_

#ifdef __cplusplus
extern "C" {
#endif

struct Keywords {
  int type;
  int fn;
  int match;
  int self;
  int Self;
  int crate;
};

#ifdef __cplusplus
}
#endif

#endif  // KEYWORDS_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kKeywordsSize;
extern const size_t kKeywordsAlign;
#ifdef __cplusplus
}
#endif
