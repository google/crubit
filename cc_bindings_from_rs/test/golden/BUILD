load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
)
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_cli_flag_aspect_hint.bzl",
    "cc_bindings_from_rust_cli_flag",
)
load(
    "//cc_bindings_from_rs/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

# These are needed by golden_test, which can be run in other packages.
exports_files([
    "test.sh",
    "LICENSE_HEADER",
])

TESTS = [name[:-3] for name in glob(
    ["*.rs"],
    exclude = ["*cc_api_impl.rs"],
)]

cc_bindings_from_rust_cli_flag(
    name = "no_thunk_name_mangling",
    flags = "--no-thunk-name-mangling",
)

[rust_library(
    name = name + "_rust",
    srcs = [name + ".rs"],
    aspect_hints = [
        "//features:experimental",
        ":no_thunk_name_mangling",
    ],
) for name in TESTS]

# TODO(yongheng): Add a test to compile the generated bindings.

[golden_test(
    name = name + "_test",
    basename = name,
    golden_h = name + "_cc_api.h",
    golden_rs = name + "_cc_api_impl.rs",
    rust_library = name + "_rust",
) for name in TESTS]

bzl_library(
    name = "golden_test",
    srcs = ["golden_test.bzl"],
    visibility = ["//visibility:private"],
    deps = [
        "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule",
    ],
)
