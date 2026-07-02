load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "isolated",
    hdrs = ["isolated.h"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        "@abseil-cpp//absl/container:flat_hash_map",
    ],
)

golden_test(
    name = "isolated_golden_test",
    basename = "isolated",
    cc_library = "isolated",
    golden_cc = "isolated_api_impl.cc",
    golden_rs = "isolated_rs_api.rs",
)
