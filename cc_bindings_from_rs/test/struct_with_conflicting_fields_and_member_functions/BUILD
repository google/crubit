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
    name = "struct_with_conflicting_fields_and_member_functions",
    testonly = 1,
    srcs = ["struct_with_conflicting_fields_and_member_functions.rs"],
    aspect_hints = [
        "//features:supported",
    ],
    edition = "2024",
    deps = [
        "//support:bridge_rust",
    ],
)

cpp_api_from_rust(
    name = "struct_with_conflicting_fields_and_member_functions_cc_api",
    testonly = 1,
    crate = ":struct_with_conflicting_fields_and_member_functions",
)

golden_test(
    name = "struct_with_conflicting_fields_and_member_functions_golden_test",
    basename = "struct_with_conflicting_fields_and_member_functions",
    golden_h = "struct_with_conflicting_fields_and_member_functions_cc_api.h",
    golden_rs = "struct_with_conflicting_fields_and_member_functions_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "struct_with_conflicting_fields_and_member_functions",
)

crubit_cc_test(
    name = "struct_with_conflicting_fields_and_member_functions_test",
    srcs = ["struct_with_conflicting_fields_and_member_functions_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":struct_with_conflicting_fields_and_member_functions_cc_api",
        "@googletest//:gtest_main",
    ],
)
