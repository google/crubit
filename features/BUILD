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
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES,
    visibility = ["//visibility:public"],
)

_WRAPPER_FEATURES = [
    "wrapper",
    "non_unpin_ctor",
]

# A feature set containing wrapper-library Crubit features, in addition to the officially supported
# features.
#
# See <internal link>
crubit_feature_hint(
    name = "wrapper",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + _WRAPPER_FEATURES,
    visibility = _EXPERIMENTAL_CLIENTS,
)

# A feature set which specifically std::vector support.
#
# See <internal link>
crubit_feature_hint(
    name = "std_vector",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES,
    visibility = ["//visibility:public"],
)

# A feature set which specifically enables std::unique_ptr support.
#
# See <internal link>
crubit_feature_hint(
    name = "std_unique_ptr",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["std_unique_ptr"],
    visibility = ["//visibility:public"],
)

# A feature set which specifically enables non-`Unpin` type handling with the `ctor` crate.
#
# See <internal link>
crubit_feature_hint(
    name = "non_unpin_ctor",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["non_unpin_ctor"],
    visibility = ["//visibility:public"],
)

# A feature set enabling emitting `ffi_11` types in bindings.
#
# See <internal link>
crubit_feature_hint(
    name = "custom_ffi_types",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["custom_ffi_types"],
    visibility = ["//visibility:public"],
)

# A feature set enabling emitting source annotations in bindings.
#
# See <internal link>
crubit_feature_hint(
    name = "force_annotations",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["force_annotations"],
    visibility = ["//visibility:public"],
)

# A feature set containing experimental Crubit features, in addition to the officially supported
# features.
#
# See <internal link>
crubit_feature_hint(
    name = "experimental",
    compatible_with = ["//buildenv/target:non_prod"],
    # TODO(b/409128537): Add _WRAPPER_FEATURES on next binary release.
    crubit_features = [
        "custom_ffi_types",
        "experimental",
        "non_unpin_ctor",
        "std_unique_ptr",
        "std_vector",
        "supported",
        "wrapper",
    ],
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
