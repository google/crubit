load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl", "additional_rust_srcs_for_crubit_bindings")
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
        "//support/public:bindings_support",
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
        "@crates//:googletest",
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
        "@crates//:googletest",
    ],
)

crubit_test_cc_library(
    name = "must_bind",
    hdrs = ["must_bind.h"],
    deps = [
        "//support:annotations",
        "//support/public:bindings_support",
    ],
)

crubit_rust_test(
    name = "must_bind_test",
    srcs = ["must_bind_test.rs"],
    cc_deps = [
        ":must_bind",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "additional_deps_for_owned_ptr",
    srcs = [":owned_ptr_rust_thing.rs"],
)

crubit_test_cc_library(
    name = "owned_ptr",
    hdrs = ["owned_ptr.h"],
    aspect_hints = [
        ":additional_deps_for_owned_ptr",
        "//features:supported",
    ],
    deps = [
        "//support:annotations",
        "//support/public:bindings_support",
    ],
)

crubit_test_cc_library(
    name = "owned_ptr_user",
    srcs = ["owned_ptr_user.cc"],
    hdrs = ["owned_ptr_user.h"],
    deps = [
        ":owned_ptr",
        "//support:annotations",
        "//support/public:bindings_support",
    ],
)

crubit_rust_test(
    name = "owned_ptr_test",
    srcs = ["owned_ptr_test.rs"],
    cc_deps = [
        ":owned_ptr",
        ":owned_ptr_user",
    ],
    deps = [
        "@crates//:googletest",
    ],
)

golden_test(
    name = "owned_ptr_golden_test",
    basename = "owned_ptr",
    cc_library = "owned_ptr",
    golden_cc = "owned_ptr_api_impl.cc",
    golden_rs = "owned_ptr_rs_api.rs",
)

golden_test(
    name = "owned_ptr_user_golden_test",
    basename = "owned_ptr_user",
    cc_library = "owned_ptr_user",
    golden_cc = "owned_ptr_user_api_impl.cc",
    golden_rs = "owned_ptr_user_rs_api.rs",
)

crubit_test_cc_library(
    name = "rust_name",
    hdrs = ["rust_name.h"],
    deps = [
        "//support:annotations",
        "//support/public:bindings_support",
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
        "@crates//:googletest",
    ],
)
