load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//bazel:cpp_api_from_rust.bzl",
    "cpp_api_from_rust",
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
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

golden_test(
    name = "vec_golden_test",
    basename = "vec",
    golden_h = "vec_cc_api.h",
    golden_rs = "vec_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "vec",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

cpp_api_from_rust(
    name = "vec_cc_api",
    testonly = 1,
    crate = ":vec",
)

crubit_cc_test(
    name = "vec_test",
    srcs = ["vec_test.cc"],
    platforms = [
        "android",
        "ios",
    ],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":vec_cc_api",
        "//support/rs_std:rs_alloc",
        "//support/rs_std:slice_ref",
        "@abseil-cpp//absl/types:span",
        "@googletest//:gtest_main",
    ],
)
