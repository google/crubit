// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(rustc_private)]
#![deny(rustc::internal)]

pub mod code_snippet;

mod adt_core_bindings;
pub use adt_core_bindings::{AdtCoreBindings, NoMoveOrAssign};
pub mod cpp_type;
mod db;
pub use db::{BindingsGenerator, Database};
mod fine_grained_feature;
pub use fine_grained_feature::FineGrainedFeature;
mod fully_qualified_name;
pub use fully_qualified_name::{ExportedPath, FullyQualifiedName, PublicPaths, UnqualifiedName};
mod include_guard;
pub use include_guard::IncludeGuard;
mod sugared_ty;
pub use sugared_ty::SugaredTy;
mod type_location;
pub use type_location::TypeLocation;
