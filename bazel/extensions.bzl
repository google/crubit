# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Module extension for configuring Crubit toolchains."""

load("@toolchains_llvm//toolchain:rules.bzl", _llvm_toolchain_config = "toolchain")

# buildifier: disable=bzl-visibility
load(
    "@toolchains_llvm//toolchain/internal:repo.bzl",
    "common_attrs",
    "llvm_config_attrs",
    "llvm_repo_attrs",
    "llvm_repo_impl",
)
load("//bazel:llvm_version_check.bzl", "llvm_version_check")

# buildifier: disable=bzl-visibility
load("@rules_rust//rust/private:repositories.bzl", "DEFAULT_TOOLCHAIN_TRIPLES", "rust_register_toolchains")

# buildifier: disable=bzl-visibility
load(
    "@rules_rust//rust/private:repository_utils.bzl",
    "DEFAULT_EXTRA_TARGET_TRIPLES",
    "DEFAULT_STATIC_RUST_URL_TEMPLATES",
)

# These attributes are mirrored from rules_rust's toolchain tag to allow
# configuration without duplicating the implementation logic.
_RUST_TAG_ATTRS = {
    "rust_version": attr.string(doc = "The version of Rust to install."),
    "edition": attr.string(doc = "The rust edition to be used by default.", default = "2024"),
    "dev_components": attr.bool(doc = "Whether to download the rustc-dev components.", default = True),
    "extra_rustc_flags": attr.string_list(doc = "Extra flags to pass to rustc in non-exec configuration."),
    "extra_exec_rustc_flags": attr.string_list(doc = "Extra flags to pass to rustc in exec configuration."),
    "rustfmt_version": attr.string(doc = "The version of rustfmt."),
    "rust_analyzer_version": attr.string(doc = "The version of rust-analyzer."),
    "sha256s": attr.string_dict(doc = "A dict associating tool subdirectories to sha256 hashes."),
    "extra_target_triples": attr.string_list(doc = "Additional rust-style targets.", default = DEFAULT_EXTRA_TARGET_TRIPLES),
    "opt_level": attr.string_dict(doc = "Rustc optimization levels."),
    "strip_level": attr.string_dict(doc = "Rustc strip levels."),
    "urls": attr.string_list(doc = "A list of mirror urls.", default = DEFAULT_STATIC_RUST_URL_TEMPLATES),
    "allocator_library": attr.label(doc = "Target that provides allocator functions."),
    "global_allocator_library": attr.label(doc = "Target that provides allocator functions when global allocator is used."),
    "target_settings": attr.label_list(doc = "Config settings for toolchain selection."),
    "aliases": attr.string_dict(doc = "Toolchain repository aliases."),
}

_LLVM_TAG_ATTRS = {
    "llvm_version": attr.string(doc = "The version of LLVM to install."),
    "llvm_urls": attr.string_list(doc = "Custom URLs for LLVM distribution."),
    "llvm_sha256": attr.string(doc = "SHA256 for LLVM distribution."),
    "llvm_strip_prefix": attr.string(doc = "Strip prefix for LLVM distribution."),
}

# Upstream LLVM's Linux x86_64 release tarballs (e.g. LLVM 23.1.0) are built on
# Ubuntu 22.04 with LLVM_USE_STATIC_LIBXML2=ON against the system libxml2.a,
# which gives bin/ld.lld a dynamic DT_NEEDED dependency on Ubuntu 22.04's
# libicu70 (libicui18n.so.70, libicuuc.so.70, libicudata.so.70).
# Because bin/ld.lld has RUNPATH=$ORIGIN/../lib, unpacking libicu70 into lib/
# and adding it to the :ld filegroup makes the linker hermetic on newer OS
# images (such as the Ubuntu 24.04 RBE container).
_UBUNTU_JAMMY_LIBICU70_URLS = [
    "https://launchpad.net/ubuntu/+source/icu/70.1-2/+build/23145450/+files/libicu70_70.1-2_amd64.deb",
    "http://archive.ubuntu.com/ubuntu/pool/main/i/icu/libicu70_70.1-2_amd64.deb",
    "http://mirrors.kernel.org/ubuntu/pool/main/i/icu/libicu70_70.1-2_amd64.deb",
]
_UBUNTU_JAMMY_LIBICU70_SHA256 = "58a154f6307289813da2276f900498ef536ae7c0522d2cf31a3c3c5cf62dfd9a"

def _crubit_llvm_repo_impl(rctx):
    res = llvm_repo_impl(rctx)
    if rctx.os.name == "linux" and rctx.os.arch in ("amd64", "x86_64"):
        rctx.download_and_extract(
            url = _UBUNTU_JAMMY_LIBICU70_URLS,
            output = "_libicu70",
            sha256 = _UBUNTU_JAMMY_LIBICU70_SHA256,
        )
        rctx.extract(
            archive = "_libicu70/data.tar.zst",
            output = "lib",
            stripPrefix = "usr/lib/x86_64-linux-gnu",
        )
        rctx.delete("_libicu70")

        build_content = rctx.read("BUILD.bazel")
        if '["bin/wasm-ld"],' not in build_content:
            fail("Expected '[\"bin/wasm-ld\"],' in @llvm_toolchain_llvm//:BUILD.bazel")
        rctx.file(
            "BUILD.bazel",
            build_content.replace(
                '["bin/wasm-ld"],',
                '["bin/wasm-ld", "lib/libicu*.so*"],',
            ),
        )
    return res

