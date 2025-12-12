# This end-to-end test shows that we still generate correct bindings when layering check is
# disabled. Rust equivalent of layering check is still enforced when compiling user Rust code,
# but to be safe we work around it for generated bindings Rust code.

load("//common:crubit_wrapper_macros_oss.bzl", "crubit_cc_test", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_rust_test(
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
    deps = [
        "@crates//:googletest",
    ],
)

crubit_test_cc_library(
    name = "top_lib",
    srcs = ["top_lib.cc"],
    hdrs = ["top_lib.h"],
    features = ["-layering_check"],
    deps = [":middleperson_without_public_headers"],
)

crubit_test_cc_library(
    name = "middleperson_without_public_headers",
    deps = [":my_lib"],
)

crubit_test_cc_library(
    name = "my_lib",
    srcs = ["my_lib.cc"],
    hdrs = ["my_lib.h"],
)

# This test only exists to make sure :top_lib is disabling layering check correctly (layering
# checks are only validated on binary rules, not on library rules)
crubit_cc_test(
    name = "layering_check_validation_test",
    srcs = ["layering_check_validation_test.cc"],
    deps = [":top_lib"],
)
