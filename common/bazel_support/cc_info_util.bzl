# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""Utility functions for working with CcInfo objects."""

def get_static_libraries_from_cc_info(cc_info):
    """Returns a list of all static library files in a CcInfo.

    Args:
      cc_info: A `CcInfo` object.

    Returns:
      A list containing all of the static library `File`s found in `cc_info`.
    """

    return [
        library_to_link.static_library
        for linker_input in cc_info.linking_context.linker_inputs.to_list()
        for library_to_link in linker_input.libraries
        if library_to_link.static_library != None
    ]
