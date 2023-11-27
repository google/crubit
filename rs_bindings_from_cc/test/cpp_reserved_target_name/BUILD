load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        "//rs_bindings_from_cc/test/cpp_reserved_target_name/subdir2",
    ],
    tags = [
        # b/312438838: It currently fails with "failed to resolve: could not find `prelude` in `core`".
        "manual",
        "nobuilder",
        "notap",
    ],
)
