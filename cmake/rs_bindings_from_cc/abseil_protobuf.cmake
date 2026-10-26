# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Functions that find installed Abseil and Protobuf packages, or build them
# from source.
#
# Abseil and Protobuf are used together or not at all. Protobuf has to link
# against the Abseil it was built with (Abseil's symbol names include its
# version and options): when no suitable Protobuf is found, both are built from
# source.
#
# _crubit_use_installed_abseil_protobuf() and _crubit_build_abseil_protobuf() set
# the same variables in the caller, so later steps don't need to know which one
# ran:
#   absl_include_dirs, protobuf_include_dirs: header directories, as lists.
#   protoc: the protoc executable.
#   protoc_depends: what a step that runs protoc has to depend on.
#   absl_archives, protobuf_archives: arguments for _crubit_merge_archives()
#     that say which static libraries make up Abseil and Protobuf.

# _crubit_find_abseil_protobuf(<out_found>)
#
# Looks for installed Protobuf and Abseil packages with find_package, and sets
# <out_found> to TRUE if they're suitable: Protobuf has exactly the version
# CRUBIT_PROTOBUF_VERSION that the Rust `protobuf` crate needs, comes with
# protoc, and both are static libraries. Otherwise it says why not, and sets
# <out_found> to FALSE.
#
# The packages' targets (protobuf::libprotobuf, absl::base, ...) stay available
# after it returns.
function(_crubit_find_abseil_protobuf out_found)
  set(${out_found} FALSE PARENT_SCOPE)
  find_package(protobuf CONFIG QUIET)
  if(NOT protobuf_FOUND)
    return()
  endif()

  set(reason "")
  if(NOT protobuf_VERSION VERSION_EQUAL CRUBIT_PROTOBUF_VERSION)
    set(reason "version ${protobuf_VERSION}, not ${CRUBIT_PROTOBUF_VERSION}")
  else()
    foreach(target protobuf::libprotobuf utf8_range::utf8_validity absl::base)
      if(NOT TARGET ${target})
        set(reason "no ${target} target")
        break()
      endif()
      get_target_property(type ${target} TYPE)
      if(NOT type STREQUAL "STATIC_LIBRARY")
        set(reason "${target} is not a static library")
        break()
      endif()
    endforeach()
    if(NOT reason AND NOT TARGET protobuf::protoc)
      set(reason "no protoc")
    endif()
  endif()

  if(reason)
    message(STATUS "Not using installed Protobuf and Abseil (${reason}); building them")
  else()
    message(STATUS "Using installed Protobuf ${protobuf_VERSION} and Abseil")
    set(${out_found} TRUE PARENT_SCOPE)
  endif()
endfunction()

# _crubit_installed_abseil_dir(<out_dir>)
#
# Sets <out_dir> to the directory that holds the installed Abseil's static
# libraries: the directory of absl::base's library file.
#
# Abseil is a collection of many small libraries. Depending on how it was
# installed, it may not be the case that they live under a dedicated directory
# with no other libraries (e.g., it's not uncommon for them to be located in
# "/opt/homebrew/lib" on macOS). In order to determine which libraries come
# from Abseil we rely on the naming convention that these libraries all match
# the glob "libabsl_*.a".
#
# A package's targets can be imported for several build types (Release,
# Debug, ...), each with its own file. Any of them gives the directory.
function(_crubit_installed_abseil_dir out_dir)
  get_target_property(location absl::base IMPORTED_LOCATION)
  if(NOT location)
    get_target_property(build_types absl::base IMPORTED_CONFIGURATIONS)
    foreach(build_type IN LISTS build_types)
      get_target_property(location absl::base IMPORTED_LOCATION_${build_type})
      if(location)
        break()
      endif()
    endforeach()
  endif()
  if(NOT location)
    message(FATAL_ERROR "Can't find the library file of the installed absl::base")
  endif()
  get_filename_component(dir "${location}" DIRECTORY)
  set(${out_dir} "${dir}" PARENT_SCOPE)
endfunction()

# _crubit_use_installed_abseil_protobuf()
#
# Uses the packages that _crubit_find_abseil_protobuf() found. Sets the
# variables listed at the top of this file.
function(_crubit_use_installed_abseil_protobuf)
  get_target_property(absl_include_dirs absl::base INTERFACE_INCLUDE_DIRECTORIES)
  get_target_property(protobuf_include_dirs protobuf::libprotobuf INTERFACE_INCLUDE_DIRECTORIES)
  _crubit_installed_abseil_dir(absl_lib_dir)
  file(GLOB absl_libraries "${absl_lib_dir}/libabsl_*.a")

  set(absl_include_dirs "${absl_include_dirs}" PARENT_SCOPE)
  set(protobuf_include_dirs "${protobuf_include_dirs}" PARENT_SCOPE)
  set(protoc "$<TARGET_FILE:protobuf::protoc>" PARENT_SCOPE)
  set(protoc_depends "$<TARGET_FILE:protobuf::protoc>" PARENT_SCOPE)
  set(absl_archives
    MODE absl INPUT_DIR "${absl_lib_dir}" DEPENDS ${absl_libraries}
    PARENT_SCOPE)
  set(protobuf_archives INPUTS
    "$<TARGET_FILE:protobuf::libprotobuf>"
    "$<TARGET_FILE:utf8_range::utf8_validity>"
    PARENT_SCOPE)
