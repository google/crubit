load(
    "@rules_cc//cc:cc_library.bzl",
    "cc_library",
)
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
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_cc_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "cc_struct",
    hdrs = ["cc_struct.h"],
    aspect_hints = [":cc_struct_rust.hint"],
)

rust_api_from_cpp(
    name = "cc_struct_rust",
    testonly = 1,
    cpp_target = ":cc_struct",
)

rust_library(
    name = "const_field",
    testonly = 1,
    srcs = ["lib.rs"],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
    deps = [":cc_struct_rust"],
)

cpp_api_from_rust(
    name = "lib_cc_api",
    testonly = 1,
    crate = ":const_field",
)

golden_test(
    name = "golden_test",
    basename = "const_field",
    golden_h = "const_field_cc_api.h",
    golden_rs = "const_field_cc_api_impl.rs",
    rust_library = "const_field",
    # TODO(b/580795136): header analysis of `cc_std` fails for windows_arm64.
    # TODO(b/483962985): Add 32-bit Android goldens and enable these golden tests.
    tags = [
        "no_test_android_arm",
        "no_test_android_x86",
        "no_test_windows_arm64",
    ],
)

crubit_cc_test(
    name = "test",
    srcs = ["test.cc"],
    tags = ["no_test_android_x86"],
    deps = [
        ":cc_struct",
        ":lib_cc_api",
        "@googletest//:gtest_main",
    ],
)
