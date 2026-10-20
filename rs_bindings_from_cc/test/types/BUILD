load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "types_nonptr",
    hdrs = ["types_nonptr.h"],
    deps = [
        "//support:annotations_internal",
        "//support/rs_std:slice_ref",
    ],
    # TODO(b/356479163): use aspect_hints = ["//features:supported"],
    # This is blocked on static methods, or else redesigning the test.
)

rust_api_from_cpp(
    name = "types_nonptr_rust",
    cpp_target = ":types_nonptr",
)

crubit_test_cc_library(
    name = "types_lifetimes",
    hdrs = ["types_lifetimes.h"],
    aspect_hints = ["//features:assume_lifetimes"],
)

rust_api_from_cpp(
    name = "types_lifetimes_rust",
    cpp_target = ":types_lifetimes",
)

crubit_test_cc_library(
    name = "types_nolifetimes",
    hdrs = ["types_nolifetimes.h"],
    aspect_hints = ["//features:no_assume_lifetimes"],
)

rust_api_from_cpp(
    name = "types_nolifetimes_rust",
    cpp_target = ":types_nolifetimes",
)

crubit_test_cc_library(
    name = "types_inferred_lifetimes",
    hdrs = ["types_inferred_lifetimes.h"],
    aspect_hints = ["//features:experimental"],
)

rust_api_from_cpp(
    name = "types_inferred_lifetimes_rust",
    cpp_target = ":types_inferred_lifetimes",
)

crubit_rust_test(
    name = "types",
    srcs = ["types_test.rs"],
    proc_macro_deps = ["//common:item_exists"],
    deps = [
        ":types_inferred_lifetimes_rust",
        ":types_lifetimes_rust",
        ":types_nolifetimes_rust",
        ":types_nonptr_rust",
        "//support:cref",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)
