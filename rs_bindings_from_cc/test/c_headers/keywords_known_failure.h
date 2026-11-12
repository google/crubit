// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef KEYWORDS_KNOWN_FAILURE_H_
#define KEYWORDS_KNOWN_FAILURE_H_

#ifdef __cplusplus
extern "C" {
#endif

struct KeywordsKnownFailure {
  int class;  // Error: declaration of anonymous class must be a definition
};

#ifdef __cplusplus
}
#endif

#endif  // KEYWORDS_KNOWN_FAILURE_H_
