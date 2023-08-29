# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Test-only bindings generation macros."""

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
    kwargs.setdefault("aspect_hints", ["//:experimental"])
    native.cc_library(
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
    )

def _write_crubit_outs_impl(ctx):
    cc_library = ctx.attr.cc_library[0]
    if not GeneratedBindingsInfo in cc_library:
        fail("Bindings were not generated for the given cc_library.")
    bindings = cc_library[GeneratedBindingsInfo]
    symlinks = []
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
        symlinks.append(out)
    return []

write_crubit_outs = rule(
    attrs = {
        "cc_library": attr.label(
            providers = [CcInfo],
            aspects = [rust_bindings_from_cc_aspect],
            cfg = crubit_flavor_transition,
        ),
        "outs": attr.output_list(),
        "_allowlist_function_transition": attr.label(
            default = "@bazel_tools//tools/allowlists/function_transition_allowlist",
        ),
    },
    implementation = _write_crubit_outs_impl,
)
