// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef ALIGNMENT_KNOWN_FAILURE_H_
#define ALIGNMENT_KNOWN_FAILURE_H_

#ifdef __cplusplus
extern "C" {
#endif

_Alignas(16) struct AlignedStruct {
  int i;
};  // Fails to compile because _Alignas is C11 but parsed as C++

#ifdef __cplusplus
}
#endif

#endif  // ALIGNMENT_KNOWN_FAILURE_H_
