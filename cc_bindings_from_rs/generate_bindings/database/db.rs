// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use crate::adt_core_bindings::AdtCoreBindings;
use crate::code_snippet::{ApiSnippets, CcSnippet};
use crate::fully_qualified_name::FullyQualifiedName;
use crate::include_guard::IncludeGuard;
use crate::sugared_ty::SugaredTy;
use crate::type_location::TypeLocation;
use arc_anyhow::Result;
use code_gen_utils::CcInclude;
use error_report::{ErrorReporting, ReportFatalError};
use proc_macro2::TokenStream;
use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::{CrateNum, DefId, LocalDefId};
use rustc_span::Symbol;
use std::collections::HashMap;
use std::rc::Rc;

memoized::query_group! {
  pub trait BindingsGenerator<'tcx> {
      /// Compilation context for the crate that the bindings should be generated
      /// for.
      #[input]
      fn tcx(&self) -> TyCtxt<'tcx>;

      /// Format specifier for `#include` Crubit C++ support library headers,
      /// using `{header}` as the place holder.  Example:
      /// `<crubit/support/{header}>` results in `#include
      /// <crubit/support/hdr.h>`.
      #[input]
      fn crubit_support_path_format(&self) -> Rc<str>;

      /// The default features enabled on all crates, if not present in `crate_name_to_features`.
      #[input]
      fn default_features(&self) -> flagset::FlagSet<crubit_feature::CrubitFeature>;

      /// A map from a crate name to the include paths of the corresponding C++
      /// headers This is used when formatting a type exported from another
      /// crate.
      // TODO(b/271857814): A crate name might not be globally unique - the key needs to also cover
      // a "hash" of the crate version and compilation flags.
      #[input]
      fn crate_name_to_include_paths(&self) -> Rc<HashMap<Rc<str>, Vec<CcInclude>>>;

      /// A map from a crate name to the features enabled on that crate. The special name `self`
      /// refers to the current crate.
      // TODO(b/271857814): A crate name might not be globally unique - the key needs to also cover
      // a "hash" of the crate version and compilation flags.
      #[input]
      fn crate_name_to_features(&self) -> Rc<HashMap<Rc<str>, flagset::FlagSet<crubit_feature::CrubitFeature>>>;

      /// A map from a crate name to the top-level namespace of the C++ bindings. The special name
      /// `self` refers to the current crate.
      #[input]
      fn crate_name_to_namespace(&self) -> Rc<HashMap<Rc<str>, Rc<str>>>;

      #[input]
      fn crate_renames(&self) -> Rc<HashMap<Rc<str>, Rc<str>>>;

      /// Error collector for generating reports of errors encountered during the generation of bindings.
      #[input]
      fn errors(&self) -> Rc<dyn ErrorReporting>;

      /// A collection of errors that should cause bindings generation to fail.
      ///
      /// These errors should be issued only in response to misusage of Crubit itself, such as
      /// incorrect use of Crubit-specific annotations.
      #[input]
      fn fatal_errors(&self) -> Rc<dyn ReportFatalError>;

      #[input]
      fn no_thunk_name_mangling(&self) -> bool;

      #[input]
      fn h_out_include_guard(&self) -> IncludeGuard;

      fn support_header(&self, suffix: &'tcx str) -> CcInclude;

      fn repr_attrs(&self, did: DefId) -> Rc<[rustc_attr_data_structures::ReprAttr]>;

      fn reexported_symbol_canonical_name_mapping(&self) -> HashMap<DefId, FullyQualifiedName>;

      fn format_cc_ident(&self, ident: Symbol) -> Result<TokenStream>;

      fn format_top_level_ns_for_crate(&self, krate: CrateNum) -> Symbol;

      fn format_ty_for_cc(
          &self,
          ty: SugaredTy<'tcx>,
          location: TypeLocation,
      ) -> Result<CcSnippet>;

      fn generate_default_ctor(
          &self,
          core: Rc<AdtCoreBindings<'tcx>>,
      ) -> Result<ApiSnippets, ApiSnippets>;
      fn generate_copy_ctor_and_assignment_operator(
          &self,
          core: Rc<AdtCoreBindings<'tcx>>,
      ) -> Result<ApiSnippets, ApiSnippets>;
      fn generate_move_ctor_and_assignment_operator(
          &self,
          core: Rc<AdtCoreBindings<'tcx>>,
      ) -> Result<ApiSnippets, ApiSnippets>;

      fn generate_item(&self, def_id: LocalDefId) -> Result<Option<ApiSnippets>>;
      fn generate_function(&self, local_def_id: LocalDefId) -> Result<ApiSnippets>;
      fn generate_adt_core(&self, def_id: DefId) -> Result<Rc<AdtCoreBindings<'tcx>>>;
  }
  pub struct Database;
}
