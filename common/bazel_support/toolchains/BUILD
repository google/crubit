# TOOLCHAINS
# ==========
#
# This BUILD file containts all of the toolchains we import into MODULE.bazel.
# See https://bazel.build/extending/toolchains
#
# Do not add anything to this file except additional toolchain() calls.

package(default_applicable_licenses = ["//:license"])

toolchain(
    name = "crubit_on_demand",
    target_settings = [
        "//common/bazel_support:use_actual_rs_bindings_from_cc_setting",
    ],
    toolchain = "//common/bazel_support:crubit_on_demand_impl",
    toolchain_type = "//rs_bindings_from_cc/bazel_support:toolchain_type",
)

toolchain(
    name = "crubit_fake",
    target_settings = [
        "//common/bazel_support:use_fake_rs_bindings_from_cc_setting",
    ],
    toolchain = "//common/bazel_support:crubit_fake_impl",
    toolchain_type = "//rs_bindings_from_cc/bazel_support:toolchain_type",
)
