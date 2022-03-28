load("//third_party/bazel_rules/rules_rust/rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "has-dashes-in-name",
    hdrs = ["has-dashes-in-name.h"],
)

cc_library(
    name = "uses-target-with-dashes",
    srcs = ["uses-target-with-dashes.cc"],
    hdrs = ["uses-target-with-dashes.h"],
    deps = [":has-dashes-in-name"],
)

rust_test(
    name = "escaping_target_names",
    srcs = ["escaping_target_names.rs"],
    cc_deps = [
        ":has-dashes-in-name",
        ":uses-target-with-dashes",
    ],
)
