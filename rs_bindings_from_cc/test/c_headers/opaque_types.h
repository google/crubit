// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef OPAQUE_TYPES_H_
#define OPAQUE_TYPES_H_

#ifdef __cplusplus
extern "C" {
#endif

struct Opaque;
void process_opaque(struct Opaque* o);

#ifdef __cplusplus
}
#endif

#endif  // OPAQUE_TYPES_H_
