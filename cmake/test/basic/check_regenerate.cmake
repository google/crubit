# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

if(GENERATOR MATCHES "Ninja")
  set(dry_run -n)
else()
  set(dry_run --dry-run)
endif()

function(dry_run_generate out_var)
  execute_process(
    COMMAND ${CMAKE_COMMAND} --build ${BINARY_DIR} --target crubit_generate_basic_rs
            -- ${dry_run}
    OUTPUT_VARIABLE output
    ERROR_VARIABLE output
    COMMAND_ERROR_IS_FATAL ANY)
  set(${out_var} "${output}" PARENT_SCOPE)
endfunction()

set(regenerates "Generating Rust bindings for C\\+\\+: basic_rs")
dry_run_generate(before)
if(before MATCHES "${regenerates}")
  message(FATAL_ERROR "bindings are out of date before touching the header:\n${before}")
endif()
file(TOUCH "${HEADER}")
dry_run_generate(after)
if(NOT after MATCHES "${regenerates}")
  message(FATAL_ERROR "touching ${HEADER} does not regenerate bindings:\n${after}")
endif()
