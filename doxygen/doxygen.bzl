# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Rules for generating Doxygen documentation from C++ headers."""

load("@rules_cc//cc/common:cc_info.bzl", "CcInfo")
load("//cc_bindings_from_rs/bazel_support:providers.bzl", "CcBindingsFromRustInfo")

def _crubit_doxygen_impl(ctx):
    doxygen_bin = ctx.executable._doxygen
    doxyfile_template = ctx.file.doxyfile

    headers_depset = []
    for dep in ctx.attr.deps:
        if CcBindingsFromRustInfo in dep:
            headers_depset.append(depset(dep[CcBindingsFromRustInfo].headers))
        elif CcInfo in dep:
            headers_depset.append(depset(dep[CcInfo].compilation_context.direct_public_headers))

    headers = depset(transitive = headers_depset)

    output_zip = ctx.actions.declare_file(ctx.label.name + ".zip")
    doxyfile = ctx.actions.declare_file(ctx.label.name + "_Doxyfile")

    tmp_out_dir = ctx.label.name + "_html"

    input_paths = [f.path for f in headers.to_list()]
    include_paths = [
        "third_party",
        ".",
        ctx.bin_dir.path,
    ]

    content = [
        "@INCLUDE = %s" % doxyfile_template.path,
        "OUTPUT_DIRECTORY = .",
        "HTML_OUTPUT = %s" % tmp_out_dir,
        "INPUT = %s" % " ".join(input_paths),
        "INCLUDE_PATH = %s" % " ".join(include_paths),
        "STRIP_FROM_PATH = %s . third_party" % ctx.bin_dir.path,
    ]

    ctx.actions.write(
        output = doxyfile,
        content = "\n".join(content) + "\n",
    )

    # Let Doxygen find all the Doxyfiles and then run Doxygen binary
    cmd = """
    ROOT=$PWD
    mkdir -p {tmp_out}
    {doxygen} {doxyfile}
    cd {tmp_out} && zip -q -r $ROOT/{zip_out} .
    """.format(
        tmp_out = tmp_out_dir,
        doxygen = doxygen_bin.path,
        doxyfile = doxyfile.path,
        zip_out = output_zip.path,
    )

    ctx.actions.run_shell(
        inputs = depset([doxyfile_template, doxyfile], transitive = [headers]),
        outputs = [output_zip],
        tools = [doxygen_bin],
        command = cmd,
        mnemonic = "CrubitDoxygen",
        progress_message = "Generating Doxygen for %s" % ctx.label.name,
    )

    return [DefaultInfo(files = depset([output_zip]))]

crubit_doxygen = rule(
    implementation = _crubit_doxygen_impl,
    attrs = {
        "deps": attr.label_list(
            doc = "Targets to collect headers from.",
            providers = [[CcInfo], [CcBindingsFromRustInfo]],
        ),
        "doxyfile": attr.label(
            doc = "Doxyfile template file",
            allow_single_file = True,
            mandatory = True,
        ),
        "_doxygen": attr.label(
            default = "@doxygen//:doxygen",
            executable = True,
            allow_single_file = True,
            cfg = "exec",
        ),
    },
)
