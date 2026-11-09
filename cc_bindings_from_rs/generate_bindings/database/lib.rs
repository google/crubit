// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_middle;

pub mod code_snippet;

mod adt_core_bindings;
pub use adt_core_bindings::{
    AdtCoreBindings, CopyCodegenStyle, CoreBindingsCommon, MoveCodegenStyle,
};
mod db;
pub use code_gen_utils::is_cpp_pointer_type;
pub use db::{BindingsGenerator, CppTypeSpecialization, DisambiguatedAliases, RefTraitImpls};
mod fully_qualified_name;
pub use fully_qualified_name::{
    rename_c_stdlib_functions, rename_clang_builtin_macros, ExportedPath, FullyQualifiedName,
    PublicPaths, UnqualifiedName,
};
mod include_guard;
pub use include_guard::IncludeGuard;
mod static_method_mode;
pub use static_method_mode::StaticMethodMode;
mod type_location;
pub use type_location::TypeLocation;

#[rustversion::before(2026-10-31)]
pub type GenericArgs<'tcx> = rustc_middle::ty::GenericArgsRef<'tcx>;
#[rustversion::since(2026-10-31)]
pub type GenericArgs<'tcx> = rustc_middle::ty::GenericArgs<'tcx>;

#[rustversion::before(2026-10-31)]
pub fn empty_generic_args<'tcx>() -> GenericArgs<'tcx> {
    rustc_middle::ty::List::empty()
}
#[rustversion::since(2026-10-31)]
pub fn empty_generic_args<'tcx>() -> GenericArgs<'tcx> {
    rustc_middle::ty::GenericArgs::empty()
}
