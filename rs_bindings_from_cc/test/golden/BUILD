load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_cli_flag_aspect_hint.bzl",
    "rust_bindings_from_cc_cli_flag",
)
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

TESTS = [name[:-2] for name in glob(
    ["*.h"],
    exclude = [
        "*namespaces_json.h",
        "not_crubit_exposed.h",
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
    "types": ["//support/rs_std:char"],
    "definition_of_forward_declaration": [
        "forward_declaration",
    ],
    "uses_not_crubit_exposed": ["not_crubit_exposed"],
    "depends_on_nested_types": ["nested_types"],
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
        "//features:infer_operator_lifetimes",
        ":disable_source_location_in_doc_comment",
    ],
    deps = [
        ((d + "_cc") if d in TESTS else d)
        for d in (DEPS[name] if name in DEPS else [])
    ],
) for name in TESTS]

[golden_test(
    name = name + "_test",
    basename = name,
    cc_library = name + "_cc",
    golden_cc = name + "_rs_api_impl.cc",
    golden_rs = name + "_rs_api.rs",
    tags = [tag for tag in (TAGS[name] if name in TAGS else [])],
) for name in TESTS]

NON_BUILDABLE_TEST = [
    # The bridge type is not defined in the C++ header so it cannot be built.
    "bridge_type",
    # We can't support additional Rust srcs so we can't run the test.
    "composable_bridging",
]

BUILDABLE_TESTS = [name for name in TESTS if name not in NON_BUILDABLE_TEST]

[crubit_rust_test(
    name = name + "_rs_test",
    srcs = ["empty_rs_test.rs"],
    cc_deps = ["%s_cc" % name],
) for name in BUILDABLE_TESTS]

cc_library(
    name = "namespaces_json",
    hdrs = ["namespaces_json.h"],
    aspect_hints = [
        ":disable_source_location_in_doc_comment",
        "//features:supported",
    ],
)

golden_test(
    name = "namespaces_json_test",
    basename = "namespaces_json",
    cc_library = "namespaces_json",
    golden_namespaces = "namespaces_json.json",
)

bzl_library(
    name = "golden_test",
    srcs = ["golden_test.bzl"],
    visibility = ["//visibility:private"],
    deps = [
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect",
        "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_utils_bzl",
    ],
)

cc_library(
    name = "not_crubit_exposed",
    hdrs = ["not_crubit_exposed.h"],
)
