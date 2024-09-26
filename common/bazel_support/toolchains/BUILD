# TOOLCHAINS
# ==========
#
# This BUILD file containts all of the toolchains we import into MODULE.bazel.
# See https://bazel.build/extending/toolchains
#
# Do not add anything to this file except additional toolchain() calls.

load("//common/bazel_support:grte_versions.bzl", "CPU", "CPU_GRTE_VERSIONS", "GRTE")

package(default_applicable_licenses = ["//:license"])

# cc_bindings_from_rs
# ===================

toolchain(
    name = "cc_bindings_from_rs_on_demand",
    target_settings = [
    ],
    toolchain = "//common/bazel_support:cc_bindings_from_rs_on_demand_impl",
    toolchain_type = "//cc_bindings_from_rs/bazel_support:toolchain_type",
)

# rs_bindings_from_cc
# ===================

toolchain(
    name = "rs_bindings_from_cc_on_demand",
    target_settings = [
        "//common/bazel_support:use_actual_rs_bindings_from_cc_setting",
    ],
    toolchain = "//common/bazel_support:rs_bindings_from_cc_on_demand_impl",
    toolchain_type = "//rs_bindings_from_cc/bazel_support:toolchain_type",
)

toolchain(
    name = "rs_bindings_from_cc_fake",
    target_settings = [
        "//common/bazel_support:use_fake_rs_bindings_from_cc_setting",
    ],
    toolchain = "//common/bazel_support:rs_bindings_from_cc_fake_impl",
    toolchain_type = "//rs_bindings_from_cc/bazel_support:toolchain_type",
)

[
    toolchain(
        name = "grte_{grte_name}_cpu_{cpu_name}".format(
            cpu_name = cpu_name,
            grte_name = grte_name,
        ),
        target_settings = [
            cpu_config_setting,
            grte_config_setting,
        ],
        toolchain = "//common/bazel_support:grte_{grte_name}_cpu_{cpu_name}_toolchain".format(
            cpu_name = cpu_name,
            grte_name = grte_name,
        ),
        toolchain_type = "//rs_bindings_from_cc/bazel_support:grte_toolchain_type",
    )
    for (cpu_name, cpu_config_setting) in CPU.items()
    for (grte_name, grte_config_setting) in GRTE.items()
    if grte_name in CPU_GRTE_VERSIONS[cpu_name]
]
