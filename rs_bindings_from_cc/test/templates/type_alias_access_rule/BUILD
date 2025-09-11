"""Test that Crubit-generated bindings conform to C++ access specifier rules, when template
instantiation references a private type.
"""

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "type_alias_access_rule",
    hdrs = ["type_alias_access_rule.h"],
    aspect_hints = ["//features:experimental"],
)

golden_test(
    name = "type_alias_access_rule_golden_test",
    basename = "type_alias_access_rule",
    cc_library = "type_alias_access_rule",
    golden_rs = "type_alias_access_rule_rs_api.rs",
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":type_alias_access_rule",
    ],
)
