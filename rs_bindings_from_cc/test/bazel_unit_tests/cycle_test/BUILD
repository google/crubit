load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)

package(default_applicable_licenses = ["//:license"])

exports_files(["additional_cpp_srcs.cc"])

cc_library(
    name = "original_library",
    hdrs = ["library_to_bind.h"],
    aspect_hints = [
        ":additional_rust_srcs",
    ],
)

cc_library(
    name = "other_library",
    hdrs = ["external_dependency.h"],
)

additional_rust_srcs_for_crubit_bindings(
    name = "additional_rust_srcs",
    srcs = ["additional_rust_srcs.rs"],
    cc_support_deps = [":other_library"],
    cpp_srcs = ["additional_cpp_srcs.cc"],
)

# Tests that additional Rust sources and C++ sources can be passed to Crubit and reference each other.
crubit_rust_test(
    name = "cycle_test",
    srcs = ["cycle_test.rs"],
    cc_deps = [":original_library"],
    deps = [
        "@crate_index//:googletest",
    ],
)
