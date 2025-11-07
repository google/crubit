// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

extern crate rustc_hir;
extern crate rustc_middle;
extern crate rustc_span;

use crate::adt_core_bindings::{AdtCoreBindings, NoMoveOrAssign};
use crate::code_snippet::{ApiSnippets, CcSnippet, CrubitAbiTypeWithCcPrereqs};
use crate::fully_qualified_name::{FullyQualifiedName, PublicPaths, UnqualifiedName};
use crate::include_guard::IncludeGuard;
use crate::sugared_ty::SugaredTy;
use crate::type_location::TypeLocation;
use arc_anyhow::Result;
use code_gen_utils::CcInclude;
use dyn_format::Format;
use error_report::{ErrorReporting, ReportFatalError};
use proc_macro2::{Ident, TokenStream};
use rustc_middle::ty::{Ty, TyCtxt};
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
      /// Name of the `extern` crate that bindings should be generated for.
      /// If `None`, bindings will be generated for the crate currently being compiled.
      fn source_crate_name(&self) -> Option<Rc<str>>;

      #[input]
      /// Format specifier for `#include` Crubit C++ support library headers,
      /// using `{header}` as the place holder.  Example:
      /// `<crubit/support/{header}>` results in `#include
      /// <crubit/support/hdr.h>`.
      fn crubit_support_path_format(&self) -> Format<1>;

      #[input]
      /// Format specifier for path cross-references in "Generated from" comments.
      ///
      /// Inputs are `{path}` and `{line}` respectively.
      fn crubit_debug_path_format(&self) -> Option<Format<2>>;

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

      /// The `CrateNum` of the crate that the bindings should be generated for.
      /// This will be `LOCAL_CRATE` if no `source_crate_name` was provided.
      fn source_crate_num(&self) -> CrateNum;

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

      /// Computes the unqualified name of the symbol identified by `def_id`.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/lib.rs?q=function:symbol_unqualified_name
      fn symbol_unqualified_name(&self, def_id: DefId) -> Option<UnqualifiedName>;

      /// Computes the canonical name of the symbol identified by `def_id`. For most cases, this
      /// will be the fully qualified path to the definition `def_id` references. However, some
      /// definitions are private and may not be referenced by the path they are defined at. This
      /// will pick a path that can be referenced publicly and treat it as the canonical name for
      /// that definition. For example, if we have a rust module:
      /// ```
      /// pub mod foo {
      ///   mod private {
      ///     pub struct Bar {
      ///       pub x: i32,
      ///     }
      ///   }
      ///   pub use private::Bar;
      /// }
      /// pub use foo::*;
      /// ```
      /// Bar's defining path `foo::private::Bar` is not publicly visible, but it can be referenced
      /// at either `Bar` or `foo::Bar` due to our `use` statements. This method would give `Bar`
      /// the canonical name `foo::Bar`, preferring the more specific of our two available paths.
      ///
      /// If no canonical name can be determined, `None` is returned. This will occur when our
      /// `def_id` has no publicly visible paths, for example.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/lib.rs?q=function:symbol_canonical_name
      fn symbol_canonical_name(&self, def_id: DefId) -> Option<FullyQualifiedName>;

      /// Computes a mapping from a `DefId` to a list of public paths that reference it in a given
      /// crate. This accounts for `use` statements that reexport, and optionally alias, the same
      /// DefId. This will not account for paths available in the entire forest of crates, unlike
      /// visible_parent_map(), strictly those within the provided crate.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/lib.rs?q=function:public_paths_by_def_id
      fn public_paths_by_def_id(&self, crate_num: CrateNum) -> HashMap<DefId, PublicPaths>;

      /// Formats a C++ identifier, if possible.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/format_type.rs?q=function:format_cc_ident
      fn format_cc_ident(&self, ident: Symbol) -> Result<Ident>;

      /// Formats the top-level namespace for the given crate, e.g. as `self::foo`, or
      /// `somecrate::foo`.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/format_type.rs?q=function:format_top_level_ns_for_crate
      fn format_top_level_ns_for_crate(&self, krate: CrateNum) -> Rc<[Symbol]>;

      /// Formats `ty` into a `CcSnippet` that represents how the type should be
      /// spelled in a C++ declaration of a function parameter or field.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/format_type.rs?q=function:format_ty_for_cc
      fn format_ty_for_cc(
          &self,
          ty: SugaredTy<'tcx>,
          location: TypeLocation,
      ) -> Result<CcSnippet>;

      /// Formats `ty` into a `CcSnippet` that represents how the type should be
      /// spelled in a C++ declaration of a function parameter or field.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/format_type.rs?q=function:format_ty_for_rs
      fn format_ty_for_rs(
          &self,
          ty: Ty<'tcx>
      ) -> Result<TokenStream>;

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

      /// Generates the move constructor and the move-assignment operator for an ADT if possible
      /// (it depends on various factors like `needs_drop`, `is_unpin` and implementations of
      /// `Default` and/or `Clone` traits).
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/lib.rs?q=function:generate_move_ctor_and_assignment_operator
      fn generate_move_ctor_and_assignment_operator(
          &self,
          core: Rc<AdtCoreBindings<'tcx>>,
      ) -> Result<ApiSnippets, NoMoveOrAssign>;

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

      fn crubit_abi_type_from_ty(&self, ty: Ty<'tcx>) -> Result<CrubitAbiTypeWithCcPrereqs>;

      /// Gathers all  `From` trait impls for the current crate and provides a mapping from the
      /// argument type to the impl. This is useful for determining `From` impls of ADTs where the
      /// ADT appears as an argument rather than a self type (i.e. `impl From<Adt> for i32`).
      ///
      /// It could make sense to expand this functionality to other single argument traits, but
      /// there are open design questions if we want to support _any_ trait. How do we map from a
      /// list of arguments to an implementation? For more complicated use cases it's probably
      /// better to evoke the trait solver directly rather than going through this mapping. For that
      /// reason, this function is currently limited to `From` specifically.
      ///
      /// Implementation: cc_bindings_from_rs/generate_bindings/generate_struct_and_union.rs?q=function:local_from_trait_impls_by_argument
      fn from_trait_impls_by_argument(&self, crate_num: CrateNum) -> Rc<HashMap<Ty<'tcx>, Vec<DefId>>>;
  }
  pub struct Database;
}
