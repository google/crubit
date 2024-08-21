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
        "//support/internal:bindings_support",
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
