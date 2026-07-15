# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
"""
Repository rule, used by Bazel, to check the configured LLVM version is compatible with the LLVM version of the configured Rust toolchain.
"""

def _parse_version(version_str):
    """Parses a semver-like string into a list of integers.

    e.g. "22.1.8" -> [22, 1, 8]
    Handles pre-release tags like "22.1.0-rc2" by stripping the suffix.
    """
    clean_str = version_str.split("-")[0]
    parts = clean_str.split(".")
    version_ints = []

    for p in parts:
        if p.isdigit():
            version_ints.append(int(p))
        else:
            version_ints.append(0)

    # Fill in any missing versions with 0's
    missing_versions = 3 - len(version_ints)
    if missing_versions > 0:
        version_ints.append([0] * missing_versions)

    return version_ints

def _llvm_version_check_impl(repository_ctx):
    # 1. Locate the rustc binary
    rustc_path = repository_ctx.which("rustc")
    if not rustc_path:
        fail("llvm_version_check: Could not find 'rustc' on system PATH.")

    # 2. Execute rustc -vV
    res = repository_ctx.execute([rustc_path, "-vV"])
    if res.return_code != 0:
        fail("llvm_version_check: Failed to execute rustc -vV: %s" % res.stderr)

    # 3. Parse rustc LLVM version
    rust_llvm_version_str = None
    for line in res.stdout.splitlines():
        if line.startswith("LLVM version:"):
            rust_llvm_version_str = line.split(":", 1)[1].strip()
            break

    if not rust_llvm_version_str:
        fail("llvm_version_check: Could not find 'LLVM version:' in rustc -vV output:\n%s" % res.stdout)

    # 4. Compare the versions
    input_version = _parse_version(repository_ctx.attr.llvm_version)
    rust_version = _parse_version(rust_llvm_version_str)

    if input_version < rust_version:
        fail(
            """
========================================================================
ERROR: LLVM Version Mismatch! Build Failed.
------------------------------------------------------------------------
Your Rust compiler expects at least LLVM version: {rust}
The configured LLVM version for your CC toolchain is: {input}
Please specify an LLVM version at least as new as the LLVM used by your
Rust toolchain.
========================================================================
            """.format(
                rust = rust_llvm_version_str,
                input = repository_ctx.attr.llvm_version,
            ),
        )

    # If the check passes, generate placeholder build files to satisfy Bazel
    repository_ctx.file("BUILD.bazel", "# LLVM version check passed successfully.")
    repository_ctx.file("defs.bzl", "PASSED = True")

llvm_version_check = repository_rule(
    implementation = _llvm_version_check_impl,
    attrs = {
        "llvm_version": attr.string(
            mandatory = True,
            doc = "The LLVM version configured for the build.",
        ),
    },
    environ = ["PATH"],
    local = True,
)
