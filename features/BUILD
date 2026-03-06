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

# Explicitly disables assume_lifetimes for a target.
#
# See crubit.rs-features#other
crubit_feature_hint(
    name = "no_assume_lifetimes",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["no_assume_lifetimes"],
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

# A feature set enabling the `this` pointer to be interpreted as a reference.
#
# See crubit.rs-features#other
crubit_feature_hint(
    name = "assume_this_lifetimes",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["assume_this_lifetimes"],
    visibility = ["//visibility:public"],
)

# A feature set enabling bindings to Rust's `Display` trait.
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

# A feature set where  C++ default constructors are checked to fully initialize all public fields.
#
# See crubit.rs-features#other
crubit_feature_hint(
    name = "check_default_initialized",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = SUPPORTED_FEATURES + ["check_default_initialized"],
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
