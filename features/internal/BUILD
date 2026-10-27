"""Internal features intended for internal unit and integration tests only.

Do not use these in production code.
"""

load(
    "//features:crubit_feature_hint.bzl",
    "crubit_feature_hint",
)

licenses(["notice"])

# A feature set with a stable expansion, only for use in Bazel unit tests.
crubit_feature_hint(
    name = "testonly_supported",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = ["supported"],
    visibility = ["//:__subpackages__"],
)

# A feature set with a stable expansion, only for use in Bazel unit tests.
crubit_feature_hint(
    name = "testonly_types",
    compatible_with = ["//buildenv/target:non_prod"],
    crubit_features = ["types"],
    visibility = ["//:__subpackages__"],
)
