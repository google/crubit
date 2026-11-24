# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

include_guard(GLOBAL)

# _crubit_std_write_tu(<tu> <header>...)
#
# Writes the translation unit `<tu>`, which includes each `<header>`.
function(_crubit_std_write_tu tu)
  list(TRANSFORM ARGN PREPEND "#include <" OUTPUT_VARIABLE includes)
  list(TRANSFORM includes APPEND ">\n")
  list(JOIN includes "" content)
  file(WRITE "${tu}" "${content}")
endfunction()

# _crubit_std_parse_included(<out_var> <output>)
#
# Sets `out_var` to the normalized paths of the headers listed in `<output>`,
# compiler output for `-H`, which prints a line for each header the compiler
# reads: one dot per level of inclusion, a space, and the path.
#
# Unlike a depfile, it leaves out headers the compiler skips without reading
# them because their include guard is already defined.
function(_crubit_std_parse_included out_var output)
  string(REGEX MATCHALL "\n\\.+ [^\n]+" lines "\n${output}")
  set(headers "")
  foreach(line IN LISTS lines)
    string(REGEX REPLACE "^\n\\.+ " "" header "${line}")
    string(STRIP "${header}" header)
    cmake_path(NORMAL_PATH header)
    list(APPEND headers "${header}")
  endforeach()
  list(REMOVE_DUPLICATES headers)
  set(${out_var} ${headers} PARENT_SCOPE)
endfunction()

# _crubit_std_build_headers(<out_var> <tu> <std>)
#
# Sets `out_var` to the headers that the project's real C++ compiles read for
# `<tu>`, in C++ standard `<std>`. They come from compiling `<tu>` with
# try_compile, so that CMake picks the flags, as for the project's own code:
# the toolchain file, CMAKE_CXX_FLAGS, the sysroot, the target, and so on.
function(_crubit_std_build_headers out_var tu std)
  set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)
  try_compile(compiled "${CMAKE_BINARY_DIR}/CMakeFiles/crubit_std_headers"
    SOURCES "${tu}"
    COMPILE_DEFINITIONS -H
    CXX_STANDARD ${std}
    CXX_STANDARD_REQUIRED ON
    OUTPUT_VARIABLE output)
  if(NOT compiled)
    message(FATAL_ERROR
      "Crubit: the C++ compiler can't compile the C++ standard library "
      "headers:\n${output}")
  endif()
  _crubit_std_parse_included(headers "${output}")
  if(NOT headers)
    message(FATAL_ERROR
      "Crubit: could not read which headers the C++ compiler includes "
      "(from -H):\n${output}")
  endif()
  set(${out_var} ${headers} PARENT_SCOPE)
endfunction()

# _crubit_std_preprocessed_headers(<out_var> <tu> <compile_arg>...)
#
# Preprocesses `<tu>` with the C++ compiler and `<compile_arg>...`, and sets
# `out_var` to the headers it reads.
function(_crubit_std_preprocessed_headers out_var tu)
  execute_process(
    COMMAND "${CMAKE_CXX_COMPILER}" ${ARGN} -E -H "${tu}"
    OUTPUT_QUIET
    ERROR_VARIABLE output
    RESULT_VARIABLE result)
  if(NOT result EQUAL 0)
    message(FATAL_ERROR
      "Crubit: failed to locate C++ standard library headers:\n${output}")
  endif()
  _crubit_std_parse_included(headers "${output}")
  set(${out_var} ${headers} PARENT_SCOPE)
endfunction()

# _crubit_std_check_headers(BUILD <path>... CRUBIT <path>...)
#
# Stops with an error if the headers that the real build reads (`BUILD`)
# do not match the ones Crubit's clang args finds (`CRUBIT`). Crubit
# does not require builtin headers to match between the generator tool
# and the actual build, so these headers should be removed via a call
# to `_crubit_drop_builtin_headers` before invoking this function.
function(_crubit_std_check_headers)
  cmake_parse_arguments(PARSE_ARGV 0 ARG "" "" "BUILD;CRUBIT")
  set(only_build ${ARG_BUILD})
  list(REMOVE_ITEM only_build ${ARG_CRUBIT})
  set(only_crubit ${ARG_CRUBIT})
  list(REMOVE_ITEM only_crubit ${ARG_BUILD})
  if(NOT only_build AND NOT only_crubit)
    return()
  endif()
  list(JOIN only_build "\n  " only_build)
  list(JOIN only_crubit "\n  " only_crubit)
  message(FATAL_ERROR
    "Crubit: the C++ standard library headers that rs_bindings_from_cc would "
    "parse are not the ones the build compiles with, so the cc_std bindings "
    "would not match the C++ code. A toolchain setting is probably not passed "
    "on to rs_bindings_from_cc.\n"
    "Only the build reads:\n  ${only_build}\n"
    "Only rs_bindings_from_cc would read:\n  ${only_crubit}")
endfunction()

