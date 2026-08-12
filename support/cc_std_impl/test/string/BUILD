load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load("//rs_bindings_from_cc/test/golden:golden_test.bzl", "golden_test")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "test_helpers",
    testonly = 1,
    hdrs = ["test_helpers.h"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":test_helpers",
        "//support/public:cc_std",
    ],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:rstest",  # v0_16
        "@crate_index//:static_assertions",  # v1
    ],
)

crubit_test_cc_library(
    name = "test_helpers_layout_compatible",
    testonly = 1,
    hdrs = ["test_helpers.h"],
    aspect_hints = [
        "//features:layout_compatible_string",
    ],
)

crubit_rust_test(
    name = "test_layout_compatible",
    srcs = ["test_layout_compatible.rs"],
    cc_deps = [
        ":test_helpers_layout_compatible",
        "//support/public:cc_std",
    ],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
        "@crate_index//:rstest",  # v0_16
    ],
)

cc_library(
    name = "layout_compatible_string_mock_cc",
    hdrs = ["layout_compatible_string_mock.h"],
    aspect_hints = [
        "//features:layout_compatible_string",
    ],
)

golden_test(
    name = "layout_compatible_string_mock_golden_test",
    cc_library = ":layout_compatible_string_mock_cc",
    golden_cc = "layout_compatible_string_mock_rs_api_impl.cc",
    golden_namespaces = "layout_compatible_string_mock_namespaces.json",
    golden_rs = "layout_compatible_string_mock_rs_api.rs",
)
