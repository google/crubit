load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "types_nonptr",
    hdrs = ["types_nonptr.h"],
    deps = [
        "//support:annotations_internal",
        "//support/public:slice_ref",
    ],
    # TODO(b/356479163): use aspect_hints = ["//features:supported"],
    # This is blocked on static methods, or else redesigning the test.
)

crubit_test_cc_library(
    name = "types_lifetimes",
    hdrs = ["types_lifetimes.h"],
)

crubit_test_cc_library(
    name = "types_nolifetimes",
    hdrs = ["types_nolifetimes.h"],
)

crubit_test_cc_library(
    name = "types_inferred_lifetimes",
    hdrs = ["types_inferred_lifetimes.h"],
    aspect_hints = ["//features:experimental"],
)

crubit_rust_test(
    name = "types",
    srcs = ["types_test.rs"],
    cc_deps = [
        ":types_inferred_lifetimes",
        ":types_lifetimes",
        ":types_nolifetimes",
        ":types_nonptr",
    ],
    proc_macro_deps = ["//common:item_exists"],
    deps = [
        "@crates//:googletest",
    ],
)
