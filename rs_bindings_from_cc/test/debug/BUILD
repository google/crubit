load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

rust_library(
    name = "existing_rust_debuggable",
    srcs = ["existing_rust_debuggable.rs"],
    aspect_hints = ["//features:supported"],
    proc_macro_deps = ["//support:crubit_annotate"],
)

cc_bindings_from_rust(
    name = "existing_rust_debuggable_cc",
    crate = ":existing_rust_debuggable",
)

cc_library(
    name = "debuggables",
    hdrs = ["debuggables.h"],
    aspect_hints = [
        "//features:record_impl_debug",
    ],
    deps = [
        ":existing_rust_debuggable_cc",
        "//support:annotations",
        "@abseil-cpp//absl/base:no_destructor",
        "@abseil-cpp//absl/base:nullability",
    ],
)

crubit_rust_test(
    name = "debuggables_test",
    srcs = ["debuggables_test.rs"],
    cc_deps = [
        ":debuggables",
    ],
    deps = [
        ":existing_rust_debuggable",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
