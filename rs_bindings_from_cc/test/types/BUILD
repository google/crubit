load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

cc_library(
    name = "types_nonptr",
    hdrs = ["types_nonptr.h"],
)

cc_library(
    name = "types_lifetimes",
    hdrs = ["types_lifetimes.h"],
)

cc_library(
    name = "types_nolifetimes",
    hdrs = ["types_nolifetimes.h"],
)

cc_library(
    name = "types_inferred_lifetimes",
    hdrs = ["types_inferred_lifetimes.h"],
)

rust_test(
    name = "types",
    srcs = ["types_test.rs"],
    cc_deps = [
        ":types_inferred_lifetimes",
        ":types_lifetimes",
        ":types_nolifetimes",
        ":types_nonptr",
    ],
)
