# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "edb87c0d2ba70823fe3df7862676d695599314a4634b9758bd55f0e8f19c2751",
    urls = [
        "https://github.com/bazelbuild/rules_rust/releases/download/0.4.0/rules_rust-v0.4.0.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    iso_date = "2022-07-07",
    version = "nightly",)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "render_config")

# after changing:
#   CARGO_BAZEL_REPIN=1 bazelisk sync --only=crate_index
crates_repository(
    name = "crate_index",
    lockfile = "//:Cargo.Bazel.lock",
    packages = {
        "anyhow": crate.spec(
            version = ">0.0.0",
        ),
        "itertools": crate.spec(
            version = ">0.0.0",
        ),
        "maplit": crate.spec(
            version = ">0.0.0",
        ),
        "memoffset": crate.spec(
            version = ">0.0.0",
        ),
        "once_cell": crate.spec(
            version = ">0.0.0",
        ),
        "pin-project": crate.spec(
            version = ">0.0.0",
        ),
        "proc-macro2": crate.spec(
            version = ">0.0.0",
        ),
        "quote": crate.spec(
            version = ">0.0.0",
        ),
        "salsa": crate.spec(
            version = ">0.0.0",
        ),
        "serde": crate.spec(
            features = ["derive", "rc",],
            version = ">0.0.0",
        ),
        "serde_json": crate.spec(
            version = ">0.0.0",
        ),
        "static_assertions": crate.spec(
            version = ">0.0.0",
        ),
        "syn": crate.spec(
            features = ["extra-traits"],
            version = ">0.0.0",
        ),
    },

    render_config = render_config(
        default_package_name = "",
    ),
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

# https://github.com/bazelbuild/bazel-skylib/releases/tag/1.2.1

http_archive(
    name = "bazel_skylib",
    sha256 = "f7be3474d42aae265405a592bb7da8e171919d74c16f082a5457840f06054728",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.2.1/bazel-skylib-1.2.1.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.2.1/bazel-skylib-1.2.1.tar.gz",
    ],
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

bazel_skylib_workspace()

# https://abseil.io/docs/cpp/quickstart#set-up-a-bazel-workspace-to-work-with-abseil
#
# From https://abseil.io/about/releases: Abseil encourages developers to “live
# at head”.  92fdbfb301f8b301b28ab5c99e7361e775c2fb8a was the head/ToT on
# 2022-08-25.
http_archive(
    name = "absl",
    sha256 = "feea6e761ab9df667f054531b4e4d5d143eaa78f2c40dbf5ba45289142cbcc1d",
    strip_prefix = "abseil-cpp-92fdbfb301f8b301b28ab5c99e7361e775c2fb8a",
    urls = ["https://github.com/abseil/abseil-cpp/archive/92fdbfb301f8b301b28ab5c99e7361e775c2fb8a.zip"],
)

# https://google.github.io/googletest/quickstart-bazel.html

http_archive(
  name = "com_google_googletest",
  urls = ["https://github.com/google/googletest/archive/609281088cfefc76f9d0ce82e1ff6c30cc3591e5.zip"],
  strip_prefix = "googletest-609281088cfefc76f9d0ce82e1ff6c30cc3591e5",
)

# Create the "loader" repository, then use it to configure the desired LLVM
# repository. For more details, see the comment in bazel/llvm.bzl.

load("//bazel:llvm.bzl", "llvm_loader_repository_dependencies", "llvm_loader_repository")
llvm_loader_repository_dependencies()
llvm_loader_repository(name = "llvm-loader")

load("@llvm-loader//:llvm.bzl", "llvm_repository")
llvm_repository(name = "llvm-project")
