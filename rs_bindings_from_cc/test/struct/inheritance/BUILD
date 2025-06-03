load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "upcast",
    hdrs = ["upcast.h"],
)

crubit_rust_test(
    name = "upcast_test",
    srcs = ["upcast_test.rs"],
    cc_deps = [":upcast"],
    deps = [
        "//support:ctor",
        "//support:oops",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "inherited_methods",
    hdrs = ["inherited_methods.h"],
)

golden_test(
    name = "inherited_methods_golden_test",
    basename = "inherited_methods",
    cc_library = "inherited_methods",
    golden_rs = "inherited_methods_rs_api.rs",
)

crubit_rust_test(
    name = "inherited_methods_test",
    srcs = ["inherited_methods_test.rs"],
    cc_deps = [":inherited_methods"],
    deps = [
        "@crate_index//:googletest",
    ],
)
