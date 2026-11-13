# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

file(READ "${RSP}" content)
if(content MATCHES "(^|\n)\n")
  message(FATAL_ERROR "empty argument line in ${RSP}:\n${content}")
endif()
file(STRINGS "${RSP}" lines)

function(expect_line line)
  if(NOT "${line}" IN_LIST lines)
    message(FATAL_ERROR "expected line `${line}` in ${RSP}:\n${lines}")
  endif()
endfunction()

expect_line("-DFIXTURE_ALWAYS=1")
expect_line("-DFIXTURE_FROM_DEP")
expect_line("-I${INC}")
expect_line("-nostdinc++")
if(CONFIG STREQUAL "Debug")
  expect_line("-DFIXTURE_DEBUG")
elseif("-DFIXTURE_DEBUG" IN_LIST lines)
  message(FATAL_ERROR "-DFIXTURE_DEBUG present in ${CONFIG} build")
endif()

foreach(line IN LISTS lines)
  if(line MATCHES "\\$<")
    message(FATAL_ERROR "unevaluated generator expression: ${line}")
  endif()
  if(line MATCHES "/lib/clang/[0-9]+/include$")
    message(FATAL_ERROR "clang resource dir must not be passed: ${line}")
  endif()
endforeach()
