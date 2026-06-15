load("@bazel_tools//tools/build_defs/build_test:build_test.bzl", "build_test")
load("@rules_cc//cc:cc_test.bzl", "cc_test")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")
load("//support/extract_cpp_from_rust:cc_library_extracted_from_rust.bzl", "cc_library_extracted_from_rust")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "test_source",
    srcs = ["test_source.rs"],
)

cc_library_extracted_from_rust(
    name = "test_extracted_cc",
    srcs = ["test_source.rs"],
    deps = [
        "@abseil-cpp//absl/strings:string_view",
    ],
)

cc_test(
    name = "verify_test",
    srcs = [
        "verify_test.cc",
    ],
    deps = [
        ":test_extracted_cc",
        "//testing/base/public:gunit_main",
    ],
)

build_test(
    name = "verify_rust_api_outputs_test",
    targets = [
        ":test_extracted_cc_rust",
    ],
)

rust_test(
    name = "bindings_test",
    srcs = ["bindings_test.rs"],
    deps = [
        ":test_extracted_cc_rust",
        "@crate_index//:googletest",
    ],
)
