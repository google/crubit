load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "vec",
    srcs = ["vec.rs"],
    aspect_hints = [
        "//features:experimental",
    ],
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "vec_golden_test",
    basename = "vec",
    golden_h = "vec_cc_api.h",
    golden_rs = "vec_cc_api_impl.rs",
    rust_library = "vec",
)

cc_bindings_from_rust(
    name = "vec_cc_api",
    testonly = 1,
    crate = ":vec",
)

crubit_cc_test(
    name = "vec_test",
    srcs = ["vec_test.cc"],
    deps = [
        ":vec_cc_api",
        "//support/rs_std:rs_alloc",
        "//support/rs_std:slice_ref",
        "//support/rs_std:vec",
        "//testing/base/public:gunit_main",
        "@abseil-cpp//absl/types:span",
    ],
)
