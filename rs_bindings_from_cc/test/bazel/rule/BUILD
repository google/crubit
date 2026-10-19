load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_test")
load(
    "//bazel:rust_api_from_cpp.bzl",
    "rust_api_from_cpp",
)
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

package(default_applicable_licenses = ["//:license"])

licenses(["notice"])

cc_library(
    name = "lib",
    srcs = ["lib.cc"],
    hdrs = ["lib.h"],
    aspect_hints = [":lib_rust.hint"],
)

rust_api_from_cpp(
    name = "lib_rust",
    cpp_target = ":lib",
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [":lib_rust"],
)

crubit_rust_test(
    name = "crubit_test",
    srcs = ["test.rs"],
    deps = [":lib_rust"],
)

cc_library(
    name = "my_lib",
    hdrs = ["math_dependency.h"],
    aspect_hints = [
        ":my_lib_rust.hint",
    ],
)

rust_api_from_cpp(
    name = "my_lib_rust",
    srcs = ["additional_rust_srcs.rs"],
    cpp_srcs = ["additional_cpp_srcs.cc"],
    cpp_target = ":my_lib",
)

crubit_rust_test(
    name = "my_lib_rust_test",
    srcs = ["additional_rust_srcs_test.rs"],
    deps = [
        ":my_lib_rust",
        "@crate_index//:googletest",
    ],
)

# A target without public headers. Crubit generates bindings for it only because of the extra
# Rust `srcs`; its generated `_rust_api_impl.cc` includes a synthetic empty header that the impl
# compile itself owns, and is compiled against precompiled header modules like any other.
cc_library(
    name = "no_hdrs_lib",
    aspect_hints = [":no_hdrs_lib_rust.hint"],
)

rust_api_from_cpp(
    name = "no_hdrs_lib_rust",
    srcs = ["no_hdrs_srcs.rs"],
    cpp_target = ":no_hdrs_lib",
)

crubit_rust_test(
    name = "no_hdrs_test",
    srcs = ["no_hdrs_test.rs"],
    deps = [
        ":no_hdrs_lib_rust",
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "compile_data_lib",
    hdrs = ["compile_data_lib.h"],
    aspect_hints = [":compile_data_lib_rust.hint"],
)

genrule(
    name = "generated_compile_data",
    outs = ["generated_compile_data.txt"],
    cmd = "echo 'Hello from generated compile_data!' > $@",
)

rust_api_from_cpp(
    name = "compile_data_lib_rust",
    srcs = ["compile_data_srcs.rs"],
    compile_data = [
        "testdata/compile_data.yaml",
        ":generated_compile_data",
    ],
    cpp_target = ":compile_data_lib",
    srcs_mod = "compile_data_ns",
)

crubit_rust_test(
    name = "compile_data_test",
    srcs = ["compile_data_test.rs"],
    deps = [
        ":compile_data_lib_rust",
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "features_lib_types",
    hdrs = ["features_lib.h"],
    aspect_hints = [":features_lib_types_rust.hint"],
)

rust_api_from_cpp(
    name = "features_lib_types_rust",
    cpp_target = ":features_lib_types",
    types_only = True,
)

cc_library(
    name = "features_lib_multi",
    hdrs = ["features_lib.h"],
    aspect_hints = [":features_lib_multi_rust.hint"],
)

rust_api_from_cpp(
    name = "features_lib_multi_rust",
    cpp_target = ":features_lib_multi",
    crubit_features = [
        "infer_send_sync",
        "record_impl_debug",
    ],
)

cc_library(
    name = "features_lib_srcs",
    hdrs = ["features_lib.h"],
    aspect_hints = [":features_lib_srcs_rust.hint"],
)

rust_api_from_cpp(
    name = "features_lib_srcs_rust",
    srcs = ["features_extra_src.rs"],
    cpp_target = ":features_lib_srcs",
    types_only = True,
)

crubit_rust_test(
    name = "features_test",
    srcs = ["features_test.rs"],
    deps = [
        ":features_lib_multi_rust",
        ":features_lib_srcs_rust",
        ":features_lib_types_rust",
        "@crate_index//:googletest",
    ],
)
