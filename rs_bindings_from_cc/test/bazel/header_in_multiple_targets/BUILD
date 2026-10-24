load("@rules_shell//shell:sh_test.bzl", "sh_test")
load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load("//rs_bindings_from_cc/test/golden:golden_test.bzl", "generated_bindings")

package(
    # Build and use precompiled header modules (where supported).
    features = ["header_modules"],
)

licenses(["notice"])

# `shared.h` is a public header of both `:lib` and `:shared`.
#
# When header modules are used, `:lib`'s header analysis parses `shared.h` textually (as a header
# of `:lib` itself), and also loads the declarations of `shared.h` from the precompiled module of
# `:shared` (which `uses_shared.h` imports via `:user_of_shared`). Clang doesn't merge all of these
# declarations, so some of them are in the AST twice. Crubit must still bind each one only once.
crubit_test_cc_library(
    name = "lib",
    hdrs = [
        "shared.h",
        "uses_shared.h",
    ],
    deps = [
        ":shared",
        ":user_of_shared",
    ],
)

crubit_test_cc_library(
    name = "user_of_shared",
    hdrs = ["user_of_shared.h"],
    deps = [":shared"],
)

crubit_test_cc_library(
    name = "shared",
    hdrs = ["shared.h"],
)

crubit_rust_test(
    name = "test",
    srcs = ["test.rs"],
    cc_deps = [":lib"],
    proc_macro_deps = [
        "//common:item_exists",
    ],
    deps = [
        "@crate_index//:googletest",
    ],
)

generated_bindings(
    name = "lib_bindings",
    cc_library = ":lib",
)

filegroup(
    name = "lib_rust_file",
    srcs = [":lib_bindings"],
    output_group = "rust_file",
)

# Checks that ordinary comments are in the bindings of `:lib` (see `comments_test.sh`).
sh_test(
    name = "comments_test",
    srcs = ["comments_test.sh"],
    args = ["$(rootpath :lib_rust_file)"],
    data = [":lib_rust_file"],
)
