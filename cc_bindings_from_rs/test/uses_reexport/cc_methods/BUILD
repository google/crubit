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
    name = "cc_type",
    testonly = 1,
    hdrs = ["cc_type.h"],
    aspect_hints = [":cc_type_rust.hint"],
)

rust_api_from_cpp(
    name = "cc_type_rust",
    testonly = 1,
    cpp_target = ":cc_type",
)

rust_library(
    name = "reexport",
    testonly = 1,
    srcs = ["reexport.rs"],
    edition = "2024",
    deps = [":cc_type_rust"],
)

cpp_api_from_rust(
    name = "reexport_cc_api",
    testonly = 1,
    crate = ":reexport",
)

golden_test(
    name = "reexport_golden_test",
    basename = "reexport",
    golden_h = "reexport_cc_api.h",
    golden_rs = "reexport_cc_api_impl.rs",
    rust_library = "reexport",
    # `cc_std` (the `rs_bindings_from_cc` aspect) doesn't support Windows yet (b/561578105).
    tags = ["no_windows"],
)

crubit_cc_test(
    name = "reexport_test",
    srcs = ["reexport_test.cc"],
    # `cc_std` (the `rs_bindings_from_cc` aspect) doesn't support Windows yet (b/561578105).
    tags = ["no_windows"],
    deps = [
        ":cc_type",
        ":reexport_cc_api",
        "@googletest//:gtest_main",
    ],
)
