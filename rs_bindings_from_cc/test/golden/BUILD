load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(
    "//rs_bindings_from_cc/test/golden:generate_bindings.bzl",
    "generate_bindings",
)
load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

TESTS = [name[:-2] for name in glob(["*.h"])]

# Maps a test to the list of tests it depends on.
DEPS = {
    "user_of_unsupported": ["unsupported"],
    "user_of_imported_type": ["trivial_type"],
    "user_of_base_class": ["inheritance"],
}

[cc_library(
    name = name + "_cc",
    hdrs = [name + ".h"],
    deps = [d + "_cc" for d in (DEPS[name] if name in DEPS else [])],
) for name in TESTS]

[generate_bindings(
    name = name + "_generated_bindings",
    cc_library = name + "_cc",
) for name in TESTS]

[filegroup(
    name = name + "_rs_file",
    srcs = [name + "_generated_bindings"],
    output_group = "rust_file",
) for name in TESTS]

[filegroup(
    name = name + "_cc_file",
    srcs = [name + "_generated_bindings"],
    output_group = "cc_file",
) for name in TESTS]

[sh_test(
    name = name + "_test",
    srcs = ["test.sh"],
    args = [
        "$(location %s_rs_api_impl.cc)" % name,
        "$(location %s_cc_file)" % name,
        "$(location %s_rs_api.rs)" % name,
        "$(location %s_rs_file)" % name,
    ],
    data = [
        name + "_rs_api_impl.cc",
        name + "_cc_file",
        name + "_rs_api.rs",
        name + "_rs_file",
        "LICENSE_HEADER",
    ],
) for name in TESTS]

[rust_test(
    name = name + "_rs_test",
    srcs = ["empty_rs_test.rs"],
    cc_deps = ["%s_cc" % name],
) for name in TESTS if name not in [
    # TODO(hlopko): These tests fail to compile, fix them.
    "namespace",
    "types",
]]

# Only there so build-cleaner doesn't try to add separate targets for generated files.
filegroup(
    name = "generated_outputs",
    srcs = [t + "_rs_api_impl.cc" for t in TESTS] + [t + "_rs_api.rs" for t in TESTS],
    tags = ["ignore_srcs"],
)

bzl_library(
    name = "generate_bindings_bzl",
    srcs = ["generate_bindings.bzl"],
    parse_tests = False,
    visibility = ["//visibility:private"],
    deps = [
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils_bzl",
    ],
)
