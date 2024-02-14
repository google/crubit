# TODO(b/276366603): Use a regular rust_binary, once stable is sufficiently up to date.
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_binary",
)
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

cc_library(
    name = "example_lib",
    hdrs = ["example.h"],

    # Opt into using Crubit.
    aspect_hints = ["//features:extern_c"],
    deps = ["@absl//absl/base:core_headers"],
)

crubit_rust_binary(
    name = "main",
    srcs = ["main.rs"],

    # Declare a dependency on Rust bindings for calling into the C++ `example_lib` library:
    cc_deps = [":example_lib"],
)

# This test rule is just to make sure we don't forget to keep the `example_generated.rs` updated!
golden_test(
    name = "example_golden_test",
    cc_library = "example_lib",
    golden_rs = "example_generated.rs",
)
