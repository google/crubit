load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)
load(
    "//rs_bindings_from_cc/bazel_support:additional_rust_srcs_for_crubit_bindings_aspect_hint.bzl",
    "additional_rust_srcs_for_crubit_bindings",
)
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

cc_library(
    name = "displayables",
    hdrs = ["displayables.h"],
    aspect_hints = [
        ":displayables_rs",
        "//features:supported",
    ],
    deps = [
        "//support:annotations",
        "@abseil-cpp//absl/strings:has_ostream_operator",
        "@abseil-cpp//absl/strings:str_format",
        "@abseil-cpp//absl/strings:string_view",
    ],
)

additional_rust_srcs_for_crubit_bindings(
    name = "displayables_rs",
    srcs = ["displayables.rs"],
)

golden_test(
    name = "displayables_golden_test",
    basename = "displayables",
    cc_library = "displayables",
    golden_cc = "displayables_api_impl.cc",
    golden_rs = "displayables_api.rs",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":displayables",
        "//support/public:cc_std",
    ],
    deps = [
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)
