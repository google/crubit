# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

# Common setup for fixture projects: makes Crubit (from CRUBIT_SOURCE_DIR) and
# Corrosion available. Include this after `project()`.

include(FetchContent)
FetchContent_Declare(crubit SOURCE_DIR "${CRUBIT_SOURCE_DIR}")
FetchContent_Declare(
  Corrosion
  GIT_REPOSITORY https://github.com/corrosion-rs/corrosion
  GIT_TAG v0.6.1
  GIT_SHALLOW TRUE
)
FetchContent_MakeAvailable(crubit Corrosion)
enable_testing()
