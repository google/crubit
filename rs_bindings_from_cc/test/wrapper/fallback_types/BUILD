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

crubit_test_cc_library(
    name = "unsupported_library",
    hdrs = ["unsupported_library.h"],
    aspect_hints = [],
)

crubit_test_cc_library(
    name = "wrapper_library",
    hdrs = ["wrapper_library.h"],
    aspect_hints = [
        "//features:wrapper",
        ":wrapper_library_extra",
    ],
    deps = [":unsupported_library"],
)

additional_rust_srcs_for_crubit_bindings(
    name = "wrapper_library_extra",
    srcs = ["wrapper_library_extra.rs"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":wrapper_library"],
    deps = [
        "@crates//:googletest",
    ],
)

golden_test(
    name = "wrapper_library_golden_test",
    basename = "wrapper_library",
    cc_library = "wrapper_library",
    golden_rs = "wrapper_library_rs_api.rs",
)
