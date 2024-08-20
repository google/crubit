load("@rules_rust//rust:defs.bzl", "rust_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "rust_library",
    srcs = [
        "rust.rs",
    ],
)

crubit_test_cc_library(
    name = "cc_library_with_rust_deps",
    srcs = [
        "empty.cc",
    ],
    deps = [
        ":rust_library",
    ],
)

# Regression test to ensure Crubit can have a C++ target depending on a Rust target in `cc_deps`.
crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":cc_library_with_rust_deps",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
