# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

package(default_applicable_licenses = ["//:license"])

exports_files(["get_llvm_commit_date.py"])

# Exempt scripts used only in GitHub Actions / Cargo builds from build_cleaner checks.
filegroup(
    name = "cargo_build_scripts",
    srcs = [
        "generate_proto_headers.py",
        "setup_bazel_env.py",
    ],
    tags = ["ignore_srcs"],
    visibility = ["//visibility:private"],
)
