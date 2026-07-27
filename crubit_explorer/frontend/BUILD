# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@npm//:defs.bzl", "npm_link_all_packages")
load("@npm//frontend:@angular/cli/package_json.bzl", "bin")

package(
    default_applicable_licenses = ["//:license"],
)

npm_link_all_packages(name = "node_modules")

bin.ng(
    name = "build",
    srcs = [
        "angular.json",
        "package.json",
        "tsconfig.app.json",
        "tsconfig.json",
        ":node_modules",
    ] + glob([
        "public/**/*",
        "src/**/*",
    ]),
    args = [
        "build",
        "--configuration",
        "production",
    ],
    chdir = "frontend",
    # Angular build with modern application builder outputs to dist/frontend.
    out_dirs = [
        "dist/frontend",
    ],
    visibility = ["//visibility:public"],
)