_crubit_llvm_repo = repository_rule(
    attrs = llvm_repo_attrs,
    local = False,
    implementation = _crubit_llvm_repo_impl,
)

def _crubit_llvm_toolchain(name, **kwargs):
    if kwargs.get("llvm_version") and kwargs.get("llvm_versions"):
        fail("Exactly one of llvm_version or llvm_versions must be set")
    if not kwargs.get("llvm_versions"):
        if not kwargs.get("llvm_version"):
            fail("One of llvm_version or llvm_versions must be set")
        kwargs.update(llvm_versions = {"": kwargs.get("llvm_version")})

    if not kwargs.get("toolchain_roots"):
        llvm_args = {
            k: v
            for k, v in kwargs.items()
            if (k not in llvm_config_attrs.keys()) or (k in common_attrs.keys())
        }
        _crubit_llvm_repo(name = name + "_llvm", **llvm_args)

    toolchain_args = {
        k: v
        for k, v in kwargs.items()
        if (k not in llvm_repo_attrs.keys()) or (k in common_attrs.keys())
    }
    _llvm_toolchain_config(name = name, **toolchain_args)

def _crubit_toolchains_impl(ctx):
    # Prefer configuration from the root module.
    config = None
    for mod in ctx.modules:
        if mod.is_root and mod.tags.configure:
            config = mod.tags.configure[0]
            break
    if not config:
        for mod in ctx.modules:
            if mod.tags.configure:
                config = mod.tags.configure[0]
                break

    # 1. Coordinate versions
    rust_version = getattr(config, "rust_version", None)
    if not rust_version:
        fail("Please specify a Rust version with the `rust_version` attribute.")

    # 2. Define LLVM repository
    final_llvm_version = getattr(config, "llvm_version", None)
    final_urls = getattr(config, "llvm_urls", None)
    final_sha256 = getattr(config, "llvm_sha256", None)
    final_strip_prefix = getattr(config, "llvm_strip_prefix", None)

    # Perform LLVM version check against system rustc
    llvm_version_check(
        name = "llvm_version_check",
        llvm_version = final_llvm_version,
    )

    if not final_urls and not final_llvm_version:
        fail("Please specify an LLVM version or a custom LLVM via llvm_urls, llvm_sha256, and llvm_strip_prefix.")
    elif final_urls:
        _crubit_llvm_toolchain(
            name = "llvm_toolchain",
            llvm_version = final_llvm_version,
            urls = {"": final_urls},
            sha256 = {"": final_sha256},
            strip_prefix = {"": final_strip_prefix},
        )
    else:
        _crubit_llvm_toolchain(
            name = "llvm_toolchain",
            llvm_version = final_llvm_version,
        )

    # 3. Define Rust repository (delegated to rules_rust)
    rust_kwargs = {}
    if config:
        rust_kwargs = {k: getattr(config, k) for k in _RUST_TAG_ATTRS.keys() if k != "rust_version"}
    else:
        # Defaults if no configure tag is present
        rust_kwargs = {
            "edition": "2024",
            "extra_target_triples": DEFAULT_EXTRA_TARGET_TRIPLES,
            "urls": DEFAULT_STATIC_RUST_URL_TEMPLATES,
        }

    # We need to always include dev components because Crubit needs them internally.
    rust_kwargs["dev_components"] = True

    # Stringify labels as required by the underlying repository rules
    if rust_kwargs.get("allocator_library"):
        rust_kwargs["allocator_library"] = str(rust_kwargs["allocator_library"])
    if rust_kwargs.get("global_allocator_library"):
        rust_kwargs["global_allocator_library"] = str(rust_kwargs["global_allocator_library"])
    if rust_kwargs.get("target_settings"):
        rust_kwargs["target_settings"] = [str(s) for s in rust_kwargs["target_settings"]]

    rust_register_toolchains(
        hub_name = "rust_toolchains",
        versions = [rust_version],
        toolchain_triples = dict(DEFAULT_TOOLCHAIN_TRIPLES),
        rustfmt_toolchain_triples = DEFAULT_TOOLCHAIN_TRIPLES,
        compact_windows_names = True,
        **rust_kwargs
    )

    return ctx.extension_metadata(reproducible = True)

crubit_toolchains = module_extension(
    implementation = _crubit_toolchains_impl,
    tag_classes = {
        "configure": tag_class(
            attrs = _RUST_TAG_ATTRS | _LLVM_TAG_ATTRS,
        ),
    },
)
