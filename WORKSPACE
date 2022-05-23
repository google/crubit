# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# https://bazelbuild.github.io/rules_rust/#setup
# https://github.com/bazelbuild/rules_rust/releases/tag/0.3.1

http_archive(
    name = "rules_rust",
    sha256 = "e074f1e203607c5fcd549929d956170346f8807d2bbaeb98b2ed213c37e0870f",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_rust/releases/download/0.3.1/rules_rust-v0.3.1.tar.gz",
        "https://github.com/bazelbuild/rules_rust/releases/download/0.3.1/rules_rust-v0.3.1.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(edition = "2021")

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
        "pin-project": crate.spec(
            version = ">0.0.0",
        ),
        "proc-macro2": crate.spec(
            version = ">0.0.0",
        ),
        "quote": crate.spec(
            version = ">0.0.0",
        ),
        "serde": crate.spec(
            features = ["derive"],
            version = ">0.0.0",
        ),
        "serde_json": crate.spec(
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
# https://github.com/abseil/abseil-cpp/releases/tag/20211102.0

http_archive(
    name = "absl",
    sha256 = "a4567ff02faca671b95e31d315bab18b42b6c6f1a60e91c6ea84e5a2142112c2",
    strip_prefix = "abseil-cpp-20211102.0",
    urls = ["https://github.com/abseil/abseil-cpp/archive/refs/tags/20211102.0.zip"],
)

# https://github.com/llvm/llvm-project/blob/main/utils/bazel/examples/http_archive/WORKSPACE
# https://github.com/llvm/llvm-project/releases/tag/llvmorg-14.0.0

http_archive(
    name = "llvm-raw",
    build_file_content = "# empty",
    sha256 = "eb7437b60a6f78e7910d08911975f100e99e9c714f069a5487119c7eadc79171",
    strip_prefix = "llvm-project-llvmorg-14.0.0",
    urls = ["https://github.com/llvm/llvm-project/archive/refs/tags/llvmorg-14.0.0.zip"],
)

load("@llvm-raw//utils/bazel:configure.bzl", "llvm_configure", "llvm_disable_optional_support_deps")

# this *must* be llvm-project, it's hardcoded in the Bazel build
# e.g. https://github.com/llvm/llvm-project/blob/aaddfbf9/utils/bazel/llvm-project-overlay/clang/BUILD.bazel#L1473
llvm_configure(name = "llvm-project")

llvm_disable_optional_support_deps()
