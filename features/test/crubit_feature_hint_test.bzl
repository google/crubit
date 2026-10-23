# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Tests for `crubit_feature_hint` helper functions."""

load("@bazel_skylib//lib:unittest.bzl", "asserts", "unittest")
load(
    "//features:crubit_feature_hint.bzl",
    "denied_features_for_test",
    "matches_pattern_for_test",
)
load(
    "//features/denylists:denylist_dir.bzl",
    "denylist_path_to_entry",
    "merge_denylists",
)

def _matches_pattern_test_impl(ctx):
    env = unittest.begin(ctx)

    lbl1 = struct(package = "foo/bar", name = "baz")
    asserts.true(env, matches_pattern_for_test(lbl1, "//..."))
    asserts.true(env, matches_pattern_for_test(lbl1, "//foo/bar/..."))
    asserts.false(env, matches_pattern_for_test(lbl1, "//foo/bar:..."))
    asserts.false(env, matches_pattern_for_test(lbl1, "//foo/bar:all"))
    asserts.false(env, matches_pattern_for_test(lbl1, "//foo/bar:*"))
    asserts.true(env, matches_pattern_for_test(lbl1, "//foo/bar:baz"))
    asserts.false(env, matches_pattern_for_test(lbl1, "//foo/bar"))
    asserts.false(env, matches_pattern_for_test(lbl1, "//foo/bar:qux"))
    asserts.false(env, matches_pattern_for_test(lbl1, "//foo/baz/..."))

    lbl2 = struct(package = "foo/bar", name = "bar")
    asserts.true(env, matches_pattern_for_test(lbl2, "//foo/bar"))
    asserts.true(env, matches_pattern_for_test(lbl2, "//foo/..."))

    lbl3 = struct(package = "foo/bar/baz", name = "qux")
    asserts.true(env, matches_pattern_for_test(lbl3, "//foo/bar/..."))
    asserts.false(env, matches_pattern_for_test(lbl3, "//foo/bar:..."))
    asserts.false(env, matches_pattern_for_test(lbl3, "//foo/bar:all"))

    lbl4 = struct(package = "", name = "root_target")
    asserts.true(env, matches_pattern_for_test(lbl4, "//..."))
    asserts.true(env, matches_pattern_for_test(lbl4, "//:root_target"))
    asserts.false(env, matches_pattern_for_test(lbl4, "//:all"))
    asserts.false(env, matches_pattern_for_test(lbl4, "//foo/..."))

    return unittest.end(env)

matches_pattern_test = unittest.make(_matches_pattern_test_impl)

def _hint(package, name):
    return struct(label = struct(package = package, name = name))

def _denied_features_test_impl(ctx):
    env = unittest.begin(ctx)

    denylists = {
        "feature_a": ["//foo/..."],
        "feature_b": ["//foo/bar:baz", "//qux/..."],
    }
    lbl = struct(package = "foo/bar", name = "baz")
    other = struct(package = "other", name = "other")

    asserts.equals(env, set(["feature_a", "feature_b"]), denied_features_for_test(lbl, [], denylists))
    asserts.equals(env, set(), denied_features_for_test(other, [], denylists))
    asserts.equals(env, set(), denied_features_for_test(lbl, [], {}))

    # The feature's own aspect hint takes precedence over the denylist.
    asserts.equals(
        env,
        set(["feature_b"]),
        denied_features_for_test(lbl, [_hint("features", "feature_a")], denylists),
    )
    asserts.equals(
        env,
        set(["feature_a"]),
        denied_features_for_test(lbl, [_hint("features/internal", "testonly_feature_b")], denylists),
    )

    # Other hints, including `:supported`, are not an explicit opt-in.
    asserts.equals(
        env,
        set(["feature_a", "feature_b"]),
        denied_features_for_test(lbl, [_hint("features", "supported")], denylists),
    )
    asserts.equals(
        env,
        set(["feature_a", "feature_b"]),
        denied_features_for_test(lbl, [_hint("some/other/package", "feature_a")], denylists),
    )

    return unittest.end(env)

denied_features_test = unittest.make(_denied_features_test_impl)

def _denylist_path_to_entry_test_impl(ctx):
    env = unittest.begin(ctx)

    asserts.equals(env, ("f", "//foo/bar:baz"), denylist_path_to_entry("f/foo/bar/baz.json"))
    asserts.equals(env, ("f", "//foo:bar"), denylist_path_to_entry("f/foo/bar.json"))
    asserts.equals(env, ("f", "//foo/bar/..."), denylist_path_to_entry("f/foo/bar/__subpackages__.json"))

    # Every generated pattern is one that `_matches_pattern` understands.
    lbl = struct(package = "foo/bar", name = "baz")
    asserts.true(env, matches_pattern_for_test(lbl, denylist_path_to_entry("f/foo/bar/baz.json")[1]))
    asserts.true(env, matches_pattern_for_test(lbl, denylist_path_to_entry("f/foo/__subpackages__.json")[1]))
    asserts.false(env, matches_pattern_for_test(lbl, denylist_path_to_entry("f/foo/baz.json")[1]))

    return unittest.end(env)

denylist_path_to_entry_test = unittest.make(_denylist_path_to_entry_test_impl)

def _merge_denylists_test_impl(ctx):
    env = unittest.begin(ctx)

    asserts.equals(
        env,
        {
            "inline": ["//a:a"],
            "from_files": ["//b/...", "//c:d"],
        },
        merge_denylists(
            {"inline": ["//a:a"]},
            ["from_files/c/d.json", "from_files/b/__subpackages__.json"],
            ["inline", "from_files", "unused"],
        ),
    )
    asserts.equals(env, {"inline": ["//a:a"]}, merge_denylists({"inline": ["//a:a"]}, [], ["inline"]))

    # A feature can have both an inline denylist and files.
    asserts.equals(
        env,
        {"both": ["//a:a", "//b:b", "//c:c"]},
        merge_denylists({"both": ["//a:a"]}, ["both/c/c.json", "both/b/b.json"], ["both"]),
    )

    return unittest.end(env)

merge_denylists_test = unittest.make(_merge_denylists_test_impl)

def crubit_feature_hint_test_suite(name):
    unittest.suite(
        name,
        matches_pattern_test,
        denied_features_test,
        denylist_path_to_entry_test,
        merge_denylists_test,
    )
