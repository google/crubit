# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Functions that find or download LLVM/Clang. CMakeLists.txt calls them.
#
# A note on CMake functions: a function can read every variable of the code
# that calls it, but the variables it sets stay inside it unless it sets them
# with PARENT_SCOPE. Each function below says what it reads and what it sets.

# _crubit_find_llvm(<out_root>)
#
# Sets <out_root> to the root of a usable LLVM install, or to "" if there is
# none. It tries, in order:
#   1. CRUBIT_LLVM_ROOT, if it's set.
#   2. An installed LLVM of version CRUBIT_LLVM_VERSION, found with
#      find_package(LLVM) (see CMAKE_PREFIX_PATH and LLVM_DIR).
function(_crubit_find_llvm out_root)
  if(CRUBIT_LLVM_ROOT)
    set(${out_root} "${CRUBIT_LLVM_ROOT}" PARENT_SCOPE)
    return()
  endif()

  set(${out_root} "" PARENT_SCOPE)
  find_package(LLVM "${CRUBIT_LLVM_VERSION}" CONFIG QUIET)
  if(NOT LLVM_FOUND)
    return()
  endif()

  # Cargo needs llvm-config and static Clang libraries, and many Linux
  # distributions only ship shared ones.
  if(EXISTS "${LLVM_INSTALL_PREFIX}/bin/llvm-config"
     AND EXISTS "${LLVM_LIBRARY_DIR}/libclangAST.a")
    set(${out_root} "${LLVM_INSTALL_PREFIX}" PARENT_SCOPE)
  else()
    message(STATUS
      "Not using LLVM ${LLVM_PACKAGE_VERSION} at ${LLVM_INSTALL_PREFIX}: "
      "it has no llvm-config or no static Clang libraries")
  endif()
endfunction()

# _crubit_llvm_release_url(<out_url> <out_file_name>)
#
# Sets <out_url> to the official LLVM CRUBIT_LLVM_VERSION release for this
# machine, and <out_file_name> to its file name. Stops with an error on
# machines that LLVM doesn't publish releases for.
function(_crubit_llvm_release_url out_url out_file_name)
  if(CMAKE_HOST_SYSTEM_PROCESSOR MATCHES "^(arm64|aarch64|ARM64)$")
    set(arch "ARM64")
  elseif(CMAKE_HOST_SYSTEM_PROCESSOR MATCHES "^(x86_64|AMD64|amd64)$")
    set(arch "X64")
  else()
    message(FATAL_ERROR
      "No LLVM release for processor '${CMAKE_HOST_SYSTEM_PROCESSOR}'. Set CRUBIT_LLVM_ROOT.")
  endif()

  if(CMAKE_HOST_APPLE)
    if(arch STREQUAL "X64")
      message(FATAL_ERROR
        "LLVM doesn't publish releases for x86_64 macOS. Set CRUBIT_LLVM_ROOT.")
    endif()
    set(os "macOS")
  elseif(CMAKE_HOST_SYSTEM_NAME STREQUAL "Linux")
    set(os "Linux")
  else()
    message(FATAL_ERROR
      "No LLVM release with static libraries for '${CMAKE_HOST_SYSTEM_NAME}'. Set CRUBIT_LLVM_ROOT.")
  endif()

  set(file_name "LLVM-${CRUBIT_LLVM_VERSION}-${os}-${arch}.tar.xz")
  set(${out_file_name} "${file_name}" PARENT_SCOPE)
  set(${out_url}
    "https://github.com/llvm/llvm-project/releases/download/llvmorg-${CRUBIT_LLVM_VERSION}/${file_name}"
    PARENT_SCOPE)
endfunction()

