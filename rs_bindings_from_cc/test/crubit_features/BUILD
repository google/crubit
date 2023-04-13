load("@rules_rust//rust:defs.bzl", "rust_proc_macro", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

crubit_test_cc_library(
    name = "definition_disabled",
    hdrs = ["definition_disabled.h"],
    aspect_hints = [],  # deliberately disable Crubit.
)

crubit_test_cc_library(
    name = "definition_enabled",
    hdrs = ["definition_enabled.h"],
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":definition_disabled",
        ":definition_enabled",
    ],
    proc_macro_deps = [
        ":type_exists",
    ],
)

rust_proc_macro(
    name = "type_exists",
    testonly = 1,
    srcs = ["type_exists.rs"],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
    ],
)

rust_test(
    name = "type_exists_test",
    srcs = ["type_exists_test.rs"],
    proc_macro_deps = [
        ":type_exists",
    ],
)
