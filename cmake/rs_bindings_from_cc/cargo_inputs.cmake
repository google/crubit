# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Functions that prepare the files Cargo's build scripts (cargo/build) expect:
# merged static libraries, and C++ sources generated from .proto files.
# CMakeLists.txt calls them.

# _crubit_merge_archives(<name> MODE <mode> INPUT_DIR <dir> DEPENDS <dep>...)
# _crubit_merge_archives(<name> INPUTS <library>...)
#
# Adds a build step that merges static libraries into the single library
# <cargo_lib_dir>/<name>. Merging means that the order in which the libraries
# are linked doesn't matter (it does for linkers like GNU ld).
#
# The first form merges the libraries in `<dir>` that `<mode>` selects (llvm,
# absl or protobuf; see merge_archives.cmake), and reruns when any `<dep>`
# changes. The second form merges the given libraries, which may be $<...>
# generator expressions.
#
# Reads: cargo_lib_dir, llvm_ar.
function(_crubit_merge_archives name)
  cmake_parse_arguments(PARSE_ARGV 1 arg "" "MODE;INPUT_DIR" "INPUTS;DEPENDS")
  set(script "${CMAKE_CURRENT_FUNCTION_LIST_DIR}/merge_archives.cmake")
  set(output "${cargo_lib_dir}/${name}")

  if(arg_INPUTS)
    set(list_file "${CMAKE_CURRENT_BINARY_DIR}/cargo_deps/${name}.inputs")
    file(GENERATE OUTPUT "${list_file}" CONTENT "$<JOIN:${arg_INPUTS},\n>\n")
    set(script_args -DINPUT_LIST=${list_file})
    set(depends "${list_file}" ${arg_INPUTS})
  else()
    set(script_args -DMODE=${arg_MODE} -DINPUT_DIR=${arg_INPUT_DIR})
    set(depends ${arg_DEPENDS})
  endif()

  add_custom_command(
    OUTPUT "${output}"
    COMMAND "${CMAKE_COMMAND}" -DLLVM_AR=${llvm_ar} ${script_args} -DOUTPUT=${output}
      -P "${script}"
    DEPENDS "${script}" ${depends}
    COMMENT "Merging libraries into ${name}"
    VERBATIM
  )
endfunction()

# _crubit_generate_protos(<out_outputs>)
#
# Adds a build step that runs protoc on Crubit's .proto files, writing the C++
# sources to proto_gen_dir. Sets <out_outputs> to the generated files.
#
# These are generated before Cargo runs; see
# docs/overview/cargo_build_protobuf.md for an explanation.
#
# Reads: proto_gen_dir, protoc, protoc_depends, CRUBIT_SOURCE_ROOT.
function(_crubit_generate_protos out_outputs)
  set(protos
    rs_bindings_from_cc/ir.proto
    rs_bindings_from_cc/generate_bindings/generate_bindings.proto
  )
  set(inputs "")
  set(outputs "")
  foreach(proto IN LISTS protos)
    string(REGEX REPLACE "\\.proto$" "" stem "${proto}")
    list(APPEND inputs "${CRUBIT_SOURCE_ROOT}/${proto}")
    list(APPEND outputs "${proto_gen_dir}/${stem}.pb.h" "${proto_gen_dir}/${stem}.pb.cc")
  endforeach()

  add_custom_command(
    OUTPUT ${outputs}
    COMMAND "${protoc}" "--cpp_out=${proto_gen_dir}" "-I${CRUBIT_SOURCE_ROOT}" ${inputs}
    DEPENDS ${inputs} ${protoc_depends}
    COMMENT "Generating C++ Protobuf sources"
    VERBATIM
  )
  set(${out_outputs} "${outputs}" PARENT_SCOPE)
endfunction()
