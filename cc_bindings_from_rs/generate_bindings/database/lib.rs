// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_middle;

pub mod code_snippet;

mod adt_core_bindings;
pub use adt_core_bindings::{AdtCoreBindings, NoMoveOrAssign};
pub mod cpp_type;
mod db;
pub use db::BindingsGenerator;
mod fine_grained_feature;
pub use fine_grained_feature::FineGrainedFeature;
mod fully_qualified_name;
pub use fully_qualified_name::{ExportedPath, FullyQualifiedName, PublicPaths, UnqualifiedName};
mod include_guard;
pub use include_guard::IncludeGuard;
mod type_location;
pub use type_location::TypeLocation;
