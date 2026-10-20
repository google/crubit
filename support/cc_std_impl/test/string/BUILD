load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "test_helpers",
    testonly = 1,
    hdrs = ["test_helpers.h"],
    deps = ["//support:annotations"],
)

rust_api_from_cpp(
    name = "test_helpers_rust",
    testonly = 1,
    cpp_target = ":test_helpers",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":test_helpers_rust",
        "//support:ctor",
        "//support/cc_std",
        "@crate_index//:googletest",
        "@crate_index//:rstest",  # v0_16
        "@crate_index//:static_assertions",  # v1
    ],
)

crubit_rust_test(
    name = "test_layout_compatible",
    srcs = ["test_layout_compatible.rs"],
    deps = [
        ":test_helpers_rust",
        "//support:cref",
        "//support:ctor",
        "//support/cc_std",
        "@crate_index//:googletest",
        "@crate_index//:rstest",  # v0_16
    ],
)

crubit_test_cc_library(
    name = "test_helpers_with_layout_compat_string",
    testonly = 1,
    hdrs = ["test_helpers.h"],
    aspect_hints = ["//features:layout_compat_string"],
    deps = ["//support:annotations"],
)

rust_api_from_cpp(
    name = "test_helpers_with_layout_compat_string_rust",
    testonly = 1,
    cpp_target = ":test_helpers_with_layout_compat_string",
)

crubit_rust_test(
    name = "test_layout_compat_feature",
    srcs = ["test_layout_compat_feature.rs"],
    deps = [
        ":test_helpers_with_layout_compat_string_rust",
        "//support:cpp_std",
        "//support:cref",
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:rstest",  # v0_16
    ],
)
