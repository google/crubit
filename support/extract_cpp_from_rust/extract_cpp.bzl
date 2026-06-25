# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Bazel integration to scrape `global_cpp!` blocks into a generated C++ header."""

def _extract_cpp_impl(ctx):
    out_file = ctx.outputs.out

    args = ctx.actions.args()
    args.add("--out", out_file.path)
    args.add("--target", ctx.attr.target)
    args.add_all("--srcs", ctx.files.srcs)

    ctx.actions.run(
        outputs = [out_file],
        inputs = ctx.files.srcs,
        executable = ctx.executable._extractor,
        arguments = [args],
        mnemonic = "ExtractCppFromRust",
        progress_message = "Extracting C++ payloads from Rust files...",
    )

    return [DefaultInfo(files = depset([out_file]))]

extract_cpp = rule(
    implementation = _extract_cpp_impl,
    attrs = {
        "srcs": attr.label_list(
            allow_files = [".rs"],
            doc = "Rust source files to extract C++ payloads from",
        ),
        "target": attr.string(
            doc = "The label of the parent target",
            mandatory = True,
        ),
        "out": attr.output(doc = "The output C++ header file"),
        "_extractor": attr.label(
            default = Label("//support/extract_cpp_from_rust:extract_cpp_from_rust"),
            executable = True,
            cfg = "exec",
        ),
    },
)
