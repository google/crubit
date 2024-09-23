// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Rust dependencies from the C++ library.
//
// Rust/Cargo needs to see all Rust dependencies in the build graph to avoid
// pruning them from the link line. By making them a explicit dependency of
// this crate, we ensure the C++ libraries are included in the link step.
extern crate collect_instantiations;
extern crate common_sys;
extern crate generate_bindings;
extern crate lifetime_analysis_sys;
extern crate lifetime_annotations_sys;
