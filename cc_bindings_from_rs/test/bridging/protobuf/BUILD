"""End-to-end tests of `cc_bindings_from_rs`, focusing on bridging Protobuf types."""

load("@protobuf//bazel:cc_proto_library.bzl", "cc_proto_library")
load("@protobuf//bazel:proto_library.bzl", "proto_library")
load("@protobuf//rust:defs.bzl", "rust_proto_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")
load(
    "//google_internal/build_flavors:crubit_build_flavors_ios_device.bzl",
    "crubit_ios_device_test",
)

package(default_applicable_licenses = ["//:license"])

proto_library(
    name = "foo_proto",
    testonly = True,
    srcs = ["foo.proto"],
)

rust_proto_library(
    name = "foo_rust_proto",
    testonly = True,
    deps = [":foo_proto"],
)

cc_proto_library(
    name = "foo_cc_proto",
    testonly = True,
    deps = [":foo_proto"],
)

rust_library(
    name = "rust_lib",
    testonly = True,
    srcs = ["rust_lib.rs"],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",  # build_cleaner: keep
    ],
    deps = [":foo_rust_proto"],
)

cpp_api_from_rust(
    name = "rust_lib_cc_api",
    testonly = True,
    crate = ":rust_lib",
)

golden_test(
    name = "rust_lib_golden_test",
    basename = "rust_lib",
    golden_h = "rust_lib_cc_api.h",
    golden_rs = "rust_lib_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "rust_lib",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

crubit_cc_test(
    name = "user_of_rust_lib",
    srcs = ["user_of_rust_lib.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): `absl::StatusOr` has a different layout under the MSVC ABI
    # than its Rust mirror expects.
    tags = ["no_windows"],
    deps = [
        ":foo_cc_proto",
        ":rust_lib_cc_api",
        "@googletest//:gtest_main",
    ],
)

crubit_ios_device_test(
    name = "user_of_rust_lib_on_ios_device",
    srcs = ["user_of_rust_lib.cc"],
    deps = [
        ":foo_cc_proto",
        ":rust_lib_cc_api",
        "@googletest//:gtest_main",
    ],
)
