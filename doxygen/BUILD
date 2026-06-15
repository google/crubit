# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load(":doxygen.bzl", "crubit_doxygen")

package(default_applicable_licenses = ["//:license"])

# TODO: Add more Crate deps here b/523385040
crubit_doxygen(
    name = "crubit_stdlib_doxygen",
    doxyfile = "Doxyfile",
    deps = [
        "//support/rs_std",
        "//support/rs_std:char",
        "//support/rs_std:rs_alloc",
        "//support/rs_std:rs_core",
        "//support/rs_std:slice_ref",
        "//support/rs_std:str_ref",
    ],
)
