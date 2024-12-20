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

cc_library(
    name = "string_test_lib",
    hdrs = ["string_test_lib.h"],
    aspect_hints = [
        "//features:experimental",
    ],
    deps = [
        "//support/internal:bindings_support",
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
