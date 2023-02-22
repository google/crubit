"""End-to-end tests of `cc_bindings_from_rs` where the input crates (the ones
fed as input of the bindings generator tool) come from https://crates.io/.

The main verification here is that
1) `cc_bindings_from_rs` can successfully complete bindings generation for
   a broad set of input crates (e.g. without panicking)
2) The generated bindings compile and link successfully into the
   `crates_io_test` binary.

Some opportunistic, secondary, less important verification is also happening
in individual tests in `crates_io_test.cc`.  For example, there is a test
that bindings for `fastrand` can actually be used in practice.
"""

load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust",
)

licenses(["notice"])

cc_bindings_from_rust(
    name = "fastrand_cc_api",
    testonly = 1,
    crate = "@crate_index//:fastrand",
)

cc_test(
    name = "crates_io_test",
    srcs = ["crates_io_test.cc"],
    deps = [
        ":fastrand_cc_api",
        "@com_google_googletest//:gtest_main",
        "//support/rs_std:rs_char",
        "//common:rust_allocator_shims",
    ],
)
