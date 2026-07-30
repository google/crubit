load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(
    default_applicable_licenses = ["//:license"],
)

cc_library(
    name = "base",
    hdrs = ["base.h"],
    aspect_hints = [":base_rust.hint"],
)

cc_library(
    name = "example_lib",
    hdrs = ["example.h"],
    aspect_hints = [":example_lib_rust.hint"],
    deps = [
        ":base",
        ":definition_cc",
    ],
)

rust_api_from_cpp(
    name = "base_rust",
    cpp_target = ":base",
)

rust_api_from_cpp(
    name = "example_lib_rust",
    cpp_target = ":example_lib",
)

rust_binary(
    name = "main",
    srcs = ["main.rs"],
    deps = [
        ":base_rust",
        ":definition",
        ":example_lib_rust",
        "//support:ctor",
    ],
)

# This test rule is just to make sure we don't forget to keep the `example_generated.rs` updated!
golden_test(
    name = "example_golden_test",
    cc_library = "example_lib",
    golden_rs = "example_generated.rs",
)

rust_library(
    name = "definition",
    srcs = ["definition.rs"],
    deps = [
        ":base_rust",
        "//support/ffi_11",
    ],
)

cc_bindings_from_rust(
    name = "definition_cc",
    crate = ":definition",
)
