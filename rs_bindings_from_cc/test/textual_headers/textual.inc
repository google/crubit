// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This header ensures that the interop tooling doesn't try to parse the
// textual header (otherwise the build would fail).

#ifndef USING_TEXTUAL_HEADER_IS_FINE
#error "define USING_TEXTUAL_HEADER_IS_FINE to make this textual header work"
#endif
