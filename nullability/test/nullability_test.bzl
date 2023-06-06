# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""
A nullability_test target analyzes a small C++ source file and validates
contained assertions about the nullability of expressions.
See nullability_test.h for details.
"""

def nullability_test(name, srcs):
    native.sh_test(
        name = name,
        data = srcs + [
            "//nullability/test:nullability_test",
            "//nullability/test:nullability_test.h",
        ],
        srcs = ["nullability_test.sh"],
        args = [
            "$(location //nullability/test:nullability_test)",
            "nullability/test",
        ] + ["$(location " + src + ")" for src in srcs],
    )
