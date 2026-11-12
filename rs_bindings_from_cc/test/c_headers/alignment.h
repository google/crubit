// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <stddef.h>
#ifndef ALIGNMENT_H_
#define ALIGNMENT_H_

#ifdef __cplusplus
extern "C" {
#endif

struct __attribute__((packed)) PackedStruct {
  char c;
  int i;
};
#pragma pack(push, 1)
struct PragmaPacked {
  char c;
  int i;
};
#pragma pack(pop)

#ifdef __cplusplus
}
#endif

#endif  // ALIGNMENT_H_

#ifdef __cplusplus
extern "C" {
#endif
extern const size_t kPackedStructSize;
extern const size_t kPackedStructAlign;
extern const size_t kPackedStruct_cOffset;
extern const size_t kPackedStruct_iOffset;
extern const size_t kPragmaPackedSize;
extern const size_t kPragmaPackedAlign;
extern const size_t kPragmaPacked_cOffset;
extern const size_t kPragmaPacked_iOffset;
#ifdef __cplusplus
}
#endif
