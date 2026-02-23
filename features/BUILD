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
# See crubit.rs-features#supported
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
# See crubit.rs-features#wrapper
crubit_feature_hint(
    name = "wrapper",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + _WRAPPER_FEATURES,
    visibility = _EXPERIMENTAL_CLIENTS,
)

# A feature set which specifically std::vector support.
#
# See crubit.rs-features#other
crubit_feature_hint(
    name = "std_vector",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES,
    visibility = ["//visibility:public"],
)

# A feature set which specifically enables std::unique_ptr support.
#
# See crubit.rs-features#other
crubit_feature_hint(
    name = "std_unique_ptr",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["std_unique_ptr"],
    visibility = ["//visibility:public"],
)

# A feature set which specifically enables non-`Unpin` type handling with the `ctor` crate.
#
# See crubit.rs-features#other
crubit_feature_hint(
    name = "non_unpin_ctor",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["non_unpin_ctor"],
    visibility = ["//visibility:public"],
)

# A feature set enabling ergonomic lifetime defaults when interpreting lifetime annotations.
#
# See crubit.rs-features#other
crubit_feature_hint(
    name = "assume_lifetimes",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["assume_lifetimes"],
    visibility = ["//visibility:public"],
)

# A feature set enabling formatting to Rust via C++.
#
# See crubit.rs-features#other
crubit_feature_hint(
    name = "fmt",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["fmt"],
    visibility = ["//visibility:public"],
)

# A feature set containing experimental Crubit features, in addition to the officially supported
# features.
#
# See crubit.rs-features#experimental
crubit_feature_hint(
    name = "experimental",
    compatible_with = ["//buildenv/target:non_prod"],
    # TODO(b/409128537): Add _WRAPPER_FEATURES on next binary release.
    crubit_features = ["all"],
    visibility = _EXPERIMENTAL_CLIENTS,
)

crubit_feature_hint(
    name = "callables",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["callables"],
    visibility = ["//visibility:public"],
)

# A feature set marking C++ classes with `[[gsl::Pointer]]` as unsafe.
#
# See crubit.rs-features#other
crubit_feature_hint(
    name = "unsafe_view",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["unsafe_view"],
    visibility = ["//visibility:public"],
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
