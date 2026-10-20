load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "types_only",
    hdrs = ["types_only.h"],
    aspect_hints = ["//features:types"],
    deps = ["//support:annotations"],
)

rust_api_from_cpp(
    name = "types_only_rust",
    cpp_target = ":types_only",
)

crubit_test_cc_library(
    name = "functions_using_types",
    hdrs = ["functions_using_types.h"],
    aspect_hints = ["//features:supported"],
    deps = [
        ":types_only",
    ],
)

rust_api_from_cpp(
    name = "functions_using_types_rust",
    cpp_target = ":functions_using_types",
)

golden_test(
    name = "types_only_golden_test",
    basename = "types_only",
    cc_library = "types_only",
    golden_cc = "types_only_api_impl.cc",
    golden_rs = "types_only_rs_api.rs",
)

crubit_rust_test(
    name = "types_only_test",
    srcs = ["types_only_test.rs"],
    deps = [
        ":functions_using_types_rust",
        ":types_only_rust",
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)