# _crubit_download_llvm(<out_root>)
#
# Downloads and unpacks LLVM while CMake configures the project, and sets
# <out_root> to where it was unpacked. The download is CRUBIT_LLVM_URL if
# that's set, otherwise the official release for this machine.
#
# The download is checked against CRUBIT_LLVM_URL_HASH if that's set, or else
# the hash in hashes.cmake for official releases.
function(_crubit_download_llvm out_root)
  if(CRUBIT_LLVM_URL)
    set(url "${CRUBIT_LLVM_URL}")
    if(CRUBIT_LLVM_URL_HASH)
      set(hash URL_HASH "${CRUBIT_LLVM_URL_HASH}")
    else()
      message(WARNING "CRUBIT_LLVM_URL_HASH is not set; the LLVM download is not verified.")
      set(hash "")
    endif()
  else()
    _crubit_llvm_release_url(url file_name)
    if(CRUBIT_LLVM_URL_HASH)
      set(hash URL_HASH "${CRUBIT_LLVM_URL_HASH}")
    else()
      _crubit_url_hash(hash "${file_name}")
    endif()
  endif()

  message(STATUS "Using LLVM from ${url} (this is a large download)")
  FetchContent_Declare(crubit_llvm
    URL "${url}"
    ${hash}
    DOWNLOAD_EXTRACT_TIMESTAMP TRUE
  )
  FetchContent_MakeAvailable(crubit_llvm)
  set(${out_root} "${crubit_llvm_SOURCE_DIR}" PARENT_SCOPE)
endfunction()

# _crubit_llvm_dev_date(<out_date> <llvm_version>)
#
# Sets <out_date> to the date (YYYYMMDD) of the LLVM commit that Crubit is
# built against. Crubit's C++ code uses it to handle differences between LLVM
# versions. It's CRUBIT_LLVM_DEV_DATE if that's set, otherwise the date that
# the release branch of <llvm_version> was created.
function(_crubit_llvm_dev_date out_date llvm_version)
  if(CRUBIT_LLVM_DEV_DATE)
    set(date "${CRUBIT_LLVM_DEV_DATE}")
  elseif(llvm_version MATCHES "^23\\.")
    set(date "20260714")
  else()
    message(FATAL_ERROR
      "Don't know the LLVM commit date for LLVM ${llvm_version}. Set CRUBIT_LLVM_DEV_DATE=YYYYMMDD "
      "(see cargo/build/get_llvm_commit_date.py).")
  endif()
  message(STATUS "Using CRUBIT_LLVM_DEV_DATE=${date}")
  set(${out_date} "${date}" PARENT_SCOPE)
endfunction()

# _crubit_llvm_config(<llvm_config> <out> <args>...)
#
# Runs the llvm-config executable <llvm_config> with <args>, and sets <out> to
# what it prints.
function(_crubit_llvm_config llvm_config out)
  execute_process(
    COMMAND "${llvm_config}" ${ARGN}
    OUTPUT_VARIABLE output
    OUTPUT_STRIP_TRAILING_WHITESPACE
    COMMAND_ERROR_IS_FATAL ANY
  )
  set(${out} "${output}" PARENT_SCOPE)
endfunction()

# _crubit_check_llvm_config(<llvm_config>)
#
# Stops with an error if the llvm-config executable <llvm_config> doesn't run.
# Without this check, a broken llvm-config (for example, one built for another
# machine) would only fail with a bare "process failed" error.
function(_crubit_check_llvm_config llvm_config)
  execute_process(
    COMMAND "${llvm_config}" --version
    RESULT_VARIABLE result
    OUTPUT_QUIET
    ERROR_VARIABLE error
  )
  if(NOT result EQUAL 0)
    message(FATAL_ERROR
      "${llvm_config} doesn't run (${result}): ${error}")
  endif()
endfunction()

# _crubit_check_llvm_ar(<llvm_ar> <llvm_version> <root>)
#
# Stops with an error unless the llvm-ar executable <llvm_ar> is of LLVM
# version <llvm_version>. LLVM releases ship their static libraries as LTO
# bitcode, which only an llvm-ar of the same LLVM version can read. A
# mismatched llvm-ar can come from the PATH, when the LLVM install at <root>
# doesn't have one.
function(_crubit_check_llvm_ar llvm_ar llvm_version root)
  execute_process(
    COMMAND "${llvm_ar}" --version
    RESULT_VARIABLE result
    OUTPUT_VARIABLE output
    ERROR_VARIABLE error
  )
  if(NOT result EQUAL 0)
    message(FATAL_ERROR "${llvm_ar} doesn't run (${result}): ${error}")
  endif()
  # llvm-ar prints "LLVM version <version>", possibly after a vendor name such
  # as "Homebrew".
  if(NOT output MATCHES "LLVM version ([^ \t\r\n]+)")
    message(FATAL_ERROR
      "Can't tell the LLVM version of ${llvm_ar}. It printed:\n${output}")
  endif()
  if(NOT CMAKE_MATCH_1 STREQUAL llvm_version)
    message(FATAL_ERROR
      "${llvm_ar} is from LLVM ${CMAKE_MATCH_1}, but the LLVM install at "
      "${root} is LLVM ${llvm_version}. They must be the same version. Put an "
      "llvm-ar of LLVM ${llvm_version} in ${root}/bin or on the PATH.")
  endif()
