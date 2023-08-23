load("//common:crubit_wrapper_macros_oss.bzl", crubit_rust_test = "crubit_on_demand_rust_test")
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

crubit_test_cc_library(
    name = "definition_enabled",
    hdrs = ["definition_enabled.h"],
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

crubit_test_cc_library(
    name = "wrapper_struct_enabled",
    hdrs = ["wrapper_struct_enabled.h"],
    deps = [":definition_disabled"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":alias_disabled",
        ":alias_enabled",
        ":definition_disabled",
        ":definition_enabled",
        ":func_enabled",
        ":wrapper_struct_enabled",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
)
