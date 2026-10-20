load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
)
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
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
    edition = "2024",
    proc_macro_deps = ["//support:crubit_annotate"],
)

cpp_api_from_rust(
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

rust_api_from_cpp(
    name = "debuggables_rust",
    cpp_target = ":debuggables",
)

crubit_rust_test(
    name = "debuggables_test",
    srcs = ["debuggables_test.rs"],
    deps = [
        ":debuggables_rust",
        ":existing_rust_debuggable",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)

cc_library(
    name = "derived_debuggables",
    hdrs = ["derived_debuggables.h"],
    aspect_hints = [
        "//features:experimental",
        "//features:oo_casting",
    ],
    deps = [
        "//support:annotations",
    ],
)

rust_api_from_cpp(
    name = "derived_debuggables_rust",
    cpp_target = ":derived_debuggables",
)

crubit_rust_test(
    name = "derived_debuggables_test",
    srcs = ["derived_debuggables_test.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        ":derived_debuggables_rust",
        "@crate_index//:googletest",
    ],
)
