"""End-to-end tests of `cc_bindings_from_rs`, focusing on function-related
bindings."""

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)

licenses(["notice"])

rust_library(
    name = "functions",
    testonly = 1,
    srcs = ["functions.rs"],
    deps = [
        "//common:rust_allocator_shims",
    ],
)

alias(
    name = "cc_bindings_from_rs_tool",
    actual = "//cc_bindings_from_rs:cc_bindings_from_rs_legacy_toolchain_runner.sar",
)

# TODO): Author a reusable "macro" [1] that automates writing
# the `genrule` here, the `cc_library` below, and the `alias` above.
# [1] https://bazel.build/extending/macros
genrule(
    name = "functions_cc_api_genrule",
    testonly = 1,
    srcs = ["functions.rs"],

    # TODO(b/254097223): Also cover `functions_cc_api_impl.rs`
    # (wrapping it in a `rust_library` that `functions_cc_api` depends on).
    outs = ["functions_cc_api.h"],
    cmd = """
        $(location :cc_bindings_from_rs_tool) \
            "--h-out=$@" \
            -- \
            $(SRCS) \
            --crate-type=lib \
            --codegen=panic=abort \
    """,
    message = "Running cc_bindings_from_rs",
    tools = [":cc_bindings_from_rs_tool"],
)

cc_library(
    name = "functions_cc_api",
    testonly = 1,
    hdrs = [":functions_cc_api.h"],
    deps = [":functions"],
)

cc_test(
    name = "functions_test",
    srcs = ["functions_test.cc"],
    deps = [
        ":functions_cc_api",
        "@com_google_googletest//:gtest_main",
    ],
)
