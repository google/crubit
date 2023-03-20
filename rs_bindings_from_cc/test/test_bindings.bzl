# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Test-only bindings generation macros."""

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
    kwargs.setdefault("aspect_hints", ["//third_party/crubit:experimental"])
    native.cc_library(
        name = name,
        **kwargs
    )
    outs_name = name + ".outs"
    write_crubit_outs(
        name = outs_name,
        cc_library = ":" + name,
        outs = [name + "_rust_api.rs"],
    )

def _write_crubit_outs_impl(ctx):
    if not GeneratedBindingsInfo in ctx.attr.cc_library:
        fail("Bindings were not generated for the given cc_library.")
    bindings = ctx.attr.cc_library[GeneratedBindingsInfo]
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
        "cc_library": attr.label(providers = [CcInfo], aspects = [rust_bindings_from_cc_aspect]),
        "outs": attr.output_list(),
    },
    implementation = _write_crubit_outs_impl,
)
