load(
    "//rs_bindings_from_cc/bazel_support:crubit_feature_hint.bzl",
    "crubit_feature_hint",
)

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//visibility:private"],
)

_SUPPORTED_FEATURES = [
    "supported",
]

_EXPERIMENTAL_CLIENTS = [
    "//:__subpackages__",
]

crubit_feature_hint(
    name = "supported",
    crubit_features = _SUPPORTED_FEATURES,
    visibility = ["//visibility:public"],
)

crubit_feature_hint(
    name = "experimental",
    crubit_features = _SUPPORTED_FEATURES + [
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
