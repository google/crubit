// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_CC_PUBLIC_LIBRARY_AMBIGUOUS_HEADER_RDEP_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_CC_PUBLIC_LIBRARY_AMBIGUOUS_HEADER_RDEP_H_

#include "rs_bindings_from_cc/test/bazel/cc_public_library/public.h"

inline void MyFunction(PublicType*) {}

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_BAZEL_CC_PUBLIC_LIBRARY_AMBIGUOUS_HEADER_RDEP_H_
