load("@rules_rust//rust:defs.bzl", "rust_library")
load("//build_integration:build_integration.bzl", "bazel_to_builds")

package(default_applicable_licenses = ["//:license"])

rust_library(
    name = "ffi_11",
    srcs = glob(["src/*.rs"]),
    compatible_with = ["//buildenv/target:non_prod"],
    crate_features = ["crubit"],
    visibility = ["//visibility:public"],
)

bazel_to_builds(
    name = "generate_builds",
    cmake = "CMakeLists.txt",
    gn = "BUILD.gn",
    targets = [
        ":ffi_11",
    ],
)
