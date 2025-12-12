load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

crubit_test_cc_library(
    name = "lib",
    hdrs = [
        "lib.h",
        "lib_textual.h",
    ],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        "textual_impl",
        ":impl",
    ],
)

crubit_test_cc_library(
    name = "impl",
    # This has the same header as `lib`, but because it's private (in srcs, not hdrs), it
    # is not considered owned by this target.
    srcs = ["lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
)

crubit_test_cc_library(
    name = "textual_impl",
    aspect_hints = [
        "//features:supported",
    ],
    # This has the same header as `lib`, but because it's a textual header, it
    # is not considered owned by this target.
    textual_hdrs = ["lib_textual.h"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [
        ":lib",
    ],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crates//:googletest",
    ],
)
