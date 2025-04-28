"""Internal features intended for internal unit and integration tests only.

Do not use these in production code.
"""

load(
    "//features:crubit_feature_hint.bzl",
    "crubit_feature_hint",
)

# A feature set with a stable expansion, only for use in Bazel unit tests.
crubit_feature_hint(
    name = "testonly_supported",
    crubit_features = ["supported"],
    visibility = ["//:__subpackages__"],
)

# A feature set with a stable expansion, only for use in Bazel unit tests.
crubit_feature_hint(
    name = "testonly_experimental",
    crubit_features = ["experimental"],
    visibility = ["//:__subpackages__"],
)
