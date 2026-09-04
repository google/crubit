// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_TEST_GENERATOR_COMMAND_LINE_CONSUMES_LOCAL_DEFINES_DEP_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_TEST_GENERATOR_COMMAND_LINE_CONSUMES_LOCAL_DEFINES_DEP_H_

#ifdef DEFINE_FROM_LOCAL_DEFINES
#error \
    "Preprocessor define from the local_defines attribute was unexpectedly propagated"
#endif

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_TEST_GENERATOR_COMMAND_LINE_CONSUMES_LOCAL_DEFINES_DEP_H_
