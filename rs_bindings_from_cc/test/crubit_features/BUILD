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
    name = "alias_enabled",
    hdrs = ["alias_enabled.h"],
    deps = [":definition_disabled"],
)

crubit_test_cc_library(
    name = "definition_enabled",
    hdrs = ["definition_enabled.h"],
)

crubit_test_cc_library(
    name = "alias_disabled",
    hdrs = ["alias_disabled.h"],
    aspect_hints = [],  # deliberately disable Crubit.
    deps = [":definition_enabled"],
)

crubit_test_cc_library(
    name = "func_enabled",
    hdrs = ["func_enabled.h"],
    deps = [":definition_disabled"],
)

crubit_test_cc_library(
    name = "wrapper_struct_enabled",
    hdrs = ["wrapper_struct_enabled.h"],
    deps = [":definition_disabled"],
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":alias_disabled",
        ":alias_enabled",
        ":definition_disabled",
        ":definition_enabled",
        ":func_enabled",
        ":wrapper_struct_enabled",
    ],
    proc_macro_deps = [
        ":item_exists",
    ],
)

rust_proc_macro(
    name = "item_exists",
    testonly = 1,
    srcs = ["item_exists.rs"],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
    ],
)

rust_test(
    name = "item_exists_test",
    srcs = ["item_exists_test.rs"],
    proc_macro_deps = [
        ":item_exists",
    ],
)
