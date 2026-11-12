load("@rules_rust//rust:defs.bzl", "rust_test")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "basic_structs",
    srcs = ["basic_structs.c"],
    hdrs = ["basic_structs.h"],
)

rust_api_from_cpp(
    name = "basic_structs_rust",
    cpp_target = ":basic_structs",
)

crubit_rust_test(
    name = "basic_structs_test",
    srcs = ["basic_structs_test.rs"],
    deps = [
        ":basic_structs_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "nested_anonymous",
    srcs = ["nested_anonymous.c"],
    hdrs = ["nested_anonymous.h"],
)

rust_api_from_cpp(
    name = "nested_anonymous_rust",
    cpp_target = ":nested_anonymous",
)

crubit_rust_test(
    name = "nested_anonymous_test",
    srcs = ["nested_anonymous_test.rs"],
    deps = [
        ":nested_anonymous_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "unions",
    srcs = ["unions.c"],
    hdrs = ["unions.h"],
)

rust_api_from_cpp(
    name = "unions_rust",
    cpp_target = ":unions",
)

crubit_rust_test(
    name = "unions_test",
    srcs = ["unions_test.rs"],
    deps = [
        ":unions_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "opaque_types",
    srcs = ["opaque_types.c"],
    hdrs = ["opaque_types.h"],
)

rust_api_from_cpp(
    name = "opaque_types_rust",
    cpp_target = ":opaque_types",
)

crubit_rust_test(
    name = "opaque_types_test",
    srcs = ["opaque_types_test.rs"],
    deps = [
        ":opaque_types_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "self_referential",
    srcs = ["self_referential.c"],
    hdrs = ["self_referential.h"],
)

rust_api_from_cpp(
    name = "self_referential_rust",
    cpp_target = ":self_referential",
)

crubit_rust_test(
    name = "self_referential_test",
    srcs = ["self_referential_test.rs"],
    deps = [
        ":self_referential_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "mutually_referential",
    srcs = ["mutually_referential.c"],
    hdrs = ["mutually_referential.h"],
)

rust_api_from_cpp(
    name = "mutually_referential_rust",
    cpp_target = ":mutually_referential",
)

crubit_rust_test(
    name = "mutually_referential_test",
    srcs = ["mutually_referential_test.rs"],
    deps = [
        ":mutually_referential_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "typedef_anon",
    srcs = ["typedef_anon.c"],
    hdrs = ["typedef_anon.h"],
)

rust_api_from_cpp(
    name = "typedef_anon_rust",
    cpp_target = ":typedef_anon",
)

crubit_rust_test(
    name = "typedef_anon_test",
    srcs = ["typedef_anon_test.rs"],
    deps = [
        ":typedef_anon_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "same_name_tag",
    srcs = ["same_name_tag.c"],
    hdrs = ["same_name_tag.h"],
)

rust_api_from_cpp(
    name = "same_name_tag_rust",
    cpp_target = ":same_name_tag",
)

crubit_rust_test(
    name = "same_name_tag_test",
    srcs = ["same_name_tag_test.rs"],
    deps = [
        ":same_name_tag_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "enums",
    srcs = ["enums.c"],
    hdrs = ["enums.h"],
)

rust_api_from_cpp(
    name = "enums_rust",
    cpp_target = ":enums",
)

crubit_rust_test(
    name = "enums_test",
    srcs = ["enums_test.rs"],
    deps = [
        ":enums_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "bitfields",
    srcs = ["bitfields.c"],
    hdrs = ["bitfields.h"],
)

rust_api_from_cpp(
    name = "bitfields_rust",
    cpp_target = ":bitfields",
)

crubit_rust_test(
    name = "bitfields_test",
    srcs = ["bitfields_test.rs"],
    deps = [
        ":bitfields_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "bitfields_known_failure",
    srcs = ["bitfields_known_failure.c"],
    hdrs = ["bitfields_known_failure.h"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_api_from_cpp(
    name = "bitfields_known_failure_rust",
    cpp_target = ":bitfields_known_failure",
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

# Uses `rust_test` instead of `crubit_rust_test` because `crubit_rust_test`
# calls `inject_cross_compile_tests`, which generates non-manual, non-notap
# cross-compile `build_test` targets that fail TAP. Reproduce manual targets
# with `--config=crubit-unstable` to test Crubit-at-HEAD.
rust_test(
    name = "bitfields_known_failure_test",
    srcs = ["bitfields_known_failure_test.rs"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
    deps = [
        ":bitfields_known_failure_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "flexible_arrays",
    srcs = ["flexible_arrays.c"],
    hdrs = ["flexible_arrays.h"],
)

rust_api_from_cpp(
    name = "flexible_arrays_rust",
    cpp_target = ":flexible_arrays",
)

crubit_rust_test(
    name = "flexible_arrays_test",
    srcs = ["flexible_arrays_test.rs"],
    deps = [
        ":flexible_arrays_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "zero_length_arrays",
    srcs = ["zero_length_arrays.c"],
    hdrs = ["zero_length_arrays.h"],
)

rust_api_from_cpp(
    name = "zero_length_arrays_rust",
    cpp_target = ":zero_length_arrays",
)

crubit_rust_test(
    name = "zero_length_arrays_test",
    srcs = ["zero_length_arrays_test.rs"],
    deps = [
        ":zero_length_arrays_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "alignment",
    srcs = ["alignment.c"],
    hdrs = ["alignment.h"],
)

rust_api_from_cpp(
    name = "alignment_rust",
    cpp_target = ":alignment",
)

crubit_rust_test(
    name = "alignment_test",
    srcs = ["alignment_test.rs"],
    deps = [
        ":alignment_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "alignment_known_failure",
    srcs = ["alignment_known_failure.c"],
    hdrs = ["alignment_known_failure.h"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_api_from_cpp(
    name = "alignment_known_failure_rust",
    cpp_target = ":alignment_known_failure",
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_test(
    name = "alignment_known_failure_test",
    srcs = ["alignment_known_failure_test.rs"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
    deps = [
        ":alignment_known_failure_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "functions",
    srcs = ["functions.c"],
    hdrs = ["functions.h"],
)

rust_api_from_cpp(
    name = "functions_rust",
    cpp_target = ":functions",
)

crubit_rust_test(
    name = "functions_test",
    srcs = ["functions_test.rs"],
    deps = [
        ":functions_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "static_inline",
    srcs = ["static_inline.c"],
    hdrs = ["static_inline.h"],
)

rust_api_from_cpp(
    name = "static_inline_rust",
    cpp_target = ":static_inline",
)

crubit_rust_test(
    name = "static_inline_test",
    srcs = ["static_inline_test.rs"],
    deps = [
        ":static_inline_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "globals",
    srcs = ["globals.c"],
    hdrs = ["globals.h"],
)

rust_api_from_cpp(
    name = "globals_rust",
    cpp_target = ":globals",
)

crubit_rust_test(
    name = "globals_test",
    srcs = ["globals_test.rs"],
    deps = [
        ":globals_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "macros",
    srcs = ["macros.c"],
    hdrs = ["macros.h"],
    aspect_hints = [
        "//features:macro_constants",
        "//features:supported",
    ],
)

rust_api_from_cpp(
    name = "macros_rust",
    cpp_target = ":macros",
)

crubit_rust_test(
    name = "macros_test",
    srcs = ["macros_test.rs"],
    deps = [
        ":macros_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "types",
    srcs = ["types.c"],
    hdrs = ["types.h"],
)

rust_api_from_cpp(
    name = "types_rust",
    cpp_target = ":types",
)

crubit_rust_test(
    name = "types_test",
    srcs = ["types_test.rs"],
    deps = [
        ":types_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "types_known_failure",
    srcs = ["types_known_failure.c"],
    hdrs = ["types_known_failure.h"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_api_from_cpp(
    name = "types_known_failure_rust",
    cpp_target = ":types_known_failure",
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_test(
    name = "types_known_failure_test",
    srcs = ["types_known_failure_test.rs"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
    deps = [
        ":types_known_failure_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "qualifiers",
    srcs = ["qualifiers.c"],
    hdrs = ["qualifiers.h"],
)

rust_api_from_cpp(
    name = "qualifiers_rust",
    cpp_target = ":qualifiers",
)

crubit_rust_test(
    name = "qualifiers_test",
    srcs = ["qualifiers_test.rs"],
    deps = [
        ":qualifiers_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "qualifiers_known_failure",
    srcs = ["qualifiers_known_failure.c"],
    hdrs = ["qualifiers_known_failure.h"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_api_from_cpp(
    name = "qualifiers_known_failure_rust",
    cpp_target = ":qualifiers_known_failure",
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_test(
    name = "qualifiers_known_failure_test",
    srcs = ["qualifiers_known_failure_test.rs"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
    deps = [
        ":qualifiers_known_failure_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "keywords",
    srcs = ["keywords.c"],
    hdrs = ["keywords.h"],
)

rust_api_from_cpp(
    name = "keywords_rust",
    cpp_target = ":keywords",
)

crubit_rust_test(
    name = "keywords_test",
    srcs = ["keywords_test.rs"],
    deps = [
        ":keywords_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "keywords_known_failure",
    srcs = ["keywords_known_failure.c"],
    hdrs = ["keywords_known_failure.h"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_api_from_cpp(
    name = "keywords_known_failure_rust",
    cpp_target = ":keywords_known_failure",
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_test(
    name = "keywords_known_failure_test",
    srcs = ["keywords_known_failure_test.rs"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
    deps = [
        ":keywords_known_failure_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "no_extern_c_known_failure",
    srcs = ["no_extern_c_known_failure.c"],
    hdrs = ["no_extern_c_known_failure.h"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_api_from_cpp(
    name = "no_extern_c_known_failure_rust",
    cpp_target = ":no_extern_c_known_failure",
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_test(
    name = "no_extern_c_known_failure_test",
    srcs = ["no_extern_c_known_failure_test.rs"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
    deps = [
        ":no_extern_c_known_failure_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "static_assert",
    srcs = ["static_assert.c"],
    hdrs = ["static_assert.h"],
)

rust_api_from_cpp(
    name = "static_assert_rust",
    cpp_target = ":static_assert",
)

crubit_rust_test(
    name = "static_assert_test",
    srcs = ["static_assert_test.rs"],
    deps = [
        ":static_assert_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "static_assert_known_failure",
    srcs = ["static_assert_known_failure.c"],
    hdrs = ["static_assert_known_failure.h"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_api_from_cpp(
    name = "static_assert_known_failure_rust",
    cpp_target = ":static_assert_known_failure",
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
)

rust_test(
    name = "static_assert_known_failure_test",
    srcs = ["static_assert_known_failure_test.rs"],
    tags = [
        "manual",
        "nobuilder",
        "notap",
    ],
    deps = [
        ":static_assert_known_failure_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "attributes",
    srcs = ["attributes.c"],
    hdrs = ["attributes.h"],
)

rust_api_from_cpp(
    name = "attributes_rust",
    cpp_target = ":attributes",
)

crubit_rust_test(
    name = "attributes_test",
    srcs = ["attributes_test.rs"],
    deps = [
        ":attributes_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "nested_scope",
    srcs = ["nested_scope.c"],
    hdrs = ["nested_scope.h"],
)

rust_api_from_cpp(
    name = "nested_scope_rust",
    cpp_target = ":nested_scope",
)

crubit_rust_test(
    name = "nested_scope_test",
    srcs = ["nested_scope_test.rs"],
    deps = [
        ":nested_scope_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "pointers",
    srcs = ["pointers.c"],
    hdrs = ["pointers.h"],
)

rust_api_from_cpp(
    name = "pointers_rust",
    cpp_target = ":pointers",
)

crubit_rust_test(
    name = "pointers_test",
    srcs = ["pointers_test.rs"],
    deps = [
        ":pointers_rust",
        "//support/ffi_11",
        "@crate_index//:googletest",
    ],
)
