package(default_applicable_licenses = ["//:license"])

# Intentionally not use crubit_cc_library, which enables Crubit on the target, to demonstrate that
# having a non-Crubit-enabled dependency named "core" is enough to cause build error.
cc_library(
    name = "core",
    hdrs = ["header.h"],
    visibility = [
        "//rs_bindings_from_cc/test/cpp_reserved_target_name:__subpackages__",
    ],
)
