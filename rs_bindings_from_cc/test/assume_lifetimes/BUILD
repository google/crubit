load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load(
    "//rs_bindings_from_cc/test/golden:golden_test.bzl",
    "golden_test",
)

package(default_applicable_licenses = ["//:license"])

cc_library(
    name = "test_annotations",
    hdrs = ["test_annotations.h"],
)

crubit_test_cc_library(
    name = "free_function",
    srcs = ["free_function.cc"],
    hdrs = ["free_function.h"],
    aspect_hints = [
        "//features:assume_lifetimes",
        "//features:supported",
    ],
)

rust_api_from_cpp(
    name = "free_function_rust",
    cpp_target = ":free_function",
)

golden_test(
    name = "free_function_golden_test",
    basename = "free_function",
    cc_library = "free_function",
    golden_cc = "free_function_api_impl.cc",
    golden_rs = "free_function_rs_api.rs",
)

crubit_rust_test(
    name = "free_function_test",
    srcs = ["free_function_test.rs"],
    deps = [
        ":free_function_rust",
        "//support:cref",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "member_function",
    srcs = ["member_function.cc"],
    hdrs = ["member_function.h"],
    aspect_hints = [
        "//features:assume_lifetimes",
        "//features:supported",
    ],
)

rust_api_from_cpp(
    name = "member_function_rust",
    cpp_target = ":member_function",
)

golden_test(
    name = "member_function_golden_test",
    basename = "member_function",
    cc_library = "member_function",
    golden_cc = "member_function_api_impl.cc",
    golden_rs = "member_function_rs_api.rs",
)

crubit_rust_test(
    name = "member_function_test",
    srcs = ["member_function_test.rs"],
    deps = [
        ":member_function_rust",
        "//support:cref",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "simple_string_view",
    srcs = ["simple_string_view.cc"],
    hdrs = ["simple_string_view.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [":test_annotations"],
)

rust_api_from_cpp(
    name = "simple_string_view_rust",
    cpp_target = ":simple_string_view",
)

golden_test(
    name = "simple_string_view_golden_test",
    basename = "simple_string_view",
    cc_library = "simple_string_view",
    golden_cc = "simple_string_view_api_impl.cc",
    golden_rs = "simple_string_view_rs_api.rs",
)

crubit_rust_test(
    name = "simple_string_view_test",
    srcs = ["simple_string_view_test.rs"],
    deps = [
        ":simple_string_view_rust",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "crefs",
    srcs = ["crefs.cc"],
    hdrs = ["crefs.h"],
    aspect_hints = [
        "//features:supported",
    ],
)

rust_api_from_cpp(
    name = "crefs_rust",
    cpp_target = ":crefs",
)

golden_test(
    name = "crefs_golden_test",
    basename = "crefs",
    cc_library = "crefs",
    golden_cc = "crefs_api_impl.cc",
    golden_rs = "crefs_rs_api.rs",
)

crubit_rust_test(
    name = "crefs_test",
    srcs = ["crefs_test.rs"],
    deps = [
        ":crefs_rust",
        "//support:cref",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "assumed_span",
    srcs = ["assumed_span.cc"],
    hdrs = ["assumed_span.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = ["@abseil-cpp//absl/types:span"],
)

rust_api_from_cpp(
    name = "assumed_span_rust",
    cpp_target = ":assumed_span",
)

rust_api_from_cpp(
    name = "span_rust",
    cpp_target = "@abseil-cpp//absl/types:span",
)

crubit_rust_test(
    name = "assumed_span_test",
    srcs = ["assumed_span_test.rs"],
    deps = [
        ":assumed_span_rust",
        ":span_rust",
        "//support:cref",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "string_view",
    srcs = ["string_view.cc"],
    hdrs = ["string_view.h"],
    aspect_hints = [
        "//features:assume_lifetimes",
        "//features:supported",
    ],
)

rust_api_from_cpp(
    name = "string_view_rust",
    cpp_target = ":string_view",
)

golden_test(
    name = "string_view_golden_test",
    basename = "string_view",
    cc_library = "string_view",
    golden_cc = "string_view_api_impl.cc",
    golden_rs = "string_view_rs_api.rs",
)

crubit_rust_test(
    name = "string_view_test",
    srcs = ["string_view_test.rs"],
    deps = [
        ":string_view_rust",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "struct_with_lifetimebound",
    srcs = ["struct_with_lifetimebound.cc"],
    hdrs = ["struct_with_lifetimebound.h"],
    aspect_hints = [
        "//features:assume_lifetimes",
        "//features:supported",
    ],
)

rust_api_from_cpp(
    name = "struct_with_lifetimebound_rust",
    cpp_target = ":struct_with_lifetimebound",
)

golden_test(
    name = "struct_with_lifetimebound_golden_test",
    basename = "struct_with_lifetimebound",
    cc_library = "struct_with_lifetimebound",
    golden_cc = "struct_with_lifetimebound_api_impl.cc",
    golden_rs = "struct_with_lifetimebound_rs_api.rs",
)

crubit_rust_test(
    name = "struct_with_lifetimebound_test",
    srcs = ["struct_with_lifetimebound_test.rs"],
    deps = [
        ":struct_with_lifetimebound_rust",
        "//support:cref",
        "//support:ctor",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "view_struct_with_lifetimebound",
    srcs = ["view_struct_with_lifetimebound.cc"],
    hdrs = ["view_struct_with_lifetimebound.h"],
    aspect_hints = [
        "//features:assume_lifetimes",
        "//features:supported",
    ],
)

rust_api_from_cpp(
    name = "view_struct_with_lifetimebound_rust",
    cpp_target = ":view_struct_with_lifetimebound",
)

golden_test(
    name = "view_struct_with_lifetimebound_golden_test",
    basename = "view_struct_with_lifetimebound",
    cc_library = "view_struct_with_lifetimebound",
    golden_cc = "view_struct_with_lifetimebound_api_impl.cc",
    golden_rs = "view_struct_with_lifetimebound_rs_api.rs",
)

crubit_rust_test(
    name = "view_struct_with_lifetimebound_test",
    srcs = ["view_struct_with_lifetimebound_test.rs"],
    deps = [
        ":view_struct_with_lifetimebound_rust",
        "//support:cref",
        "//support:ctor",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "type_alias_target_without_assume_lifetimes",
    hdrs = ["type_alias_target_without_assume_lifetimes.h"],
    aspect_hints = [
        "//features:no_assume_lifetimes",
        "//features:supported",
    ],
)

crubit_test_cc_library(
    name = "type_alias",
    srcs = ["type_alias.cc"],
    hdrs = ["type_alias.h"],
    aspect_hints = [
        "//features:assume_lifetimes",
        "//features:supported",
    ],
    deps = [
        ":type_alias_target_without_assume_lifetimes",
    ],
)

rust_api_from_cpp(
    name = "type_alias_rust",
    cpp_target = ":type_alias",
)

golden_test(
    name = "type_alias_golden_test",
    basename = "type_alias",
    cc_library = "type_alias",
    golden_cc = "type_alias_api_impl.cc",
    golden_rs = "type_alias_rs_api.rs",
)

crubit_rust_test(
    name = "type_alias_test",
    srcs = ["type_alias_test.rs"],
    deps = [
        ":type_alias_rust",
        "//support:ctor",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)
