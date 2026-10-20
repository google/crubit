load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "propagates_define",
    defines = ["DEFINE_FROM_DEPENDENCY"],
)

crubit_test_cc_library(
    name = "consumes_define",
    hdrs = ["consumes_define.h"],
    deps = [":propagates_define"],
)

rust_api_from_cpp(
    name = "consumes_define_rust",
    cpp_target = ":consumes_define",
)

crubit_rust_test(
    name = "calls_generator_on_consumes_define",
    srcs = ["empty.rs"],
    deps = [":consumes_define_rust"],
)

crubit_test_cc_library(
    name = "sets_define",
    hdrs = ["sets_define.h"],
    defines = ["DEFINE_FROM_BUILD_FILE"],
)

rust_api_from_cpp(
    name = "sets_define_rust",
    cpp_target = ":sets_define",
)

crubit_rust_test(
    name = "calls_generator_on_sets_define",
    srcs = ["empty.rs"],
    deps = [":sets_define_rust"],
)

crubit_test_cc_library(
    name = "uses_copts",
    hdrs = ["uses_copts.h"],
    copts = ["-DDEFINE_FROM_COPTS"],
)

rust_api_from_cpp(
    name = "uses_copts_rust",
    cpp_target = ":uses_copts",
)

crubit_rust_test(
    name = "calls_generator_on_uses_copts",
    srcs = ["empty.rs"],
    deps = [":uses_copts_rust"],
)

crubit_test_cc_library(
    name = "uses_local_defines",
    hdrs = ["uses_local_defines.h"],
    local_defines = ["DEFINE_FROM_LOCAL_DEFINES"],
)

rust_api_from_cpp(
    name = "uses_local_defines_rust",
    cpp_target = ":uses_local_defines",
)

crubit_rust_test(
    name = "calls_generator_on_uses_local_defines",
    srcs = ["empty.rs"],
    deps = [":uses_local_defines_rust"],
)

crubit_test_cc_library(
    name = "consumes_local_defines_dep",
    hdrs = ["consumes_local_defines_dep.h"],
    deps = [":uses_local_defines"],
)

rust_api_from_cpp(
    name = "consumes_local_defines_dep_rust",
    cpp_target = ":consumes_local_defines_dep",
)

crubit_rust_test(
    name = "calls_generator_on_consumes_local_defines_dep",
    srcs = ["empty.rs"],
    deps = [":consumes_local_defines_dep_rust"],
)

crubit_test_cc_library(
    name = "uses_includes",
    hdrs = ["subdir_for_includes/header_from_includes.h"],
    includes = ["subdir_for_includes"],
)

crubit_test_cc_library(
    name = "consumes_header_from_includes",
    hdrs = ["consumes_header_from_includes.h"],
    deps = [":uses_includes"],
)

rust_api_from_cpp(
    name = "consumes_header_from_includes_rust",
    cpp_target = ":consumes_header_from_includes",
)

crubit_rust_test(
    name = "calls_generator_on_consumes_header_from_includes",
    srcs = ["empty.rs"],
    deps = [":consumes_header_from_includes_rust"],
)

crubit_test_cc_library(
    name = "uses_include_prefix",
    hdrs = [
        "subdir_for_includes/also_uses_include_prefix.h",
        "subdir_for_includes/uses_include_prefix.h",
    ],
    include_prefix = "using_include_prefix",
    strip_include_prefix = "subdir_for_includes",
)

crubit_test_cc_library(
    name = "consumes_header_with_include_prefix",
    hdrs = ["consumes_header_with_include_prefix.h"],
    deps = [":uses_include_prefix"],
)

rust_api_from_cpp(
    name = "consumes_header_with_include_prefix_rust",
    cpp_target = ":consumes_header_with_include_prefix",
)

crubit_rust_test(
    name = "calls_generator_on_consumes_header_with_include_prefix",
    srcs = ["empty.rs"],
    deps = [":consumes_header_with_include_prefix_rust"],
)