# _crubit_std_public_headers(<out_var>
#   LIBCXX_HEADERS <name>...
#   LIBC_HEADERS <name>...
#   DEPS <path>...)
#
# Sets `out_var` to the path in `DEPS` of each header in `LIBCXX_HEADERS` (found
# under `c++/v1/`) and `LIBC_HEADERS` (found outside it), in that order.
#
# rs_bindings_from_cc's `--public_headers` are the headers it generates
# bindings for: it parses a file that `#include`s each of them. It takes paths,
# not `#include <...>` names, and each path must also be one of the headers
# given to the cc_std target in `--target_args` (`DEPS`), spelled exactly the
# same. So each name is looked up in `DEPS` instead of being searched for in the
# include directories. That gives the file the compiler actually used, in the
# compiler's spelling of its path.
#
# A name can match more than one file in `DEPS`. libc++ has its own wrappers for
# some C headers (such as `c++/v1/stdlib.h`), and a C++ header name could match
# a file of the same name elsewhere. Which directory each kind comes from tells
# them apart, as in the Bazel rules (toolchain_headers.bzl).
function(_crubit_std_public_headers out_var)
  cmake_parse_arguments(PARSE_ARGV 1 ARG "" "" "LIBCXX_HEADERS;LIBC_HEADERS;DEPS")
  set(public_headers "")
  foreach(header IN LISTS ARG_LIBCXX_HEADERS ARG_LIBC_HEADERS)
    set(found "")
    foreach(dep IN LISTS ARG_DEPS)
      if(header IN_LIST ARG_LIBCXX_HEADERS AND dep MATCHES "/c\\+\\+/v1/${header}$")
        set(found "${dep}")
      elseif(header IN_LIST ARG_LIBC_HEADERS AND dep MATCHES "/${header}$"
             AND NOT dep MATCHES "/c\\+\\+/v1/")
        set(found "${dep}")
      endif()
      if(found)
        break()
      endif()
    endforeach()
    if(NOT found)
      message(FATAL_ERROR "Crubit: standard library header <${header}> not found")
    endif()
    list(APPEND public_headers "${found}")
  endforeach()
  set(${out_var} ${public_headers} PARENT_SCOPE)
endfunction()

# Creates the `cc_std` bindings crate (Rust bindings for the C++ standard
# library) and `crubit::cc_std`, once per build.
function(_crubit_ensure_std)
  if(TARGET crubit_impl_cc_std)
    return()
  endif()
  _crubit_cxx_standard(std)
  _crubit_system_include_args(system_args)
  separate_arguments(cxx_flags UNIX_COMMAND "${CMAKE_CXX_FLAGS}")
  set(compile_args -x c++ -std=c++${std} ${cxx_flags} ${system_args})

  _crubit_source_dir(crubit_dir)
  set(impl "${crubit_dir}/support/cc_std_impl")
  include("${impl}/cc_std_lists.cmake")

  # Find the standard library headers that the build reads, exactly as Clang
  # spells them, from a file that includes all of them. Check that
  # rs_bindings_from_cc's args find the same ones.
  set(libcxx_headers ${_CRUBIT_STD_LIBCXX_HEADERS})
  set(libc_headers ${_CRUBIT_STD_LIBC_HEADERS})
  if(NOT APPLE)
    list(APPEND libc_headers ${_CRUBIT_STD_NON_APPLE_LIBC_HEADERS})
  endif()
  set(tu "${CRUBIT_CRATES_DIR}/cc_std/all_std_headers.cc")
  _crubit_std_write_tu("${tu}" ${libcxx_headers} ${libc_headers})
  _crubit_std_build_headers(build_headers "${tu}" ${std})
  _crubit_std_preprocessed_headers(crubit_headers "${tu}" ${compile_args})
  # The compiler's builtin headers are not part of cc_std: rs_bindings_from_cc
  # uses its own.
  _crubit_drop_builtin_headers(build_headers)
  _crubit_drop_builtin_headers(crubit_headers)
  _crubit_std_check_headers(BUILD ${build_headers} CRUBIT ${crubit_headers})

  set(all_headers ${build_headers})
  _crubit_std_public_headers(public_headers
    LIBCXX_HEADERS ${libcxx_headers}
    LIBC_HEADERS ${libc_headers}
    DEPS ${all_headers})

  list(TRANSFORM _CRUBIT_STD_HELPER_HEADERS PREPEND "${impl}/" OUTPUT_VARIABLE helpers)
  list(APPEND public_headers ${helpers})
  list(APPEND all_headers ${helpers})

  list(TRANSFORM _CRUBIT_STD_EXTRA_RS_SRCS PREPEND "${impl}/" OUTPUT_VARIABLE extra_rs_srcs)

  list(JOIN all_headers "\",\"" json_headers)
  set(label "//support/cc_std:cc_std")
  _crubit_generate(
    CRATE cc_std
    LABEL "${label}"
    CLANG_ARGS ${compile_args}
    PUBLIC_HEADERS ${public_headers}
    TARGET_ARGS_JSON
      "{\"t\":\"${label}\",\"h\":[\"${json_headers}\"],\"f\":[\"all\",\"no_impl_cxx_extern_type\"],\"c\":\"cc_std\"}"
    EXTRA_RS_SRCS ${extra_rs_srcs})
endfunction()
