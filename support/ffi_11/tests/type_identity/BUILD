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

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "rust_api",
    testonly = 1,
    srcs = ["rust_api.rs"],
    edition = "2024",
    features = ["crubit"],
    deps = [
        "//support/ffi_11",
    ],
)

cpp_api_from_rust(
    name = "rust_api_cc_api",
    testonly = 1,
    crate = ":rust_api",
)

golden_test(
    name = "rust_api_golden_test",
    basename = "rust_api",
    golden_h = "rust_api_cc_api.h",
    rust_library = "rust_api",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

crubit_cc_test(
    name = "type_identity_test",
    srcs = ["type_identity_test.cc"],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":rust_api_cc_api",
    ],
)
