load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

rust_api_from_cpp(
    name = "subdir2_rust",
    cpp_target = "//rs_bindings_from_cc/test/cpp_reserved_target_name/subdir2",
)

# Tests that it's OK to include a Crubit-disabled cc_library named 'core' in transitive deps.
crubit_rust_test(
    name = "test_crubit_disabled_cc_library_named_core_in_transitive_deps",
    srcs = ["test_crubit_disabled_cc_library_named_core_in_transitive_deps.rs"],
    deps = [
        ":subdir2_rust",
        "@crate_index//:googletest",
    ],
)

# Tests that the name of a Crubit-enabled cc_library named 'core'.
crubit_test_cc_library(
    name = "core",
    hdrs = ["core.h"],
)

rust_api_from_cpp(
    name = "core_rust",
    cpp_target = ":core",
)

crubit_rust_test(
    name = "test_map_crubit_enabled_cc_library_named_core",
    srcs = ["test_map_crubit_enabled_cc_library_named_core.rs"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        ":core_rust",
        "@crate_index//:googletest",
    ],
)

# Test that it's OK to include a Crubit-enabled cc_library named 'core' in transitive deps, i.e.,
# the code generation logic in Crubit is aware of the name mapping.
crubit_test_cc_library(
    name = "core_user",
    hdrs = ["core_user.h"],
    deps = [
        ":core",
    ],
)

rust_api_from_cpp(
    name = "core_user_rust",
    cpp_target = ":core_user",
)

crubit_rust_test(
    name = "test_crubit_enabled_cc_library_named_core_in_transitive_deps",
    srcs = ["test_crubit_enabled_cc_library_named_core_in_transitive_deps.rs"],
    deps = [
        ":core_user_rust",
        "@crate_index//:googletest",
    ],
)
