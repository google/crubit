load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "methods_qualifiers",
    srcs = ["methods_qualifiers.cc"],
    hdrs = ["methods_qualifiers.h"],
    aspect_hints = ["//features:experimental"],
)

golden_test(
    name = "methods_qualifiers_golden_test",
    basename = "methods_qualifiers",
    cc_library = ":methods_qualifiers",
    golden_cc = "methods_qualifiers_api_impl.cc",
    golden_rs = "methods_qualifiers_rs_api.rs",
)

crubit_rust_test(
    name = "methods_qualifiers_test",
    srcs = ["methods_qualifiers_test.rs"],
    cc_deps = [":methods_qualifiers"],
    deps = [
        "//support:ctor",
        "@crates//:googletest",
    ],
)
