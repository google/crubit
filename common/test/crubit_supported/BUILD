load(
    "//common/test/crubit_supported:test.bzl",
    "check_expected_file",
    "check_unsupported_expected_file",
)

package(
    default_applicable_licenses = ["//:license"],
)

# This is used by `//common/test/crubit_supported:test.bzl` to force an
# unsupported platform for testing below.
platform(
    name = "unsupported_platform",
    constraint_values = [
        "@platforms//os:android",
        "@platforms//cpu:aarch64",
    ],
)

filegroup(
    name = "selected_via_crubit_supported",
    srcs = select({
        "//:crubit_supported": ["supported.txt"],
        "//conditions:default": ["unsupported.txt"],
    }),
)

filegroup(
    name = "selected_via_supported_platforms",
    srcs = select({
        "//:supported_platforms": ["supported.txt"],
        "//conditions:default": ["unsupported.txt"],
    }),
)

check_expected_file(
    name = "test_default_crubit_supported",
    src = ":selected_via_crubit_supported",
    expected_file = "supported.txt",
)

check_expected_file(
    name = "test_default_supported_platforms",
    src = ":selected_via_supported_platforms",
    expected_file = "supported.txt",
)

check_unsupported_expected_file(
    name = "test_transition_crubit_supported",
    src = ":selected_via_crubit_supported",
    expected_file = "unsupported.txt",
)

check_unsupported_expected_file(
    name = "test_transition_supported_platforms",
    src = ":selected_via_supported_platforms",
    expected_file = "unsupported.txt",
)
