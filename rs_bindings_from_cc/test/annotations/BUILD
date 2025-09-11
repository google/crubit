load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "do_not_bind",
    hdrs = ["do_not_bind.h"],
    deps = [
        "//support:annotations",
        "//support/internal:bindings_support",
    ],
)

golden_test(
    name = "do_not_bind_golden_test",
    basename = "do_not_bind",
    cc_library = "do_not_bind",
    golden_cc = "do_not_bind_api_impl.cc",
    golden_rs = "do_not_bind_rs_api.rs",
)

crubit_rust_test(
    name = "do_not_bind_test",
    srcs = ["do_not_bind_test.rs"],
    cc_deps = [
        ":do_not_bind",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "ignore_attr",
    hdrs = ["ignore_attr.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        "//support:annotations",
    ],
)

crubit_rust_test(
    name = "ignore_attr_test",
    srcs = ["ignore_attr_test.rs"],
    cc_deps = [
        ":ignore_attr",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "must_bind",
    hdrs = ["must_bind.h"],
    deps = [
        "//support:annotations",
        "//support/internal:bindings_support",
    ],
)

crubit_rust_test(
    name = "must_bind_test",
    srcs = ["must_bind_test.rs"],
    cc_deps = [
        ":must_bind",
    ],
)

crubit_test_cc_library(
    name = "rust_name",
    hdrs = ["rust_name.h"],
    aspect_hints = [
        "//features:infer_operator_lifetimes",
    ],
    deps = [
        "//support:annotations",
        "//support/internal:bindings_support",
    ],
)

golden_test(
    name = "rust_name_golden_test",
    basename = "rust_name",
    cc_library = "rust_name",
    golden_cc = "rust_name_api_impl.cc",
    golden_rs = "rust_name_rs_api.rs",
)

crubit_rust_test(
    name = "rust_name_test",
    srcs = ["rust_name_test.rs"],
    cc_deps = [
        ":rust_name",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
