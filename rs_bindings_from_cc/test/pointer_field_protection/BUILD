"""End-to-end test of Rust moves of C++ types that use pointer field protection (PFP)."""

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
    # `-tagged` makes the pointer encoding depend on the address of the object
    # that contains the pointer field.
    #
    # It's okay that only this library (and its generated bindings and thunks,
    # which reuse these copts) uses these flags.  This is enough for this test,
    # because no other PFP-affected type crosses the library boundary.
    copts = select({
        "@platforms//cpu:aarch64": [
            "-fexperimental-pointer-field-protection-abi",
            "-fexperimental-pointer-field-protection-tagged",
        ],
        "//conditions:default": [],
    }),

    # The flags above change the language options, so the prebuilt header
    # modules (e.g. the one for the C++ standard library) cannot be used.
    features = ["-use_header_modules"],
)

crubit_rust_test(
    name = "pointer_field_protection_test",
    srcs = ["pointer_field_protection_test.rs"],
    cc_deps = [":pointer_field_protection"],

    # TODO(b/565847600): Remove `tags = ["manual"]` after fixing the bug / UB.
    #
    # `test_get_after_rust_move` fails on AArch64 with SIGILL in
    # `__emupac_autda_impl`, because Crubit lets Rust `memcpy` a type whose
    # pointer field is encoded with the object address.  See the bug for repro
    # steps.
    #
    # `-c opt` is necessary: without optimizations, Clang turns the PFP
    # instructions into NOPs (see "deactivation symbols" in
    # `clang/docs/StructureProtection.md`), and the test passes.
    tags = ["manual"],
    deps = [
        "@crate_index//:googletest",
    ],
)
