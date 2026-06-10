load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_cc//cc:cc_test.bzl", "cc_test")
load("@rules_rust//rust:defs.bzl", "rust_library")
load("//support/extract_cpp_from_rust:extract_cpp.bzl", "extract_cpp")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "test_source",
    srcs = ["test_source.rs"],
)

extract_cpp(
    name = "test_extracted",
    srcs = ["test_source.rs"],
    out = "test_extracted.h",
)

cc_library(
    name = "test_extracted_cc",
    hdrs = [":test_extracted"],
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
