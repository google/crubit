load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_test",
)

rust_library(
    name = "two",
    srcs = ["two.rs"],
)

rust_library(
    name = "three",
    srcs = ["three.rs"],
)

rust_library(
    name = "math",
    srcs = ["math.rs"],
    data = [
        "three_namespaces.json",
        "two_namespaces.json",
    ],
    proc_macro_deps = [
        "//cc_import",
    ],
    rustc_env = {"CC_IMPORT_NAMESPACES": (
        "[" +
        "    \"rs_bindings_from_cc/test/cc_import/two_namespaces.json\"," +
        "    \"rs_bindings_from_cc/test/cc_import/three_namespaces.json\"" +
        "]"
    )},
    deps = [
        ":three",
        ":two",
    ],
)

rust_test(
    name = "math_test",
    crate = ":math",
    data = [
        "three_namespaces.json",
        "two_namespaces.json",
    ],
)
