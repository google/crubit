# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# The SHA-256 hashes of the files this project downloads, and a function to
# look them up. Pinning the hashes means a download can't silently change: if
# a file doesn't match, the build stops.

# SHA-256 hashes of the archives this project downloads by default. LLVM's are
# the digests GitHub publishes for the release assets.
set(_crubit_sha256_LLVM-23.1.3-Linux-ARM64.tar.xz
  56d005e339c979a696ed6c244e6472e16f3b1371e48987a07e6145bda7ee0d03)
set(_crubit_sha256_LLVM-23.1.3-Linux-X64.tar.xz
  ca9fe385a5d9e2d88132dc0e23a00e9f87f38fa75f67207b5e457d208e71d013)
set(_crubit_sha256_LLVM-23.1.3-macOS-ARM64.tar.xz
  3dcfd88aec10be204a24037bcb0dc0ad777882e9dbe8ac60e2639060d8a794b1)
set(_crubit_sha256_abseil-cpp-20260526.0.tar.gz
  6e1aee535473414164bf83e4ebc40240dec71a4701f8a642d906e95bea1aea0c)
set(_crubit_sha256_protobuf-36.1.tar.gz
  7a7baff8207a57f3fe5c233d432a09eba67bdfb55a3145839d0817cae8e807a4)

# _crubit_url_hash(<out_var> <file_name>)
#
# Sets <out_var> to the `URL_HASH SHA256=...` arguments that make
# FetchContent_Declare or ExternalProject_Add check the download <file_name>.
# If its hash isn't known, it warns and sets <out_var> to nothing, so the
# download isn't checked.
function(_crubit_url_hash out_var file_name)
  if(DEFINED _crubit_sha256_${file_name})
    set(${out_var} URL_HASH "SHA256=${_crubit_sha256_${file_name}}" PARENT_SCOPE)
  else()
    message(WARNING "No known hash for ${file_name}; its download is not verified.")
    set(${out_var} "" PARENT_SCOPE)
  endif()
endfunction()