endfunction()

# _crubit_build_abseil_protobuf()
#
# Adds build steps that download Abseil CRUBIT_ABSL_VERSION and Protobuf
# CRUBIT_PROTOBUF_VERSION, then build and install them into deps_dir. They
# run when the project is built, not while CMake configures it. Sets the
# variables listed at the top of this file.
#
# Reads: deps_dir.
function(_crubit_build_abseil_protobuf)
  # Settings for both builds: static, optimized, position-independent code
  # with the same compilers as this project.
  set(common_args
    -DCMAKE_BUILD_TYPE=Release
    -DCMAKE_C_COMPILER=${CMAKE_C_COMPILER}
    -DCMAKE_CXX_COMPILER=${CMAKE_CXX_COMPILER}
    -DCMAKE_CXX_STANDARD=20
    -DCMAKE_POSITION_INDEPENDENT_CODE=ON
    -DCMAKE_INSTALL_LIBDIR=lib
    -DBUILD_SHARED_LIBS=OFF
    -DBUILD_TESTING=OFF
  )
  if(CMAKE_OSX_SYSROOT)
    list(APPEND common_args -DCMAKE_OSX_SYSROOT=${CMAKE_OSX_SYSROOT})
  endif()

  set(absl_prefix "${deps_dir}/absl")
  _crubit_url_hash(absl_hash "abseil-cpp-${CRUBIT_ABSL_VERSION}.tar.gz")
  ExternalProject_Add(crubit_absl
    URL "https://github.com/abseil/abseil-cpp/archive/refs/tags/${CRUBIT_ABSL_VERSION}.tar.gz"
    ${absl_hash}
    DOWNLOAD_EXTRACT_TIMESTAMP TRUE
    PREFIX "${deps_dir}/absl-build"
    INSTALL_DIR "${absl_prefix}"
    CMAKE_ARGS
      ${common_args}
      -DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>
      -DABSL_PROPAGATE_CXX_STD=ON
      -DABSL_BUILD_TESTING=OFF
    USES_TERMINAL_DOWNLOAD TRUE
    USES_TERMINAL_BUILD TRUE
  )

  set(protobuf_prefix "${deps_dir}/protobuf")
  _crubit_url_hash(protobuf_hash "protobuf-${CRUBIT_PROTOBUF_VERSION}.tar.gz")
  ExternalProject_Add(crubit_protobuf
    URL "https://github.com/protocolbuffers/protobuf/archive/refs/tags/v${CRUBIT_PROTOBUF_VERSION}.tar.gz"
    ${protobuf_hash}
    DOWNLOAD_EXTRACT_TIMESTAMP TRUE
    PREFIX "${deps_dir}/protobuf-build"
    INSTALL_DIR "${protobuf_prefix}"
    DEPENDS crubit_absl
    CMAKE_ARGS
      ${common_args}
      -DCMAKE_INSTALL_PREFIX=<INSTALL_DIR>
      -DCMAKE_PREFIX_PATH=${absl_prefix}
      -Dprotobuf_BUILD_TESTS=OFF
      -Dprotobuf_BUILD_CONFORMANCE=OFF
      -Dprotobuf_BUILD_EXAMPLES=OFF
      -Dprotobuf_BUILD_PROTOC_BINARIES=ON
      -Dprotobuf_INSTALL=ON
      -Dprotobuf_WITH_ZLIB=OFF
    USES_TERMINAL_DOWNLOAD TRUE
    USES_TERMINAL_BUILD TRUE
  )

  # Each build touches a "done" file when it finishes. Steps that use a build's
  # output depend on that file, so they rerun when the build reruns, and on its
  # target, so they run after it.
  ExternalProject_Get_Property(crubit_absl STAMP_DIR)
  set(absl_done "${STAMP_DIR}/crubit_absl-done")
  ExternalProject_Get_Property(crubit_protobuf STAMP_DIR)
  set(protobuf_done "${STAMP_DIR}/crubit_protobuf-done")

  set(absl_include_dirs "${absl_prefix}/include" PARENT_SCOPE)
  set(protobuf_include_dirs "${protobuf_prefix}/include" PARENT_SCOPE)
  set(protoc "${protobuf_prefix}/bin/protoc${CMAKE_EXECUTABLE_SUFFIX}" PARENT_SCOPE)
  set(protoc_depends "${protobuf_done}" crubit_protobuf PARENT_SCOPE)
  # These install directories hold only Abseil or Protobuf, so the merge step
  # can take the libraries it needs straight from them.
  set(absl_archives
    MODE absl INPUT_DIR "${absl_prefix}/lib" DEPENDS "${absl_done}" crubit_absl
    PARENT_SCOPE)
  set(protobuf_archives
    MODE protobuf INPUT_DIR "${protobuf_prefix}/lib" DEPENDS "${protobuf_done}" crubit_protobuf
    PARENT_SCOPE)
endfunction()
