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
    name = "subpackage",
    hdrs = ["subpackage.h"],
    aspect_hints = [
        "//features:supported",
    ],
)

golden_test(
    name = "subpackage_golden_test",
    basename = "subpackage",
    cc_library = "subpackage",
    golden_cc = "subpackage_api_impl.cc",
    golden_rs = "subpackage_rs_api.rs",
)

crubit_test_cc_library(
    name = "opt_in",
    hdrs = ["opt_in.h"],
    aspect_hints = [
        "//features:assume_lifetimes",
        "//features:supported",
    ],
)

golden_test(
    name = "opt_in_golden_test",
    basename = "opt_in",
    cc_library = "opt_in",
    golden_cc = "opt_in_api_impl.cc",
    golden_rs = "opt_in_rs_api.rs",
)

crubit_test_cc_library(
    name = "supported",
    hdrs = ["supported.h"],
    aspect_hints = [
        "//features:supported",
    ],
)

golden_test(
    name = "supported_golden_test",
    basename = "supported",
    cc_library = "supported",
    golden_cc = "supported_api_impl.cc",
    golden_rs = "supported_rs_api.rs",
)

crubit_test_cc_library(
    name = "opt_out",
    hdrs = ["opt_out.h"],
    aspect_hints = [
        "//features:no_assume_lifetimes",
        "//features:supported",
    ],
)

golden_test(
    name = "opt_out_golden_test",
    basename = "opt_out",
    cc_library = "opt_out",
    golden_cc = "opt_out_api_impl.cc",
    golden_rs = "opt_out_rs_api.rs",
)

crubit_test_cc_library(
    name = "experimental",
    hdrs = ["experimental.h"],
    aspect_hints = [
        "//features:experimental",
    ],
)

golden_test(
    name = "experimental_golden_test",
    basename = "experimental",
    cc_library = "experimental",
    golden_cc = "experimental_api_impl.cc",
    golden_rs = "experimental_rs_api.rs",
)

crubit_test_cc_library(
    name = "experimental_opt_out",
    hdrs = ["experimental_opt_out.h"],
    aspect_hints = [
        "//features:experimental",
        "//features:no_assume_lifetimes",
    ],
)

golden_test(
    name = "experimental_opt_out_golden_test",
    basename = "experimental_opt_out",
    cc_library = "experimental_opt_out",
    golden_cc = "experimental_opt_out_api_impl.cc",
    golden_rs = "experimental_opt_out_rs_api.rs",
)
