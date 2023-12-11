load(
    "//rs_bindings_from_cc/bazel_support:crubit_feature_hint.bzl",
    "crubit_feature_hint",
)

_SUPPORTED_FEATURES = [
    "supported",
]

_EXPERIMENTAL_CLIENTS = [
    "//security/ise_memory_safety/safe_json_parsing:__subpackages__",
    "//:__subpackages__",
]

crubit_feature_hint(
    name = "supported",
    crubit_features = _SUPPORTED_FEATURES,
    visibility = ["//visibility:public"],
)

crubit_feature_hint(
    name = "extern_c",
    crubit_features = ["extern_c"],
    visibility = _EXPERIMENTAL_CLIENTS,
)

crubit_feature_hint(
    name = "experimental",
    crubit_features = _SUPPORTED_FEATURES + ["experimental"],
    visibility = _EXPERIMENTAL_CLIENTS,
)