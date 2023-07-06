load("@rules_license//rules:license.bzl", "license")
load(
    "//rs_bindings_from_cc/bazel_support:crubit_feature_hint.bzl",
    "crubit_feature_hint",
)

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//visibility:private"],
)

license(
    name = "license",
    package_name = "crubit",
)

licenses(["notice"])

exports_files(["LICENSE"])

_SUPPORTED_FEATURES = [
    "supported",
]

# Aspect hints

# Enable all Crubit features.
# TODO(jeanpierreda): Write compatibility doc to link here, guiding how to support Crubit / when to use
# `:supported`.
crubit_feature_hint(
    name = "supported",
    crubit_features = _SUPPORTED_FEATURES,
    visibility = ["//visibility:public"],
)

# Enable experimental/unstable crubit features. Also includes `:supported`.
crubit_feature_hint(
    name = "experimental",
    crubit_features = _SUPPORTED_FEATURES + ["experimental"],
    visibility = [
        "//:__subpackages__",
        "//security/ise_cloud/projects/safe_json_parsing:__subpackages__",
    ],
)
