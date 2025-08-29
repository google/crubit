# TOOLCHAINS
# ==========
#
# This BUILD file containts all of the toolchains we import into MODULE.bazel.
# See https://bazel.build/extending/toolchains
#
# Do not add anything to this file except additional toolchain() calls.

package(default_applicable_licenses = ["//:license"])

[[
    # cc_bindings_from_rs
    # ===================
    toolchain(
        name = "cc_bindings_from_rs_on_demand_" + os,
        target_compatible_with = ["@platforms//os:" + os],
        target_settings = [
        ],
        toolchain = "//common/bazel_support:cc_bindings_from_rs_on_demand_impl",
        toolchain_type = "//cc_bindings_from_rs/bazel_support:toolchain_type",
    ),

    # rs_bindings_from_cc
    # ===================
    toolchain(
        name = "rs_bindings_from_cc_on_demand_" + os,
        target_compatible_with = ["@platforms//os:" + os],
        target_settings = [
            "//common/bazel_support:use_actual_rs_bindings_from_cc_setting",
        ],
        toolchain = "//common/bazel_support:rs_bindings_from_cc_on_demand_impl",
        toolchain_type = "//rs_bindings_from_cc/bazel_support:toolchain_type",
    ),
] for os in [
    # OSes we at least attempt to support.
    # Add an OS to this list if Crubit either works, or at least has an
    # error message more legible than random internal build issues.
    "linux",
    "ios",
    "macos",
]]

toolchain(
    name = "rs_bindings_from_cc_fake",
    target_settings = [
        "//common/bazel_support:use_fake_rs_bindings_from_cc_setting",
    ],
    toolchain = "//common/bazel_support:rs_bindings_from_cc_fake_impl",
    toolchain_type = "//rs_bindings_from_cc/bazel_support:toolchain_type",
)
