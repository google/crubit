load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

# The real `absl::flat_hash_map` target would provide this dep through an aspect hint, but we need
# to do it ourselves here for the tests that avoid taking a direct dependency.
additional_rust_srcs_for_crubit_bindings(
    name = "absl_container_support",
    srcs = [],
    deps = [
        "//support/absl_container",
    ],
)

crubit_test_cc_library(
    name = "isolated",
    hdrs = ["isolated.h"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        "//support:annotations",
    ],
)

golden_test(
    name = "isolated_golden_test",
    basename = "isolated",
    cc_library = "isolated",
    golden_cc = "isolated_api_impl.cc",
    golden_rs = "isolated_rs_api.rs",
)

crubit_test_cc_library(
    name = "stubbed",
    testonly = 1,
    hdrs = ["stubbed.h"],
    aspect_hints = [
        ":absl_container_support",
        "//features:experimental",
    ],
)

crubit_rust_test(
    name = "stubbed_test",
    srcs = ["stubbed_test.rs"],
    cc_deps = [
        ":stubbed",
    ],
    deps = [
        "//support:ctor",
        "//support/absl_container",
        "@crate_index//:googletest",
    ],
)
