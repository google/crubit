load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "send_sync_inference",
    hdrs = ["send_sync_inference.h"],
    aspect_hints = [
        "//features:infer_send_sync",
    ],
    deps = [
        "//support:annotations",
    ],
)

rust_api_from_cpp(
    name = "send_sync_inference_rust",
    cpp_target = ":send_sync_inference",
)

crubit_rust_test(
    name = "send_sync_inference_test",
    srcs = ["send_sync_inference_test.rs"],
    deps = [
        ":send_sync_inference_rust",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)

golden_test(
    name = "send_sync_inference_golden_test",
    basename = "send_sync_inference",
    cc_library = "send_sync_inference",
    golden_cc = "send_sync_inference_api_impl.cc",
    golden_rs = "send_sync_inference_rs_api.rs",
)
