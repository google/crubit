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
#
# See <internal link>
crubit_feature_hint(
    name = "supported",
    crubit_features = SUPPORTED_FEATURES,
    visibility = ["//visibility:public"],
)

_WRAPPER_FEATURES = ["wrapper"]

# A feature set containing wrapper-library Crubit features, in addition to the officially supported
# features.
#
# See <internal link>
crubit_feature_hint(
    name = "wrapper",
    crubit_features = SUPPORTED_FEATURES + _WRAPPER_FEATURES,
    visibility = _EXPERIMENTAL_CLIENTS,
)

# A feature set which specifically enables inferred operator lifetimes.
#
# See <internal link>
crubit_feature_hint(
    name = "infer_operator_lifetimes",
    crubit_features = SUPPORTED_FEATURES + ["infer_operator_lifetimes"],
    visibility = ["//visibility:public"],
)

# A feature set containing experimental Crubit features, in addition to the officially supported
# features.
#
# See <internal link>
crubit_feature_hint(
    name = "experimental",
    # TODO(b/409128537): Add _WRAPPER_FEATURES on next binary release.
    crubit_features = ["all"],
    visibility = _EXPERIMENTAL_CLIENTS,
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
