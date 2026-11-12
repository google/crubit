// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef TYPES_KNOWN_FAILURE_H_
#define TYPES_KNOWN_FAILURE_H_

#ifdef __cplusplus
extern "C" {
#endif

struct TypesKnownFailure {
  __int128 i128;  // Error: __int128 is not supported on this target
};

#ifdef __cplusplus
}
#endif

#endif  // TYPES_KNOWN_FAILURE_H_
