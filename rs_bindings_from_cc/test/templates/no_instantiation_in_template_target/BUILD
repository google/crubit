"""End-to-end example of using type aliases that refer to fully-instantiated
templates in a different target (where the target with the header doesn't
have this particular instantiation)."""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "template_without_instantiation",
    hdrs = ["template_without_instantiation.h"],
)

crubit_test_cc_library(
    name = "type_alias_in_different_target",
    hdrs = ["type_alias_in_different_target.h"],
    deps = [":template_without_instantiation"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":type_alias_in_different_target",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
