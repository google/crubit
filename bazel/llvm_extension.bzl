# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Module extension for downloading LLVM source and configuring host C dependencies."""

_GCS_LLVM_URL = "https://storage.googleapis.com/crubit-ci-bazel/llvm-project/{commit}.tar.zst"
_GITHUB_LLVM_URL = "https://github.com/llvm/llvm-project/archive/{commit}.tar.gz"

def _llvm_source_fetch_impl(repository_ctx):
    commit = repository_ctx.os.environ.get("CRUBIT_LLVM_COMMIT") or repository_ctx.attr.commit

    gcs_archive = "llvm-project.tar.zst"
    gcs_res = repository_ctx.download(
        url = _GCS_LLVM_URL.format(commit = commit),
        output = gcs_archive,
        allow_fail = True,
    )
    if gcs_res.success:
        repository_ctx.extract(
            archive = gcs_archive,
            stripPrefix = "llvm-project-" + commit,
        )
        repository_ctx.delete(gcs_archive)
    else:
        repository_ctx.download_and_extract(
            url = _GITHUB_LLVM_URL.format(commit = commit),
            stripPrefix = "llvm-project-" + commit,
        )

    # Extract the LLVM commit date (YYYYmmDD UTC) from the archive's mtime (or Gitiles fallback).
    python = repository_ctx.which("python3")
    if not python:
        python = repository_ctx.which("python")
    if not python:
        fail("Neither python3 nor python found in PATH")

    script = repository_ctx.path(repository_ctx.attr._get_llvm_commit_date)
    res = repository_ctx.execute([python, str(script), commit])
    if res.return_code != 0:
        fail("Failed to get LLVM commit date for commit %s: %s" % (commit, res.stderr))
    dev_date = res.stdout.strip()

    # Create a `BUILD` to expose the date of the LLVM/Clang commit.
    repository_ctx.file("BUILD.bazel", """load("@rules_cc//cc:cc_library.bzl", "cc_library")

cc_library(
    name = "llvm_dev_date",
    defines = ["CRUBIT_LLVM_DEV_DATE={dev_date}"],
    visibility = ["//visibility:public"],
)
""".format(dev_date = dev_date))

llvm_source_fetch = repository_rule(
    implementation = _llvm_source_fetch_impl,
    attrs = {
        "commit": attr.string(mandatory = True),
        "_get_llvm_commit_date": attr.label(
            default = Label("//cargo/build:get_llvm_commit_date.py"),
            allow_single_file = True,
        ),
    },
    environ = [
        "CRUBIT_LLVM_COMMIT",
        "CRUBIT_LLVM_DEV_DATE",
    ],
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

def _stub_llvm_raw_impl(repository_ctx):
    repository_ctx.file("BUILD.bazel", """load("@rules_cc//cc:cc_library.bzl", "cc_library")

cc_library(
    name = "llvm_dev_date",
    defines = ["CRUBIT_LLVM_DEV_DATE=0"],
    visibility = ["//visibility:public"],
)
""")
    repository_ctx.file("utils/bazel/BUILD.bazel")
    repository_ctx.file("utils/bazel/configure.bzl", """
def _stub_llvm_configure_impl(repository_ctx):
    repository_ctx.file("BUILD.bazel")

llvm_configure = repository_rule(
    implementation = _stub_llvm_configure_impl,
    attrs = {"targets": attr.string_list(default = [])},
)
""")

stub_llvm_raw = repository_rule(implementation = _stub_llvm_raw_impl)

def _empty_repo_impl(repository_ctx):
    repository_ctx.file("BUILD.bazel")

empty_repo = repository_rule(implementation = _empty_repo_impl)

llvm_commit_tag = tag_class(
    attrs = {
        "commit": attr.string(default = "", doc = "LLVM commit to fetch if building from source."),
        "use_crubit_llvm": attr.bool(default = True, doc = "Whether to use Crubit's LLVM source build."),
    },
)

def _llvm_extension_impl(module_ctx):
    root_cfg = None
    crubit_commit = None

    for mod in module_ctx.modules:
        for cfg in mod.tags.configure:
            if mod.is_root:
                root_cfg = cfg
            elif mod.name == "rules_crubit":
                crubit_commit = cfg.commit

    if not root_cfg or not root_cfg.use_crubit_llvm:
        # Stub out LLVM so downstream projects avoid downloading the
        # full source archive un-necessarily.
        stub_llvm_raw(name = "llvm-raw")
        empty_repo(name = "llvm_zlib")
        empty_repo(name = "llvm_libxml2")
        empty_repo(name = "llvm_zstd")
        return

    commit = root_cfg.commit or crubit_commit
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
