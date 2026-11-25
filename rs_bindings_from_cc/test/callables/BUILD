load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

licenses(["notice"])

crubit_test_cc_library(
    name = "calls_back_to_rust",
    srcs = ["calls_back_to_rust.cc"],
    hdrs = ["calls_back_to_rust.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        "//support:annotations",
        "//support/rs_std:dyn_callable",
        "//support/rs_std:fn_ref",
        "@abseil-cpp//absl/functional:any_invocable",
    ],
)

rust_api_from_cpp(
    name = "calls_back_to_rust_rust",
    cpp_target = ":calls_back_to_rust",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = [
        ":calls_back_to_rust_rust",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "std_function_ref",
    srcs = ["std_function_ref.cc"],
    hdrs = ["std_function_ref.h"],
    aspect_hints = ["//features:supported"],
    features = [
        "-c++default",
        "c++26",
        # The prebuilt libc++ header module is built in the default C++ mode.
        "-use_header_modules",
    ],
    target_compatible_with = LLVM_UNSTABLE_ONLY,
)

rust_api_from_cpp(
    name = "std_function_ref_rust",
    cpp_target = ":std_function_ref",
)

crubit_rust_test(
    name = "std_function_ref_test",
    srcs = ["std_function_ref_test.rs"],
    tags = [
        "no_android",
        "no_ios",
    ],
    target_compatible_with = LLVM_UNSTABLE_ONLY,
    deps = [
        ":std_function_ref_rust",
        "@crate_index//:googletest",
    ],
)
