# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

"""A shared list of supported features, including fine grained additions to `:supported`."""

visibility(["//..."])

SUPPORTED_FEATURES = [
    "supported",
    "std_vector",
    "std_unique_ptr",
]
