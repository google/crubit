load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "noescape_pointers_cc",
    srcs = ["noescape_pointers.cc"],
    hdrs = ["noescape_pointers.h"],
    aspect_hints = [":noescape_pointers_cc_rust.hint"],
)

rust_api_from_cpp(
    name = "noescape_pointers_cc_rust",
    cpp_target = ":noescape_pointers_cc",
    crubit_features = ["noescape_pointer_params_as_references"],
)

crubit_rust_test(
    name = "noescape_pointers_test",
    srcs = ["noescape_pointers_test.rs"],
    deps = [
        ":noescape_pointers_cc_rust",
        "@crate_index//:googletest",
    ],
)
