load(
    "@rules_cc//cc:cc_library.bzl",
    "cc_library",
)
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "v1_test",
    testonly = 1,
    srcs = ["v1.rs"],
    crate_name = "my_crate",
)

rust_library(
    name = "v2_test",
    testonly = 1,
    srcs = ["v2.rs"],
    crate_name = "my_crate",
)

cc_bindings_from_rust(
    name = "v1_cc_api",
    testonly = 1,
    crate = ":v1_test",
)

cc_bindings_from_rust(
    name = "v2_cc_api",
    testonly = 1,
    crate = ":v2_test",
)

cc_library(
    name = "consumer_v1",
    testonly = 1,
    srcs = ["consumer_v1.cc"],
    hdrs = ["consumer_v1.h"],
    deps = [
        ":v1_cc_api",
        "@abseil-cpp//absl/strings",
    ],
)

cc_library(
    name = "consumer_v2",
    testonly = 1,
    srcs = ["consumer_v2.cc"],
    hdrs = ["consumer_v2.h"],
    deps = [
        ":v2_cc_api",
        "@abseil-cpp//absl/strings",
    ],
)

crubit_cc_test(
    name = "multiple_crate_versions_test",
    srcs = ["multiple_crate_versions_test.cc"],
    deps = [
        ":consumer_v1",
        ":consumer_v2",
        "//testing/base/public:gunit_main",
    ],
)
