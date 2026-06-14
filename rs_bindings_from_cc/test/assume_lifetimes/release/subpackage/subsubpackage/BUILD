load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//visibility:private"],
)

crubit_test_cc_library(
    name = "subsubpackage",
    hdrs = ["subsubpackage.h"],
    aspect_hints = [
        "//features:supported",
    ],
)

golden_test(
    name = "subsubpackage_golden_test",
    basename = "subsubpackage",
    cc_library = "subsubpackage",
    golden_cc = "subsubpackage_api_impl.cc",
    golden_rs = "subsubpackage_rs_api.rs",
)
