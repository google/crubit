# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# https://github.com/bazelbuild/bazel-skylib/releases/tag/1.3.0

http_archive(
    name = "bazel_skylib",
    sha256 = "74d544d96f4a5bb630d465ca8bbcfe231e3594e5aae57e1edbf17a6eb3ca2506",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.3.0/bazel-skylib-1.3.0.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.3.0/bazel-skylib-1.3.0.tar.gz",
    ],
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

bazel_skylib_workspace()

http_archive(
    name = "rules_license",
    sha256 = "6157e1e68378532d0241ecd15d3c45f6e5cfd98fc10846045509fb2a7cc9e381",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_license/releases/download/0.0.4/rules_license-0.0.4.tar.gz",
        "https://github.com/bazelbuild/rules_license/releases/download/0.0.4/rules_license-0.0.4.tar.gz",
    ],
)

http_archive(
    name = "rules_rust",
    patch_args = [
        "-p1",
    ],
    patches = [
        # copybara:strip_begin(Google-internal)
        # The patch is based on the copybara transformations here:
        # http://google3/third_party/bazel_rules/rules_rust/copy.bara.sky;l=398;rcl=549936350
        # copybara:strip_end
        "@@//bazel/rules_rust:attach_rust_bindings_from_cc_aspect.patch",
    ],
    sha256 = "4a9cb4fda6ccd5b5ec393b2e944822a62e050c7c06f1ea41607f14c4fdec57a2",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.25.1/rules_rust-v0.25.1.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

RUST_TOOLCHAIN_VERSION = "nightly/2023-08-03"

rust_register_toolchains(
    allocator_library = "@//common:rust_allocator_shims",
    dev_components = True,
    edition = "2021",
    versions = [
        RUST_TOOLCHAIN_VERSION,
    ],
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "render_config")

# after changing `packages`, re-generate Cargo.lock:
#   CARGO_BAZEL_REPIN=1 bazelisk sync --only=crate_index
crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    packages = {
        "anyhow": crate.spec(
            version = ">0.0.0",
        ),
        "clap": crate.spec(
            features = [
                "derive",
                "color",
                "derive",
                "env",
                "std",
                "string",
                "suggestions",
            ],
            version = ">=4.3.12",
        ),
        "either": crate.spec(
            version = ">1.0.0",
        ),
        "flagset": crate.spec(
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
        "regex": crate.spec(
            version = ">=1.6.0",
        ),
        "salsa": crate.spec(
            version = ">0.0.0",
        ),
        "serde": crate.spec(
            features = [
                "derive",
                "rc",
            ],
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
        "tempfile": crate.spec(
            version = "=3.4.0",
        ),
        "unicode-ident": crate.spec(
            version = ">0.0.0",
        ),
    },
    render_config = render_config(
        default_package_name = "",
    ),
    rust_version = RUST_TOOLCHAIN_VERSION,
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

# https://abseil.io/docs/cpp/quickstart#set-up-a-bazel-workspace-to-work-with-abseil
#
# From https://abseil.io/about/releases: Abseil encourages developers to “live
# at head”.  c16a2f43206b0235d49d4f6155f285a4d4939c58 was the head/ToT on
# 2023-07-14.
http_archive(
    name = "absl",
    sha256 = "725f25f09d68a7b61c861c2b5467e662a35f3fddc95b50501f3245c95285a688",
    strip_prefix = "abseil-cpp-c16a2f43206b0235d49d4f6155f285a4d4939c58",
    urls = ["https://github.com/abseil/abseil-cpp/archive/c16a2f43206b0235d49d4f6155f285a4d4939c58.zip"],
)

# https://google.github.io/googletest/quickstart-bazel.html
http_archive(
    name = "com_google_googletest",
    sha256 = "7fda611bceb5a793824a3c63ecbf68d2389e70c38f5763e9b1d415ca24912f44",
    strip_prefix = "googletest-1336c4b6d1a6f4bc6beebccb920e5ff858889292",
    urls = ["https://github.com/google/googletest/archive/1336c4b6d1a6f4bc6beebccb920e5ff858889292.zip"],
)

# zstd is a dependency of llvm.  See https://reviews.llvm.org/D143344#4232172
http_archive(
    name = "llvm_zstd",
    build_file = "@llvm-raw//utils/bazel/third_party_build:zstd.BUILD",
    sha256 = "7c42d56fac126929a6a85dbc73ff1db2411d04f104fae9bdea51305663a83fd0",
    strip_prefix = "zstd-1.5.2",
    urls = [
        "https://github.com/facebook/zstd/releases/download/v1.5.2/zstd-1.5.2.tar.gz",
    ],
)

# @llvm-project//llvm:Support needs zlib.
http_archive(
    name = "llvm_zlib",
    build_file = "@llvm-raw//utils/bazel/third_party_build:zlib-ng.BUILD",
    sha256 = "e36bb346c00472a1f9ff2a0a4643e590a254be6379da7cddd9daeb9a7f296731",
    strip_prefix = "zlib-ng-2.0.7",
    urls = [
        "https://github.com/zlib-ng/zlib-ng/archive/refs/tags/2.0.7.zip",
    ],
)

# Create the "loader" repository, then use it to configure the desired LLVM
# repository. For more details, see the comment in bazel/llvm.bzl.

load("//bazel:llvm.bzl", "llvm_loader_repository", "llvm_loader_repository_dependencies")

llvm_loader_repository_dependencies()

llvm_loader_repository(name = "llvm-loader")

load("@llvm-loader//:llvm.bzl", "llvm_repository")

llvm_repository(name = "llvm-project")

# protobuf (used in nullability/; crubit proper should not depend on it)
http_archive(
    name = "rules_proto",
    sha256 = "dc3fb206a2cb3441b485eb1e423165b231235a1ea9b031b4433cf7bc1fa460dd",
    strip_prefix = "rules_proto-5.3.0-21.7",
    urls = [
        "https://github.com/bazelbuild/rules_proto/archive/refs/tags/5.3.0-21.7.tar.gz",
    ],
)

load("@rules_proto//proto:repositories.bzl", "rules_proto_dependencies", "rules_proto_toolchains")

rules_proto_dependencies()

rules_proto_toolchains()
