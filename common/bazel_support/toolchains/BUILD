# TOOLCHAINS
# ==========
#
# This BUILD file containts all of the toolchains we import into MODULE.bazel.
# See https://bazel.build/extending/toolchains
#
# Do not add anything to this file except additional toolchain() calls.

package(default_applicable_licenses = ["//:license"])

# 1. PROD PLATFORMS (Linux)
_PROD_PLATFORMS = {
    "linux": [
        "@platforms//os:linux",
    ],
}

# 2. NON-PROD PLATFORMS (Android / iOS)
_NON_PROD_PLATFORMS = {
    "ios_aarch64": [
        "@platforms//cpu:aarch64",
        "@platforms//os:ios",
        "//third_party/bazel_rules/apple_support/constraints:device",
    ],
    "ios_aarch64_sim": [
        "@platforms//cpu:aarch64",
        "@platforms//os:ios",
        "//third_party/bazel_rules/apple_support/constraints:simulator",
    ],
    "ios_x86_64": [
        "@platforms//cpu:x86_64",
        "@platforms//os:ios",
        "//third_party/bazel_rules/apple_support/constraints:simulator",
    ],
    "android_x86_64": [
        "@platforms//cpu:x86_64",
        "@platforms//os:android",
    ],
    "android_arm64_v8a": [
        "@platforms//cpu:arm64",
        "@platforms//os:android",
    ],
    "android_armv7": [
        "@platforms//cpu:armv7",
        "@platforms//os:android",
    ],
    "android_x86": [
        "@platforms//cpu:x86_32",
        "@platforms//os:android",
    ],
    "android_riscv64": [
        "@platforms//cpu:riscv64",
        "@platforms//os:android",
    ],
}

# cc_bindings_from_rs
# ====================

[[
    toolchain(
        name = "cc_bindings_from_rs_on_demand_%s" % platform,
        target_compatible_with = _PROD_PLATFORMS[platform],
        target_settings = [
        ],
        toolchain = "//common/bazel_support:cc_bindings_from_rs_on_demand_impl",
        toolchain_type = "//cc_bindings_from_rs/bazel_support:toolchain_type",
    ),
] for platform in _PROD_PLATFORMS]

[[
    toolchain(
        name = "cc_bindings_from_rs_on_demand_%s" % platform,
        target_compatible_with = _NON_PROD_PLATFORMS[platform],
        target_settings = [
        ],
        toolchain = "//common/bazel_support:cc_bindings_from_rs_on_demand_impl",
        toolchain_type = "//cc_bindings_from_rs/bazel_support:toolchain_type",
    ),
] for platform in _NON_PROD_PLATFORMS]

# rs_bindings_from_cc
# ====================
# This direction is currently only available for prod platforms.

[[
    toolchain(
        name = "rs_bindings_from_cc_on_demand_%s" % platform,
        target_compatible_with = _PROD_PLATFORMS[platform],
        target_settings = [
            "//common/bazel_support:use_actual_rs_bindings_from_cc_setting",
        ],
        toolchain = "//common/bazel_support:rs_bindings_from_cc_on_demand_impl",
        toolchain_type = "//rs_bindings_from_cc/bazel_support:toolchain_type",
    ),
] for platform in _PROD_PLATFORMS]

toolchain(
    name = "rs_bindings_from_cc_fake",
    target_settings = [
        "//common/bazel_support:use_fake_rs_bindings_from_cc_setting",
    ],
    toolchain = "//common/bazel_support:rs_bindings_from_cc_fake_impl",
    toolchain_type = "//rs_bindings_from_cc/bazel_support:toolchain_type",
)
