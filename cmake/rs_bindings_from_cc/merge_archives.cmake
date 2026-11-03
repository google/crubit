# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Merges static libraries into the single static library OUTPUT.
#
# This is a script that _crubit_merge_archives() (in cargo_inputs.cmake) runs
# during the build, with `cmake -P`. Its arguments are -D variable settings:
#
#   cmake -DLLVM_AR=<llvm-ar> -DOUTPUT=<library> -DINPUT_LIST=<file> \
#     -P merge_archives.cmake
#       merges the libraries listed in <file>, one per line.
#
#   cmake -DLLVM_AR=<llvm-ar> -DOUTPUT=<library> -DMODE=<mode> -DINPUT_DIR=<dir> \
#     -P merge_archives.cmake
#       merges the libraries in <dir> that <mode> selects (see
#       _crubit_wanted_library() below).
#
# The merge uses llvm-ar, because LLVM's libraries may be LTO bitcode that
# other archivers can't read. The script handed to llvm-ar is kept next to
# OUTPUT, as OUTPUT.mri, for debugging.

# _crubit_require(<variable>...)
#
# Stops with an error if any `<variable>` wasn't set with -D.
function(_crubit_require)
  foreach(variable IN LISTS ARGN)
    if(NOT DEFINED ${variable})
      message(FATAL_ERROR "merge_archives.cmake: ${variable} is not set")
    endif()
  endforeach()
endfunction()

# _crubit_wanted_library(<out_wanted> <mode> <library>)
#
# Sets <out_wanted> to TRUE if <mode> selects <library>, and to FALSE if not.
# The modes mirror what cargo/build links:
#   llvm:     the Clang and LLVM libraries, as in cargo/build/clang.rs. It
#             skips libraries with a main() function, the code generators for
#             specific processors (Crubit doesn't generate code), and the
#             clangd and clang-tidy tools.
#   absl:     every Abseil library (named libabsl_*.a).
#   protobuf: libprotobuf and the libutf8_validity it needs, as in
#             cargo/build/protobuf.rs. The other libraries in a Protobuf
#             install would clash with these, or aren't needed.
function(_crubit_wanted_library out_wanted mode library)
  # The library's name, without directory, "lib" prefix and ".a" suffix.
  get_filename_component(name "${library}" NAME_WE)
  string(REGEX REPLACE "^lib" "" name "${name}")

  set(wanted TRUE)
  if(mode STREQUAL "llvm")
    if(NOT name MATCHES "^(clang|LLVM)"
       OR name MATCHES "Main$"
       OR name MATCHES "^LLVM(X86|WebAssem|RISCV|Mips|LoongArch|ARM|AArch)"
       OR name MATCHES "clangd|clangTidy")
      set(wanted FALSE)
    endif()
  elseif(mode STREQUAL "protobuf")
    if(NOT name MATCHES "^(protobuf|utf8_validity)$")
      set(wanted FALSE)
    endif()
  elseif(mode STREQUAL "absl")
    if(NOT name MATCHES "^absl_")
      set(wanted FALSE)
    endif()
  else()
    message(FATAL_ERROR "merge_archives.cmake: unknown MODE '${mode}'")
  endif()
  set(${out_wanted} ${wanted} PARENT_SCOPE)
endfunction()

# _crubit_libraries_in_dir(<out_list> <mode> <dir>)
#
# Sets <out_list> to the static libraries in <dir> that <mode> selects.
function(_crubit_libraries_in_dir out_list mode dir)
  file(GLOB candidates "${dir}/*.a")
  set(libraries "")
  foreach(library IN LISTS candidates)
    _crubit_wanted_library(wanted "${mode}" "${library}")
    if(wanted)
      list(APPEND libraries "${library}")
    endif()
  endforeach()
  set(${out_list} "${libraries}" PARENT_SCOPE)
endfunction()

# _crubit_merge_with_llvm_ar(<output> <library>...)
#
# Merges the libraries into <output> with llvm-ar. Stops with an error if
# llvm-ar fails.
#
# Like cargo/build/setup_bazel_env.py, this hands llvm-ar the merge as a
# script in the "MRI" format, which comes from GNU ar:
#   CREATE <output>
#   ADDLIB <library>    (one per library: adds all of its object files)
#   SAVE
#   END
#
# Reads: LLVM_AR.
function(_crubit_merge_with_llvm_ar output)
  set(mri "CREATE ${output}\n")
  foreach(library IN LISTS ARGN)
    string(APPEND mri "ADDLIB ${library}\n")
  endforeach()
  string(APPEND mri "SAVE\nEND\n")
  set(mri_file "${output}.mri")
  file(WRITE "${mri_file}" "${mri}")

  # llvm-ar's CREATE replaces OUTPUT, so this REMOVE is not necessary for
  # correctness. However if the merge fails this avoids leaving the intermediate
  # state behind.
  file(REMOVE "${output}")

  list(LENGTH ARGN count)
  message(STATUS "Merging ${count} archives into ${output}")
  execute_process(
    COMMAND "${LLVM_AR}" -M
    INPUT_FILE "${mri_file}"
    RESULT_VARIABLE result
    ERROR_VARIABLE error
  )
  if(NOT result EQUAL 0)
    message(FATAL_ERROR "llvm-ar failed (${result}): ${error}")
  endif()
endfunction()

# ===== Main =====

_crubit_require(LLVM_AR OUTPUT)

if(DEFINED INPUT_LIST)
  file(STRINGS "${INPUT_LIST}" libraries)
  set(source "libraries in ${INPUT_LIST}")
else()
  _crubit_require(MODE INPUT_DIR)
  _crubit_libraries_in_dir(libraries "${MODE}" "${INPUT_DIR}")
  set(source "${MODE} libraries in ${INPUT_DIR}")
endif()

if(NOT libraries)
  message(FATAL_ERROR "merge_archives.cmake: found no ${source} to merge")
endif()

# Sorting makes OUTPUT the same from one build to the next.
list(SORT libraries)
list(REMOVE_DUPLICATES libraries)
_crubit_merge_with_llvm_ar("${OUTPUT}" ${libraries})
