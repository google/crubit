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
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)

rust_library(
    name = "rust_library",
    srcs = ["rust_library.rs"],
)

cc_bindings_from_rust(
    name = "rust_library_cc",
    crate = ":rust_library",
)

cc_library(
    name = "bridging_lib",
    hdrs = ["bridging_lib.h"],
    aspect_hints = [
        "//features:experimental",
        ":converter",
    ],
    deps = [
        "//support/public:bindings_support",  # build_cleaner: keep
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "converter",
    srcs = ["converter.rs"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":bridging_lib",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "string_test_lib",
    hdrs = ["string_test_lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        "//support/public:bindings_support",  # build_cleaner: keep
    ],
)

crubit_rust_test(
    name = "string_test",
    srcs = ["string_test.rs"],
    cc_deps = [
        ":string_test_lib",
        "//support/cc_std",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "composable_bridging_lib",
    srcs = ["composable_bridging_lib.cc"],
    hdrs = ["composable_bridging_lib.h"],
    aspect_hints = [
        "//features:experimental",
        ":additional_composable_bridging_lib_src",
    ],
    deps = [
        ":rust_library_cc",
        "//support:annotations",
        "//support:bridge_cpp",
        "//support/rs_std:slice_ref",
        "@abseil-cpp//absl/status",
        "@abseil-cpp//absl/status:statusor",
        "@abseil-cpp//absl/types:span",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "additional_composable_bridging_lib_src",
    srcs = ["additional_composable_bridging_lib_src.rs"],
)

crubit_rust_test(
    name = "composable_bridging_test",
    srcs = ["composable_bridging_test.rs"],
    cc_deps = [
        ":composable_bridging_lib",
        "//support/cc_std",
        "@abseil-cpp//absl/status",
    ],
    deps = [
        "@abseil-cpp//absl/status:status_rs_matchers",
        "@crate_index//:googletest",
    ],
)
