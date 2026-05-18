load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "named_instantiation",
    srcs = ["named_instantiation.cc"],
    hdrs = ["named_instantiation.h"],
    aspect_hints = [
        "//features:supported",
        "//features:template_instantiation",
    ],
)

golden_test(
    name = "named_instantiation_golden_test",
    basename = "named_instantiation",
    cc_library = "named_instantiation",
    golden_cc = "named_instantiation_api_impl.cc",
    golden_rs = "named_instantiation_rs_api.rs",
)

crubit_rust_test(
    name = "named_instantiation_test",
    srcs = ["named_instantiation_test.rs"],
    cc_deps = [
        ":named_instantiation",
    ],
    deps = [
        "//support:cref",
        "@crate_index//:googletest",
    ],
)
