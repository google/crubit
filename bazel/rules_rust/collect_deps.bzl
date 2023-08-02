# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
""""A helper function to gather providers from rust_bindings_from_cc_aspect."""

load(
    "@@//rs_bindings_from_cc/bazel_support:providers.bzl",
    "RustBindingsFromCcInfo",
)

# buildifier: disable=bzl-visibility
load(
    "@rules_rust//rust/private:providers.bzl",
    "BuildInfo",
    "CrateInfo",
    "DepInfo",
    "DepVariantInfo",
)

def collect_transformed_deps(ctx):
    """Creates DepVariantInfos from all dependencies and from the rust_bindings_from_cc_aspect.

    Args:
        ctx (ctx): The target's context object.

    Returns:
        list[DepVariantInfo]: a list of DepVariantInfos that this target depends on.
    """
    deps = [DepVariantInfo(
        crate_info = dep[CrateInfo] if CrateInfo in dep else None,
        dep_info = dep[DepInfo] if DepInfo in dep else None,
        build_info = dep[BuildInfo] if BuildInfo in dep else None,
        cc_info = dep[CcInfo] if CcInfo in dep else None,
    ) for dep in ctx.attr.deps]

    for dep in ctx.attr.deps:
        if RustBindingsFromCcInfo in dep:
            deps.append(dep[RustBindingsFromCcInfo].dep_variant_info)
            deps.append(
                DepVariantInfo(
                    cc_info = dep[RustBindingsFromCcInfo].cc_info,
                    crate_info = None,
                    dep_info = None,
                    build_info = None,
                ),
            )

    if hasattr(ctx.attr, "cc_deps"):
        for dep in ctx.attr.cc_deps:
            if RustBindingsFromCcInfo in dep:
                deps.append(dep[RustBindingsFromCcInfo].dep_variant_info)
                deps.append(
                    DepVariantInfo(
                        cc_info = dep[RustBindingsFromCcInfo].cc_info,
                        crate_info = None,
                        dep_info = None,
                        build_info = None,
                    ),
                )

    return deps

def get_cc_import_namespace_variable(ctx):
    """Returns a dictionary containing the CC_IMPORT_NAMESPACES environment variable.

    Args:
        ctx (ctx): The target's context object.

    Returns:
        dict {String: String}: A dictionary with a single "CC_IMPORT_NAMESPACES" key, whose value
            are json encoded paths to the C++ dependencies' namespaces files.
    """
    namespace_json_filepaths = []
    if hasattr(ctx.attr, "cc_deps"):
        namespace_json_filepaths.extend([
            dep[RustBindingsFromCcInfo].namespaces.path
            for dep in ctx.attr.cc_deps
            if RustBindingsFromCcInfo in dep
        ])

    cc_import_namespaces_var_name = "CC_IMPORT_NAMESPACES"

    # For `rust_test` we need to collect the json namespace files listed in `cc_deps`, as well as
    # the json namespace files listed in the underlying `crate`'s `cc_deps`.
    crate = getattr(ctx.attr, "crate", None)
    if crate:
        if CrateInfo not in crate:
            fail("A rust target must provide a CrateInfo")

        if cc_import_namespaces_var_name in crate[CrateInfo].rustc_env:
            namespace_json_filepaths.extend(json.decode(crate[CrateInfo].rustc_env[cc_import_namespaces_var_name]))

    if namespace_json_filepaths:
        return {
            cc_import_namespaces_var_name: json.encode(namespace_json_filepaths),
        }
    return {}

def get_namespace_json_files(ctx):
    """Returns the C++ dependencies' namespace json files.

    Args:
        ctx (ctx): The target's context object.

    Returns:
        list[Artifact]: The C++ dependencies' namespace json files.
    """
    if hasattr(ctx.attr, "cc_deps"):
        return [
            dep[RustBindingsFromCcInfo].namespaces
            for dep in ctx.attr.cc_deps
            if RustBindingsFromCcInfo in dep
        ]
    return []
