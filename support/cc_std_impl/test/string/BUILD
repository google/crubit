load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "test_helpers",
    testonly = 1,
    hdrs = ["test_helpers.h"],
    deps = ["//support:annotations"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":test_helpers",
        "//support/cc_std",
    ],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:rstest",  # v0_16
        "@crate_index//:static_assertions",  # v1
    ],
)

crubit_rust_test(
    name = "test_layout_compatible",
    srcs = ["test_layout_compatible.rs"],
    cc_deps = [
        ":test_helpers",
        "//support/cc_std",
    ],
    deps = [
        "//support:cref",
        "//support:ctor",
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

crubit_rust_test(
    name = "test_layout_compat_feature",
    srcs = ["test_layout_compat_feature.rs"],
    cc_deps = [
        ":test_helpers_with_layout_compat_string",
    ],
    deps = [
        "//support:cpp_std",
        "//support:cref",
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:rstest",  # v0_16
    ],
)
