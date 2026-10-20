# End-to-end test of !Unpin classes.

load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "nonunpin",
    hdrs = ["nonunpin.h"],
    deps = ["@abseil-cpp//absl/log:check"],
)

rust_api_from_cpp(
    name = "nonunpin_rust",
    cpp_target = ":nonunpin",
)

crubit_rust_test(
    name = "nonunpin_test",
    srcs = ["nonunpin_test.rs"],
    deps = [
        ":nonunpin_rust",
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "nonunpin_experimental",
    hdrs = ["nonunpin.h"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = ["@abseil-cpp//absl/log:check"],
)

rust_api_from_cpp(
    name = "nonunpin_experimental_rust",
    cpp_target = ":nonunpin_experimental",
)

crubit_rust_test(
    name = "nonunpin_experimental_test",
    srcs = ["nonunpin_experimental_test.rs"],
    deps = [
        ":nonunpin_experimental_rust",
        "//support:cref",
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)
