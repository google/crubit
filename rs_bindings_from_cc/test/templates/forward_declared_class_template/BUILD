load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//:license"])

crubit_test_cc_library(
    name = "use_forward_declared_template",
    hdrs = ["use_forward_declared_template.h"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test_forward_declaration.rs"],
    cc_deps = [
        ":use_forward_declared_template",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "definition_forward_declared_template_with_crubit_disabled",
    hdrs = ["definition_forward_declared_template.h"],
    aspect_hints = [],
    deps = [
        ":use_forward_declared_template",
    ],
)

# The bindings generated for `definition_forward_declared_template.h` doesn't compile (when Crubit
# is enabled), but we can now import this header with Crubit disabled.
crubit_rust_test(
    name = "test_definition_with_crubit_disabled",
    srcs = ["test_definition_with_crubit_disabled.rs"],
    cc_deps = [
        ":definition_forward_declared_template_with_crubit_disabled",
        ":use_forward_declared_template",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

crubit_test_cc_library(
    name = "definition_forward_declared_template",
    hdrs = ["definition_forward_declared_template.h"],
    deps = [
        ":use_forward_declared_template",
    ],
)

crubit_rust_test(
    name = "test_definition",
    srcs = ["test_definition.rs"],
    cc_deps = [
        ":definition_forward_declared_template",
        ":use_forward_declared_template",
    ],
    tags = [
        # TODO(b/318690257): Enable when fixed.
        "manual",
        "nobuilder",
        "notap",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)
