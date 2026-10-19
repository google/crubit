load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(
    # Build and use precompiled header modules (where supported).
    features = ["header_modules"],
)

licenses(["notice"])

# `template_ns::Box<int>` is implicitly instantiated independently in the precompiled modules of
# `:a` and `:b`, which both declare its implicit copy constructor.
#
# When `:lib`'s header analysis loads both modules, Clang merges the two definitions of `Box<int>`:
# it keeps one of them and demotes the other to a declaration. Members of the demoted definition
# (including implicit special members that only it declares) stay lexically inside it, and members
# of the kept definition may have their canonical declaration in the demoted one. Crubit must still
# find all of the members, so that `Box<int>` is bound as a movable type with all its methods.
crubit_test_cc_library(
    name = "lib",
    hdrs = ["lib.h"],
    deps = [
        ":a",
        ":b",
        ":template",
    ],
)

crubit_test_cc_library(
    name = "a",
    hdrs = ["a.h"],
    deps = [":template"],
)

crubit_test_cc_library(
    name = "b",
    hdrs = ["b.h"],
    deps = [":template"],
)

crubit_test_cc_library(
    name = "template",
    hdrs = ["template.h"],
    deps = ["//support:annotations"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":lib"],
    deps = [
        "@crate_index//:googletest",
    ],
)
