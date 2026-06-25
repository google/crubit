load("@bazel_tools//tools/build_defs/build_test:build_test.bzl", "build_test")
load("@rules_cc//cc:cc_test.bzl", "cc_test")
load("@rules_rust//rust:defs.bzl", "rust_test")
load("//support/extract_cpp_from_rust:rust_library_with_embedded_cpp.bzl", "rust_library_with_embedded_cpp")

package(default_applicable_licenses = ["//:license"])

rust_library_with_embedded_cpp(
    name = "library_with_embedded_cpp",
    srcs = ["library_with_embedded_cpp.rs"],
    deps_of_cc_library = [
        "@abseil-cpp//absl/strings:string_view",
    ],
)

cc_test(
    name = "verify_test",
    srcs = [
        "verify_test.cc",
    ],
    deps = [
        ":library_with_embedded_cpp_extracted_cc",
        "//testing/base/public:gunit_main",
    ],
)

build_test(
    name = "verify_rust_api_outputs_test",
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
