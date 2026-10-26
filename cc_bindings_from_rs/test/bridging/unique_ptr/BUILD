load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load(
    "//bazel:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "test_helpers",
    testonly = 1,
    srcs = ["test_helpers.cc"],
    hdrs = ["test_helpers.h"],
    aspect_hints = [
        ":test_helpers_rust.hint",
    ],
    deps = [
        "//support:annotations",
        "@abseil-cpp//absl/base:nullability",
    ],
)

rust_api_from_cpp(
    name = "test_helpers_rust",
    testonly = 1,
    cpp_target = ":test_helpers",
)

rust_library(
    name = "unique_ptr",
    testonly = 1,
    srcs = [
        "unique_ptr.rs",
    ],
    aspect_hints = ["//features:nonnull_smart_pointers"],
    edition = "2024",
    proc_macro_deps = ["//support:crubit_annotate"],
    deps = [
        ":test_helpers_rust",
        "//support/cc_std",
    ],
)

cpp_api_from_rust(
    name = "unique_ptr_cc_api",
    testonly = 1,
    crate = ":unique_ptr",
)

crubit_cc_test(
    name = "unique_ptr_test",
    srcs = ["unique_ptr_test.cc"],
    # `cc_std` (the `rs_bindings_from_cc` aspect) doesn't support Windows yet (b/561578105).
    tags = ["no_windows"],
    deps = [
        ":unique_ptr_cc_api",
        "@googletest//:gtest_main",
    ],
)

# Deliberately does not enable `nonnull_smart_pointers`, to test that the `Nullable` bridge is not
# gated on it.
rust_library(
    name = "nullable",
    testonly = 1,
    srcs = [
        "nullable.rs",
    ],
    edition = "2024",
    proc_macro_deps = ["//support:crubit_annotate"],
    deps = [
        ":test_helpers_rust",
        "//support/cc_std",
    ],
)

cpp_api_from_rust(
    name = "nullable_cc_api",
    testonly = 1,
    crate = ":nullable",
)

crubit_cc_test(
    name = "nullable_test",
    srcs = ["nullable_test.cc"],
    # `cc_std` (the `rs_bindings_from_cc` aspect) doesn't support Windows yet (b/561578105).
    tags = ["no_windows"],
    deps = [
        ":nullable_cc_api",
        ":test_helpers",
        "//support:annotations",
        "@googletest//:gtest_main",
    ],
)
