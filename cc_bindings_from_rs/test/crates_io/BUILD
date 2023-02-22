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

TEST_TARGETS = [
    "//third_party/rust/adler32/v1:adler32",
    "@crate_index//:atomic_refcell",
    "@crate_index//:bit_field",
    "@crate_index//:compiler_builtins",
    "@crate_index//:doc_comment",
    "@crate_index//:downcast_rs",
    "@crate_index//:easy_cast",
    "@crate_index//:fallible_iterator",
    "@crate_index//:fastrand",
    "@crate_index//:flagset",
    "@crate_index//:float_ord",
    "@crate_index//:funty",
    "@crate_index//:jni_sys",
    "@crate_index//:linked_hash_map",
    "@crate_index//:linux_raw_sys",
    "@crate_index//:maplit",
    "@crate_index//:nohash_hasher",
    "@crate_index//:once_cell",
    "@crate_index//:oorandom",
    "@crate_index//:opaque_debug",
    "@crate_index//:percent_encoding",
    "@crate_index//:pkg_config",
    "@crate_index//:precomputed_hash",
    "@crate_index//:quick_error",
    "@crate_index//:rawpointer",
    "@crate_index//:roff",
    "@crate_index//:rustc_hash",
    "@crate_index//:string_cache_shared",
    "@crate_index//:strsim",
    "@crate_index//:subtle",
    "@crate_index//:tinyvec_macros",
    "@crate_index//:tower_service",
    "@crate_index//:try_lock",
    "@crate_index//:vec_map",
    "//third_party/rust/write16/v1:write16",
]

[cc_bindings_from_rust(
    # "@crate_index//:some_crate",
    name = test_target[test_target.find(":") + 1:] + "_cc_api",
    testonly = 1,
    crate = test_target,
) for test_target in TEST_TARGETS]

cc_test(
    name = "crates_io_test",
    srcs = ["crates_io_test.cc"],
    deps = [
        "@com_google_googletest//:gtest_main",
        "//support/rs_std:rs_char",
        "//common:rust_allocator_shims",
    ] + [
        # "@crate_index//:some_crate",
        ":" + test_target[test_target.find(":") + 1:] + "_cc_api"
        for test_target in TEST_TARGETS
    ],
)
