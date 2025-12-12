load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "has_bindings",
    srcs = ["has_bindings.cc"],
    hdrs = ["has_bindings.h"],
    aspect_hints = ["//features:supported"],
    copts = ["-Wno-nullability-completeness"],
    deps = ["@abseil-cpp//absl/base:core_headers"],
)

crubit_test_cc_library(
    name = "no_bindings",
    hdrs = ["no_bindings.h"],
    aspect_hints = ["//features:supported"],
)

crubit_rust_test(
    name = "has_bindings_test",
    srcs = ["has_bindings_test.rs"],
    cc_deps = [
        ":has_bindings",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "//support:oops",
        "@crates//:googletest",
        "@crates//:static_assertions",  # v1
    ],
)

crubit_rust_test(
    name = "no_bindings_test",
    srcs = ["no_bindings_test.rs"],
    cc_deps = [
        ":no_bindings",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crates//:googletest",
    ],
)
