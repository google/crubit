load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "upcast",
    hdrs = ["upcast.h"],
    aspect_hints = ["//features:oo_casting"],
)

rust_api_from_cpp(
    name = "upcast_rust",
    cpp_target = ":upcast",
)

crubit_rust_test(
    name = "upcast_test",
    srcs = ["upcast_test.rs"],
    deps = [
        ":upcast_rust",
        "//support:ctor",
        "//support:oops",
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "not_crubit_enabled",
    hdrs = ["not_crubit_enabled.h"],
)

crubit_test_cc_library(
    name = "inherited_methods",
    hdrs = ["inherited_methods.h"],
    aspect_hints = ["//features:oo_casting"],
    deps = [":not_crubit_enabled"],
)

rust_api_from_cpp(
    name = "inherited_methods_rust",
    cpp_target = ":inherited_methods",
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
    deps = [
        ":inherited_methods_rust",
        "//support:oops",
        "@crate_index//:googletest",
    ],
)
