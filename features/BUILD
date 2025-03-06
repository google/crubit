load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load(
    "//features:global_features.bzl",
    "SUPPORTED_FEATURES",
)
load(
    ":crubit_feature_hint.bzl",
    "crubit_feature_hint",
)

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//visibility:private"],
)

_EXPERIMENTAL_CLIENTS = [
    "//:__subpackages__",
]

# A feature set containing all supported Crubit features.
# Add this to the `aspect_hints` of a `cc_library` to generate Rust bindings for that library.
crubit_feature_hint(
    name = "supported",
    crubit_features = SUPPORTED_FEATURES,
    visibility = ["//visibility:public"],
)

# A feature set containing experimental Crubit features, in addition to the officially supported
# features.
crubit_feature_hint(
    name = "experimental",
    crubit_features = SUPPORTED_FEATURES + [
        "experimental",
    ],
    visibility = _EXPERIMENTAL_CLIENTS,
)

# A feature set with a stable expansion, only for use in Bazel unit tests.
crubit_feature_hint(
    name = "internal_testonly_supported",
    crubit_features = ["supported"],
    visibility = ["//:__subpackages__"],
)

# A feature set with a stable expansion, only for use in Bazel unit tests.
crubit_feature_hint(
    name = "internal_testonly_experimental",
    crubit_features = ["experimental"],
    visibility = ["//:__subpackages__"],
)

# Implementation details follow.

bzl_library(
    name = "crubit_feature_hint_bzl",
    srcs = ["crubit_feature_hint.bzl"],
    visibility = [
        "//:__subpackages__",
    ],
)

bzl_library(
    name = "global_features_bzl",
    srcs = ["global_features.bzl"],
    visibility = ["//cc_bindings_from_rs/bazel_support:__pkg__"],
)
