# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Test-only bindings generation macros."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_flavor_transition",
)
load(
    "//rs_bindings_from_cc/bazel_support:providers.bzl",
    "GeneratedBindingsInfo",
)
load(
    "//rs_bindings_from_cc/bazel_support:rust_bindings_from_cc_aspect.bzl",
    "rust_bindings_from_cc_aspect",
)

def crubit_test_cc_library(name, **kwargs):
    """A wrapper for cc_library in Crubit integration tests.

    This is equivalent to cc_library, but it sets the default aspect_hints to `:experimental`.
    """
    kwargs.setdefault("aspect_hints", ["//features:experimental"])
    cc_library(
        name = name,
        **kwargs
    )
    outs_name = name + ".outs"
    write_crubit_outs(
        name = outs_name,
        cc_library = ":" + name,
        # note: this cannot be just + "_rust_api.rs", etc., because then two different actions would
        # produce the same file.
        outs = [
            "generated_bindings/" + name + "_rust_api.rs",
            "generated_bindings/" + name + "_rust_api_impl.cc",
        ],
        **_get_forwarded_kwargs(kwargs)
    )

def _get_forwarded_kwargs(
        kwargs,
        attrs = [
            # These are built-in starlark attributes for every rule. Take from Bazel's Java
            # implementation: analysis.BaseRuleClasses.commonCoreAndStarlarkAttributes.
            "visibility",
            "transitive_configs",
            "deprecation",
            "tags",
            "generator_name",
            "generator_function",
            "generator_location",
            "testonly",
            "compatible_with",
            "restricted_to",
            "applicable_licenses",
            "aspect_hints",
        ]):
    forwarded_kwargs = {}
    for attr in attrs:
        if attr in kwargs:
            forwarded_kwargs[attr] = kwargs[attr]
    return forwarded_kwargs

def _write_crubit_outs_impl(ctx):
    cc_library = ctx.attr.cc_library[0]
    if GeneratedBindingsInfo not in cc_library:
        # If there are no headers, or the library doesn't enable Crubit, we skip Crubit.
        # Just use empty files for this.
        for out in ctx.outputs.outs:
            ctx.actions.write(output = out, content = "")
        return []

    bindings = cc_library[GeneratedBindingsInfo]
    for out in ctx.outputs.outs:
        if out.extension == "cc":
            f = bindings.cc_file
        elif out.extension == "rs":
            f = bindings.rust_file
        elif out.extension == "json":
            f = bindings.namespaces_file
        else:
            fail("Unknown file extension; can't infer which output to copy out: " + out)
        ctx.actions.symlink(output = out, target_file = f)
    return []

write_crubit_outs = rule(
    attrs = {
        "cc_library": attr.label(
            providers = [CcInfo],
            aspects = [rust_bindings_from_cc_aspect],
            cfg = crubit_flavor_transition,
        ),
        "outs": attr.output_list(),
    },
    implementation = _write_crubit_outs_impl,
)
