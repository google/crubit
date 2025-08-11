// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

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
use rustc_span::def_id::{CrateNum, DefId};
use rustc_span::Symbol;
use std::collections::HashMap;
use std::rc::Rc;

memoized::query_group! {
  pub trait BindingsGenerator<'tcx> {
      #[input]
      /// Compilation context for the crate that the bindings should be generated
      /// for.
      fn tcx(&self) -> TyCtxt<'tcx>;

      #[input]
      /// Format specifier for `#include` Crubit C++ support library headers,
      /// using `{header}` as the place holder.  Example:
      /// `<crubit/support/{header}>` results in `#include
      /// <crubit/support/hdr.h>`.
      fn crubit_support_path_format(&self) -> Rc<str>;

      #[input]
      /// The default features enabled on all crates, if not present in `crate_name_to_features`.
      fn default_features(&self) -> flagset::FlagSet<crubit_feature::CrubitFeature>;

      #[input]
      /// A map from a crate name to the include paths of the corresponding C++
      /// headers This is used when formatting a type exported from another
      /// crate.
      // TODO(b/271857814): A crate name might not be globally unique - the key needs to also cover
      // a "hash" of the crate version and compilation flags.
      fn crate_name_to_include_paths(&self) -> Rc<HashMap<Rc<str>, Vec<CcInclude>>>;

      #[input]
      /// A map from a crate name to the features enabled on that crate. The special name `self`
      /// refers to the current crate.
      // TODO(b/271857814): A crate name might not be globally unique - the key needs to also cover
      // a "hash" of the crate version and compilation flags.
      fn crate_name_to_features(&self) -> Rc<HashMap<Rc<str>, flagset::FlagSet<crubit_feature::CrubitFeature>>>;

      #[input]
      /// A map from a crate name to the top-level namespace of the C++ bindings. The special name
      /// `self` refers to the current crate.
      fn crate_name_to_namespace(&self) -> Rc<HashMap<Rc<str>, Rc<str>>>;

      #[input]
      fn crate_renames(&self) -> Rc<HashMap<Rc<str>, Rc<str>>>;

      #[input]
      /// Error collector for generating reports of errors encountered during the generation of bindings.
      fn errors(&self) -> Rc<dyn ErrorReporting>;

      #[input]
      /// A collection of errors that should cause bindings generation to fail.
      ///
      /// These errors should be issued only in response to misusage of Crubit itself, such as
      /// incorrect use of Crubit-specific annotations.
      fn fatal_errors(&self) -> Rc<dyn ReportFatalError>;

      #[input]
      fn no_thunk_name_mangling(&self) -> bool;

      #[input]
      fn h_out_include_guard(&self) -> IncludeGuard;

      /// Returns the include for the given Crubit runtime support header.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/lib.rs?q=function:support_header
      fn support_header(&self, suffix: &'tcx str) -> CcInclude;

      /// Returns the representation attributes for the given definition.
      ///
      /// TODO: Replace calls to this function with direct call to `repr.transparent()`
      /// and `repr.c()` etc.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/query_compiler.rs?q=function:repr_attrs
      fn repr_attrs(&self, did: DefId) -> Rc<[rustc_hir::attrs::ReprAttr]>;

      /// Computes a mapping from a `DefId` to a `FullyQualifiedName` for all
      /// not-directly-public symbols that are reexported by a `use` statement.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/lib.rs?q=function:reexported_symbol_canonical_name_mapping
      fn reexported_symbol_canonical_name_mapping(&self) -> HashMap<DefId, FullyQualifiedName>;

      /// Formats a C++ identifier, if possible.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/format_type.rs?q=function:format_cc_ident
      fn format_cc_ident(&self, ident: Symbol) -> Result<TokenStream>;

      /// Formats the top-level namespace for the given crate, e.g. as `self::foo`, or
      /// `somecrate::foo`.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/format_type.rs?q=function:format_top_level_ns_for_crate
      fn format_top_level_ns_for_crate(&self, krate: CrateNum) -> Symbol;

      /// Formats `ty` into a `CcSnippet` that represents how the type should be
      /// spelled in a C++ declaration of a function parameter or field.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/format_type.rs?q=function:format_ty_for_cc
      fn format_ty_for_cc(
          &self,
          ty: SugaredTy<'tcx>,
          location: TypeLocation,
      ) -> Result<CcSnippet>;

      /// Generates a default constructor for an ADT if possible (i.e. if the `Default`
      /// trait is implemented for the ADT).  Returns an error otherwise (e.g. if
      /// there is no `Default` impl, then the default constructor will be
      /// `=delete`d in the returned snippet).
      /// Implementation: cc_bindings_from_rs/generate_bindings/lib.rs?q=function:generate_default_ctor
      fn generate_default_ctor(
          &self,
          core: Rc<AdtCoreBindings<'tcx>>,
      ) -> Result<ApiSnippets, ApiSnippets>;

      /// Generates the copy constructor and the copy-assignment operator for an ADT if
      /// possible (i.e. if the `Clone` trait is implemented for the ADT).  Returns an
      /// error otherwise (e.g. if there is no `Clone` impl, then the copy constructor
      /// and assignment operator will be `=delete`d in the returned snippet).
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/lib.rs?q=function:generate_copy_ctor_and_assignment_operator
      fn generate_copy_ctor_and_assignment_operator(
          &self,
          core: Rc<AdtCoreBindings<'tcx>>,
      ) -> Result<ApiSnippets, ApiSnippets>;

      /// Generates the move constructor and the move-assignment operator for an ADT if
      /// possible (it depends on various factors like `needs_drop`, `is_unpin` and
      /// implementations of `Default` and/or `Clone` traits).  Returns an error
      /// otherwise (the error's `ApiSnippets` contain a `=delete`d declaration).
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/lib.rs?q=function:generate_move_ctor_and_assignment_operator
      fn generate_move_ctor_and_assignment_operator(
          &self,
          core: Rc<AdtCoreBindings<'tcx>>,
      ) -> Result<ApiSnippets, ApiSnippets>;

      /// Generates bindings for a HIR item idenfied by `def_id`.  Returns `None` if
      /// the item can be ignored. Returns an `Err` if the bindings could not be
      /// generated.
      ///
      /// Will panic if `def_id` is invalid (i.e. doesn't identify a HIR item).
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/lib.rs?q=function:generate_item
      fn generate_item(&self, def_id: DefId) -> Result<Option<ApiSnippets>>;

      /// Generates bindings for a function with the given `local_def_id`.
      ///
      /// Will panic if `local_def_id`
      /// - is invalid
      /// - doesn't identify a function
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/generate_function.rs?q=function:generate_function
      fn generate_function(&self, def_id: DefId) -> Result<ApiSnippets>;

      /// Generates the bindings for the core of an algebraic data type (an ADT - a
      /// struct, an enum, or a union) represented by `def_id`.
      ///
      /// The "core" means things that are necessary for a successful binding (e.g.
      /// inability to generate a correct C++ destructor means that the ADT cannot
      /// have any bindings).  "core" excludes things that are A) infallible (e.g.
      /// struct or union fields which can always be translated into private, opaque
      /// blobs of bytes) or B) optional (e.g. a problematic instance method
      /// can just be ignored, unlike a problematic destructor).  The split between
      /// fallible "core" and non-fallible "rest" is motivated by the need to avoid
      /// cycles / infinite recursion (e.g. when processing fields that refer back to
      /// the struct type, possible with an indirection of a pointer).
      ///
      /// `generate_adt_core` is used both to 1) generate bindings for the core of an
      /// ADT, and 2) check if formatting would have succeeded (e.g. when called from
      /// `format_ty`).  The 2nd case is needed for ADTs defined in any crate.
      fn generate_adt_core(&self, def_id: DefId) -> Result<Rc<AdtCoreBindings<'tcx>>>;
  }
  pub struct Database;
}
