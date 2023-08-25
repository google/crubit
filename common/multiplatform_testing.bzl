# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""Supporting macro for multiplatform code."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

_PLATFORMS = [
    "x86_linux",
    "arm_linux",
]

def multiplatform_rust_test(name, **kwargs):
    """Macro to parameterize a test target by target platform."""

    # TODO(jeanpierreda): Ideally we'd use `.`, not `-`, but this breaks for non-crate= rust_test targets
    # because they create a crate with `.` in the name. That's illegal.
    native.test_suite(
        name = name,
        tests = [name + "-" + platform for platform in _PLATFORMS],
    )
    rustc_env = kwargs.setdefault("env", {})
    for platform in _PLATFORMS:
        rustc_env["CRUBIT_TEST_PLATFORM"] = platform
        test_name = name + "-" + platform
        crubit_rust_test(
            name = test_name,
            **kwargs
        )
