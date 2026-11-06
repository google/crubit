# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Rule for embedding data files into a C++ FileToc library."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")

_EMBED_SCRIPT = """
import hashlib, os, sys
cc_path, h_path, base = sys.argv[1], sys.argv[2], sys.argv[3]
srcs = sys.argv[4:]
guard = "__STRUCT_FILE_TOC_" + "".join(c if c.isalnum() else "_" for c in base) + "_"
with open(h_path, "w", encoding="utf-8") as h:
    h.write("#ifndef CRUBIT_BAZEL_FILE_TOC_DEFINED_\\n")
    h.write("#define CRUBIT_BAZEL_FILE_TOC_DEFINED_\\n")
    h.write("#include <stddef.h>\\n")
    h.write("struct FileToc {\\n")
    h.write("  const char* name;\\n")
    h.write("  const char* data;\\n")
    h.write("  size_t size;\\n")
    h.write("  unsigned char md5digest[16];\\n")
    h.write("};\\n")
    h.write("#endif\\n")
    h.write("#ifndef " + guard + "\\n")
    h.write("#define " + guard + "\\n")
    h.write("const struct FileToc* " + base + "_create();\\n")
    h.write("size_t " + base + "_size();\\n")
    h.write("#endif\\n")
with open(cc_path, "w", encoding="utf-8") as cc:
    cc.write("#include <stddef.h>\\n")
    cc.write("#ifndef CRUBIT_BAZEL_FILE_TOC_DEFINED_\\n")
    cc.write("#define CRUBIT_BAZEL_FILE_TOC_DEFINED_\\n")
    cc.write("struct FileToc {\\n")
    cc.write("  const char* name;\\n")
    cc.write("  const char* data;\\n")
    cc.write("  size_t size;\\n")
    cc.write("  unsigned char md5digest[16];\\n")
    cc.write("};\\n")
    cc.write("#endif\\n")
    entries = []
    for idx, src in enumerate(srcs):
        raw = open(src, "rb").read()
        digest = hashlib.md5(raw).digest()
        arr_bytes = ", ".join(f"0x{b:02x}" for b in (raw + b"\\x00"))
        md5_bytes = ", ".join(f"0x{b:02x}" for b in digest)
        cc.write(f"static const unsigned char data_{idx}[] = {{{arr_bytes}}};\\n")
        entries.append((os.path.basename(src), idx, len(raw), md5_bytes))
    cc.write(f"static const struct FileToc toc[{max(1, len(entries)) + 1}] = {{\\n")
    for name, idx, size, md5_bytes in entries:
        cc.write(
            f'  {{"{name}", reinterpret_cast<const char*>(data_{idx}), {size}ULL, {{{md5_bytes}}}}},\\n'
        )
    cc.write("  {nullptr, nullptr, 0, {}}\\n};\\n")
    cc.write(f"const struct FileToc* {base}_create() {{ return toc; }}\\n")
    cc.write(f"size_t {base}_size() {{ return {len(entries)}ULL; }}\\n")
"""

def _cc_embed_data_impl(ctx):
    script = ctx.actions.declare_file(ctx.label.name + "_embed_script.py")
    ctx.actions.write(output = script, content = _EMBED_SCRIPT)
    args = ctx.actions.args()
    args.add(script)
    args.add(ctx.outputs.cc_out)
    args.add(ctx.outputs.h_out)
    args.add(ctx.attr.base)
    args.add_all(ctx.files.srcs)
    ctx.actions.run_shell(
        inputs = depset(ctx.files.srcs + [script]),
        outputs = [ctx.outputs.cc_out, ctx.outputs.h_out],
        command = 'python3 "$@"',
        arguments = [args],
        mnemonic = "CcEmbedData",
    )
    return [DefaultInfo(files = depset([ctx.outputs.cc_out, ctx.outputs.h_out]))]

_cc_embed_data_gen = rule(
    implementation = _cc_embed_data_impl,
    attrs = {
        "srcs": attr.label_list(allow_files = True, mandatory = True),
        "cc_out": attr.output(mandatory = True),
        "h_out": attr.output(mandatory = True),
        "base": attr.string(mandatory = True),
    },
)

def cc_embed_data(name, srcs, outs, visibility = None, **kwargs):
    """Embeds `srcs` into a `FileToc` C++ library compatible with `filewrapper`."""
    cc_outs = [o for o in outs if o.endswith(".cc")]
    h_outs = [o for o in outs if o.endswith(".h")]
    if len(cc_outs) != 1 or len(h_outs) != 1:
        fail("cc_embed_data expects exactly one .cc and one .h in outs, got: %s" % outs)
    cc_out = cc_outs[0]
    h_out = h_outs[0]
    base = h_out.rsplit("/", 1)[-1].removesuffix(".h")

    _cc_embed_data_gen(
        name = name + "_embed_gen",
        srcs = srcs,
        cc_out = cc_out,
        h_out = h_out,
        base = base,
        visibility = ["//visibility:private"],
    )

    cc_library(
        name = name,
        srcs = [cc_out],
        hdrs = [h_out],
        visibility = visibility,
        **kwargs
    )
