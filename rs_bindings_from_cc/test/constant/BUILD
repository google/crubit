load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "constant",
    hdrs = ["constant.h"],
    aspect_hints = ["//features:supported"],
)

golden_test(
    name = "constant_golden_test",
    basename = "constant",
    cc_library = "constant",
    golden_cc = "constant_api_impl.cc",
    golden_rs = "constant_rs_api.rs",
)
