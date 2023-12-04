load(
    "//rs_bindings_from_cc/bazel_support:crubit_feature_hint.bzl",
    "crubit_feature_hint",
)

_SUPPORTED_FEATURES = [
    "supported",
]

crubit_feature_hint(
    name = "supported",
    crubit_features = _SUPPORTED_FEATURES,
    visibility = ["//visibility:public"],
)

crubit_feature_hint(
    name = "experimental",
    crubit_features = _SUPPORTED_FEATURES + ["experimental"],
    visibility = [
        "//:__subpackages__",
        "//security/ise_cloud/projects/safe_json_parsing:__subpackages__",
        "//security/ise_memory_safety/safe_json_parsing:__subpackages__",
    ],
)
