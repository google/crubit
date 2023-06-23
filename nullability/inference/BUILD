# Inference of nullability annotations

load("@rules_proto//proto:defs.bzl", "proto_library")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "pointer_model",
    hdrs = ["pointer_model.h"],
    deps = [
        "@absl//absl/container:flat_hash_set",
        "@llvm-project//clang:analysis",
        "@llvm-project//clang:basic",
    ],
)

proto_library(
    name = "inference_proto",
    srcs = ["inference.proto"],
)

cc_proto_library(
    name = "inference_cc_proto",
    deps = [":inference_proto"],
)
