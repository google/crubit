load(
    "//rs_bindings_from_cc/bazel_support:crubit_feature_hint.bzl",
    "crubit_feature_hint",
)

_SUPPORTED_FEATURES = [
    "supported",
    "extern_c",
]

_EXPERIMENTAL_CLIENTS = [
    "//:__subpackages__",
]

crubit_feature_hint(
    name = "supported",
    crubit_features = _SUPPORTED_FEATURES,
    visibility = ["//visibility:public"],
)

# Use :supported if you're calling C++ from Rust.
crubit_feature_hint(
    name = "extern_c",
    crubit_features = _SUPPORTED_FEATURES,
    visibility = _EXPERIMENTAL_CLIENTS,
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
