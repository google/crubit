load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

#
# Test: Crubit is disabled for all cc_library targets.
#
cc_library(
    # https://bazel.build/concepts/labels#target-names, with the following modifications:
    # - Removed `"` and `'`, since .cppmap disallows them in module names, as they aren't valid C++
    # targets and we won't run Crubit on them.
    # - Moved `/.` away from the end, since bazel silently disallows `/.` at the end of the
    # the target (and it truncates `/.` from the target name).
    name = "!./%-@^#$&()*-+,;<=>?[]{|}~",
    hdrs = ["header.h"],
)

cc_library(
    name = "intermediate",
    deps = [
        ":!./%-@^#$&()*-+,;<=>?[]{|}~",
    ],
)

cc_library(
    name = "crubit_disabled_cc_library_with_invalid_character_in_transitive_deps",
    deps = [
        ":intermediate",
    ],
)

crubit_rust_test(
    name = "test_with_cc_library_with_invalid_character_in_transitive_deps",
    srcs = ["test_with_cc_library_with_invalid_character_in_transitive_deps.rs"],
    cc_deps = [
        ":crubit_disabled_cc_library_with_invalid_character_in_transitive_deps",
    ],
)

crubit_test_cc_library(
    name = "crubit_enabled_cc_library_with_invalid_character_in_transitive_deps",
    deps = [
        ":intermediate",
    ],
)

crubit_rust_test(
    name = "test_with_crubit_enabled_cc_library_with_invalid_character_in_transitive_deps",
    srcs = ["test_with_crubit_enabled_cc_library_with_invalid_character_in_transitive_deps.rs"],
    cc_deps = [
        ":crubit_enabled_cc_library_with_invalid_character_in_transitive_deps",
    ],
)

crubit_test_cc_library(
    name = "crubit_enabled_cc_library_with_!./%-@^#$&()*-+,;<=>?[]{|}~",
    hdrs = ["header.h"],
)

crubit_rust_test(
    name = "test_crubit_enabled_cc_library_with_invalid_character",
    srcs = ["test_with_crubit_enabled_cc_library_with_invalid_character.rs"],
    cc_deps = [
        ":crubit_enabled_cc_library_with_!./%-@^#$&()*-+,;<=>?[]{|}~",
    ],
)
