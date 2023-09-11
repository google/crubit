"""End-to-end tests of `cc_bindings_from_rs`, focusing on how
Bazel<->cc_bindings_from_rs integration handles `build.rs` scripts of Cargo crates
under `//third_party/rust/...`.  In particular, this test reproduces the following
error (that was discovered through cc_bindings_from_rs/test/crates_io tests):

ERROR: ... error executing command .../cc_bindings_from_rs --h-out ... (remaining 30 arguments skipped) ...
error: environment variable `OUT_DIR` not defined at compile time
  --> cc_bindings_from_rs/test/bazel/build_rs_out_dir/build_rs_out_dir.rs:10:18
   |
10 | include!(concat!(env!("OUT_DIR"), "/include_me.rs"));
   |                  ^^^^^^^^^^^^^^^
"""

load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test")

cargo_build_script(
    name = "build_rs_out_dir_build_script",
    srcs = ["build.rs"],
)

rust_library(
    name = "build_rs_out_dir",
    testonly = 1,
    srcs = ["build_rs_out_dir.rs"],
    deps = [":build_rs_out_dir_build_script"],
)

cc_bindings_from_rust(
    name = "build_rs_out_dir_cc_api",
    testonly = 1,
    crate = ":build_rs_out_dir",
)

crubit_cc_test(
    name = "build_rs_out_dir_test",
    srcs = ["build_rs_out_dir_test.cc"],
    deps = [
        ":build_rs_out_dir_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
