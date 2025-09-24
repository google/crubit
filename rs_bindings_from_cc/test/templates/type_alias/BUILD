"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "type_alias",
    hdrs = ["type_alias.h"],
    aspect_hints = ["//features:experimental"],
)

golden_test(
    name = "type_alias_golden_test",
    basename = "type_alias",
    cc_library = ":type_alias",
    golden_cc = "type_alias_api_impl.cc",
    golden_rs = "type_alias_rs_api.rs",
)

cc_library(
    name = "type_alias_in_different_target",
    hdrs = ["type_alias_in_different_target.h"],
    aspect_hints = ["//features:experimental"],
    deps = [":type_alias"],
)

crubit_rust_test(
    name = "test",
    srcs = ["type_alias_test.rs"],
    cc_deps = [
        ":type_alias",
        ":type_alias_in_different_target",
    ],
    deps = [
        "//support:forward_declare",
        "@crate_index//:googletest",
    ],
)
