# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

include_guard(GLOBAL)

# Sets `out_var` to the C++ standard (such as `20`) to parse C++ with when a
# target doesn't set its own: CMAKE_CXX_STANDARD, or 20 if that's not set.
# Stops with an error below C++20, which Crubit's support library requires.
function(_crubit_cxx_standard out_var)
  set(std 20)
  if(CMAKE_CXX_STANDARD)
    set(std "${CMAKE_CXX_STANDARD}")
  endif()
  if(std EQUAL 98 OR std LESS 20)
    message(FATAL_ERROR
      "Crubit requires C++20 or later, but CMAKE_CXX_STANDARD is ${std}.")
  endif()
  set(${out_var} "${std}" PARENT_SCOPE)
endfunction()

# Sets `out_var` to the `--target_to_arg` entry for rs_bindings_from_cc's own
# builtin headers (`stddef.h`, `stdarg.h` and so on, in
# CRUBIT_CLANG_RESOURCE_DIR).
#
# Every header rs_bindings_from_cc reads needs an owning target, and these
# belong to no library. This entry gives them to a placeholder target,
# `//:_nothing_should_depend_on_private_builtin_hdrs`, that no bindings crate
# is built for, so their declarations get no bindings anywhere. The Bazel rules
# use the same placeholder (see toolchain_headers.bzl).
#
# Without the entry, rs_bindings_from_cc would still generate no bindings for
# them, but it would label each one `//_unknown_target:<path>`. Then, when
# bindings can't be generated because of a type from one of these headers, its
# error message would tell the user to enable Crubit on the library that
# provides `<path>`, which isn't possible for builtin headers.
#
# The entry is computed once per configure.
function(_crubit_builtin_headers_target_arg out_var)
  get_property(json GLOBAL PROPERTY _CRUBIT_BUILTIN_HEADERS_TARGET_ARG)
  if(NOT json)
    file(GLOB_RECURSE headers LIST_DIRECTORIES false
      "${CRUBIT_CLANG_RESOURCE_DIR}/include/*")
    list(JOIN headers "\",\"" joined)
    set(json
      "{\"t\":\"//:_nothing_should_depend_on_private_builtin_hdrs\",\"h\":[\"${joined}\"]}")
    set_property(GLOBAL PROPERTY _CRUBIT_BUILTIN_HEADERS_TARGET_ARG "${json}")
  endif()
  set(${out_var} "${json}" PARENT_SCOPE)
endfunction()

# Sets `out_var` to a regex matching the Clang builtin header directory
# (`<resource dir>/include`) and the paths in it.
function(_crubit_builtin_headers_regex out_var)
  set(${out_var} "/lib(64)?/clang/[0-9.]+/include(/|$)" PARENT_SCOPE)
endfunction()

# _crubit_drop_builtin_headers(<list_var>)
#
# Removes the C++ compiler's builtin headers from the paths in `<list_var>`.
# rs_bindings_from_cc provides its own (see the function
# _crubit_builtin_headers_target_arg).
function(_crubit_drop_builtin_headers list_var)
  _crubit_builtin_headers_regex(builtin)
  set(paths ${${list_var}})
  list(FILTER paths EXCLUDE REGEX "${builtin}")
  set(${list_var} ${paths} PARENT_SCOPE)
endfunction()

# Sets `out_var` to the `-isysroot <sdk>` args for macOS, or to nothing when
# CMAKE_CXX_FLAGS already chooses the SDK.
#
# rs_bindings_from_cc's Clang has no default macOS SDK, so it needs an
# -isysroot. These args come after CMAKE_CXX_FLAGS, and Clang uses the last
# -isysroot, so, as CMake does, CMAKE_OSX_SYSROOT wins over the flags. Without
# either, use the SDK the compiler would default to.
function(_crubit_macos_sysroot_args out_var)
  if(CMAKE_OSX_SYSROOT)
    set(sdk "${CMAKE_OSX_SYSROOT}")
  elseif(CMAKE_CXX_FLAGS MATCHES "(^|[ \t])-isysroot")
    set(${out_var} "" PARENT_SCOPE)
    return()
  else()
    execute_process(
      COMMAND xcrun --show-sdk-path
      OUTPUT_VARIABLE sdk
      OUTPUT_STRIP_TRAILING_WHITESPACE
      COMMAND_ERROR_IS_FATAL ANY)
    if(NOT sdk)
      message(FATAL_ERROR
        "Crubit: could not find the macOS SDK; set CMAKE_OSX_SYSROOT.")
    endif()
  endif()
  set(${out_var} -isysroot "${sdk}" PARENT_SCOPE)
endfunction()

# Sets `out_var` to the clang args that select the target and locate the C++
# standard library and the system headers the way the project's C++ compiler
# does.
#
# `rs_bindings_from_cc` embeds its own Clang (and its own builtin headers), so
# the compiler's search paths are passed explicitly, minus the compiler's
# builtin header directory. Clang searches its builtin headers after the
# `-isystem` directories, so the directories that the compiler searches after
# its builtin headers are passed with `-idirafter` to keep the same order.
function(_crubit_system_include_args out_var)
  set(args "")
  if(CMAKE_CXX_COMPILER_TARGET)
    list(APPEND args "--target=${CMAKE_CXX_COMPILER_TARGET}")
  endif()
  if(CMAKE_SYSROOT_COMPILE)
    list(APPEND args "--sysroot=${CMAKE_SYSROOT_COMPILE}")
  elseif(CMAKE_SYSROOT)
    list(APPEND args "--sysroot=${CMAKE_SYSROOT}")
  endif()
  if(APPLE)
    list(LENGTH CMAKE_OSX_ARCHITECTURES num_archs)
    if(num_archs GREATER 1)
      message(FATAL_ERROR
        "Crubit can generate bindings for only one architecture, but "
        "CMAKE_OSX_ARCHITECTURES is ${CMAKE_OSX_ARCHITECTURES}.")
    elseif(num_archs EQUAL 1)
      # As CMake does, even with CMAKE_CXX_COMPILER_TARGET: Clang builds for
      # the `-arch` architecture when it differs from the `--target` one.
      list(APPEND args -arch "${CMAKE_OSX_ARCHITECTURES}")
    endif()
    _crubit_macos_sysroot_args(sysroot_args)
    list(APPEND args ${sysroot_args})
    if(CMAKE_OSX_DEPLOYMENT_TARGET)
      list(APPEND args "-mmacosx-version-min=${CMAKE_OSX_DEPLOYMENT_TARGET}")
    endif()
  endif()
  list(APPEND args -nostdinc++)
  _crubit_builtin_headers_regex(builtin)
  set(include_flag -isystem)
  foreach(dir IN LISTS CMAKE_CXX_IMPLICIT_INCLUDE_DIRECTORIES)
    if(dir MATCHES "${builtin}")
      set(include_flag -idirafter)
      continue()
    endif()
    list(APPEND args ${include_flag} "${dir}")
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
  _crubit_cxx_standard(std)
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
