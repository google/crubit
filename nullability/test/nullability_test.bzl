# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""
A nullability_test target analyzes a small C++ source file and validates
contained assertions about the nullability of expressions.
See nullability_test.h for details.
"""

load("//third_party/bazel_rules/rules_cc/cc:cc_library.bzl", "cc_library")

def nullability_test(name, srcs):
    native.sh_test(
        name = name,
        data = srcs + [
            "//nullability/test:nullability_test",
        ],
        srcs = ["nullability_test.sh"],
        args = [
            "$(location //nullability/test:nullability_test)",
        ] + ["$(location " + src + ")" for src in srcs if not src.endswith(".h")] + ["--"],
    )

    # Additional target to verify that the source file builds with non-mock headers.
    # TODO Add support for nested directories, like `absl`, so we can verify against real headers in
    # the production repo.
    cc_library(
        name = name + "_compile_test",
        srcs = srcs + ["nullability_test.h", "nullability_annotations.h", "check.h"],
    )
