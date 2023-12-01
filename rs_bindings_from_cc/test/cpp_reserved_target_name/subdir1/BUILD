package(default_applicable_licenses = ["//:license"])

# Intentionally not use crubit_cc_library, which enables Crubit on the target, as a regression test
# that Crubit should be able to handle 'core' as a Crubit-disabled cc_library.
cc_library(
    name = "core",
    hdrs = ["header.h"],
    visibility = [
        "//rs_bindings_from_cc/test/cpp_reserved_target_name:__subpackages__",
    ],
)
