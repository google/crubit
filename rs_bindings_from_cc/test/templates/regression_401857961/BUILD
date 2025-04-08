load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

cc_library(
    name = "repro",
    hdrs = ["repro.h"],
    aspect_hints = ["//features:supported"],
)

golden_test(
    name = "repro_golden_test",
    basename = "repro",
    cc_library = "repro",
    golden_cc = "repro_rs_api_impl.cc",
    golden_rs = "repro_rs_api.rs",
)

crubit_rust_test(
    name = "builds",
    srcs = ["builds.rs"],
    cc_deps = [":repro"],
)
