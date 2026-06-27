load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "allowlist_specific_instance",
    srcs = ["allowlist_specific_instance.cc"],
    hdrs = ["allowlist_specific_instance.h"],
    aspect_hints = [
        "//features:supported",
        "//features:template_instantiation",
    ],
    deps = [
        "//support:annotations",
    ],
)

golden_test(
    name = "allowlist_specific_instance_golden_test",
    basename = "allowlist_specific_instance",
    cc_library = "allowlist_specific_instance",
    golden_cc = "allowlist_specific_instance_api_impl.cc",
    golden_rs = "allowlist_specific_instance_rs_api.rs",
)

crubit_rust_test(
    name = "allowlist_specific_instance_test",
    srcs = ["allowlist_specific_instance_test.rs"],
    cc_deps = [
        ":allowlist_specific_instance",
    ],
    deps = [
        "//support:cref",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "allowlist_specific_template",
    srcs = ["allowlist_specific_template.cc"],
    hdrs = ["allowlist_specific_template.h"],
    aspect_hints = [
        "//features:supported",
        "//features:template_instantiation",
    ],
    deps = [
        "//support:annotations",
    ],
)

golden_test(
    name = "allowlist_specific_template_golden_test",
    basename = "allowlist_specific_template",
    cc_library = "allowlist_specific_template",
    golden_cc = "allowlist_specific_template_api_impl.cc",
    golden_rs = "allowlist_specific_template_rs_api.rs",
)

crubit_rust_test(
    name = "allowlist_specific_template_test",
    srcs = ["allowlist_specific_template_test.rs"],
    cc_deps = [
        ":allowlist_specific_template",
    ],
    deps = [
        "//support:cref",
        "@crate_index//:googletest",
    ],
)
