load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "forward_declaration1",
    hdrs = ["forward_declaration1.h"],
    aspect_hints = ["//features:experimental"],
)

# Ensure Crubit doesn't crash when it sees forward declarations of the same type in 2 different
# targets.
crubit_test_cc_library(
    name = "forward_declaration2",
    hdrs = ["forward_declaration2.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_test_cc_library(
    name = "definition",
    hdrs = [
        "definition.h",
    ],
    aspect_hints = ["//features:experimental"],
    deps = [
        ":forward_declaration1",
        ":forward_declaration2",
    ],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":definition",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crates//:googletest",
    ],
)
