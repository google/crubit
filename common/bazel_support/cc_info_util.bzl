# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Utility functions for working with C++ linking objects."""

def get_static_library_files(library_to_link):
    """Returns the static library file of a single `LibraryToLink`, if it has one.

    This deliberately takes one `LibraryToLink` rather than a `CcInfo`. A `CcInfo`'s linking
    context also lists the libraries of every transitive dependency, and requesting the archives
    of all of those libraries can add thousands of actions that the build would not otherwise
    run.

    Args:
      library_to_link: A `LibraryToLink` object, or `None`.

    Returns:
      A list containing the static library `File` of `library_to_link`, falling back to the PIC
      static library. The list is empty if `library_to_link` is `None` or has neither.
    """

    if library_to_link == None:
        return []
    if library_to_link.static_library != None:
        return [library_to_link.static_library]
    if library_to_link.pic_static_library != None:
        return [library_to_link.pic_static_library]
    return []
