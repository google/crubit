# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Macros to compile a Rust library or test with embedded C++ payload code."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_library")

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl", "cc_bindings_from_rust_library_config")
load("//support/extract_cpp_from_rust:extract_cpp.bzl", "extract_cpp")

def _setup_embedded_cpp_targets(name, srcs, deps_of_cc_library, kwargs, default_testonly = False):
    cc_lib_name = name + "_extracted_cc"
    extracted_header = cc_lib_name + ".h"

    target = "//{}:{}".format(native.package_name(), name)

    helper_kwargs = {}
    if "compatible_with" in kwargs:
        helper_kwargs["compatible_with"] = kwargs["compatible_with"]
    if "restricted_to" in kwargs:
        helper_kwargs["restricted_to"] = kwargs["restricted_to"]
    if "testonly" in kwargs:
        helper_kwargs["testonly"] = kwargs["testonly"]
    elif default_testonly:
        helper_kwargs["testonly"] = True

    extract_cpp(
        name = cc_lib_name + "_extract_cpp",
        srcs = srcs,
        out = extracted_header,
        target = target,
        **helper_kwargs
    )

    cc_library(
        name = cc_lib_name,
        hdrs = [extracted_header],
        deps = deps_of_cc_library,
        aspect_hints = ["//features:supported"],
        **helper_kwargs
    )

    rust_bindings_name = name + "_rust_bindings"
    rust_api_from_cpp(
        name = rust_bindings_name,
        cpp_target = ":" + cc_lib_name,
        **helper_kwargs
    )

    bindings_label = ":" + rust_bindings_name
    user_aliases = kwargs.pop("aliases", {})
    merged_aliases = dict(user_aliases)
    merged_aliases.update({
        bindings_label: "inline_cpp_generated_bindings",
        ":" + cc_lib_name: "inline_cpp_generated_bindings",
    })

    return target, extracted_header, bindings_label, merged_aliases, helper_kwargs

def rust_library_with_embedded_cpp(name, srcs, deps = [], deps_of_cc_library = [], **kwargs):
    """Compiles a Rust library containing `global_cpp!` or `inline_cpp!` blocks.

    Args:
      name: The name of the `rust_library` target.
      srcs: The Rust source files containing embedded C++.
      deps: Rust dependencies for the `rust_library`.
      deps_of_cc_library: C++ dependencies for the extracted `cc_library`.
      **kwargs: Additional arguments forwarded to `rust_library`.
    """
    target, extracted_header, bindings_label, merged_aliases, helper_kwargs = _setup_embedded_cpp_targets(
        name = name,
        srcs = srcs,
        deps_of_cc_library = deps_of_cc_library,
        kwargs = kwargs,
    )

    cc_bindings_from_rust_library_config(
        name = name + "_cc_bindings_from_rust_config",
        extra_cc_hdrs = [extracted_header],
        **helper_kwargs
    )

    rust_library(
        name = name,
        srcs = srcs,
        deps = deps + [bindings_label],
        aspect_hints = kwargs.pop("aspect_hints", ["//features:supported"]) + [
            ":" + name + "_cc_bindings_from_rust_config",
        ],
        aliases = merged_aliases,
        proc_macro_deps = kwargs.pop("proc_macro_deps", []) + [
            "//support/extract_cpp_from_rust:inline_cpp_macro",
        ],
        rustc_env = dict(kwargs.pop("rustc_env", {}), CRUBIT_TARGET = target),
        **kwargs
    )

def rust_test_with_embedded_cpp(name, srcs, deps = [], deps_of_cc_library = [], **kwargs):
    """Compiles a Rust test containing `global_cpp!` or `inline_cpp!` blocks.

    Args:
      name: The name of the `rust_test` target.
      srcs: The Rust source files containing embedded C++.
      deps: Rust dependencies for the `rust_test`.
      deps_of_cc_library: C++ dependencies for the extracted `cc_library`.
      **kwargs: Additional arguments forwarded to `rust_test`.
    """
    target, _extracted_header, bindings_label, merged_aliases, _helper_kwargs = _setup_embedded_cpp_targets(
        name = name,
        srcs = srcs,
        deps_of_cc_library = deps_of_cc_library,
        kwargs = kwargs,
        default_testonly = True,
    )

    rust_test(
        name = name,
        srcs = srcs,
        deps = deps + [bindings_label],
        aliases = merged_aliases,
        proc_macro_deps = kwargs.pop("proc_macro_deps", []) + [
            "//support/extract_cpp_from_rust:inline_cpp_macro",
        ],
        rustc_env = dict(kwargs.pop("rustc_env", {}), CRUBIT_TARGET = target),
        **kwargs
    )
