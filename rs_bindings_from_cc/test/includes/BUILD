"""End-to-end example of using `includes` attribute in `cc_library`."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "using_includes",
    # srcs = ["includes_dir/myheader.h"],
    hdrs = [
        "includes_dir/myheader.h",
        "using_includes.h",
    ],
    includes = ["includes_dir/"],
)

rust_test(
    name = "includes_test",
    srcs = ["includes_test.rs"],
    cc_deps = [":using_includes"],
)
