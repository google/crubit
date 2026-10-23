# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Reads feature denylists stored as one file per entry.

See crubit.rs/team/release#howto_denylist_files.
"""

visibility(["//features/..."])

_SUFFIX = ".json"
_SUBPACKAGES = "__subpackages__"
_DIR = "internal/"

def denylist_path_to_entry(path):
    """Converts the path of a denylist entry file to a feature and a target pattern.

    Args:
        path: The path of the file, relative to `internal/`, e.g.
            `my_feature/foo/bar/baz.json`.

    Returns:
        A `(feature, pattern)` tuple, e.g. `("my_feature", "//foo/bar:baz")`.
    """
    if not path.endswith(_SUFFIX):
        fail("Denylist entry %r must end in %r" % (path, _SUFFIX))
    parts = path[:-len(_SUFFIX)].split("/")
    if len(parts) < 3 or "" in parts:
        fail("Denylist entry %r must be of the form `<feature>/<package>/<target>%s`" % (path, _SUFFIX))
    feature, pkg, name = parts[0], "/".join(parts[1:-1]), parts[-1]
    if name == _SUBPACKAGES:
        return (feature, "//%s/..." % pkg)
    return (feature, "//%s:%s" % (pkg, name))

def merge_denylists(inline_denylists, paths, supported_features):
    """Merges inline denylists with per-entry file denylists.

    A feature can have both: e.g. Crubit's own test targets inline, and internal targets as files.

    Args:
        inline_denylists: A dict mapping features to lists of target patterns.
        paths: Paths of denylist entry files, relative to `internal/`.
        supported_features: The features which may have a denylist.

    Returns:
        A dict mapping features to lists of target patterns.
    """
    from_files = {}
    for path in paths:
        feature, pattern = denylist_path_to_entry(path)
        if feature not in supported_features:
            fail("Denylist entry %r is for %r, which is not in SUPPORTED_FEATURES" % (path, feature))
        from_files.setdefault(feature, []).append(pattern)
    merged = dict(inline_denylists)
    for feature, patterns in from_files.items():
        merged[feature] = merged.get(feature, []) + sorted(patterns)
    return merged

def denylist_paths():
    """Returns the paths of all denylist entry files in the calling package.

    Must be called from a BUILD file (it uses `glob`). The `internal/` directory must not contain
    BUILD files, since `glob` doesn't descend into subpackages.

    Returns:
        Paths relative to `internal/`, e.g. `my_feature/foo/bar/baz.json`.
    """
    return [
        path[len(_DIR):]
        for path in native.glob([_DIR + "**/*" + _SUFFIX], allow_empty = True)
    ]
