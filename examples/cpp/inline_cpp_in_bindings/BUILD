# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

licenses(["notice"])

cc_library(
    name = "widget",
    hdrs = ["widget.h"],
    aspect_hints = [":widget_rust.hint"],
)

rust_api_from_cpp(
    name = "widget_rust",
    srcs = ["widget_custom.rs"],
    cpp_target = ":widget",
    embedded_cpp = True,
    root_namespaces = ["widget"],
)

crubit_rust_test(
    name = "widget_test",
    srcs = ["widget_test.rs"],
    deps = [
        ":widget_rust",
        "@crate_index//:googletest",
    ],
)
