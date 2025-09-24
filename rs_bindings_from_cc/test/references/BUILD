"""End-to-end example of using a simple references function."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "references",
    hdrs = ["references.h"],
    aspect_hints = [
        "//features:supported",
        "//features:infer_operator_lifetimes",
    ],
)

golden_test(
    name = "references_golden_test",
    basename = "references",
    cc_library = "references",
    golden_cc = "references_api_impl.cc",
    golden_rs = "references_rs_api.rs",
)

crubit_rust_test(
    name = "references_test",
    srcs = ["references_test.rs"],
    cc_deps = [":references"],
    deps = [
        "@crate_index//:googletest",
    ],
)
