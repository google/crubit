# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Implementation of `crubit_feature_hint`.

This is never used directly; only specific instances of the hint are used, and they are
visibility-restricted.
"""

load("@bazel_skylib//rules:common_settings.bzl", "BuildSettingInfo")
load("//features:global_features.bzl", "NO_ASSUME_LIFETIMES_TARGETS")

visibility(["//..."])

# Omitted from providers.bzl: this is used internally and in tests only.
# (The "public" interface is `crubit_feature_hint` to create it,
# and `find_crubit_features` to aggregate it for collection into RustBindingsFromCcInfo.)
_CrubitFeaturesInfo = provider(
    doc = "A set of enabled Crubit features.",
    fields = {"crubit_features": "List of features"},
)

def _crubit_feature_hint_impl(ctx):
    return [_CrubitFeaturesInfo(
        crubit_features = ctx.attr.crubit_features,
    )]

crubit_feature_hint = rule(
    attrs = {
        "crubit_features": attr.string_list(doc = "Feature flags to enable. e.g. 'experimental'."),
    },
    implementation = _crubit_feature_hint_impl,
)

def _add_features(features, target):
    # Starlark doesn't have sets, so the following is O(n^2) for convenience.
    if _CrubitFeaturesInfo not in target:
        return
    for feature in target[_CrubitFeaturesInfo].crubit_features:
        if feature not in features:
            features.append(feature)

# Allegedly there aren't any builtin functions that do this outside of the loading phase, nor do
# we have regular expressions available. We only support //..., //foo/bar, //foo/..., and //foo:bar
# (not //foo:* or //foo:all).
def _matches_pattern(label, pattern):
    if pattern == "//...":
        return True
    if pattern.endswith("/..."):
        prefix = pattern[:-4]
        if prefix.startswith("//"):
            pkg_prefix = prefix[2:]
            if label.package == pkg_prefix or label.package.startswith(pkg_prefix + "/"):
                return True
    elif pattern.startswith("//"):
        if ":" in pattern:
            pkg, name = pattern[2:].split(":", 1)
        else:
            pkg, name = pattern[2:], pattern[2:].split("/")[-1]
        if label.package == pkg and label.name == name:
            return True
    return False

matches_pattern_for_test = _matches_pattern

def _matches_any_pattern(label, patterns):
    for pattern in patterns:
        if _matches_pattern(label, pattern):
            return True
    return False

def _has_explicit_opt_in(aspect_hints):
    for hint in aspect_hints:
        if hint.label.package == "features" and hint.label.name in ("assume_lifetimes", "experimental"):
            return True
        if hint.label.package == "features/internal" and hint.label.name in ("testonly_assume_lifetimes", "testonly_experimental"):
            return True
    return False

def find_crubit_features(target, aspect_ctx):
    """Returns the set of Crubit features enabled on a target.

    Args:
        target: The target, as seen in aspect_hint.
        aspect_ctx: The ctx from an aspect_hint.

    Returns:
        A list of feature strings.
    """
    features = list(aspect_ctx.attr._globally_enabled_features[BuildSettingInfo].value)
    _add_features(features, target)
    for hint in aspect_ctx.rule.attr.aspect_hints:
        _add_features(features, hint)
    if features:
        if _matches_any_pattern(target.label, NO_ASSUME_LIFETIMES_TARGETS):
            if not _has_explicit_opt_in(aspect_ctx.rule.attr.aspect_hints):
                if "no_assume_lifetimes" not in features:
                    features.append("no_assume_lifetimes")
    return sorted(features)
