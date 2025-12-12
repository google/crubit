load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

# Tests that it's OK to include a Crubit-disabled cc_library named 'core' in transitive deps.
crubit_rust_test(
    name = "test_crubit_disabled_cc_library_named_core_in_transitive_deps",
    srcs = ["test_crubit_disabled_cc_library_named_core_in_transitive_deps.rs"],
    cc_deps = [
        "//rs_bindings_from_cc/test/cpp_reserved_target_name/subdir2",
    ],
    deps = [
        "@crates//:googletest",
    ],
)

# Tests that the name of a Crubit-enabled cc_library named 'core'.
crubit_test_cc_library(
    name = "core",
    hdrs = ["core.h"],
)

crubit_rust_test(
    name = "test_map_crubit_enabled_cc_library_named_core",
    srcs = ["test_map_crubit_enabled_cc_library_named_core.rs"],
    cc_deps = [
        ":core",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crates//:googletest",
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

crubit_rust_test(
    name = "test_crubit_enabled_cc_library_named_core_in_transitive_deps",
    srcs = ["test_crubit_enabled_cc_library_named_core_in_transitive_deps.rs"],
    cc_deps = [
        ":core_user",
    ],
    deps = [
        "@crates//:googletest",
    ],
)
