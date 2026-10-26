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
    name = "do_not_bind",
    testonly = 1,
    srcs = ["do_not_bind.rs"],
    edition = "2024",
    proc_macro_deps = [
        "//support:crubit_annotate",
    ],
)

cpp_api_from_rust(
    name = "do_not_bind_cc_api",
    testonly = 1,
    crate = ":do_not_bind",
)

golden_test(
    name = "do_not_bind_golden_test",
    basename = "do_not_bind",
    golden_h = "do_not_bind_cc_api.h",
    golden_rs = "do_not_bind_cc_api_impl.rs",
    platforms = [
        "android",
        "ios",
        "macos",
    ],
    rust_library = "do_not_bind",
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
)

crubit_cc_test(
    name = "do_not_bind_test",
    srcs = ["do_not_bind_test.cc"],
    # TODO(b/522829368): remove `no_windows` to enable.
    tags = ["no_windows"],
    deps = [
        ":do_not_bind_cc_api",
        "@googletest//:gtest_main",
    ],
)
