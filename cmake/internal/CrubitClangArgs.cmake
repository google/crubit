# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

include_guard(GLOBAL)

# Sets `out_var` to the clang args that locate the C++ standard library and the
# system headers the way the project's C++ compiler does.
#
# `rs_bindings_from_cc` embeds its own Clang (and its own builtin headers), so
# the compiler's search paths are passed explicitly, minus the compiler's
# builtin header directory.
function(_crubit_system_include_args out_var)
  set(args "")
  if(APPLE)
    set(sysroot "${CMAKE_OSX_SYSROOT}")
    if(NOT sysroot)
      execute_process(
        COMMAND xcrun --show-sdk-path
        OUTPUT_VARIABLE sysroot
        OUTPUT_STRIP_TRAILING_WHITESPACE
        COMMAND_ERROR_IS_FATAL ANY)
      if(NOT sysroot)
        message(FATAL_ERROR
          "Crubit: could not find the macOS SDK; set CMAKE_OSX_SYSROOT.")
      endif()
    endif()
    list(APPEND args -isysroot "${sysroot}")
    if(CMAKE_OSX_DEPLOYMENT_TARGET)
      list(APPEND args "-mmacosx-version-min=${CMAKE_OSX_DEPLOYMENT_TARGET}")
    endif()
  endif()
  list(APPEND args -nostdinc++)
  foreach(dir IN LISTS CMAKE_CXX_IMPLICIT_INCLUDE_DIRECTORIES)
    if(dir MATCHES "/lib/clang/[0-9]+/include$")
      continue()
    endif()
    list(APPEND args -isystem "${dir}")
  endforeach()
  set(${out_var} "${args}" PARENT_SCOPE)
endfunction()

# Sets `out_var` to the clang arguments needed by `rs_bindings_from_cc` to
# parse the headers of target `tgt`, including the target's transitive usage
# requirements.
#
# The result is a list of generator expressions, one per output line once
# evaluated, e.g. by
#   file(GENERATE OUTPUT <file> CONTENT "$<JOIN:${out_var},\n>")
function(_crubit_clang_args out_var tgt)
  # Default to C++20 if `CMAKE_CXX_STANDARD` is not set.
  set(std 20)
  if(CMAKE_CXX_STANDARD)
    set(std "${CMAKE_CXX_STANDARD}")
  endif()
  set(tgt_std "$<TARGET_PROPERTY:${tgt},CXX_STANDARD>")

  set(args -x c++ "-std=c++$<IF:$<BOOL:${tgt_std}>,${tgt_std},${std}>")
  separate_arguments(cxx_flags UNIX_COMMAND "${CMAKE_CXX_FLAGS}")
  list(APPEND args ${cxx_flags})
  _crubit_system_include_args(system_args)
  list(APPEND args ${system_args})

  # Each property expands to zero or more lines.
  set(includes "$<TARGET_PROPERTY:${tgt},INCLUDE_DIRECTORIES>")
  set(defines "$<TARGET_PROPERTY:${tgt},COMPILE_DEFINITIONS>")
  set(options "$<TARGET_PROPERTY:${tgt},COMPILE_OPTIONS>")
  list(APPEND args
    "$<$<BOOL:${includes}>:-I$<JOIN:${includes},\n-I>>"
    "$<$<BOOL:${defines}>:-D$<JOIN:${defines},\n-D>>"
    "$<$<BOOL:${options}>:$<JOIN:${options},\n>>")
  set(${out_var} "${args}" PARENT_SCOPE)
endfunction()
