"""End-to-end example of using type aliases."""

load("//rs_bindings_from_cc/test:crubit_rust_test.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "type_alias",
    hdrs = ["type_alias.h"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":type_alias"],
)
