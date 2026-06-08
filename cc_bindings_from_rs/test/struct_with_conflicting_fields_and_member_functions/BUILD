load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
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
    deps = [
        "//support:bridge_rust",
    ],
)

cc_bindings_from_rust(
    name = "struct_with_conflicting_fields_and_member_functions_cc_api",
    testonly = 1,
    crate = ":struct_with_conflicting_fields_and_member_functions",
)

golden_test(
    name = "struct_with_conflicting_fields_and_member_functions_golden_test",
    basename = "struct_with_conflicting_fields_and_member_functions",
    golden_h = "struct_with_conflicting_fields_and_member_functions_cc_api.h",
    golden_rs = "struct_with_conflicting_fields_and_member_functions_cc_api_impl.rs",
    rust_library = "struct_with_conflicting_fields_and_member_functions",
)

crubit_cc_test(
    name = "struct_with_conflicting_fields_and_member_functions_test",
    srcs = ["struct_with_conflicting_fields_and_member_functions_test.cc"],
    deps = [
        ":struct_with_conflicting_fields_and_member_functions_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
