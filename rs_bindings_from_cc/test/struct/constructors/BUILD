"""End-to-end test for constructors."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "constructors",
    srcs = ["constructors.cc"],
    hdrs = ["constructors.h"],
    # operator lifetime inference and ctor
    aspect_hints = ["//features:experimental"],
)

golden_test(
    name = "constructors_golden_test",
    basename = "constructors",
    cc_library = ":constructors",
    golden_cc = "constructors_api_impl.cc",
    golden_rs = "constructors_rs_api.rs",
)

crubit_rust_test(
    name = "constructors_test",
    srcs = ["test.rs"],
    cc_deps = [
        ":constructors",
    ],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