endfunction()

# _crubit_describe_llvm(<root>)
#
# Finds the tools in the LLVM install at <root>, and asks llvm-config about it.
#
# Sets these variables in the caller:
#   llvm_version, llvm_include_dir, llvm_lib_dir, llvm_dev_date
#   llvm_libraries: the static libraries in llvm_lib_dir.
#   llvm_system_libs: the system libraries that LLVM needs, as a list.
#   llvm_ar: llvm-ar, from <root> if it has one, or else from the PATH. It must
#     be the same LLVM version as <root>.
#   llvm_clang, llvm_lld: clang and lld from <root>, or a NOTFOUND value if
#     <root> doesn't have them.
function(_crubit_describe_llvm root)
  # NO_CACHE makes find_program search again on every configure, so changing
  # CRUBIT_LLVM_ROOT takes effect. Without it, find_program saves the paths in
  # CMake's cache and keeps using the first LLVM it found. The unset()s drop
  # any variables of the same name from the caller, which would also stop the
  # search.
  unset(found_llvm_config)
  unset(found_llvm_ar)
  unset(found_llvm_clang)
  unset(found_llvm_lld)
  find_program(found_llvm_config llvm-config
    PATHS "${root}/bin" NO_DEFAULT_PATH NO_CACHE)
  if(NOT found_llvm_config)
    message(FATAL_ERROR
      "The LLVM install at ${root} has no bin/llvm-config.")
  endif()
  _crubit_check_llvm_config("${found_llvm_config}")
  _crubit_llvm_config("${found_llvm_config}" version --version)

  find_program(found_llvm_ar llvm-ar PATHS "${root}/bin" NO_DEFAULT_PATH NO_CACHE)
  if(NOT found_llvm_ar)
    find_program(found_llvm_ar llvm-ar NO_CACHE)
  endif()
  if(NOT found_llvm_ar)
    message(FATAL_ERROR
      "Found no llvm-ar in ${root}/bin or on the PATH. Crubit needs the "
      "llvm-ar of LLVM ${version}.")
  endif()
  _crubit_check_llvm_ar("${found_llvm_ar}" "${version}" "${root}")

  find_program(found_llvm_clang clang PATHS "${root}/bin" NO_DEFAULT_PATH NO_CACHE)
  find_program(found_llvm_lld lld PATHS "${root}/bin" NO_DEFAULT_PATH NO_CACHE)
  _crubit_llvm_config("${found_llvm_config}" include_dir --includedir)
  _crubit_llvm_config("${found_llvm_config}" lib_dir --libdir)
  _crubit_llvm_config("${found_llvm_config}" has_rtti --has-rtti)
  _crubit_llvm_config("${found_llvm_config}" system_libs --system-libs --link-static)
  separate_arguments(system_libs NATIVE_COMMAND "${system_libs}")
  message(STATUS "Using LLVM ${version} at ${root}")
  if(NOT has_rtti STREQUAL "NO")
    message(STATUS "LLVM was built with RTTI; Crubit's C++ code is still built with -fno-rtti")
  endif()

  _crubit_llvm_dev_date(dev_date "${version}")
  file(GLOB libraries "${lib_dir}/*.a")

  set(llvm_version "${version}" PARENT_SCOPE)
  set(llvm_include_dir "${include_dir}" PARENT_SCOPE)
  set(llvm_lib_dir "${lib_dir}" PARENT_SCOPE)
  set(llvm_libraries "${libraries}" PARENT_SCOPE)
  set(llvm_system_libs "${system_libs}" PARENT_SCOPE)
  set(llvm_dev_date "${dev_date}" PARENT_SCOPE)
  set(llvm_ar "${found_llvm_ar}" PARENT_SCOPE)
  set(llvm_clang "${found_llvm_clang}" PARENT_SCOPE)
  set(llvm_lld "${found_llvm_lld}" PARENT_SCOPE)
endfunction()
