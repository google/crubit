load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "free_function",
    srcs = ["free_function.cc"],
    hdrs = ["free_function.h"],
    aspect_hints = [
        "//features:supported",
        "//features:assume_lifetimes",
    ],
)

golden_test(
    name = "free_function_golden_test",
    basename = "free_function",
    cc_library = "free_function",
    golden_cc = "free_function_api_impl.cc",
    golden_rs = "free_function_rs_api.rs",
)

crubit_rust_test(
    name = "free_function_test",
    srcs = ["free_function_test.rs"],
    cc_deps = [
        ":free_function",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "member_function",
    srcs = ["member_function.cc"],
    hdrs = ["member_function.h"],
    aspect_hints = [
        "//features:supported",
        "//features:assume_lifetimes",
    ],
)

golden_test(
    name = "member_function_golden_test",
    basename = "member_function",
    cc_library = "member_function",
    golden_cc = "member_function_api_impl.cc",
    golden_rs = "member_function_rs_api.rs",
)

crubit_rust_test(
    name = "member_function_test",
    srcs = ["member_function_test.rs"],
    cc_deps = [
        ":member_function",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
