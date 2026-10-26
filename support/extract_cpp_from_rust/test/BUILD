load("@bazel_skylib//rules:build_test.bzl", "build_test")
load("@rules_cc//cc:cc_test.bzl", "cc_test")
load("@rules_rust//rust:defs.bzl", "rust_test")
load("//bazel:cpp_api_from_rust.bzl", "cpp_api_from_rust")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")
load("//support/extract_cpp_from_rust:rust_library_with_embedded_cpp.bzl", "rust_library_with_embedded_cpp", "rust_test_with_embedded_cpp")

package(default_applicable_licenses = ["//:license"])

rust_library_with_embedded_cpp(
    name = "library_with_embedded_cpp",
    srcs = ["library_with_embedded_cpp.rs"],
    deps_of_cc_library = [
        "@abseil-cpp//absl/strings:string_view",
    ],
    deps = [
        "//support:ctor",
    ],
)

cc_test(
    name = "verify_test",
    srcs = [
        "verify_test.cc",
    ],
    deps = [
        ":library_with_embedded_cpp_extracted_cc",
        "@googletest//:gtest_main",
    ],
)

build_test(
    name = "verify_rust_api_outputs_test",
    tags = ["manual"],
    targets = [
        ":library_with_embedded_cpp_rust_bindings",
    ],
)

rust_test(
    name = "rust_library_with_embedded_cpp_test",
    srcs = ["rust_library_with_embedded_cpp_test.rs"],
    deps = [
        ":library_with_embedded_cpp",
        "@crate_index//:googletest",
    ],
)

rust_test_with_embedded_cpp(
    name = "rust_test_with_embedded_cpp_test",
    srcs = ["rust_test_with_embedded_cpp_test.rs"],
    deps_of_cc_library = ["@abseil-cpp//absl/strings:string_view"],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)

cpp_api_from_rust(
    name = "library_with_embedded_cpp_cc_api",
    crate = ":library_with_embedded_cpp",
    tags = ["manual"],
)

crubit_cc_test(
    name = "verify_iwyu_export_test",
    srcs = [
        "verify_iwyu_export_test.cc",
    ],
    # `cc_std` (the `rs_bindings_from_cc` aspect) doesn't support Windows yet (b/561578105).
    tags = [
        "no_test_android_x86",
        "no_windows",
    ],
    deps = [
        ":library_with_embedded_cpp_cc_api",
        "@googletest//:gtest_main",
    ],
)
