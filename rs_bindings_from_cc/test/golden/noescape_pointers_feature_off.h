// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NOESCAPE_POINTERS_FEATURE_OFF_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NOESCAPE_POINTERS_FEATURE_OFF_H_

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wnullability-extension"
void takes_noescape_nonnull_feature_off(int* _Nonnull
                                        __attribute__((noescape)) p);
#pragma clang diagnostic pop

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NOESCAPE_POINTERS_FEATURE_OFF_H_
