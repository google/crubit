package(default_applicable_licenses = ["//:license"])

# Intentionally not using crubit_test_cc_library for `A` and `B`, which would enable Crubit.
cc_library(
    name = "A",
)

cc_library(
    name = "B",
    visibility = [
        "//rs_bindings_from_cc/test/cpp_duplicate_target_name:__subpackages__",
    ],
    deps = [
        ":A",
    ],
)
