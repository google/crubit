# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A wrapper rule that builds its target with a `--per_file_copt` and header modules."""

# A flag that `.pcm` header modules record, and that Clang rejects a mismatch of (it compares the
# language options of a `.pcm` with the current compilation's). Bazel applies it to the `.pcm`
# compiles of the deps; the bindings action must get it too, or it can't load them.
PER_FILE_COPT = ".@-fno-sized-deallocation"

def _per_file_copt_transition_impl(settings, _attr):
    return {
        "//command_line_option:per_file_copt": settings["//command_line_option:per_file_copt"] + [PER_FILE_COPT],
        "//rs_bindings_from_cc/bazel_support:use_header_modules": True,
    }

_per_file_copt_transition = transition(
    implementation = _per_file_copt_transition_impl,
    inputs = ["//command_line_option:per_file_copt"],
    outputs = [
        "//command_line_option:per_file_copt",
        "//rs_bindings_from_cc/bazel_support:use_header_modules",
    ],
)

def _with_per_file_copt_impl(ctx):
    return [DefaultInfo(files = ctx.attr.target[0][DefaultInfo].files)]

with_per_file_copt = rule(
    implementation = _with_per_file_copt_impl,
    doc = "Builds `target` with `PER_FILE_COPT` added to `--per_file_copt`, and header modules on.",
    attrs = {
        "target": attr.label(mandatory = True, cfg = _per_file_copt_transition),
    },
)
