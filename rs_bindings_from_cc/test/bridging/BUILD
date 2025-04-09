load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)

cc_library(
    name = "bridging_lib",
    hdrs = ["bridging_lib.h"],
    aspect_hints = [
        "//features:experimental",
        ":converter",
    ],
    deps = [
        "//support/internal:bindings_support",  # build_cleaner: keep
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
        "//support/internal:bindings_support",  # build_cleaner: keep
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
    deps = ["//support:bridge_cpp"],
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
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
