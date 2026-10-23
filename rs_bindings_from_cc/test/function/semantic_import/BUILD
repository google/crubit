"""End-to-end example of using a simple inline function."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "semantic_import_upcast",
    hdrs = ["semantic_import.h"],
    aspect_hints = [
        "//features:oo_casting",  # for oops::Upcast
        "//features:thunkless_accessors",
    ],
)

rust_api_from_cpp(
    name = "semantic_import_upcast_rust",
    cpp_target = ":semantic_import_upcast",
)

cc_library(
    name = "semantic_import",
    hdrs = ["semantic_import.h"],
    aspect_hints = [
        "//features:supported",
        # for `Incomplete*`, which is otherwise not bound at all.
        "//features:forward_declare",
        "//features:thunkless_accessors",
    ],
)

rust_api_from_cpp(
    name = "semantic_import_rust",
    cpp_target = ":semantic_import",
)

golden_test(
    name = "semantic_import_golden_test",
    basename = "semantic_import",
    cc_library = "semantic_import",
    golden_cc = "semantic_import_api_impl.cc",
    golden_rs = "semantic_import_rs_api.rs",
)

golden_test(
    name = "semantic_import_upcast_golden_test",
    basename = "semantic_import_upcast",
    cc_library = "semantic_import_upcast",
    golden_cc = "semantic_import_upcast_api_impl.cc",
    golden_rs = "semantic_import_upcast_rs_api.rs",
)

crubit_rust_test(
    name = "semantic_import_upcast_test",
    srcs = ["semantic_import_upcast_test.rs"],
    deps = [
        ":semantic_import_upcast_rust",
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)

crubit_rust_test(
    name = "semantic_import_test",
    srcs = ["semantic_import_test.rs"],
    proc_macro_deps = ["//common:item_exists"],
    deps = [
        ":semantic_import_rust",
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "slices",
    hdrs = ["slices.h"],
    aspect_hints = [
        "//features:supported",
        "//features:thunkless_accessors",
    ],
    deps = ["//support/rs_std:slice_ref"],
)

rust_api_from_cpp(
    name = "slices_rust",
    cpp_target = ":slices",
)

crubit_rust_test(
    name = "slices_test",
    srcs = ["slices_test.rs"],
    deps = [
        ":slices_rust",
        "@crate_index//:googletest",
    ],
)
