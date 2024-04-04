"""Support libraries that the generated Rust/C++ bindings depend upon."""

load("@rules_rust//rust:defs.bzl", "rust_library", "rust_proc_macro")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")

package(default_applicable_licenses = ["//:license"])

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

crubit_rust_test(
    name = "ctor_proc_macros_test",
    crate = ":ctor_proc_macros",
    # TODO(b/324398297): Reenable once cl/619591730 has landed. The crate tested should be
    # renamed to ":ctor_proc_macros_proc_macro_internal"
    tags = [
        "manual",
        "nobuild",
        "notap",
    ],
    deps = [
        "//common:token_stream_matchers",
    ],
)

crubit_rust_test(
    name = "ctor_test",
    srcs = ["ctor.rs"],
    proc_macro_deps = [":ctor_proc_macros"],
)

crubit_rust_test(
    name = "ctor_proc_macros_e2e_test",
    srcs = ["ctor_proc_macros_test.rs"],
    deps = [
        ":ctor",
    ],
)

crubit_rust_test(
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

crubit_rust_test(
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

crubit_rust_test(
    name = "oops_test",
    srcs = ["oops.rs"],
)
