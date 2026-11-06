// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NOESCAPE_POINTERS_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NOESCAPE_POINTERS_H_

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wnullability-extension"
void takes_noescape_nonnull(int* _Nonnull __attribute__((noescape)) p);
void takes_noescape_nullable(int* _Nullable __attribute__((noescape)) p);
void takes_nonnull_only(int* _Nonnull p);
void takes_noescape_void(void* _Nonnull __attribute__((noescape)) p);
struct Incomplete;
void takes_noescape_incomplete(Incomplete* _Nonnull
                               __attribute__((noescape)) p);
void takes_noescape_pointer_to_pointer(int** _Nonnull
                                       __attribute__((noescape)) p);
void takes_noescape_const_pointee(const int* _Nonnull
                                  __attribute__((noescape)) p);
#pragma clang diagnostic pop

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NOESCAPE_POINTERS_H_
