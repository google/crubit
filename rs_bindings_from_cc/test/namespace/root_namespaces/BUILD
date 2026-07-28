load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "root_namespaces_cc",
    hdrs = ["root_namespaces.h"],
    aspect_hints = [
        ":root_namespaces_rust.hint",
    ],
)

rust_api_from_cpp(
    name = "root_namespaces_rust",
    cpp_target = ":root_namespaces_cc",
    root_namespaces = ["test_namespace"],
)

golden_test(
    name = "root_namespaces_golden_test",
    basename = "root_namespaces",
    cc_library = ":root_namespaces_cc",
    golden_cc = "root_namespaces_api_impl.cc",
    golden_rs = "root_namespaces_rs_api.rs",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":root_namespaces_rust",
        "@crate_index//:googletest",
    ],
)
