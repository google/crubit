"""End-to-end tests of Rust moves of, and accessors for, C++ types that use pointer field protection (PFP)."""

load("@rules_shell//shell:sh_test.bzl", "sh_test")
load("//bazel:rust_api_from_cpp.bzl", "rust_api_from_cpp")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "pointer_field_protection",
    srcs = ["pointer_field_protection.cc"],
    hdrs = ["pointer_field_protection.h"],

    # PFP is only supported on AArch64.
    #
    # `-abi` is what the `pointer_field_protection` crosstool feature passes.
    #
    # Crubit rejects `-fexperimental-pointer-field-protection-tagged` because
    # pointer fields are encoded with the object's address, so no type could be
    # safely Rust-moved.
    #
    # It's okay that only this library (and its generated bindings and thunks,
    # which reuse these copts) uses these flags.  This is enough for this test,
    # because no other PFP-affected type crosses the library boundary.
    copts = select({
        "@platforms//cpu:aarch64": [
            "-fexperimental-pointer-field-protection-abi",
        ],
        "//conditions:default": [],
    }),

    # The flags above change the language options, so the prebuilt header
    # modules (e.g. the one for the C++ standard library) cannot be used.
    features = ["-use_header_modules"],
)

rust_api_from_cpp(
    name = "pointer_field_protection_rust",
    cpp_target = ":pointer_field_protection",
)

crubit_rust_test(
    name = "pointer_field_protection_test",
    srcs = ["pointer_field_protection_test.rs"],

    # PFP only has an effect on AArch64, and only with `-c opt` (see above), so
    # this test only checks anything with `--cpu=arm -c opt`.
    tags = ["manual"],
    deps = [
        ":pointer_field_protection_rust",
        "@crate_index//:googletest",
    ],
)

sh_test(
    name = "tagged_failure_test",
    srcs = ["tagged_failure_test.sh"],
    args = [
        "$(rootpath //rs_bindings_from_cc)",
        "$(rootpath pointer_field_protection.h)",
    ],
    data = [
        "pointer_field_protection.h",
        "//rs_bindings_from_cc",
    ],
    tags = [
        "not_run:arm",  # We don't need to run Crubit itself on aarch64.
        "not_run:mac",
    ],
)

crubit_test_cc_library(
    name = "accessors",
    srcs = ["accessors.cc"],
    hdrs = ["accessors.h"],
    aspect_hints = [
        "//features:supported",
        "//features:thunkless_accessors",
    ],

    # Only `-abi`, which is what the `pointer_field_protection` crosstool
    # feature passes. See `pointer_field_protection` above for the other flags.
    copts = select({
        "@platforms//cpu:aarch64": [
            "-fexperimental-pointer-field-protection-abi",
        ],
        "//conditions:default": [],
    }),

    # The flag above changes the language options, so the prebuilt header
    # modules (e.g. the one for the C++ standard library) cannot be used.
    features = ["-use_header_modules"],
)

rust_api_from_cpp(
    name = "accessors_rust",
    cpp_target = ":accessors",
)

crubit_rust_test(
    name = "accessors_test",
    srcs = ["accessors_test.rs"],

    # PFP only has an effect on AArch64, and only with `-c opt` (see above), so
    # this test only checks anything with `--cpu=arm -c opt`.
    tags = ["manual"],
    deps = [
        ":accessors_rust",
        "@crate_index//:googletest",
    ],
)
