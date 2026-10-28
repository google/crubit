load("//common:crubit_wrapper_macros_oss.bzl", "crubit_rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")
load("//rs_bindings_from_cc/test/golden:golden_test.bzl", "golden_test")

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

# Checks the bindings of `:lib`. In particular, they must contain the ordinary `//` comments of
# `shared.h`. When header modules are used, these comments aren't in the precompiled header
# modules, so `rs_bindings_from_cc` reads them from the source of the headers instead.
golden_test(
    name = "lib_golden_test",
    basename = "lib",
    cc_library = ":lib",
    golden_rs = "lib_rs_api.rs",
)
