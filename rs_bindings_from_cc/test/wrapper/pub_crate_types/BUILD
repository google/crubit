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
    name = "other_pub_crate_types",
    hdrs = ["other_pub_crate_types.h"],
    aspect_hints = [
        "//features:wrapper",
    ],
)

crubit_test_cc_library(
    name = "pub_crate_types",
    srcs = ["pub_crate_types.cc"],
    hdrs = ["pub_crate_types.h"],
    aspect_hints = [
        "//features:wrapper",
        ":pub_crate_types_extra",
    ],
    deps = [":other_pub_crate_types"],
)

additional_rust_srcs_for_crubit_bindings(
    name = "pub_crate_types_extra",
    srcs = ["pub_crate_types_extra.rs"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":pub_crate_types"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

golden_test(
    name = "pub_crate_types_golden_test",
    basename = "pub_crate_types",
    cc_library = "pub_crate_types",
    golden_rs = "pub_crate_types_rs_api.rs",
)
