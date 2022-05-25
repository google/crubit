# Support libraries that the generated Rust/C++ bindings depend upon.
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_proc_macro", "rust_test")

licenses(["notice"])

cc_library(
    name = "rs_api_impl_support",
    hdrs = [
        "cxx20_backports.h",
        "offsetof.h",
    ],
    visibility = ["//:__subpackages__"],
    # It is important to be thoughtful when adding new dependencies for
    # `rs_api_impl_support` (and possibly other targets in this BUILD file).
    # Even simple dependencies like `absl` may be tricky in practice, because
    # `rs_api_impl_support` may be developed and tested with one set of
    # absl/base/options.h which may be different from absl/base/options.h
    # adopted by users of Crubit.
    deps = [],
)

cc_test(
    name = "offsetof_test",
    srcs = ["offsetof_test.cc"],
    deps = [
        ":rs_api_impl_support",
        "@com_google_googletest//:gtest_main",
    ],
)

rust_library(
    name = "ctor",
    srcs = ["ctor.rs"],
    proc_macro_deps = [":ctor_proc_macros"],
    visibility = ["//:__subpackages__"],
)

rust_proc_macro(
    name = "ctor_proc_macros",
    srcs = ["ctor_proc_macros.rs"],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
    ],
)

rust_test(
    name = "ctor_test",
    srcs = ["ctor.rs"],
    proc_macro_deps = [":ctor_proc_macros"],
)

rust_test(
    name = "ctor_proc_macros_test",
    srcs = ["ctor_proc_macros_test.rs"],
    deps = [
        ":ctor",
    ],
)

rust_test(
    name = "ctor_macro_test",
    srcs = ["ctor_macro_test.rs"],
    deps = [
        ":ctor",
    ],
)

rust_library(
    name = "forward_declare",
    srcs = ["forward_declare.rs"],
    proc_macro_deps = [":forward_declare_proc_macros"],
    visibility = ["//:__subpackages__"],
)

rust_proc_macro(
    name = "forward_declare_proc_macros",
    srcs = ["forward_declare_proc_macros.rs"],
    deps = [
        "@crate_index//:proc-macro2",
        "@crate_index//:quote",
        "@crate_index//:syn",
    ],
)

rust_test(
    name = "forward_declare_macros_test",
    srcs = ["forward_declare_macros_test.rs"],
    deps = [":forward_declare"],
)

rust_library(
    name = "oops",
    srcs = ["oops.rs"],
    visibility = [
        "//:__subpackages__",
    ],
)

rust_test(
    name = "oops_test",
    srcs = ["oops.rs"],
)
