load(
    "@rules_cc//cc:cc_library.bzl",
    "cc_library",
)
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
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_cc_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "cc_struct",
    hdrs = ["cc_struct.h"],
    aspect_hints = ["//features:supported"],
)

rust_library(
    name = "const_field",
    testonly = 1,
    srcs = ["lib.rs"],
    cc_deps = [":cc_struct"],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

cc_bindings_from_rust(
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
)

crubit_cc_test(
    name = "test",
    srcs = ["test.cc"],
    deps = [
        ":cc_struct",
        ":lib_cc_api",
        "//testing/base/public:gunit_main",
    ],
)
