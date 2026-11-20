# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Checks that cc_std owns the target architecture's machine headers.
file(READ "${RSP}" content)
if(NOT content MATCHES "/usr/include/${MACHINE_DIR}/_types\\.h")
  message(FATAL_ERROR
    "cc_std doesn't own <${MACHINE_DIR}/_types.h> in ${RSP}:\n${content}")
endif()
