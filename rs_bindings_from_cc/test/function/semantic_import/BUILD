"""End-to-end example of using a simple inline function."""

load("@rules_cc//cc:cc_library.bzl", "cc_library")
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

cc_library(
    name = "semantic_import",
    hdrs = ["semantic_import.h"],
    aspect_hints = [
        "//features:supported",
        "//features:thunkless_accessors",
    ],
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
    cc_deps = [":semantic_import_upcast"],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)

crubit_rust_test(
    name = "semantic_import_test",
    srcs = ["semantic_import_test.rs"],
    cc_deps = [":semantic_import"],
    proc_macro_deps = ["//common:item_exists"],
    deps = [
        "//support:ctor",
        "@crate_index//:googletest",
    ],
)
