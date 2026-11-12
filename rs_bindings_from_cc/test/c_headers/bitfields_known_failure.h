// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef BITFIELDS_KNOWN_FAILURE_H_
#define BITFIELDS_KNOWN_FAILURE_H_

#ifdef __cplusplus
extern "C" {
#endif

struct BitfieldsKnownFailure {
  _Bool c : 1;  // Error: unknown type name '_Bool'
};

#ifdef __cplusplus
}
#endif

#endif  // BITFIELDS_KNOWN_FAILURE_H_
