load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    deps = [
        "//support:ctor",
        "//support/cc_std",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_rust_test(
    name = "check_cc_std_in_deps",
    srcs = ["test.rs"],
    deps = [
        "//support:ctor",
        "//support/cc_std",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "stl_in_deps",
    hdrs = ["empty.h"],
    deps = ["//third_party/stl"],
)

rust_api_from_cpp(
    name = "stl_in_deps_rust",
    cpp_target = ":stl_in_deps",
)

crubit_rust_test(
    name = "check_stl_in_deps",
    srcs = ["empty.rs"],
    deps = [
        ":stl_in_deps_rust",
        "//third_party/stl",
    ],
)
