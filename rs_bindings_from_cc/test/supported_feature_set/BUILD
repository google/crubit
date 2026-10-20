load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "has_bindings",
    srcs = ["has_bindings.cc"],
    hdrs = ["has_bindings.h"],
    aspect_hints = [
        "//features:supported",
        "//features:template_instantiation",
    ],
    copts = ["-Wno-nullability-completeness"],
    deps = ["@abseil-cpp//absl/base:core_headers"],
)

rust_api_from_cpp(
    name = "has_bindings_rust",
    cpp_target = ":has_bindings",
)

crubit_test_cc_library(
    name = "no_bindings",
    hdrs = ["no_bindings.h"],
    aspect_hints = [
        "//features:supported",
        "//features:template_instantiation",
    ],
)

rust_api_from_cpp(
    name = "no_bindings_rust",
    cpp_target = ":no_bindings",
)

crubit_rust_test(
    name = "has_bindings_test",
    srcs = ["has_bindings_test.rs"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        ":has_bindings_rust",
        "//support:oops",
        "//support/ffi_11",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)

crubit_rust_test(
    name = "no_bindings_test",
    srcs = ["no_bindings_test.rs"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        ":no_bindings_rust",
        "@crate_index//:googletest",
    ],
)
