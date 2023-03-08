load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(
    "//rs_bindings_from_cc/test/golden:generate_bindings.bzl",
    "generate_bindings",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_cli_flag_aspect_hint.bzl",
    "rust_bindings_from_cc_cli_flag",
)
load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

TESTS = [name[:-2] for name in glob(
    ["*.h"],
    exclude = [
        "*namespaces_json.h",
    ],
)]

# Maps a test to the list of tests it depends on.
#
# Valid keys:
#     basenames of tests (e.g. "types" is a valid key, because we
#     have rs_bindings_from_cc/test/golden/types.h.
# Valid values: a list of:
#     basenames of tests (e.g. "types")
#     other Bazel targets (e.g. "//foo/bar:baz")
DEPS = {
    "user_of_unsupported": ["unsupported"],
    "user_of_imported_type": ["trivial_type"],
    "user_of_base_class": ["inheritance"],
    "includes": [
        "comment",
        "namespace",
        "types",
        "unions",
        "unsupported",
    ],
    "types": ["//support/rs_std:rs_char"],
}

TAGS = {}

rust_bindings_from_cc_cli_flag(
    name = "disable_source_location_in_doc_comment",
    flags = "--generate_source_location_in_doc_comment=False",
)

[cc_library(
    name = name + "_cc",
    hdrs = [name + ".h"],
    aspect_hints = [
        "//third_party/crubit:experimental",
        ":disable_source_location_in_doc_comment",
    ],
    copts = ["-Wno-google3-inline-namespace"],
    deps = [
        ((d + "_cc") if d in TESTS else d)
        for d in (DEPS[name] if name in DEPS else [])
    ],
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
    tags = [tag for tag in (TAGS[name] if name in TAGS else [])],
) for name in TESTS]

[rust_test(
    name = name + "_rs_test",
    srcs = ["empty_rs_test.rs"],
    cc_deps = ["%s_cc" % name],
) for name in TESTS]

cc_library(
    name = "namespaces_json",
    hdrs = ["namespaces_json.h"],
    aspect_hints = [
        ":disable_source_location_in_doc_comment",
    ],
    copts = ["-Wno-google3-inline-namespace"],
)

generate_bindings(
    name = "namespaces_json_generated_bindings",
    cc_library = "namespaces_json",
)

filegroup(
    name = "namespaces_json_file",
    srcs = ["namespaces_json_generated_bindings"],
    output_group = "namespaces_file",
)

sh_test(
    name = "namespaces_json_test",
    srcs = ["test.sh"],
    args = [
        "$(location namespaces_json.json)",
        "$(location namespaces_json_file)",
    ],
    data = [
        "LICENSE_HEADER",
        ":namespaces_json.json",
        ":namespaces_json_file",
    ],
)

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
