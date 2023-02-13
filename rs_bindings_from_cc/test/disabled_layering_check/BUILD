# This end-to-end test shows that we still generate correct bindings when layering check is
# disabled. Rust equivalent of layering check is still enforced when compiling user Rust code,
# but to be safe we work around it for generated bindings Rust code.

load("@rules_rust//rust:defs.bzl", "rust_test")

package(default_applicable_licenses = ["//third_party/crubit:license"])

rust_test(
    name = "disabled_layering_check_test",
    srcs = ["disabled_layering_check_test.rs"],
    cc_deps = [
        # :my_lib is needed here because user code has to respect layering check
        ":my_lib",
        # Bindings for :top_lib directly depend on bindings of :my_lib, but :my_lib is only a
        # transitive dependency which violates the layering check. This test will fail if :top_lib
        # doesn't get compiled with :my_lib as direct.
        ":top_lib",
    ],
)

cc_library(
    name = "top_lib",
    srcs = ["top_lib.cc"],
    hdrs = ["top_lib.h"],
    features = ["-layering_check"],
    deps = [":middleperson_without_public_headers"],
)

cc_library(
    name = "middleperson_without_public_headers",
    deps = [":my_lib"],
)

cc_library(
    name = "my_lib",
    srcs = ["my_lib.cc"],
    hdrs = ["my_lib.h"],
)

# This test only exists to make sure :top_lib is disabling layering check correctly (layering
# checks are only validated on binary rules, not on library rules)
cc_test(
    name = "layering_check_validation_test",
    srcs = ["layering_check_validation_test.cc"],
    deps = [":top_lib"],
)
