load("@bazel_skylib//rules:build_test.bzl", "build_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load("//rs_bindings_from_cc/test/bazel/per_file_copt:per_file_copt.bzl", "with_per_file_copt")
load("//rs_bindings_from_cc/test/golden:golden_test.bzl", "generated_bindings")

package(
    # Build and use precompiled header modules (where supported).
    features = ["header_modules"],
)

licenses(["notice"])

# Regression test for b/582351896: `--per_file_copt`s must reach the bindings action.
#
# Bazel applies `--per_file_copt`s to the compile actions it creates, including the `.pcm` header
# modules of `:dep`, but not to actions created with `cc_common.create_compile_action`, like the
# bindings action of `:lib`. If the bindings action doesn't get the same flags, Clang rejects the
# `.pcm`s of the deps ("cannot be loaded due to a configuration mismatch"), and the textual fallback
# fails because the headers of the deps aren't inputs of the action when header modules are used.

crubit_test_cc_library(
    name = "lib",
    hdrs = ["lib.h"],
    deps = [":dep"],
)

crubit_test_cc_library(
    name = "dep",
    hdrs = ["dep.h"],
)

generated_bindings(
    name = "lib_bindings",
    cc_library = ":lib",
)

filegroup(
    name = "lib_rs",
    srcs = [":lib_bindings"],
    output_group = "rust_file",
)

with_per_file_copt(
    name = "lib_rs_with_per_file_copt",
    target = ":lib_rs",
)

build_test(
    name = "per_file_copt_test",
    targets = [":lib_rs_with_per_file_copt"],
)
