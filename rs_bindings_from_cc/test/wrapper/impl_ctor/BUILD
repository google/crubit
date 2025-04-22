load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "impl_ctor",
    hdrs = ["impl_ctor.h"],
    aspect_hints = [
        "//features:wrapper",
        ":impl_ctor_extra",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "impl_ctor_extra",
    srcs = ["impl_ctor_extra.rs"],
)

crubit_rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":impl_ctor"],
    deps = [
        "@crate_index//:googletest",
    ],
)

golden_test(
    name = "impl_ctor_golden_test",
    basename = "impl_ctor",
    cc_library = "impl_ctor",
    golden_rs = "impl_ctor_rs_api.rs",
)
