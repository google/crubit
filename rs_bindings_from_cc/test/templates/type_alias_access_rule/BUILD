"""Test that Crubit-generated bindings conform to C++ access specifier rules, when template
instantiation references a private type.
"""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "type_alias_access_rule",
    hdrs = ["type_alias_access_rule.h"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":type_alias_access_rule",
    ],
    # Disabled as it currently doesn't compile: b/305987013.
    tags = [
        "manual",
        "notap",
    ],
)
