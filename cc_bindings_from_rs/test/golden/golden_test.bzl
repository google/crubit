# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A rule that generates bindings source files for a given Rust library."""

load(
    "@rules_rust//rust:defs.bzl",
    rust_library_rule = "rust_library",
)
load("@rules_rust//rust/private:providers.bzl", "CrateInfo")  # buildifier: disable=bzl-visibility
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_cli_flag_aspect_hint.bzl",
    "cc_bindings_from_rust_cli_flag",
)
load("//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_library_config_aspect_hint.bzl", "cc_bindings_from_rust_library_config")
load(
    "//cc_bindings_from_rs/bazel_support:cc_bindings_from_rust_rule.bzl",
    "cc_bindings_from_rust_aspect",
)
load(
    "//cc_bindings_from_rs/bazel_support:providers.bzl",
    "GeneratedBindingsInfo",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_flavor_transition",
)

def _generate_bindings_impl(ctx):
    rust_library = ctx.attr.rust_library[0]
    if not GeneratedBindingsInfo in rust_library:
        fail("Bindings were not generated for the given rust_library.")
    bindings = rust_library[GeneratedBindingsInfo]
    return OutputGroupInfo(
        h_file = [bindings.h_file],
        rust_file = [bindings.rust_file],
    )

_generate_bindings = rule(
    attrs = {
        "rust_library": attr.label(
            providers = [CrateInfo],
            aspects = [cc_bindings_from_rust_aspect],
            cfg = crubit_flavor_transition,
        ),
    },
    implementation = _generate_bindings_impl,
)

def golden_test(
        name,
        rust_library,
        tags = None,
        basename = None,
        golden_h = None,
        golden_rs = None):
    """Generates a golden test for `rust_library`.

    Args:
        name: The name of the golden test.
        rust_library: The Rust library whose outputs should be checked.
        tags: The test tags.
        basename: The name to use for generated files.
        golden_h: The generated C++ source code for the bindings.
        golden_rs: The generated Rust source code for the bindings.

    """
    if not basename:
        basename = name
    if not tags:
        tags = []
    tags.append("crubit_golden_test")

    bindings_name = basename + ".generated_bindings"

    # Disable thunk name mangling to avoid breaking tests.
    no_mangle_cli_flag = "no_thunk_name_mangling_" + rust_library
    cc_bindings_from_rust_cli_flag(
        name = no_mangle_cli_flag,
        flags = "--no-thunk-name-mangling",
    )

    # Since we have patched the rust_library name, we need to keep the original crate
    # name as the namespace name otherwise users get confused.
    top_level_namespace = "top_level_namespace" + rust_library
    cc_bindings_from_rust_library_config(
        name = top_level_namespace,
        namespace = rust_library,
    )
    args = {}
    for key, value in native.existing_rule(rust_library).items():
        if key != "kind" and key != "name" and value:
            args[key] = value
    patched_name = rust_library + "_golden"
    args["name"] = patched_name
    if "aspect_hints" in args:
        args["aspect_hints"] = list(args["aspect_hints"])
    else:
        args["aspect_hints"] = []
    args["aspect_hints"] += [":" + no_mangle_cli_flag, ":" + top_level_namespace]
    rust_library_rule(
        **args
    )

    _generate_bindings(
        name = bindings_name,
        rust_library = patched_name,
        testonly = True,
    )
    args = []
    data = ["//common:LICENSE_HEADER"]
    owned_files = []
    if golden_h:
        new_h = basename + ".h_file"
        native.filegroup(
            name = new_h,
            srcs = [bindings_name],
            output_group = "h_file",
            testonly = True,
        )
        args += [
            "$(location %s)" % golden_h,
            "$(location %s)" % new_h,
        ]
        data += [
            golden_h,
            new_h,
        ]
        owned_files.append(golden_h)

    if golden_rs:
        new_rs = basename + ".rs_file"
        native.filegroup(
            name = new_rs,
            srcs = [bindings_name],
            output_group = "rust_file",
            testonly = True,
        )
        args += [
            "$(location %s)" % golden_rs,
            "$(location %s)" % new_rs,
        ]
        data += [
            golden_rs,
            new_rs,
        ]
        owned_files.append(golden_rs)

    # Only actually generate the test if this is in //:
    # when copied over to a release directory, the header guards won't
    # match.
    if not native.package_name().startswith("third_party/crosstool/"):
        native.sh_test(
            name = name,
            srcs = ["//common:golden_test.sh"],
            args = args,
            data = data,
            tags = tags,
            testonly = True,
        )
    native.filegroup(
        name = basename + ".build_cleaner_optout",
        srcs = owned_files,
        tags = ["ignore_srcs"],
        visibility = ["//visibility:private"],
        testonly = True,
    )
