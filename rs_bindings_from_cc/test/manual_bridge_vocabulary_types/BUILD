load("@rules_cc//cc:cc_library.bzl", "cc_library")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load(
    "//common:crubit_wrapper_macros_oss.bzl",
    "crubit_rust_test",
)

licenses(["notice"])

cc_library(
    name = "common",
    hdrs = ["common.h"],
    aspect_hints = [
        "//features:supported",
    ],
)

rust_api_from_cpp(
    name = "common_rust",
    cpp_target = ":common",
)

cc_library(
    name = "crubit_disabled",
    hdrs = ["crubit_disabled.h"],
)

cc_library(
    name = "unique_ptr_lib",
    hdrs = ["unique_ptr_lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        ":common",
        ":crubit_disabled",
        "//support:annotations",
        "@abseil-cpp//absl/base:nullability",
    ],
)

rust_api_from_cpp(
    name = "unique_ptr_lib_rust",
    cpp_target = ":unique_ptr_lib",
)

crubit_rust_test(
    name = "unique_ptr_test",
    srcs = ["unique_ptr_test.rs"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        ":common_rust",
        ":unique_ptr_lib_rust",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "atomic_lib",
    hdrs = ["atomic_lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
)

rust_api_from_cpp(
    name = "atomic_lib_rust",
    cpp_target = ":atomic_lib",
)

crubit_rust_test(
    name = "atomic_test",
    srcs = ["atomic_test.rs"],
    deps = [
        ":atomic_lib_rust",
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "vector_lib",
    hdrs = ["vector_lib.h"],
    aspect_hints = [
        "//features:supported",
    ],
    deps = [
        ":common",
        ":crubit_disabled",
        "//support:annotations",
    ],
)

rust_api_from_cpp(
    name = "vector_lib_rust",
    cpp_target = ":vector_lib",
)

crubit_rust_test(
    name = "vector_test",
    srcs = ["vector_test.rs"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        ":common_rust",
        ":vector_lib_rust",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "nonnull_smart_pointer_lib",
    # Deliberately two headers in one library: `#pragma nullability file_default`
    # (via `ABSL_POINTERS_DEFAULT_NONNULL`) applies per file, not per target.
    # Only `default_nonnull_smart_pointer_lib.h` uses it, so unannotated smart
    # pointers in `nonnull_smart_pointer_lib.h` must stay nullable.
    hdrs = [
        "default_nonnull_smart_pointer_lib.h",
        "nonnull_smart_pointer_lib.h",
    ],
    aspect_hints = [
        "//features:nonnull_smart_pointers",
    ],
    deps = [
        ":common",
        "//support:annotations",
        "@abseil-cpp//absl/base:nullability",
    ],
)

rust_api_from_cpp(
    name = "nonnull_smart_pointer_lib_rust",
    cpp_target = ":nonnull_smart_pointer_lib",
)

crubit_rust_test(
    name = "nonnull_smart_pointer_test",
    srcs = ["nonnull_smart_pointer_test.rs"],
    deps = [
        ":common_rust",
        ":nonnull_smart_pointer_lib_rust",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)

cc_library(
    name = "optional_lib",
    hdrs = ["optional_lib.h"],
    aspect_hints = [
        "//features:layout_compat_optional",
        "//features:layout_compat_tuple",
    ],
    deps = [
        "//support:annotations",
        "@abseil-cpp//absl/base:nullability",
    ],
)

rust_api_from_cpp(
    name = "optional_lib_rust",
    cpp_target = ":optional_lib",
)

crubit_rust_test(
    name = "optional_test",
    srcs = ["optional_test.rs"],
    deps = [
        ":optional_lib_rust",
        "//support:ctor",
        "//support/cc_std",
        "@crate_index//:googletest",
        "@crate_index//:static_assertions",  # v1
    ],
)

cc_library(
    name = "optional_nested_pair_lib",
    hdrs = ["optional_nested_pair_lib.h"],
    aspect_hints = [
        ":optional_nested_pair_lib_rust.hint",
        "//features:layout_compat_optional",
    ],
    deps = [
        "//support:annotations",
    ],
)

rust_api_from_cpp(
    name = "optional_nested_pair_lib_rust",
    cpp_target = ":optional_nested_pair_lib",
)

crubit_rust_test(
    name = "optional_nested_pair_test",
    srcs = ["optional_nested_pair_test.rs"],
    deps = [
        ":optional_nested_pair_lib_rust",
        "//support/cc_std",
        "@crate_index//:googletest",
    ],
)
