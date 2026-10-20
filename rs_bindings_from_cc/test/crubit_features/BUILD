load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "definition_disabled",
    hdrs = ["definition_disabled.h"],
    aspect_hints = [],  # deliberately disable Crubit.
)

crubit_test_cc_library(
    name = "alias_enabled",
    hdrs = ["alias_enabled.h"],
    deps = [":definition_disabled"],
)

rust_api_from_cpp(
    name = "alias_enabled_rust",
    cpp_target = ":alias_enabled",
)

crubit_test_cc_library(
    name = "definition_enabled",
    hdrs = ["definition_enabled.h"],
)

rust_api_from_cpp(
    name = "definition_enabled_rust",
    cpp_target = ":definition_enabled",
)

crubit_test_cc_library(
    name = "alias_disabled",
    hdrs = ["alias_disabled.h"],
    aspect_hints = [],  # deliberately disable Crubit.
    deps = [":definition_enabled"],
)

crubit_test_cc_library(
    name = "func_enabled",
    hdrs = ["func_enabled.h"],
    deps = [":definition_disabled"],
)

rust_api_from_cpp(
    name = "func_enabled_rust",
    cpp_target = ":func_enabled",
)

crubit_test_cc_library(
    name = "wrapper_struct_enabled",
    hdrs = ["wrapper_struct_enabled.h"],
    deps = [":definition_disabled"],
)

rust_api_from_cpp(
    name = "wrapper_struct_enabled_rust",
    cpp_target = ":wrapper_struct_enabled",
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        ":alias_disabled",
        ":alias_enabled_rust",
        ":definition_disabled",
        ":definition_enabled_rust",
        ":func_enabled_rust",
        ":wrapper_struct_enabled_rust",
        "@crate_index//:googletest",
    ],
)
