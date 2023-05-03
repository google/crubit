load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        "//support/cc_std",
    ],
    deps = ["//support:ctor"],
)

rust_test(
    name = "check_cc_std_in_deps",
    srcs = ["test.rs"],
    cc_deps = [
        "//support/cc_std",
    ],
    deps = ["//support:ctor"],
)

crubit_test_cc_library(
    name = "stl_in_deps",
    hdrs = ["empty.h"],
    deps = ["//third_party/stl"],
)

rust_test(
    name = "check_stl_in_deps",
    srcs = ["empty.rs"],
    cc_deps = [
        ":stl_in_deps",
        "//third_party/stl",
    ],
)
