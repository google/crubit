load(
    "@bazel_skylib//rules:common_settings.bzl",
    "string_list_flag",
)
load("//rs_bindings_from_cc/bazel_support:toolchain.bzl", "rs_bindings_from_cc_toolchain")

package(
    default_applicable_licenses = ["//:license"],
    default_visibility = ["//common/bazel_support:__subpackages__"],
)

# FLAGS
# =====
#
# The following flags can be used by either direction of interop to control behavior of the tool.

string_list_flag(
    name = "globally_enabled_features",
    build_setting_default = [],
    visibility = ["//visibility:public"],
)

# TOOLCHAINS
# ==========
#
# The following are all of the Crubit toolchain implementations.
# The actual registration happens in MODULE.bazel and toolchains/BUILD.

config_setting(
    name = "use_fake_rs_bindings_from_cc_setting",
    flag_values = {
        "//rs_bindings_from_cc/bazel_support:use_actual_bindings_generator": "False",
    },
)

config_setting(
    name = "use_actual_rs_bindings_from_cc_setting",
    flag_values = {
        "//rs_bindings_from_cc/bazel_support:use_actual_bindings_generator": "True",
    },
)

rs_bindings_from_cc_toolchain(
    name = "crubit_on_demand_impl",
    binary = "//rs_bindings_from_cc:rs_bindings_from_cc",
    builtin_headers = [
        "//llvm/llvm-project/clang:builtin_headers_filegroup",
        "//llvm/llvm-project/compiler-rt:fuzzer_filegroup",
        "//llvm/llvm-project/compiler-rt:sanitizer_filegroup",
    ],
    is_on_demand = True,
    stl_headers = ["//nowhere/llvm:cc-std-headers"],
)

rs_bindings_from_cc_toolchain(
    name = "crubit_fake_impl",
    binary = "//rs_bindings_from_cc/bazel_support:fake_rust_bindings_from_cc",
    builtin_headers = [],
    is_on_demand = True,
    stl_headers = [],
)
