# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Wrapper macros needed for Google-internal purposes."""

load("@bazel_skylib//lib:unittest.bzl", "analysistest")
load("@rules_cc//cc:cc_binary.bzl", "cc_binary")
load("@rules_cc//cc:cc_test.bzl", "cc_test")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")
load("@rules_shell//shell:sh_test.bzl", "sh_test")

def crubit_rust_test(**kwargs):
    """Wrapper around `rust_test` that provides the Rust stdlib files and `LD_LIBRARY_PATH`.

    Args:
        **kwargs: Arguments forwarded to `rust_test`.
    """
    data = list(kwargs.pop("data", default = []))
    std_libs = "@rules_rust//rust/toolchain:current_rust_stdlib_files"
    if std_libs not in data:
        data.append(std_libs)
    env = dict(kwargs.pop("env", default = {}))
    env.setdefault(
        "LD_LIBRARY_PATH",
        "../+crubit_toolchains+rust_linux_x86_64__x86_64-unknown-linux-gnu__nightly_tools/rust_toolchain/lib/rustlib/x86_64-unknown-linux-gnu/lib",
    )
    rust_test(data = data, env = env, **kwargs)

def crubit_rust_binary(**kwargs):
    # Ignore the `crubit_dep` argument, as the OSS version of
    # `crubit_rust_binary` never inherits a dep on Crubit.
    kwargs.pop("crubit_dep", default = False)
    rust_binary(**kwargs)

def crubit_cc_test(**kwargs):
    # Ignore the `platforms` argument, as the OSS version of
    # `crubit_cc_test` runs on the host platform.
    kwargs.pop("platforms", default = [])
    cc_test(**kwargs)

crubit_sh_test = sh_test
crubit_cc_binary = cc_binary

def _crubit_flavor_transition_impl(_settings, _attr):
    return {}

crubit_flavor_transition = transition(
    implementation = _crubit_flavor_transition_impl,
    inputs = [],
    outputs = [],
)

def _crubit_golden_flavor_transition_impl(settings, attr):
    flags = _crubit_flavor_transition_impl(settings, attr)
    flags["//common/bazel_support:is_golden_test"] = True
    return flags

crubit_golden_flavor_transition = transition(
    implementation = _crubit_golden_flavor_transition_impl,
    inputs = [],
    outputs = ["//common/bazel_support:is_golden_test"],
)
crubit_multiplatform_golden_transition = crubit_golden_flavor_transition
crubit_make_analysis_test = analysistest.make

def crubit_ios_device_test(**_kwargs):
    pass

CRUBIT_ANDROID_ABI_TIERS = {}
CRUBIT_ANDROID_PLATFORMS = {}
CRUBIT_TAGS_MAPPING = {}
CRUBIT_IOS_SIM_PLATFORMS = {}
CRUBIT_IOS_SIM_TAGS_MAPPING = {}
CRUBIT_MACOS_PLATFORMS = {}
CRUBIT_MACOS_TAGS_MAPPING = {}

# Placeholder for an internal attribute that asks Crubit's own tests to be
# built with debug assertions.  OSS builds neither enable nor test for debug
# assertions, so here it adds no attributes.
ENABLE_CRUBIT_ASSERTIONS_ATTR = {}
