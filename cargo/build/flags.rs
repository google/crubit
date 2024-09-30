// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

pub const CC_FLAGS: &[&str] = &[
    // TODO(danakj): Crubit has a bunch of warnings in C++ code.
    #[cfg(unix)]
    "-Wno-everything",
    #[cfg(windows)]
    "/w",
    // We build with C++20.
    //
    // TODO(danakj): This should probably be configurable.
    #[cfg(unix)]
    "-std=c++20",
    #[cfg(windows)]
    "/std:c++20",
    // Expect that LLVM is built without RTTI, and this needs to match. Otherwise we get errors
    // at link time like "ld.lld: error: undefined symbol: typeinfo for clang::ASTConsumer".
    //
    // TODO(danakj): This should probably be configurable.
    #[cfg(unix)]
    "-fno-rtti",
    #[cfg(windows)]
    "/GR-",
];
