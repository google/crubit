# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

def _llvm_source_fetch_impl(repository_ctx):
    commit = repository_ctx.attr.commit
    url = "https://github.com/llvm/llvm-project/archive/" + commit + ".tar.gz"

    print("Downloading LLVM source from " + url + "...")
    repository_ctx.download_and_extract(
        url = url,
        stripPrefix = "llvm-project-" + commit,
    )

    # Patch LLVM's config.bzl to avoid platform config errors on standard platforms.
    # See comments in //bazel:llvm_platforms_config.patch for details and precedents.
    print("Patching LLVM source...")
    repository_ctx.patch(
        Label("//bazel:llvm_platforms_config.patch"),
        strip = 1,
    )

    # Create an empty BUILD file at the root to make it a package,
    # so that files like WORKSPACE can be referenced as labels.
    repository_ctx.file("BUILD.bazel", "")

llvm_source_fetch = repository_rule(
    implementation = _llvm_source_fetch_impl,
    attrs = {
        "commit": attr.string(mandatory = True),
    },
)

def _host_c_library_impl(repository_ctx):
    repository_ctx.symlink(
        repository_ctx.attr.include_src,
        repository_ctx.attr.include_dst,
    )
    repository_ctx.file("BUILD.bazel", """
load("@rules_cc//cc:cc_library.bzl", "cc_library")

cc_library(
    name = "{target_name}",
    includes = ["include"],
    linkopts = ["{linkopt}"],
    visibility = ["//visibility:public"],
)
""".format(
        target_name = repository_ctx.attr.target_name,
        linkopt = repository_ctx.attr.linkopt,
    ))

host_c_library = repository_rule(
    implementation = _host_c_library_impl,
    attrs = {
        "target_name": attr.string(mandatory = True),
        "include_src": attr.string(mandatory = True),
        "include_dst": attr.string(mandatory = True),
        "linkopt": attr.string(mandatory = True),
    },
)

llvm_commit_tag = tag_class(
    attrs = {
        "commit": attr.string(mandatory = True),
    },
)

def _llvm_extension_impl(module_ctx):
    commit = None
    # Prefer configuration from the root module, falling back to dependent modules.
    for mod in module_ctx.modules:
        for cfg in mod.tags.configure:
            if mod.is_root:
                commit = cfg.commit
                break
            elif not commit:
                commit = cfg.commit

    if not commit:
        fail("Please configure LLVM commit via llvm_ext.configure(commit = ...)")

    # Fetch LLVM source
    llvm_source_fetch(name = "llvm-raw", commit = commit)
    host_c_library(
        name = "llvm_zlib",
        target_name = "zlib-ng",
        include_src = "/usr/include/zlib.h",
        include_dst = "include/zlib.h",
        linkopt = "-lz",
    )
    host_c_library(
        name = "llvm_libxml2",
        target_name = "libxml2",
        include_src = "/usr/include/libxml2",
        include_dst = "include",
        linkopt = "-lxml2",
    )
    host_c_library(
        name = "llvm_zstd",
        target_name = "zstd",
        include_src = "/usr/include/zstd.h",
        include_dst = "include/zstd.h",
        linkopt = "-lzstd",
    )

llvm_extension = module_extension(
    implementation = _llvm_extension_impl,
    tag_classes = {
        "configure": llvm_commit_tag,
    },
)
