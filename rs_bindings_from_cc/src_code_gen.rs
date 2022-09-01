// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use arc_anyhow::{anyhow, bail, ensure, Context, Result};
use ffi_types::*;
use ir::*;
use itertools::Itertools;
use once_cell::sync::Lazy;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use salsa_utils::RcEq;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::ffi::{OsStr, OsString};
use std::fmt::Write as _;
use std::iter::{self, Iterator};
use std::panic::catch_unwind;
use std::process;
use std::ptr;
use std::rc::Rc;
use token_stream_printer::{rs_tokens_to_formatted_string, tokens_to_string, RustfmtConfig};

/// FFI equivalent of `Bindings`.
#[repr(C)]
pub struct FfiBindings {
    rs_api: FfiU8SliceBox,
    rs_api_impl: FfiU8SliceBox,
}

/// Deserializes IR from `json` and generates bindings source code.
///
/// This function panics on error.
///
/// # Safety
///
/// Expectations:
///    * `json` should be a FfiU8Slice for a valid array of bytes with the given
///      size.
///    * `crubit_support_path` should be a FfiU8Slice for a valid array of bytes
///      representing an UTF8-encoded string
///    * `rustfmt_exe_path` and `rustfmt_config_path` should both be a
///      FfiU8Slice for a valid array of bytes representing an UTF8-encoded
///      string (without the UTF-8 requirement, it seems that Rust doesn't offer
///      a way to convert to OsString on Windows)
///    * `json`, `crubit_support_path`, `rustfmt_exe_path`, and
///      `rustfmt_config_path` shouldn't change during the call.
///
/// Ownership:
///    * function doesn't take ownership of (in other words it borrows) the
///      input params: `json`, `crubit_support_path`, `rustfmt_exe_path`, and
///      `rustfmt_config_path`
///    * function passes ownership of the returned value to the caller
#[no_mangle]
pub unsafe extern "C" fn GenerateBindingsImpl(
    json: FfiU8Slice,
    crubit_support_path: FfiU8Slice,
    rustfmt_exe_path: FfiU8Slice,
    rustfmt_config_path: FfiU8Slice,
) -> FfiBindings {
    let json: &[u8] = json.as_slice();
    let crubit_support_path: &str = std::str::from_utf8(crubit_support_path.as_slice()).unwrap();
    let rustfmt_exe_path: OsString =
        std::str::from_utf8(rustfmt_exe_path.as_slice()).unwrap().into();
    let rustfmt_config_path: OsString =
        std::str::from_utf8(rustfmt_config_path.as_slice()).unwrap().into();
    catch_unwind(|| {
        // It is ok to abort here.
        let Bindings { rs_api, rs_api_impl } =
            generate_bindings(json, crubit_support_path, &rustfmt_exe_path, &rustfmt_config_path)
                .unwrap();
        FfiBindings {
            rs_api: FfiU8SliceBox::from_boxed_slice(rs_api.into_bytes().into_boxed_slice()),
            rs_api_impl: FfiU8SliceBox::from_boxed_slice(
                rs_api_impl.into_bytes().into_boxed_slice(),
            ),
        }
    })
    .unwrap_or_else(|_| process::abort())
}

#[salsa::query_group(BindingsGeneratorStorage)]
trait BindingsGenerator {
    #[salsa::input]
    fn ir(&self) -> Rc<IR>;

    fn rs_type_kind(&self, rs_type: RsType) -> Result<RsTypeKind>;

    fn generate_func(
        &self,
        func: Rc<Func>,
    ) -> Result<Option<RcEq<(RsSnippet, RsSnippet, Rc<FunctionId>)>>>;

    fn overloaded_funcs(&self) -> Rc<HashSet<Rc<FunctionId>>>;
}

#[salsa::database(BindingsGeneratorStorage)]
#[derive(Default)]
struct Database {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Database {}

/// Source code for generated bindings.
struct Bindings {
    // Rust source code.
    rs_api: String,
    // C++ source code.
    rs_api_impl: String,
}

/// Source code for generated bindings, as tokens.
struct BindingsTokens {
    // Rust source code.
    rs_api: TokenStream,
    // C++ source code.
    rs_api_impl: TokenStream,
}

fn generate_bindings(
    json: &[u8],
    crubit_support_path: &str,
    rustfmt_exe_path: &OsStr,
    rustfmt_config_path: &OsStr,
) -> Result<Bindings> {
    let ir = Rc::new(deserialize_ir(json)?);

    let BindingsTokens { rs_api, rs_api_impl } =
        generate_bindings_tokens(ir.clone(), crubit_support_path)?;
    let rs_api = {
        let rustfmt_config = RustfmtConfig::new(rustfmt_exe_path, rustfmt_config_path);
        rs_tokens_to_formatted_string(rs_api, &rustfmt_config)?
    };

    // The code is formatted with a non-default rustfmt configuration. Prevent
    // downstream workflows from reformatting with a different configuration by
    // marking the output with `@generated`. See also
    // https://rust-lang.github.io/rustfmt/?version=v1.4.38&search=#format_generated_files
    //
    // TODO(lukasza): It would be nice to include "by $argv[0]"" in the
    // @generated comment below.  OTOH, `std::env::current_exe()` in our
    // current build environment returns a guid-like path... :-/
    //
    // TODO(lukasza): Try to remove `#![rustfmt:skip]` - in theory it shouldn't
    // be needed when `@generated` comment/keyword is present...
    let rs_api = format!(
        "// Automatically @generated Rust bindings for C++ target\n\
        // {target}\n\
        #![rustfmt::skip]\n\
        {code}",
        target = ir.current_target().0,
        code = rs_api,
    );
    let rs_api_impl = tokens_to_string(rs_api_impl)?;

    Ok(Bindings { rs_api, rs_api_impl })
}

/// Rust source code with attached information about how to modify the parent
/// crate.
///
/// For example, the snippet `vec![].into_raw_parts()` is not valid unless the
/// `vec_into_raw_parts` feature is enabled. So such a snippet should be
/// represented as:
///
/// ```
/// RsSnippet {
///   features: btree_set![make_rs_ident("vec_into_raw_parts")],
///   tokens: quote!{vec![].into_raw_parts()},
/// }
/// ```
#[derive(Clone, Debug)]
struct RsSnippet {
    /// Rust feature flags used by this snippet.
    features: BTreeSet<Ident>,
    /// The snippet itself, as a token stream.
    tokens: TokenStream,
}

impl From<TokenStream> for RsSnippet {
    fn from(tokens: TokenStream) -> Self {
        RsSnippet { features: BTreeSet::new(), tokens }
    }
}

/// If we know the original C++ function is codegenned and already compatible
/// with `extern "C"` calling convention we skip creating/calling the C++ thunk
/// since we can call the original C++ directly.
fn can_skip_cc_thunk(db: &dyn BindingsGenerator, func: &Func) -> bool {
    // ## Inline functions
    //
    // Inline functions may not be codegenned in the C++ library since Clang doesn't
    // know if Rust calls the function or not. Therefore in order to make inline
    // functions callable from Rust we need to generate a C++ file that defines
    // a thunk that delegates to the original inline function. When compiled,
    // Clang will emit code for this thunk and Rust code will call the
    // thunk when the user wants to call the original inline function.
    //
    // This is not great runtime-performance-wise in regular builds (inline function
    // will not be inlined, there will always be a function call), but it is
    // correct. ThinLTO builds will be able to see through the thunk and inline
    // code across the language boundary. For non-ThinLTO builds we plan to
    // implement <internal link> which removes the runtime performance overhead.
    if func.is_inline {
        return false;
    }
    // ## Member functions (or descendants) of class templates
    //
    // A thunk is required to force/guarantee template instantiation.
    if func.is_member_or_descendant_of_class_template {
        return false;
    }
    // ## Virtual functions
    //
    // When calling virtual `A::Method()`, it's not necessarily the case that we'll
    // specifically call the concrete `A::Method` impl. For example, if this is
    // called on something whose dynamic type is some subclass `B` with an
    // overridden `B::Method`, then we'll call that.
    //
    // We must reuse the C++ dynamic dispatching system. In this case, the easiest
    // way to do it is by resorting to a C++ thunk, whose implementation will do
    // the lookup.
    //
    // In terms of runtime performance, since this only occurs for virtual function
    // calls, which are already slow, it may not be such a big deal. We can
    // benchmark it later. :)
    if let Some(meta) = &func.member_func_metadata {
        if let Some(inst_meta) = &meta.instance_method_metadata {
            if inst_meta.is_virtual {
                return false;
            }
        }
    }
    // ## Custom calling convention requires a thunk.
    //
    // The thunk has the "C" calling convention, and internally can call the
    // C++ function using any of the calling conventions supported by the C++
    // compiler (which might not always match the set supported by Rust - e.g.,
    // abi.rs doesn't contain "swiftcall" from
    // clang::FunctionType::getNameForCallConv)
    if !func.has_c_calling_convention {
        return false;
    }

    // ## Nontrivial return types.
    //
    // If the function returns a value which is nontrivial for the purpose of calls,
    // then in the underlying ABI, it is actually returned via a hidden pointer
    // parameter not exposed anywhere in the Clang AST or the Crubit IR. For
    // now, this is worked around via an _explicit_ output parameter, used in
    // the thunk, which cannot be skipped anymore.
    //
    // Note: if the RsTypeKind cannot be parsed / rs_type_kind returns Err, then
    // bindings generation will fail for this function, so it doesn't really matter
    // what we do here.
    if let Ok(return_type) = db.rs_type_kind(func.return_type.rs_type.clone()) {
        if !return_type.is_unpin() {
            return false;
        }
    }
    // ## Nontrivial parameter types.
    //
    // If the function accepts a value which is nontrivial for the purpose of calls,
    // then in the underlying ABI, it is actually passed by pointer.
    //
    // Because there's no way to upgrade an lvalue (e.g. pointer) to a prvalue, we
    // cannot implement guaranteed copy/move elision for inline functions for
    // now: any thunk we generate would need to invoke the correct function as
    // if by magic.
    //
    // And so for now, we always use C++11 semantics, via an intermediate thunk.
    //
    // (As a side effect, this, like return values, means that support is
    // ABI-agnostic.)
    for param in &func.params {
        if let Ok(param_type) = db.rs_type_kind(param.type_.rs_type.clone()) {
            if !param_type.is_unpin() {
                return false;
            }
        }
    }

    true
}

/// Uniquely identifies a generated Rust function.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct FunctionId {
    // If the function is on a trait impl, contains the name of the Self type for
    // which the trait is being implemented.
    self_type: Option<syn::Path>,
    // Fully qualified path of the function. For functions in impl blocks, this
    // includes the name of the type or trait on which the function is being
    // implemented, e.g. `Default::default`.
    function_path: syn::Path,
}

/// Returns the name of `func` in C++ syntax.
fn cxx_function_name(func: &Func, ir: &IR) -> Result<String> {
    let record: Option<&str> = ir.record_for_member_func(func)?.map(|r| &*r.cc_name);

    let func_name = match &func.name {
        UnqualifiedIdentifier::Identifier(id) => id.identifier.clone(),
        UnqualifiedIdentifier::Operator(op) => op.cc_name(),
        UnqualifiedIdentifier::Destructor => {
            format!("~{}", record.expect("destructor must be associated with a record"))
        }
        UnqualifiedIdentifier::Constructor => {
            record.expect("constructor must be associated with a record").to_string()
        }
    };

    if let Some(record_name) = record {
        Ok(format!("{}::{}", record_name, func_name))
    } else {
        Ok(func_name)
    }
}

fn make_unsupported_fn(func: &Func, ir: &IR, message: impl ToString) -> Result<UnsupportedItem> {
    Ok(UnsupportedItem {
        name: cxx_function_name(func, ir)?,
        message: message.to_string(),
        source_loc: func.source_loc.clone(),
        id: func.id,
    })
}

/// The name of a one-function trait, with extra entries for
/// specially-understood traits and families of traits.
enum TraitName {
    /// The constructor trait for !Unpin types, with a list of parameter types.
    /// For example, `CtorNew(vec![])` is the default constructor.
    CtorNew(Vec<RsTypeKind>),
    /// An Unpin constructor trait, e.g. From or Clone, with a list of parameter
    /// types.
    UnpinConstructor { name: TokenStream, params: Vec<RsTypeKind> },
    /// Any other trait, e.g. Eq.
    Other { name: TokenStream, params: Vec<RsTypeKind>, is_unsafe_fn: bool },
}

impl TraitName {
    /// Returns the generic parameters in this trait name.
    fn params(&self) -> impl Iterator<Item = &RsTypeKind> {
        match self {
            Self::CtorNew(params)
            | Self::UnpinConstructor { params, .. }
            | Self::Other { params, .. } => params.iter(),
        }
    }

    /// Returns the lifetimes used in this trait name.
    pub fn lifetimes(&self) -> impl Iterator<Item = Lifetime> + '_ {
        self.params().flat_map(|p| p.lifetimes())
    }
}

impl ToTokens for TraitName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::UnpinConstructor { name, params } | Self::Other { name, params, .. } => {
                let params = format_generic_params(params);
                quote! {#name #params}.to_tokens(tokens)
            }
            Self::CtorNew(arg_types) => {
                let arg_types = format_tuple_except_singleton(arg_types);
                quote! { ::ctor::CtorNew < #arg_types > }.to_tokens(tokens)
            }
        }
    }
}

/// The kind of the `impl` block the function needs to be generated in.
enum ImplKind {
    /// Used for free functions for which we don't want the `impl` block.
    None { is_unsafe: bool },
    /// Used for inherent methods for which we need an `impl SomeStruct { ... }`
    /// block.
    Struct {
        /// For example, `SomeStruct`. Retrieved from
        /// `func.member_func_metadata`.
        record_name: Ident,
        is_unsafe: bool,
        /// Whether to format the first parameter as "self" (e.g. `__this:
        /// &mut T` -> `&mut self`)
        format_first_param_as_self: bool,
    },
    /// Used for trait methods for which we need an `impl TraitName for
    /// SomeStruct { ... }` block.
    Trait {
        /// For example, `SomeStruct`.
        /// Note that `record_name` might *not* be from
        /// `func.member_func_metadata`.
        record_name: Ident,
        /// For example, `quote!{ From<i32> }`.
        trait_name: TraitName,
        /// Reference style for the `impl` block and self parameters.
        impl_for: ImplFor,

        /// The generic params of trait `impl` (e.g. `vec![quote!{'b}]`). These
        /// start empty and only later are mutated into the correct value.
        trait_generic_params: Vec<TokenStream>,

        /// Whether to format the first parameter as "self" (e.g. `__this:
        /// &mut T` -> `&mut self`)
        format_first_param_as_self: bool,
        /// Whether to drop the C++ function's return value and return unit
        /// instead.
        drop_return: bool,

        /// If this trait's method returns an associated type, it has this name.
        /// For example, this is `Output` on
        /// [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html).
        associated_return_type: Option<Ident>,
    },
}
impl ImplKind {
    fn new_trait(
        trait_name: TraitName,
        record_name: Ident,
        format_first_param_as_self: bool,
    ) -> Self {
        ImplKind::Trait {
            trait_name,
            record_name,
            impl_for: ImplFor::T,
            trait_generic_params: vec![],
            format_first_param_as_self,
            drop_return: false,
            associated_return_type: None,
        }
    }
    fn format_first_param_as_self(&self) -> bool {
        matches!(
            self,
            Self::Trait { format_first_param_as_self: true, .. }
                | Self::Struct { format_first_param_as_self: true, .. }
        )
    }
    /// Returns whether the function is defined as `unsafe fn ...`.
    fn is_unsafe(&self) -> bool {
        matches!(
            self,
            Self::None { is_unsafe: true, .. }
                | Self::Struct { is_unsafe: true, .. }
                | Self::Trait { trait_name: TraitName::Other { is_unsafe_fn: true, .. }, .. }
        )
    }
}

/// Whether the impl block is for T, and the receivers take self by reference,
/// or the impl block is for a reference to T, and the method receivers take
/// self by value.
enum ImplFor {
    /// Implement the trait for `T` directly.
    ///
    /// ```
    /// impl Trait for T {
    ///     fn const_method<'a>(&'a self);
    ///     fn mut_method<'a>(&'a mut self);
    ///     fn pin_method<'a>(Pin<&'a mut self>);
    /// }
    /// ```
    T,
    /// Implement the trait for `&T`, `&mut T`, or `Pin<&mut T>`, depending on
    /// the Rust type of the self parameter.
    ///
    /// ```
    /// impl<'a> Trait for &'a T {
    ///     fn const_method(self);
    /// }
    /// impl<'a> Trait for &'a mut UnpinT {
    ///     fn mut_method(self);
    /// }
    /// impl<'a> Trait for Pin<&'a mut NonUnpinT> {
    ///     fn pin_method(self);
    /// }
    /// ```
    RefT,
}

/// Returns whether an argument of this type causes ADL to include the `record`.
fn adl_expands_to(record: &Record, rs_type_kind: &RsTypeKind) -> bool {
    match rs_type_kind {
        RsTypeKind::Record { record: nested_record, .. } => ptr::eq(record, &**nested_record),
        RsTypeKind::Reference { referent, .. } => adl_expands_to(record, &**referent),
        RsTypeKind::RvalueReference { referent, .. } => adl_expands_to(record, &**referent),
        _ => false,
    }
}

/// Returns whether any type in `param_types` causes ADL to include `record`.
///
/// This is an under-approximation. Things not considered include class template
/// arguments and the parameters and return type of function types.
///
/// See https://en.cppreference.com/w/cpp/language/adl
fn is_visible_by_adl(enclosing_record: &Record, param_types: &[RsTypeKind]) -> bool {
    param_types.iter().any(|param_type| adl_expands_to(enclosing_record, param_type))
}

#[derive(Debug)]
struct OperatorMetadata {
    by_cc_name_and_params: HashMap<(&'static str, usize), OperatorMetadataEntry>,
}

#[derive(Clone, Copy, Debug)]
struct OperatorMetadataEntry {
    cc_name: &'static str,
    cc_params: usize,
    trait_name: &'static str,
    method_name: &'static str,
    is_compound_assignment: bool,
}

impl OperatorMetadataEntry {
    const fn unary(
        cc_name: &'static str,
        trait_name: &'static str,
        method_name: &'static str,
    ) -> Self {
        Self { cc_name, cc_params: 1, trait_name, method_name, is_compound_assignment: false }
    }

    const fn binary(
        cc_name: &'static str,
        trait_name: &'static str,
        method_name: &'static str,
    ) -> Self {
        Self { cc_name, cc_params: 2, trait_name, method_name, is_compound_assignment: false }
    }

    const fn assign(
        cc_name: &'static str,
        trait_name: &'static str,
        method_name: &'static str,
    ) -> Self {
        Self { cc_name, cc_params: 2, trait_name, method_name, is_compound_assignment: true }
    }
}

static OPERATOR_METADATA: Lazy<OperatorMetadata> = Lazy::new(|| {
    const ENTRIES: &[OperatorMetadataEntry] = &[
        OperatorMetadataEntry::unary("-", "Neg", "neg"),
        // The Rust `Not` trait matches with both the C++ `!` and `~` operators to some extent. The
        // two operators appear with similar frequency in our target codebase so it's not clear
        // which is better to map here. Mapping `operator!` to `Not` as chosen here means that a
        // C++ `!` matches up with a Rust `!`.
        OperatorMetadataEntry::unary("!", "Not", "not"),
        OperatorMetadataEntry::binary("+", "Add", "add"),
        OperatorMetadataEntry::binary("-", "Sub", "sub"),
        OperatorMetadataEntry::binary("*", "Mul", "mul"),
        OperatorMetadataEntry::binary("/", "Div", "div"),
        OperatorMetadataEntry::binary("%", "Rem", "rem"),
        OperatorMetadataEntry::binary("&", "BitAnd", "bitand"),
        OperatorMetadataEntry::binary("|", "BitOr", "bitor"),
        OperatorMetadataEntry::binary("^", "BitXor", "bitxor"),
        OperatorMetadataEntry::binary("<<", "Shl", "shl"),
        OperatorMetadataEntry::binary(">>", "Shr", "shr"),
        OperatorMetadataEntry::assign("+=", "AddAssign", "add_assign"),
        OperatorMetadataEntry::assign("-=", "SubAssign", "sub_assign"),
        OperatorMetadataEntry::assign("*=", "MulAssign", "mul_assign"),
        OperatorMetadataEntry::assign("/=", "DivAssign", "div_assign"),
        OperatorMetadataEntry::assign("%=", "RemAssign", "rem_assign"),
        OperatorMetadataEntry::assign("&=", "BitAndAssign", "bitand_assign"),
        OperatorMetadataEntry::assign("|=", "BitOrAssign", "bitor_assign"),
        OperatorMetadataEntry::assign("^=", "BitXorAssign", "bitxor_assign"),
        OperatorMetadataEntry::assign("<<=", "ShlAssign", "shl_assign"),
        OperatorMetadataEntry::assign(">>=", "ShrAssign", "shr_assign"),
    ];
    OperatorMetadata {
        by_cc_name_and_params: ENTRIES.iter().map(|e| ((e.cc_name, e.cc_params), *e)).collect(),
    }
});

/// Returns the shape of the generated Rust API for a given function definition.
///
/// If the shape is a trait, this also mutates the parameter types to be
/// trait-compatible. In particular, types which would be `impl Ctor<Output=T>`
/// become a `RvalueReference<'_, T>`.
///
/// Returns:
///
///  * `Err(_)`: something went wrong importing this function.
///  * `Ok(None)`: the function imported as "nothing". (For example, a defaulted
///    destructor might be mapped to no `Drop` impl at all.)
///  * `Ok((func_name, impl_kind))`: The function name and ImplKind.
fn api_func_shape(
    db: &dyn BindingsGenerator,
    func: &Func,
    param_types: &mut [RsTypeKind],
) -> Result<Option<(Ident, ImplKind)>> {
    let ir = db.ir();
    let op_meta = &*OPERATOR_METADATA;

    let maybe_record: Option<&Rc<Record>> = ir.record_for_member_func(func)?;
    let has_pointer_params = param_types.iter().any(|p| matches!(p, RsTypeKind::Pointer { .. }));
    let impl_kind: ImplKind;
    let func_name: syn::Ident;

    let adl_check_required_and_failed = if let Some(decl_id) = func.adl_enclosing_record {
        let adl_enclosing_record = ir
            .find_decl::<Rc<Record>>(decl_id)
            .with_context(|| format!("Failed to look up `adl_enclosing_record` of {:?}", func))?;
        !is_visible_by_adl(adl_enclosing_record, param_types)
    } else {
        false
    };

    match &func.name {
        UnqualifiedIdentifier::Operator(_) | UnqualifiedIdentifier::Identifier(_)
            if adl_check_required_and_failed =>
        {
            return Ok(None);
        }
        UnqualifiedIdentifier::Operator(op) if op.name == "==" => {
            assert_eq!(
                param_types.len(),
                2,
                "Unexpected number of parameters in operator==: {func:?}"
            );
            match (&param_types[0], &param_types[1]) {
                (
                    RsTypeKind::Reference { referent: lhs, mutability: Mutability::Const, .. },
                    RsTypeKind::Reference { referent: rhs, mutability: Mutability::Const, .. },
                ) => match &**lhs {
                    RsTypeKind::Record { record: lhs_record, .. } => {
                        let lhs: Ident = make_rs_ident(&lhs_record.rs_name);
                        func_name = make_rs_ident("eq");
                        impl_kind = ImplKind::new_trait(
                            TraitName::Other {
                                name: quote! {PartialEq},
                                params: vec![(**rhs).clone()],
                                is_unsafe_fn: false,
                            },
                            lhs,
                            /* format_first_param_as_self= */ true,
                        );
                    }
                    _ => {
                        bail!("operator== where lhs doesn't refer to a record",);
                    }
                },
                _ => {
                    bail!("operator== where operands are not const references",);
                }
            };
        }
        UnqualifiedIdentifier::Operator(op) if op.name == "=" => {
            assert_eq!(
                param_types.len(),
                2,
                "Unexpected number of parameters in operator=: {func:?}"
            );
            let record =
                maybe_record.ok_or_else(|| anyhow!("operator= must be a member function."))?;
            if record.is_unpin() {
                bail!("operator= for Unpin types is not yet supported.");
            }
            materialize_ctor_in_caller(func, param_types);
            let rhs = &param_types[1];
            impl_kind = {
                ImplKind::Trait {
                    trait_name: TraitName::Other {
                        name: quote! {::ctor::Assign},
                        params: vec![rhs.clone()],
                        is_unsafe_fn: false,
                    },
                    record_name: make_rs_ident(&record.rs_name),
                    impl_for: ImplFor::T,
                    trait_generic_params: vec![],
                    format_first_param_as_self: true,
                    drop_return: true,
                    associated_return_type: None,
                }
            };
            func_name = make_rs_ident("assign");
        }
        UnqualifiedIdentifier::Operator(op) => match op_meta
            .by_cc_name_and_params
            .get(&(op.name.as_str(), param_types.len()))
        {
            Some(OperatorMetadataEntry {
                trait_name,
                method_name,
                is_compound_assignment: false,
                ..
            }) => {
                materialize_ctor_in_caller(func, param_types);
                let (record, impl_for) = if let Some(record) = maybe_record {
                    (&**record, ImplFor::RefT)
                } else {
                    match &param_types[0] {
                        RsTypeKind::Record { record, .. } => (&**record, ImplFor::T),
                        RsTypeKind::Reference { referent, .. } => (
                            match &**referent {
                                RsTypeKind::Record { record, .. } => &**record,
                                _ => bail!("Expected first parameter referent to be a record"),
                            },
                            ImplFor::RefT,
                        ),
                        RsTypeKind::RvalueReference { .. } => {
                            bail!("Not yet supported for rvalue references (b/219826128)")
                        }
                        _ => bail!("Expected first parameter to be a record or reference"),
                    }
                };

                let trait_name = make_rs_ident(trait_name);
                impl_kind = ImplKind::Trait {
                    record_name: make_rs_ident(&record.rs_name),
                    trait_name: TraitName::Other {
                        name: quote! {::std::ops::#trait_name},
                        params: param_types[1..].to_vec(),
                        is_unsafe_fn: false,
                    },
                    impl_for,
                    trait_generic_params: vec![],
                    format_first_param_as_self: true,
                    drop_return: false,
                    associated_return_type: Some(make_rs_ident("Output")),
                };
                func_name = make_rs_ident(method_name);
            }
            Some(OperatorMetadataEntry {
                trait_name,
                method_name,
                is_compound_assignment: true,
                ..
            }) => {
                materialize_ctor_in_caller(func, param_types);
                let record = match &param_types[0] {
                    RsTypeKind::Record { .. } => {
                        bail!("Compound assignment with by-value left-hand side is not supported")
                    }
                    RsTypeKind::Reference { mutability: Mutability::Const, .. } => {
                        bail!("Compound assignment with const left-hand side is not supported")
                    }
                    RsTypeKind::Reference { referent, mutability: Mutability::Mut, .. } => {
                        match &**referent {
                            RsTypeKind::Record { record, .. } => &**maybe_record.unwrap_or(record),
                            _ => bail!("Expected first parameter referent to be a record"),
                        }
                    }
                    RsTypeKind::RvalueReference { .. } => {
                        bail!("Not yet supported for rvalue references (b/219826128)")
                    }
                    RsTypeKind::Pointer { .. } => {
                        bail!("Not yet supported for pointers with unknown lifetime (b/219826128)")
                    }
                    _ => bail!("Expected first parameter to be a record or reference"),
                };

                let trait_name = make_rs_ident(trait_name);
                impl_kind = ImplKind::Trait {
                    record_name: make_rs_ident(&record.rs_name),
                    trait_name: TraitName::Other {
                        name: quote! {::std::ops::#trait_name},
                        params: param_types[1..].to_vec(),
                        is_unsafe_fn: false,
                    },
                    impl_for: ImplFor::T,
                    trait_generic_params: vec![],
                    format_first_param_as_self: true,
                    drop_return: true,
                    associated_return_type: None,
                };
                func_name = make_rs_ident(method_name);
            }
            None => {
                bail!(
                    "Bindings for this kind of operator (operator {op} with {n} parameter(s)) are not supported",
                    op = &op.name,
                    n = param_types.len(),
                );
            }
        },
        UnqualifiedIdentifier::Identifier(id) => {
            func_name = make_rs_ident(&id.identifier);
            match maybe_record {
                None => {
                    impl_kind = ImplKind::None { is_unsafe: has_pointer_params };
                }
                Some(record) => {
                    let format_first_param_as_self = if func.is_instance_method() {
                        let first_param = param_types.first().ok_or_else(|| {
                            anyhow!("Missing `__this` parameter in an instance method: {:?}", func)
                        })?;
                        first_param.is_ref_to(record)
                    } else {
                        false
                    };
                    impl_kind = ImplKind::Struct {
                        record_name: make_rs_ident(&record.rs_name),
                        format_first_param_as_self,
                        is_unsafe: has_pointer_params,
                    };
                }
            };
        }
        UnqualifiedIdentifier::Destructor => {
            // Note: to avoid double-destruction of the fields, they are all wrapped in
            // ManuallyDrop in this case. See `generate_record`.
            let record =
                maybe_record.ok_or_else(|| anyhow!("Destructors must be member functions."))?;
            if !should_implement_drop(record) {
                return Ok(None);
            }
            if record.is_unpin() {
                impl_kind = ImplKind::new_trait(
                    TraitName::Other { name: quote! {Drop}, params: vec![], is_unsafe_fn: false },
                    make_rs_ident(&record.rs_name),
                    /* format_first_param_as_self= */ true,
                );
                func_name = make_rs_ident("drop");
            } else {
                materialize_ctor_in_caller(func, param_types);
                impl_kind = ImplKind::new_trait(
                    TraitName::Other {
                        name: quote! {::ctor::PinnedDrop},
                        params: vec![],
                        is_unsafe_fn: true,
                    },
                    make_rs_ident(&record.rs_name),
                    /* format_first_param_as_self= */ true,
                );
                func_name = make_rs_ident("pinned_drop");
            }
        }
        UnqualifiedIdentifier::Constructor => {
            let member_func_metadata = func
                .member_func_metadata
                .as_ref()
                .ok_or_else(|| anyhow!("Constructors must be member functions."))?;
            let record = maybe_record
                .ok_or_else(|| anyhow!("Constructors must be associated with a record."))?;
            let instance_method_metadata =
                member_func_metadata
                    .instance_method_metadata
                    .as_ref()
                    .ok_or_else(|| anyhow!("Constructors must be instance methods."))?;
            if has_pointer_params {
                // TODO(b/216648347): Allow this outside of traits (e.g. after supporting
                // translating C++ constructors into static methods in Rust).
                bail!(
                    "Unsafe constructors (e.g. with no elided or explicit lifetimes) \
                    are intentionally not supported",
                );
            }

            check_by_value(record)?;
            materialize_ctor_in_caller(func, param_types);
            let record_name = make_rs_ident(&record.rs_name);
            if !record.is_unpin() {
                func_name = make_rs_ident("ctor_new");

                match param_types {
                    [] => bail!("Missing `__this` parameter in a constructor: {:?}", func),
                    [_this, params @ ..] => {
                        impl_kind = ImplKind::Trait {
                            record_name,
                            trait_name: TraitName::CtorNew(params.iter().cloned().collect()),
                            impl_for: ImplFor::T,
                            trait_generic_params: vec![],
                            format_first_param_as_self: false,
                            drop_return: false,
                            associated_return_type: Some(make_rs_ident("CtorType")),
                        };
                    }
                }
            } else {
                match func.params.len() {
                    0 => bail!("Missing `__this` parameter in a constructor: {:?}", func),
                    1 => {
                        impl_kind = ImplKind::new_trait(
                            TraitName::UnpinConstructor { name: quote! {Default}, params: vec![] },
                            record_name,
                            /* format_first_param_as_self= */ false,
                        );
                        func_name = make_rs_ident("default");
                    }
                    2 => {
                        if param_types[1].is_shared_ref_to(record) {
                            // Copy constructor
                            if should_derive_clone(record) {
                                return Ok(None);
                            } else {
                                impl_kind = ImplKind::new_trait(
                                    TraitName::UnpinConstructor {
                                        name: quote! {Clone},
                                        params: vec![],
                                    },
                                    record_name,
                                    /* format_first_param_as_self= */ true,
                                );
                                func_name = make_rs_ident("clone");
                            }
                        } else if !instance_method_metadata.is_explicit_ctor {
                            let param_type = &param_types[1];
                            impl_kind = ImplKind::new_trait(
                                TraitName::UnpinConstructor {
                                    name: quote! {From},
                                    params: vec![param_type.clone()],
                                },
                                record_name,
                                /* format_first_param_as_self= */ false,
                            );
                            func_name = make_rs_ident("from");
                        } else {
                            bail!("Not yet supported type of constructor parameter",);
                        }
                    }
                    _ => {
                        // TODO(b/216648347): Support bindings for other constructors.
                        bail!("More than 1 constructor parameter is not supported yet",);
                    }
                }
            }
        }
    }
    Ok(Some((func_name, impl_kind)))
}

/// Mutates the provided parameters so that nontrivial by-value parameters are,
/// instead, materialized in the caller and passed by rvalue reference.
fn materialize_ctor_in_caller(func: &Func, params: &mut [RsTypeKind]) {
    let mut existing_lifetime_params: HashSet<Rc<str>> =
        params.iter().flat_map(|param| param.lifetimes().map(|lifetime| lifetime.0)).collect();
    let mut new_lifetime_param = |mut lifetime_name: String| {
        let suffix_start = lifetime_name.len();
        let mut next_suffix = 2;
        loop {
            if !existing_lifetime_params.contains(&*lifetime_name) {
                let lifetime_name = <Rc<str>>::from(lifetime_name);
                existing_lifetime_params.insert(lifetime_name.clone());
                return Lifetime(lifetime_name);
            }
            lifetime_name.truncate(suffix_start);
            write!(lifetime_name, "_{next_suffix}").unwrap();
            next_suffix += 1;
        }
    };
    for (func_param, param) in func.params.iter().zip(params.iter_mut()) {
        if param.is_unpin() {
            continue;
        }
        let value = std::mem::replace(param, RsTypeKind::Unit); // Temporarily swap in a garbage value.
        *param = RsTypeKind::RvalueReference {
            referent: Rc::new(value),
            mutability: Mutability::Mut,
            lifetime: new_lifetime_param(func_param.identifier.identifier.clone()),
        };
    }
}

/// Generates Rust source code for a given `Func`.
///
/// Returns:
///
///  * `Err(_)`: couldn't import the function, emit an `UnsupportedItem`.
///  * `Ok(None)`: the function imported as "nothing". (For example, a defaulted
///    destructor might be mapped to no `Drop` impl at all.)
///  * `Ok((rs_api, rs_thunk, function_id))`: The Rust function definition,
///    thunk FFI definition, and function ID.
fn generate_func(
    db: &dyn BindingsGenerator,
    func: Rc<Func>,
) -> Result<Option<RcEq<(RsSnippet, RsSnippet, Rc<FunctionId>)>>> {
    let ir = db.ir();
    let mut features = BTreeSet::new();
    let mut param_types = func
        .params
        .iter()
        .map(|p| {
            db.rs_type_kind(p.type_.rs_type.clone()).with_context(|| {
                format!("Failed to process type of parameter {:?} on {:?}", p, func)
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let (func_name, mut impl_kind) =
        if let Some(values) = api_func_shape(db, &func, &mut param_types)? {
            values
        } else {
            return Ok(None);
        };
    let namespace_qualifier = format_rs_namespace_qualifier(func.id, &ir)?;

    let mut return_type = db
        .rs_type_kind(func.return_type.rs_type.clone())
        .with_context(|| format!("Failed to format return type for {:?}", &func))?;
    return_type.check_by_value()?;
    let param_idents =
        func.params.iter().map(|p| make_rs_ident(&p.identifier.identifier)).collect_vec();
    let thunk = generate_func_thunk(db, &func, &param_idents, &param_types, &return_type)?;

    let BindingsSignature {
        lifetimes,
        params: api_params,
        return_type_fragment: mut quoted_return_type,
        thunk_prepare,
        thunk_args,
    } = function_signature(
        &mut features,
        &func,
        &impl_kind,
        &param_idents,
        &mut param_types,
        &mut return_type,
    )?;

    let api_func_def = {
        // TODO(b/200067242): the Pin-wrapping code doesn't know to wrap &mut
        // MaybeUninit<T> in Pin if T is !Unpin. It should understand
        // 'structural pinning', so that we do not need into_inner_unchecked()
        // here.
        let thunk_ident = thunk_ident(&func);
        let func_body = match &impl_kind {
            ImplKind::Trait { trait_name: TraitName::UnpinConstructor { .. }, .. } => {
                // SAFETY: A user-defined constructor is not guaranteed to
                // initialize all the fields. To make the `assume_init()` call
                // below safe, the memory is zero-initialized first. This is a
                // bit safer, because zero-initialized memory represents a valid
                // value for the currently supported field types (this may
                // change once the bindings generator starts supporting
                // reference fields). TODO(b/213243309): Double-check if
                // zero-initialization is desirable here.
                quote! {
                    let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
                    unsafe {
                        crate::detail::#thunk_ident( &mut tmp #( , #thunk_args )* );
                        tmp.assume_init()
                    }
                }
            }
            _ => {
                // Note: for the time being, all !Unpin values are treated as if they were not
                // trivially relocatable. We could, in the special case of trivial !Unpin types,
                // not generate the thunk at all, but this would be a bit of extra work.
                //
                // TODO(jeanpierreda): separately handle non-Unpin and non-trivial types.
                let mut body = if return_type.is_unpin() {
                    quote! { crate::detail::#thunk_ident( #( #thunk_args ),* ) }
                } else {
                    quote! {
                        ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<#return_type>>| {
                            crate::detail::#thunk_ident(::std::pin::Pin::into_inner_unchecked(dest) #( , #thunk_args )*);
                        })
                    }
                };
                // Discard the return value if requested (for example, when calling a C++
                // operator that returns a value from a Rust trait that returns
                // unit).
                if let ImplKind::Trait { drop_return: true, .. } = impl_kind {
                    if return_type.is_unpin() {
                        // If it's unpin, just discard it:
                        body = quote! { #body; };
                    } else {
                        // Otherwise, in order to discard the return value and return void, we
                        // need to run the constructor.
                        body = quote! {let _ = ::ctor::emplace!(#body);};
                    }

                    // We would need to do this, but it's no longer used:
                    //    return_type = RsTypeKind::Unit;
                    let _ = return_type; // proof that we don't need to update it.
                    quoted_return_type = quote! {};
                }
                // Only need to wrap everything in an `unsafe { ... }` block if
                // the *whole* api function is safe.
                if !impl_kind.is_unsafe() {
                    body = quote! { unsafe { #body } };
                }
                quote! {
                    #thunk_prepare
                    #body
                }
            }
        };

        let pub_ = match impl_kind {
            ImplKind::None { .. } | ImplKind::Struct { .. } => quote! { pub },
            ImplKind::Trait { .. } => quote! {},
        };
        let unsafe_ = if impl_kind.is_unsafe() {
            quote! { unsafe }
        } else {
            quote! {}
        };

        let fn_generic_params: TokenStream;
        if let ImplKind::Trait { trait_name, trait_generic_params, impl_for, .. } = &mut impl_kind {
            // When the impl block is for some kind of reference to T, consider the lifetime
            // parameters on the self parameter to be trait lifetimes so they can be
            // introduced before they are used.
            let first_param_lifetimes = match (impl_for, param_types.first()) {
                (ImplFor::RefT, Some(first_param)) => Some(first_param.lifetimes()),
                _ => None,
            };

            let trait_lifetimes: HashSet<Lifetime> =
                trait_name.lifetimes().chain(first_param_lifetimes.into_iter().flatten()).collect();
            fn_generic_params = format_generic_params(
                lifetimes.iter().filter(|lifetime| !trait_lifetimes.contains(lifetime)),
            );
            *trait_generic_params = lifetimes
                .iter()
                .filter_map(|lifetime| {
                    if trait_lifetimes.contains(lifetime) {
                        Some(quote! {#lifetime})
                    } else {
                        None
                    }
                })
                .collect();
        } else {
            fn_generic_params = format_generic_params(lifetimes);
        }

        let function_return_type = match &impl_kind {
            ImplKind::Trait { associated_return_type: Some(ident), .. } => quote! {Self::#ident},
            _ => quoted_return_type.clone(),
        };
        let arrow = if !function_return_type.is_empty() {
            quote! {->}
        } else {
            quote! {}
        };

        quote! {
            #[inline(always)]
            #pub_ #unsafe_ fn #func_name #fn_generic_params(
                    #( #api_params ),* ) #arrow #function_return_type {
                #func_body
            }
        }
    };

    let doc_comment = generate_doc_comment(&func.doc_comment);
    let api_func: TokenStream;
    let function_id: FunctionId;
    match impl_kind {
        ImplKind::None { .. } => {
            api_func = quote! { #doc_comment #api_func_def };
            function_id = FunctionId {
                self_type: None,
                function_path: syn::parse2(quote! { #namespace_qualifier #func_name }).unwrap(),
            };
        }
        ImplKind::Struct { record_name, .. } => {
            api_func = quote! { impl #record_name { #doc_comment #api_func_def } };
            function_id = FunctionId {
                self_type: None,
                function_path: syn::parse2(quote! {
                    #namespace_qualifier #record_name :: #func_name
                }).unwrap(),
            };
        }
        ImplKind::Trait {
            trait_name,
            record_name,
            impl_for,
            trait_generic_params,
            associated_return_type,
            ..
        } => {
            let extra_body = if let Some(name) = associated_return_type {
                let quoted_return_type = if quoted_return_type.is_empty() {
                    quote! {()}
                } else {
                    quoted_return_type
                };
                quote! {
                    type #name = #quoted_return_type;
                }
            } else {
                quote! {}
            };

            let extra_items;
            let trait_generic_params = format_generic_params(trait_generic_params);
            match &trait_name {
                TraitName::CtorNew(params) => {
                    if let [single_param] = params.as_slice() {
                        extra_items = quote! {
                            impl #trait_generic_params ::ctor::CtorNew<(#single_param,)> for #record_name {
                                #extra_body

                                #[inline (always)]
                                fn ctor_new(args: (#single_param,)) -> Self::CtorType {
                                    let (arg,) = args;
                                    <Self as ::ctor::CtorNew<#single_param>>::ctor_new(arg)
                                }
                            }
                        }
                    } else {
                        extra_items = quote! {}
                    }
                }
                _ => {
                    extra_items = quote! {};
                }
            };
            let impl_for = match impl_for {
                ImplFor::T => quote! { #record_name },
                ImplFor::RefT => {
                    let param = &param_types[0];
                    quote! { #param }
                }
            };
            api_func = quote! {
                #doc_comment
                impl #trait_generic_params #trait_name for #impl_for {
                    #extra_body
                    #api_func_def
                }
                #extra_items
            };
            function_id = FunctionId {
                // Because of ADL rules the `impl_for` record is always in the same namespace as
                // the function behind the trait implementation.  Therefore, it is okay to use the
                // `namespace_qualifier` (of the function, not the record) below.
                self_type: Some(syn::parse2(quote! { #namespace_qualifier #record_name }).unwrap()),
                function_path: syn::parse2(quote! { #trait_name :: #func_name }).unwrap(),
            };
        }
    }

    Ok(Some(RcEq::new((
        RsSnippet { features, tokens: api_func },
        thunk.into(),
        Rc::new(function_id),
    ))))
}

/// The function signature for a function's bindings.
struct BindingsSignature {
    /// The lifetime parameters for the Rust function.
    lifetimes: Vec<Lifetime>,

    /// The parameter list for the Rust function.
    ///
    /// For example, `vec![quote!{self}, quote!{x: &i32}]`.
    params: Vec<TokenStream>,

    /// The return type fragment of the Rust function, as a token stream.
    ///
    /// This is the same as the actual return type, except that () is the empty
    /// tokens, non-Unpin by-value types are `impl Ctor<Output=#return_type> +
    /// ...`, and wherever the type is the type of `Self`, it gets replaced by
    /// literal `Self`.
    return_type_fragment: TokenStream,

    /// Any preparation code to define the arguments in `thunk_args`.
    thunk_prepare: TokenStream,

    /// The arguments passed to the thunk, expressed in terms of `params`.
    thunk_args: Vec<TokenStream>,
}

/// Reformats API parameters and return values to match Rust conventions and the
/// trait requirements.
///
/// For example:
///
/// * Use the `self` keyword for the this pointer.
/// * Use `Self` for the return value of constructor traits.
/// * For C++ constructors, remove `self` from the Rust side (as it becomes the
///   return value), retaining it on the C++ side / thunk args.
/// * serialize a `()` as the empty string.
fn function_signature(
    features: &mut BTreeSet<Ident>,
    func: &Func,
    impl_kind: &ImplKind,
    param_idents: &[Ident],
    param_types: &mut Vec<RsTypeKind>,
    return_type: &mut RsTypeKind,
) -> Result<BindingsSignature> {
    let mut api_params = Vec::with_capacity(func.params.len());
    let mut thunk_args = Vec::with_capacity(func.params.len());
    let mut thunk_prepare = quote! {};
    for (i, (ident, type_)) in param_idents.iter().zip(param_types.iter()).enumerate() {
        type_.check_by_value()?;
        if !type_.is_unpin() {
            // `impl Ctor` will fail to compile in a trait.
            // This will only be hit if there was a bug in api_func_shape.
            if let ImplKind::Trait { .. } = &impl_kind {
                panic!(
                    "non-Unpin types cannot work by value in traits; this should have instead \
                        become an rvalue reference to force the caller to materialize the Ctor."
                );
            }
            // The generated bindings require a move constructor.
            if !type_.is_move_constructible() {
                bail!("Non-movable, non-trivial_abi type '{type}' is not supported by value as parameter #{i}", type=quote!{#type_});
            }
            api_params.push(quote! {#ident: impl ::ctor::Ctor<Output=#type_>});
            thunk_args
                .push(quote! {::std::pin::Pin::into_inner_unchecked(::ctor::emplace!(#ident))});
        } else {
            api_params.push(quote! {#ident: #type_});
            thunk_args.push(quote! {#ident});
        }
    }

    let mut lifetimes: Vec<Lifetime> = unique_lifetimes(&*param_types).collect();

    let mut quoted_return_type = None;
    if let ImplKind::Trait {
        trait_name: trait_name @ (TraitName::UnpinConstructor { .. } | TraitName::CtorNew(..)),
        ..
    } = &impl_kind
    {
        // For constructors, we move the output parameter to be the return value.
        // The return value is "really" void.
        ensure!(
            func.return_type.rs_type.is_unit_type(),
            "Unexpectedly non-void return type of a constructor"
        );

        //  Presence of element #0 is indirectly verified by a `Constructor`-related
        // `match` branch a little bit above.
        *return_type = param_types[0]
            .referent()
            .ok_or_else(|| anyhow!("Expected pointer/reference for `__this` parameter"))?
            .clone();
        quoted_return_type = Some(quote! {Self});

        // Grab the `__this` lifetime to remove it from the lifetime parameters.
        let this_lifetime = param_types[0]
            .lifetime()
            .ok_or_else(|| anyhow!("Missing lifetime for `__this` parameter"))?;

        // Drop `__this` parameter from the public Rust API.
        api_params.remove(0);
        thunk_args.remove(0);
        param_types.remove(0);

        // Remove the lifetime associated with `__this`.
        lifetimes.retain(|l| l != &this_lifetime);
        if let Some(type_still_dependent_on_removed_lifetime) = param_types
            .iter()
            .flat_map(|t| t.lifetimes())
            .find(|lifetime| lifetime == &this_lifetime)
        {
            bail!(
                "The lifetime of `__this` is unexpectedly also used by another \
                    parameter: {type_still_dependent_on_removed_lifetime:?}",
            );
        }

        // CtorNew groups parameters into a tuple.
        if let TraitName::CtorNew(args_type) = trait_name {
            let args_type = format_tuple_except_singleton(args_type);
            api_params = vec![quote! {args: #args_type}];
            let thunk_vars = format_tuple_except_singleton(&thunk_args);
            thunk_prepare.extend(quote! {let #thunk_vars = args;});
        }
    }

    let return_type_fragment = if return_type == &RsTypeKind::Unit {
        quote! {}
    } else {
        let ty = quoted_return_type.unwrap_or_else(|| quote! {#return_type});
        if return_type.is_unpin() {
            quote! {#ty}
        } else {
            // This feature seems destined for stabilization, and makes the code
            // simpler. We don't need it for simple functions, but if the return type is
            // used as an associated type for a trait.
            features.insert(make_rs_ident("type_alias_impl_trait"));
            // The returned lazy FnCtor depends on all inputs.
            let extra_lifetimes = lifetimes.iter().map(|a| quote! {+ ::ctor::Captures<#a>});
            quote! {impl ::ctor::Ctor<Output=#ty> #(#extra_lifetimes)* }
        }
    };

    // Change `__this: &'a SomeStruct` into `&'a self` if needed.
    if impl_kind.format_first_param_as_self() {
        let first_api_param = param_types
            .get(0)
            .ok_or_else(|| anyhow!("No parameter to format as 'self': {:?}", func))?;
        // If param_types[0] exists, so do api_params[0] and thunk_args[0].
        match impl_kind {
            ImplKind::None { .. } => unreachable!(),
            ImplKind::Struct { .. } | ImplKind::Trait { impl_for: ImplFor::T, .. } => {
                // In the ImplFor::T reference style (which is implied for ImplKind::Struct) the
                // impl block is for `T`. The `self` parameter has a type determined by the
                // first parameter (typically a reference of some kind) and can be passed to a
                // thunk via the expression `self`.
                api_params[0] = first_api_param.format_as_self_param()?;
                thunk_args[0] = quote! { self };
            }
            ImplKind::Trait { impl_for: ImplFor::RefT, .. } => {
                // In the ImplFor::RefT reference style the impl block is for a reference type
                // referring to T (`&T`, `&mut T`, or `Pin<&mut T>` so a bare `self` parameter
                // has that type and can be passed to a thunk via the expression `self`.
                api_params[0] = quote! { self };
                thunk_args[0] = quote! { self };
            }
        }
    }

    Ok(BindingsSignature {
        lifetimes,
        params: api_params,
        return_type_fragment,
        thunk_prepare,
        thunk_args,
    })
}

fn generate_func_thunk(
    db: &dyn BindingsGenerator,
    func: &Func,
    param_idents: &[Ident],
    param_types: &[RsTypeKind],
    return_type: &RsTypeKind,
) -> Result<TokenStream> {
    let thunk_attr = if can_skip_cc_thunk(db, func) {
        let mangled_name = &func.mangled_name;
        quote! {#[link_name = #mangled_name]}
    } else {
        quote! {}
    };
    let lifetimes: Vec<_> = unique_lifetimes(param_types).collect();

    // The first parameter is the output parameter, if any.
    let mut param_types = param_types.into_iter();
    let mut param_idents = param_idents.into_iter();
    let mut out_param = None;
    let mut out_param_ident = None;
    let mut return_type_fragment = return_type.format_as_return_type_fragment();
    if func.name == UnqualifiedIdentifier::Constructor {
        // For constructors, inject MaybeUninit into the type of `__this_` parameter.
        let first_param = param_types
            .next()
            .ok_or_else(|| anyhow!("Constructors should have at least one parameter (__this)"))?;
        out_param = Some(first_param.format_mut_ref_as_uninitialized().with_context(|| {
            format!(
                "Failed to format `__this` param for a constructor thunk: {:?}",
                func.params.get(0)
            )
        })?);
        out_param_ident = Some(param_idents.next().unwrap().clone());
    } else if !return_type.is_unpin() {
        // For nontrivial return types, create a new out parameter.
        // The lifetime doesn't matter, so we can insert a new anonymous lifetime here.
        out_param = Some(quote! {
            &mut ::std::mem::MaybeUninit< #return_type >
        });
        out_param_ident = Some(make_rs_ident("__return"));
        return_type_fragment = quote! {};
    }

    let thunk_ident = thunk_ident(&func);

    let generic_params = format_generic_params(lifetimes);
    let param_idents = out_param_ident.as_ref().into_iter().chain(param_idents);
    let param_types = out_param.into_iter().chain(param_types.map(|t| {
        if !t.is_unpin() {
            quote! {&mut #t}
        } else {
            quote! {#t}
        }
    }));

    Ok(quote! {
        #thunk_attr
        pub(crate) fn #thunk_ident #generic_params( #( #param_idents: #param_types ),*
        ) #return_type_fragment ;
    })
}

fn generate_doc_comment(comment: &Option<String>) -> TokenStream {
    match comment {
        Some(text) => {
            // token_stream_printer (and rustfmt) don't put a space between /// and the doc
            // comment, let's add it here so our comments are pretty.
            let doc = format!(" {}", text.replace('\n', "\n "));
            quote! {#[doc=#doc]}
        }
        None => quote! {},
    }
}

fn format_generic_params<T: ToTokens>(params: impl IntoIterator<Item = T>) -> TokenStream {
    let mut params = params.into_iter().peekable();
    if params.peek().is_none() {
        quote! {}
    } else {
        quote! { < #( #params ),* > }
    }
}

/// Formats singletons as themselves, and collections of n!=1 items as a tuple.
///
/// In other words, this formats a collection of things as if via `#(#items),*`,
/// but without lint warnings.
///
/// For example:
///
/// * [] => ()
/// * [x] => x  // equivalent to (x), but lint-free.
/// * [x, y] => (x, y)
fn format_tuple_except_singleton<T: ToTokens>(items: &[T]) -> TokenStream {
    match items {
        [singleton] => quote! {#singleton},
        items => quote! {(#(#items),*)},
    }
}

fn should_implement_drop(record: &Record) -> bool {
    match record.destructor {
        // TODO(b/202258760): Only omit destructor if `Copy` is specified.
        SpecialMemberFunc::Trivial => false,

        // TODO(b/212690698): Avoid calling into the C++ destructor (e.g. let
        // Rust drive `drop`-ing) to avoid (somewhat unergonomic) ManuallyDrop
        // if we can ask Rust to preserve C++ field destruction order in
        // NontrivialMembers case.
        SpecialMemberFunc::NontrivialMembers => true,

        // The `impl Drop` for NontrivialUserDefined needs to call into the
        // user-defined destructor on C++ side.
        SpecialMemberFunc::NontrivialUserDefined => true,

        // TODO(b/213516512): Today the IR doesn't contain Func entries for
        // deleted functions/destructors/etc. But, maybe we should generate
        // `impl Drop` in this case? With `unreachable!`? With
        // `std::mem::forget`?
        SpecialMemberFunc::Unavailable => false,
    }
}

/// Returns whether fields of type `ty` need to be wrapped in `ManuallyDrop<T>`
/// to prevent the fields from being destructed twice (once by the C++
/// destructor calkled from the `impl Drop` of the struct and once by `drop` on
/// the Rust side).
///
/// A type is safe to destroy twice if it implements `Copy`. Fields of such
/// don't need to be wrapped in `ManuallyDrop<T>` even if the struct
/// containing the fields provides an `impl Drop` that calles into a C++
/// destructor (in addition to dropping the fields on the Rust side).
///
/// Note that it is not enough to just be `!needs_drop<T>()`: Rust only
/// guarantees that it is safe to use-after-destroy for `Copy` types. See
/// e.g. the documentation for
/// [`drop_in_place`](https://doc.rust-lang.org/std/ptr/fn.drop_in_place.html):
///
/// > if `T` is not `Copy`, using the pointed-to value after calling
/// > `drop_in_place` can cause undefined behavior
///
/// For non-Copy union fields, failing to use `ManuallyDrop<T>` would
/// additionally cause a compile-time error until https://github.com/rust-lang/rust/issues/55149 is stabilized.
fn needs_manually_drop(db: &Database, ty: ir::RsType) -> Result<bool> {
    let ty_implements_copy = db.rs_type_kind(ty)?.implements_copy();
    Ok(!ty_implements_copy)
}

fn get_namespaces(item_id: ItemId, ir: &IR) -> Result<impl Iterator<Item = &Namespace>> {
    let mut namespaces = vec![];
    let item: &Item = ir.find_decl(item_id)?;
    let mut enclosing_namespace_id = item.enclosing_namespace_id();
    while let Some(parent_id) = enclosing_namespace_id {
        let namespace_item = ir.find_decl(parent_id)?;
        match namespace_item {
            Item::Namespace(ns) => {
                namespaces.push(ns.as_ref());
                enclosing_namespace_id = ns.enclosing_namespace_id;
            }
            _ => {
                bail!("Expected namespace");
            }
        }
    }
    Ok(namespaces.into_iter().rev())
}

/// Returns the Rust namespace qualifier for the given item.
fn format_rs_namespace_qualifier(item_id: ItemId, ir: &IR) -> Result<TokenStream> {
    let namespace_rs_idents =
        get_namespaces(item_id, ir)?.map(|ns| make_rs_ident(&ns.name.identifier));
    Ok(quote! { #(#namespace_rs_idents::)* })
}

/// Returns the C++ namespace qualifier for the given item.
fn format_cc_namespace_qualifier(item_id: ItemId, ir: &IR) -> Result<TokenStream> {
    let namespace_cc_idents =
        get_namespaces(item_id, ir)?.map(|ns| format_cc_ident(&ns.name.identifier));
    Ok(quote! { #(#namespace_cc_idents::)* })
}

/// Generates Rust source code for a given incomplete record declaration.
fn generate_incomplete_record(incomplete_record: &IncompleteRecord) -> Result<TokenStream> {
    let ident = make_rs_ident(&incomplete_record.rs_name);
    let name = &incomplete_record.rs_name;
    Ok(quote! {
        forward_declare::forward_declare!(
            pub #ident __SPACE__ = __SPACE__ forward_declare::symbol!(#name)
        );
    })
}

fn make_rs_field_ident(field: &Field, field_index: usize) -> Ident {
    match field.identifier.as_ref() {
        None => make_rs_ident(&format!("__unnamed_field{}", field_index)),
        Some(Identifier { identifier }) => make_rs_ident(identifier),
    }
}

/// Gets the type of `field` for layout purposes.
///
/// Note that `get_field_rs_type_for_layout` may return Err (for
/// `is_no_unique_address` fields) even if `field.type_` is Ok.
fn get_field_rs_type_for_layout(field: &Field) -> Result<&RsType, &str> {
    // [[no_unique_address]] fields are replaced by a type-less, unaligned block of
    // memory which fills space up to the next field.
    // See: docs/struct_layout
    if field.is_no_unique_address {
        return Err("`[[no_unique_address]]` attribute was present.");
    }

    field.type_.as_ref().map(|t| &t.rs_type).map_err(String::as_str)
}

/// Returns the type of a type-less, unaligned block of memory that can hold a
/// specified number of bits, rounded up to the next multiple of 8.
fn bit_padding(padding_size_in_bits: usize) -> TokenStream {
    let padding_size = Literal::usize_unsuffixed((padding_size_in_bits + 7) / 8);
    quote! { [::std::mem::MaybeUninit<u8>; #padding_size] }
}

/// Generates Rust source code for a given `Record` and associated assertions as
/// a tuple.
fn generate_record(db: &Database, record: &Rc<Record>) -> Result<GeneratedItem> {
    let ir = db.ir();
    let ident = make_rs_ident(&record.rs_name);
    let namespace_qualifier = format_rs_namespace_qualifier(record.id, &ir)?;
    let qualified_ident = {
        quote! { crate:: #namespace_qualifier #ident }
    };
    let doc_comment = generate_doc_comment(&record.doc_comment);

    let mut field_copy_trait_assertions: Vec<TokenStream> = vec![];

    let fields_with_bounds = (record.fields.iter())
        .map(|field| {
            (
                // We don't represent bitfields directly in Rust. We drop the field itself here
                // and only retain the offset information. Adjacent bitfields then get merged in
                // the next step.
                if field.is_bitfield { None } else { Some(field) },
                field.offset,
                // We retain the end offset of fields only if we have a matching Rust type
                // to represent them. Otherwise we'll fill up all the space to the next field.
                // See: docs/struct_layout
                match get_field_rs_type_for_layout(field) {
                    // Regular field
                    Ok(_rs_type) => Some(field.offset + field.size),
                    // Opaque field
                    Err(_error) => {
                        if record.is_union() {
                            Some(field.size)
                        } else {
                            None
                        }
                    }
                },
                vec![format!(
                    "{} : {} bits",
                    field.identifier.as_ref().map(|i| i.identifier.clone()).unwrap_or("".into()),
                    field.size
                )],
            )
        })
        // Merge consecutive bitfields. This is necessary, because they may share storage in the
        // same byte.
        .coalesce(|first, second| match (first, second) {
            ((None, offset, _, desc1), (None, _, end, desc2)) => {
                Ok((None, offset, end, [desc1, desc2].concat()))
            }
            pair => Err(pair),
        });

    // Pair up fields with the preceeding and following fields (if any):
    // - the end offset of the previous field determines if we need to insert
    //   padding.
    // - the start offset of the next field may be need to grow the current field to
    //   there.
    // This uses two separate `map` invocations on purpose to limit available state.
    let field_definitions = iter::once(None)
        .chain(fields_with_bounds.clone().map(Some))
        .chain(iter::once(None))
        .tuple_windows()
        .map(|(prev, cur, next)| {
            let (field, offset, end, desc) = cur.unwrap();
            let prev_end = prev.as_ref().map(|(_, _, e, _)| *e).flatten().unwrap_or(offset);
            let next_offset = next.map(|(_, o, _, _)| o);
            let end = end.or(next_offset).unwrap_or(record.size * 8);

            if let Some((Some(prev_field), _, Some(prev_end), _)) = prev {
                assert!(
                    record.is_union() || prev_end <= offset,
                    "Unexpected offset+size for field {:?} in record {}",
                    prev_field,
                    record.cc_name
                );
            }

            (field, prev_end, offset, end, desc)
        })
        .enumerate()
        .map(|(field_index, (field, prev_end, offset, end, desc))| {
            // `is_opaque_blob` and bitfield representations are always
            // unaligned, even though the actual C++ field might be aligned.
            // To put the current field at the right offset, we might need to
            // insert some extra padding.
            //
            // No padding should be needed if the type of the current field is
            // known (i.e. if the current field is correctly aligned based on
            // its original type).
            //
            // We also don't need padding if we're in a union.
            let padding_size_in_bits = if record.is_union()
                || (field.is_some() && get_field_rs_type_for_layout(field.unwrap()).is_ok())
            {
                0
            } else {
                let padding_start = (prev_end + 7) / 8 * 8; // round up to byte boundary
                offset - padding_start
            };

            let padding = if padding_size_in_bits == 0 {
                quote! {}
            } else {
                let padding_name = make_rs_ident(&format!("__padding{}", field_index));
                let padding_type = bit_padding(padding_size_in_bits);
                quote! { #padding_name: #padding_type, }
            };

            // Bitfields get represented by private padding to ensure overall
            // struct layout is compatible.
            if field.is_none() {
                let name = make_rs_ident(&format!("__bitfields{}", field_index));
                let bitfield_padding = bit_padding(end - offset);
                return Ok(quote! {
                    __NEWLINE__ #(  __COMMENT__ #desc )*
                    #padding #name: #bitfield_padding
                });
            }
            let field = field.unwrap();

            let ident = make_rs_field_ident(field, field_index);
            let doc_comment = match field.type_.as_ref() {
                Ok(_) => generate_doc_comment(&field.doc_comment),
                Err(msg) => {
                    let supplemental_text =
                        format!("Reason for representing this field as a blob of bytes:\n{}", msg);
                    let new_text = match field.doc_comment.as_ref() {
                        None => supplemental_text,
                        Some(old_text) => format!("{}\n\n{}", old_text, supplemental_text),
                    };
                    generate_doc_comment(&Some(new_text))
                }
            };
            let access = if field.access == AccessSpecifier::Public
                && get_field_rs_type_for_layout(field).is_ok()
            {
                quote! { pub }
            } else {
                quote! { pub(crate) }
            };

            let field_type = match get_field_rs_type_for_layout(field) {
                Err(_) => bit_padding(end - field.offset),
                Ok(rs_type) => {
                    let type_kind = db.rs_type_kind(rs_type.clone()).with_context(|| {
                        format!(
                            "Failed to format type for field {:?} on record {:?}",
                            field, record
                        )
                    })?;
                    let mut formatted = quote! {#type_kind};
                    if should_implement_drop(record) || record.is_union() {
                        if needs_manually_drop(db, rs_type.clone())? {
                            // TODO(b/212690698): Avoid (somewhat unergonomic) ManuallyDrop
                            // if we can ask Rust to preserve field destruction order if the
                            // destructor is the SpecialMemberFunc::NontrivialMembers
                            // case.
                            formatted = quote! { ::std::mem::ManuallyDrop<#formatted> }
                        } else {
                            field_copy_trait_assertions.push(quote! {
                                const _: () = {
                                    static_assertions::assert_impl_all!(#formatted: Copy);
                                };
                            });
                        }
                    };
                    formatted
                }
            };

            Ok(quote! { #padding #doc_comment #access #ident: #field_type })
        })
        .collect::<Result<Vec<_>>>()?;

    let size = Literal::usize_unsuffixed(record.size);
    let alignment = Literal::usize_unsuffixed(record.alignment);
    let field_offset_assertions = if record.is_union() {
        // TODO(https://github.com/Gilnaa/memoffset/issues/66): generate assertions for unions once
        // offsetof supports them.
        vec![]
    } else {
        fields_with_bounds
            .enumerate()
            .map(|(field_index, (field, _, _, _))| {
                if let Some(field) = field {
                    let field_ident = make_rs_field_ident(field, field_index);

                    // The assertion below reinforces that the division by 8 on the next line is
                    // justified (because the bitfields have been coallesced / filtered out
                    // earlier).
                    assert_eq!(field.offset % 8, 0);
                    let expected_offset = Literal::usize_unsuffixed(field.offset / 8);

                    let actual_offset_expr = quote! {
                        memoffset_unstable_const::offset_of!(#qualified_ident, #field_ident)
                    };
                    quote! {
                        const _: () = assert!(#actual_offset_expr == #expected_offset);
                    }
                } else {
                    quote! {}
                }
            })
            .collect_vec()
    };
    // TODO(b/212696226): Generate `assert_impl_all!` or `assert_not_impl_any!`
    // assertions about the `Copy` trait - this trait should be implemented
    // iff `should_implement_drop(record)` is false.
    let mut record_features = BTreeSet::new();
    let mut assertion_features = BTreeSet::new();

    // TODO(mboehme): For the time being, we're using unstable features to
    // be able to use offset_of!() in static assertions. This is fine for a
    // prototype, but longer-term we want to either get those features
    // stabilized or find an alternative. For more details, see
    // b/200120034#comment15
    assertion_features.insert(make_rs_ident("const_ptr_offset_from"));

    let derives = generate_derives(record);
    let derives = if derives.is_empty() {
        quote! {}
    } else {
        quote! {#[derive( #(#derives),* )]}
    };
    let record_kind = if record.is_union() {
        quote! { union }
    } else {
        quote! { struct }
    };

    let recursively_pinned_attribute = if record.is_unpin() {
        quote! {}
    } else {
        // negative_impls are necessary for universal initialization due to Rust's
        // coherence rules: PhantomPinned isn't enough to prove to Rust that a
        // blanket impl that requires Unpin doesn't apply. See http://<internal link>=h.f6jp8ifzgt3n
        record_features.insert(make_rs_ident("negative_impls"));
        if should_implement_drop(record) {
            quote! {#[::ctor::recursively_pinned(PinnedDrop)]}
        } else {
            quote! {#[::ctor::recursively_pinned]}
        }
    };

    let mut repr_attributes = vec![quote! {C}];
    if record.override_alignment && record.alignment > 1 {
        let alignment = Literal::usize_unsuffixed(record.alignment);
        repr_attributes.push(quote! {align(#alignment)});
    }

    // Adjust the struct to also include base class subobjects, vtables, etc.
    let head_padding = if let Some(first_field) = record.fields.first() {
        first_field.offset / 8
    } else {
        record.size
    };
    // Prevent direct initialization for non-aggregate structs.
    //
    // Technically, any implicit-lifetime type is going to be fine to initialize
    // using direct initialization of the fields, even if it is not an aggregate,
    // because this is "just" setting memory to the appropriate values, and
    // implicit-lifetime types can automatically begin their lifetime without
    // running a constructor at all.
    //
    // However, not all types used in interop are implicit-lifetime. For example,
    // while any `Unpin` C++ value is, some `!Unpin` structs (e.g. `std::list`)
    // will not be. So for consistency, we apply the same rule for both
    // implicit-lifetime and non-implicit-lifetime types: the C++ rule, that the
    // type must be an *aggregate* type.
    //
    // TODO(b/232969667): Protect unions from direct initialization, too.
    let allow_direct_init = record.is_aggregate || record.is_union();
    let head_padding = if head_padding > 0 || !allow_direct_init {
        let n = proc_macro2::Literal::usize_unsuffixed(head_padding);
        quote! {
            __non_field_data: [::std::mem::MaybeUninit<u8>; #n],
        }
    } else {
        quote! {}
    };

    // TODO(b/227442773): After namespace support is added, use the fully-namespaced
    // name.
    let incomplete_symbol = &record.cc_name;
    let incomplete_definition = quote! {
        forward_declare::unsafe_define!(forward_declare::symbol!(#incomplete_symbol), #qualified_ident);
    };

    let no_unique_address_accessors = cc_struct_no_unique_address_impl(db, record)?;
    let mut record_generated_items = record
        .child_item_ids
        .iter()
        .map(|id| {
            let item = ir
                .find_decl(*id)
                .with_context(|| format!("Failed to look up `record.child_item_ids` for {:?}", record))?;
            generate_item(db, item)
        })
        .collect::<Result<Vec<_>>>()?;

    record_generated_items.push(cc_struct_upcast_impl(record, &ir)?);

    let mut items = vec![];
    let mut thunks_from_record_items = vec![];
    let mut thunk_impls_from_record_items = vec![];
    let mut assertions_from_record_items = vec![];

    for generated in record_generated_items {
        items.push(generated.item);
        if !generated.thunks.is_empty() {
            thunks_from_record_items.push(generated.thunks);
        }
        if !generated.assertions.is_empty() {
            assertions_from_record_items.push(generated.assertions);
        }
        if !generated.thunk_impls.is_empty() {
            thunk_impls_from_record_items.push(generated.thunk_impls);
        }
        record_features.extend(generated.features.clone());
    }

    let record_tokens = quote! {
        #doc_comment
        #derives
        #recursively_pinned_attribute
        #[repr(#( #repr_attributes ),*)]
        pub #record_kind #ident {
            #head_padding
            #( #field_definitions, )*
        }

        #incomplete_definition

        #no_unique_address_accessors

        __NEWLINE__ __NEWLINE__
        #( #items __NEWLINE__ __NEWLINE__)*
    };

    let record_trait_assertions = {
        let record_type_name = RsTypeKind::new_record(record.clone(), &ir)?.to_token_stream();
        let mut assertions: Vec<TokenStream> = vec![];
        let mut add_assertion = |assert_impl_macro: TokenStream, trait_name: TokenStream| {
            assertions.push(quote! {
                const _: () = { static_assertions::#assert_impl_macro (#record_type_name: #trait_name); };
            });
        };
        if should_derive_clone(record) {
            add_assertion(quote! { assert_impl_all! }, quote! { Clone });
        } else {
            // Can't `assert_not_impl_any!` here, because `Clone` may be
            // implemented rather than derived.
        }
        let mut add_conditional_assertion = |should_impl_trait: bool, trait_name: TokenStream| {
            let assert_impl_macro = if should_impl_trait {
                quote! { assert_impl_all! }
            } else {
                quote! { assert_not_impl_any! }
            };
            add_assertion(assert_impl_macro, trait_name);
        };
        add_conditional_assertion(should_derive_copy(record), quote! { Copy });
        add_conditional_assertion(should_implement_drop(record), quote! { Drop });
        assertions
    };
    let assertion_tokens = quote! {
        const _: () = assert!(::std::mem::size_of::<#qualified_ident>() == #size);
        const _: () = assert!(::std::mem::align_of::<#qualified_ident>() == #alignment);
        #( #record_trait_assertions )*
        #( #field_offset_assertions )*
        #( #field_copy_trait_assertions )*
        #( #assertions_from_record_items )*
    };

    let thunk_tokens = quote! {
        #( #thunks_from_record_items )*
    };

    Ok(GeneratedItem {
        item: record_tokens,
        features: record_features.union(&assertion_features).cloned().collect(),
        assertions: assertion_tokens,
        thunks: thunk_tokens,
        thunk_impls: quote! {#(#thunk_impls_from_record_items __NEWLINE__ __NEWLINE__)*},
        has_record: true,
    })
}

fn check_by_value(record: &Record) -> Result<()> {
    if record.destructor == SpecialMemberFunc::Unavailable {
        bail!(
            "Can't directly construct values of type `{}` as it has a non-public or deleted destructor",
            record.cc_name
        )
    }
    if record.is_abstract {
        bail!("Can't directly construct values of type `{}`: it is abstract", record.cc_name);
    }
    Ok(())
}

fn should_derive_clone(record: &Record) -> bool {
    if record.is_union() {
        // `union`s (unlike `struct`s) should only derive `Clone` if they are `Copy`.
        should_derive_copy(record)
    } else {
        record.is_unpin()
            && record.copy_constructor == SpecialMemberFunc::Trivial
            && check_by_value(record).is_ok()
    }
}

fn should_derive_copy(record: &Record) -> bool {
    // TODO(b/202258760): Make `Copy` inclusion configurable.
    record.is_unpin()
        && record.copy_constructor == SpecialMemberFunc::Trivial
        && record.destructor == ir::SpecialMemberFunc::Trivial
        && check_by_value(record).is_ok()
}

fn generate_derives(record: &Record) -> Vec<Ident> {
    let mut derives = vec![];
    if should_derive_clone(record) {
        derives.push(make_rs_ident("Clone"));
    }
    if should_derive_copy(record) {
        derives.push(make_rs_ident("Copy"));
    }
    derives
}

fn generate_enum(db: &Database, enum_: &Enum) -> Result<TokenStream> {
    let name = make_rs_ident(&enum_.identifier.identifier);
    let underlying_type = db.rs_type_kind(enum_.underlying_type.rs_type.clone())?;
    let enumerator_names =
        enum_.enumerators.iter().map(|enumerator| make_rs_ident(&enumerator.identifier.identifier));
    let enumerator_values = enum_.enumerators.iter().map(|enumerator| enumerator.value);
    Ok(quote! {
        #[repr(transparent)]
        #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
        pub struct #name(#underlying_type);
        impl #name {
            #(pub const #enumerator_names: #name = #name(#enumerator_values);)*
        }
        impl From<#underlying_type> for #name {
            fn from(value: #underlying_type) -> #name {
                #name(value)
            }
        }
        impl From<#name> for #underlying_type {
            fn from(value: #name) -> #underlying_type {
                value.0
            }
        }
    })
}

fn generate_type_alias(db: &Database, type_alias: &TypeAlias) -> Result<TokenStream> {
    let ident = make_rs_ident(&type_alias.identifier.identifier);
    let doc_comment = generate_doc_comment(&type_alias.doc_comment);
    let underlying_type = db
        .rs_type_kind(type_alias.underlying_type.rs_type.clone())
        .with_context(|| format!("Failed to format underlying type for {:?}", type_alias))?;
    Ok(quote! {
        #doc_comment
        pub type #ident = #underlying_type;
    })
}

/// Generates Rust source code for a given `UnsupportedItem`.
fn generate_unsupported(item: &UnsupportedItem) -> Result<TokenStream> {
    let location = if item.source_loc.filename.is_empty() {
        "<unknown location>".to_string()
    } else {
        // TODO(forster): The "google3" prefix should probably come from a command line
        // argument.
        // TODO(forster): Consider linking to the symbol instead of to the line number
        // to avoid wrong links while generated files have not caught up.
        format!("google3/{};l={}", &item.source_loc.filename, &item.source_loc.line)
    };
    let message = format!(
        "{}\nError while generating bindings for item '{}':\n{}",
        &location, &item.name, &item.message
    );
    Ok(quote! { __COMMENT__ #message })
}

/// Generates Rust source code for a given `Comment`.
fn generate_comment(comment: &Comment) -> Result<TokenStream> {
    let text = &comment.text;
    Ok(quote! { __COMMENT__ #text })
}

fn generate_namespace(db: &Database, namespace: &Namespace) -> Result<GeneratedItem> {
    let ir = db.ir();
    let mut items = vec![];
    let mut thunks = vec![];
    let mut thunk_impls = vec![];
    let mut assertions = vec![];
    let mut has_record = false;
    let mut features = BTreeSet::new();

    for item_id in namespace.child_item_ids.iter() {
        let item = ir
            .find_decl(*item_id)
            .with_context(|| format!("Failed to look up namespace.child_item_ids for {:?}", namespace))?;
        let generated = generate_item(db, item)?;
        items.push(generated.item);
        if !generated.thunks.is_empty() {
            thunks.push(generated.thunks);
        }
        if !generated.thunk_impls.is_empty() {
            thunk_impls.push(generated.thunk_impls);
        }
        if !generated.assertions.is_empty() {
            assertions.push(generated.assertions);
        }
        features.extend(generated.features);
        has_record = has_record || generated.has_record;
    }

    let reopened_namespace_idx = ir.get_reopened_namespace_idx(namespace.id)?;
    let should_skip_index =
        ir.is_last_reopened_namespace(namespace.id, namespace.canonical_namespace_id)?;

    let name = if should_skip_index {
        make_rs_ident(&namespace.name.identifier)
    } else {
        make_rs_ident(&format!("{}_{}", &namespace.name.identifier, reopened_namespace_idx))
    };

    let use_stmt_for_previous_namespace = if reopened_namespace_idx == 0 {
        quote! {}
    } else {
        let previous_namespace_ident = make_rs_ident(&format!(
            "{}_{}",
            &namespace.name.identifier,
            reopened_namespace_idx - 1
        ));
        quote! { pub use super::#previous_namespace_ident::*; __NEWLINE__ __NEWLINE__ }
    };

    let namespace_tokens = quote! {
        pub mod #name {
            #use_stmt_for_previous_namespace

            #( #items __NEWLINE__ __NEWLINE__ )*
        }
    };

    Ok(GeneratedItem {
        item: namespace_tokens,
        features: features,
        has_record: has_record,
        thunks: quote! { #( #thunks )* },
        thunk_impls: quote! { #( #thunk_impls )* },
        assertions: quote! { #( #assertions )* },
        ..Default::default()
    })
}

#[derive(Clone, Debug, Default)]
struct GeneratedItem {
    item: TokenStream,
    thunks: TokenStream,
    // C++ source code for helper functions.
    thunk_impls: TokenStream,
    assertions: TokenStream,
    features: BTreeSet<Ident>,
    has_record: bool,
}

fn generate_item(db: &Database, item: &Item) -> Result<GeneratedItem> {
    let ir = db.ir();
    let overloaded_funcs = db.overloaded_funcs();
    let generated_item = match item {
        Item::Func(func) => match db.generate_func(func.clone()) {
            Err(e) => GeneratedItem {
                item: generate_unsupported(&make_unsupported_fn(func, &ir, format!("{e}"))?)?,
                ..Default::default()
            },
            Ok(None) => GeneratedItem::default(),
            Ok(Some(f)) => {
                let (api_func, thunk, function_id) = &*f;
                if overloaded_funcs.contains(function_id) {
                    GeneratedItem {
                        item: generate_unsupported(&make_unsupported_fn(
                            func,
                            &ir,
                            "Cannot generate bindings for overloaded function",
                        )?)?,
                        ..Default::default()
                    }
                } else {
                    // TODO(b/236687702): Use Rc for these, or else split this into a non-query
                    // and only use the query for Function IDs.
                    GeneratedItem {
                        item: api_func.tokens.clone(),
                        thunks: thunk.tokens.clone(),
                        features: api_func.features.union(&thunk.features).cloned().collect(),
                        ..Default::default()
                    }
                }
            }
        },
        Item::IncompleteRecord(incomplete_record) => {
            if !ir.is_current_target(&incomplete_record.owning_target)
                && !ir.is_stdlib_target(&incomplete_record.owning_target)
            {
                GeneratedItem::default()
            } else {
                GeneratedItem {
                    item: generate_incomplete_record(incomplete_record)?,
                    ..Default::default()
                }
            }
        }
        Item::Record(record) => {
            if !ir.is_current_target(&record.owning_target)
                && !ir.is_stdlib_target(&record.owning_target)
            {
                GeneratedItem::default()
            } else {
                generate_record(db, record)?
            }
        }
        Item::Enum(enum_) => {
            if !ir.is_current_target(&enum_.owning_target)
                && !ir.is_stdlib_target(&enum_.owning_target)
            {
                GeneratedItem::default()
            } else {
                GeneratedItem { item: generate_enum(db, enum_)?, ..Default::default() }
            }
        }
        Item::TypeAlias(type_alias) => {
            if !ir.is_current_target(&type_alias.owning_target)
                && !ir.is_stdlib_target(&type_alias.owning_target)
            {
                GeneratedItem::default()
            } else {
                GeneratedItem { item: generate_type_alias(db, type_alias)?, ..Default::default() }
            }
        }
        Item::UnsupportedItem(unsupported) => {
            GeneratedItem { item: generate_unsupported(unsupported)?, ..Default::default() }
        }
        Item::Comment(comment) => {
            GeneratedItem { item: generate_comment(comment)?, ..Default::default() }
        }
        Item::Namespace(namespace) => generate_namespace(db, namespace)?,
    };

    Ok(generated_item)
}

/// Identifies all functions having overloads that we can't import (yet).
///
/// TODO(b/213280424): Implement support for overloaded functions.
fn overloaded_funcs(db: &dyn BindingsGenerator) -> Rc<HashSet<Rc<FunctionId>>> {
    let mut seen_funcs = HashSet::new();
    let mut overloaded_funcs = HashSet::new();
    for func in db.ir().functions() {
        if let Ok(Some(f)) = db.generate_func(func.clone()) {
            let (.., function_id) = &*f;
            if !seen_funcs.insert(function_id.clone()) {
                overloaded_funcs.insert(function_id.clone());
            }
        }
    }
    Rc::new(overloaded_funcs)
}

// Returns the Rust code implementing bindings, plus any auxiliary C++ code
// needed to support it.
fn generate_bindings_tokens(ir: Rc<IR>, crubit_support_path: &str) -> Result<BindingsTokens> {
    let mut db = Database::default();
    db.set_ir(ir.clone());

    let mut items = vec![];
    let mut thunks = vec![];
    let mut thunk_impls = vec![generate_rs_api_impl(&mut db, crubit_support_path)?];
    let mut assertions = vec![];

    // We import nullable pointers as an Option<&T> and assume that at the ABI
    // level, None is represented as a zero pointer value whereas Some is
    // represented as as non-zero pointer value. This seems like a pretty safe
    // assumption to make, but to provide some safeguard, assert that
    // `Option<&i32>` and `&i32` have the same size.
    assertions.push(quote! {
        const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());
    });

    // TODO(jeanpierreda): Delete has_record, either in favor of using RsSnippet, or not
    // having uses. See https://chat.google.com/room/AAAAnQmj8Qs/6QbkSvWcfhA
    let mut has_record = false;
    let mut features = BTreeSet::new();

    // For #![rustfmt::skip].
    features.insert(make_rs_ident("custom_inner_attributes"));

    for top_level_item_id in ir.top_level_item_ids() {
        let item = ir
            .find_decl(*top_level_item_id)
            .context("Failed to look up ir.top_level_item_ids")?;
        let generated = generate_item(&db, item)?;
        items.push(generated.item);
        if !generated.thunks.is_empty() {
            thunks.push(generated.thunks);
        }
        if !generated.assertions.is_empty() {
            assertions.push(generated.assertions);
        }
        if !generated.thunk_impls.is_empty() {
            thunk_impls.push(generated.thunk_impls);
        }
        features.extend(generated.features);
        has_record = has_record || generated.has_record;
    }

    let mod_detail = if thunks.is_empty() {
        quote! {}
    } else {
        quote! {
            mod detail {
                #[allow(unused_imports)]
                use super::*;
                extern "C" {
                    #( #thunks )*
                }
            }
        }
    };

    let features = if features.is_empty() {
        quote! {}
    } else {
        quote! {
            #![feature( #(#features),* )]
        }
    };

    Ok(BindingsTokens {
        rs_api: quote! {
            #features __NEWLINE__
            #![allow(non_camel_case_types)] __NEWLINE__
            #![allow(non_snake_case)] __NEWLINE__
            #![allow(non_upper_case_globals)] __NEWLINE__
            #![deny(warnings)] __NEWLINE__ __NEWLINE__

            #( #items __NEWLINE__ __NEWLINE__ )*

            #mod_detail __NEWLINE__ __NEWLINE__

            #( #assertions __NEWLINE__ __NEWLINE__ )*
        },
        rs_api_impl: quote! {#(#thunk_impls  __NEWLINE__ __NEWLINE__ )*},
    })
}

/// Makes an 'Ident' to be used in the Rust source code. Escapes Rust keywords.
fn make_rs_ident(ident: &str) -> Ident {
    // TODO(https://github.com/dtolnay/syn/pull/1098): Remove the hardcoded list once syn recognizes
    // 2018 and 2021 keywords.
    if ["async", "await", "try", "dyn"].contains(&ident) {
        return format_ident!("r#{}", ident);
    }
    match syn::parse_str::<syn::Ident>(ident) {
        Ok(_) => format_ident!("{}", ident),
        Err(_) => format_ident!("r#{}", ident),
    }
}

/// Formats a C++ identifier. Does not escape C++ keywords.
fn format_cc_ident(ident: &str) -> TokenStream {
    ident.parse().unwrap()
}

/// Returns Some(crate_ident) if this is an imported crate.
fn rs_imported_crate_name(owning_target: &BazelLabel, ir: &IR) -> Option<Ident> {
    if ir.is_current_target(owning_target) || ir.is_stdlib_target(owning_target) {
        None
    } else {
        let owning_crate_name = owning_target.target_name();
        // TODO(b/216587072): Remove this hacky escaping and use the import! macro once
        // available
        let escaped_owning_crate_name = owning_crate_name.replace('-', "_");
        let owning_crate = make_rs_ident(&escaped_owning_crate_name);
        Some(owning_crate)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Mutability {
    Const,
    Mut,
}

impl Mutability {
    fn format_for_pointer(&self) -> TokenStream {
        match self {
            Mutability::Mut => quote! {mut},
            Mutability::Const => quote! {const},
        }
    }

    fn format_for_reference(&self) -> TokenStream {
        match self {
            Mutability::Mut => quote! {mut},
            Mutability::Const => quote! {},
        }
    }
}

/// Either a named lifetime, or the magic `'_` elided lifetime.
///
/// Warning: elided lifetimes are not always valid, and sometimes named
/// lifetimes are required. In particular, this should never be used for
/// output lifetimes.
///
/// However, because output lifetimes are never elided, a lifetime that only
/// occurs in a single input position can always be elided.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Lifetime(pub Rc<str>);

impl From<&ir::LifetimeName> for Lifetime {
    fn from(lifetime_name: &ir::LifetimeName) -> Self {
        Lifetime(lifetime_name.name.clone())
    }
}

impl Lifetime {
    pub fn new(name: &str) -> Self {
        Lifetime(Rc::from(name))
    }
    /// Formats a lifetime for use as a reference lifetime parameter.
    ///
    /// In this case, elided lifetimes are empty.
    pub fn format_for_reference(&self) -> TokenStream {
        match &*self.0 {
            "_" => quote! {},
            _ => quote! {#self},
        }
    }
}

/// Formats a lifetime for use anywhere.
///
/// For the specific context of references, prefer `format_for_reference`, as it
/// gives a more idiomatic formatting for elided lifetimes.
impl ToTokens for Lifetime {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(name) = self;
        let lifetime = syn::Lifetime::new(&format!("'{name}"), proc_macro2::Span::call_site());
        lifetime.to_tokens(tokens);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RsTypeKind {
    Pointer {
        pointee: Rc<RsTypeKind>,
        mutability: Mutability,
    },
    Reference {
        referent: Rc<RsTypeKind>,
        mutability: Mutability,
        lifetime: Lifetime,
    },
    RvalueReference {
        referent: Rc<RsTypeKind>,
        mutability: Mutability,
        lifetime: Lifetime,
    },
    FuncPtr {
        abi: Rc<str>,
        return_type: Rc<RsTypeKind>,
        param_types: Rc<[RsTypeKind]>,
    },
    /// An incomplete record type.
    IncompleteRecord {
        incomplete_record: Rc<IncompleteRecord>,

        /// The imported crate this comes from, or None if the current crate.
        crate_ident: Option<Ident>,
        /// The namespace qualifier for this record.
        namespace_qualifier: RcEq<TokenStream>,
    },
    /// A complete record type.
    Record {
        record: Rc<Record>,
        /// The namespace qualifier for this record.
        namespace_qualifier: RcEq<TokenStream>,
        /// The imported crate this comes from, or None if the current crate.
        crate_ident: Option<Ident>,
    },
    TypeAlias {
        type_alias: Rc<TypeAlias>,
        underlying_type: Rc<RsTypeKind>,
        /// The namespace qualifier for this alias.
        namespace_qualifier: RcEq<TokenStream>,
        /// The imported crate this comes from, or None if the current crate.
        crate_ident: Option<Ident>,
    },
    Unit,
    Other {
        name: Rc<str>,
        type_args: Rc<[RsTypeKind]>,
    },
}

impl RsTypeKind {
    pub fn new_record(record: Rc<Record>, ir: &IR) -> Result<Self> {
        let namespace_qualifier = RcEq::new(format_rs_namespace_qualifier(record.id, ir)?);
        let crate_ident = rs_imported_crate_name(&record.owning_target, ir);
        Ok(RsTypeKind::Record { record, namespace_qualifier, crate_ident })
    }

    /// Returns true if the type is known to be `Unpin`, false otherwise.
    pub fn is_unpin(&self) -> bool {
        match self {
            RsTypeKind::IncompleteRecord { .. } => false,
            RsTypeKind::Record { record, .. } => record.is_unpin(),
            RsTypeKind::TypeAlias { underlying_type, .. } => underlying_type.is_unpin(),
            _ => true,
        }
    }

    /// Returns true if the type is known to be move-constructible, false
    /// otherwise.
    ///
    /// For the purposes of this method, references are considered
    /// move-constructible (as if they were pointers).
    pub fn is_move_constructible(&self) -> bool {
        match self {
            RsTypeKind::IncompleteRecord { .. } => false,
            RsTypeKind::Record { record, .. } => {
                record.move_constructor != ir::SpecialMemberFunc::Unavailable
            }
            RsTypeKind::TypeAlias { underlying_type, .. } => {
                underlying_type.is_move_constructible()
            }
            _ => true,
        }
    }

    /// Returns Ok if the type can be used by value, or an error describing why
    /// it can't.
    pub fn check_by_value(&self) -> Result<()> {
        match self {
            RsTypeKind::Record { record, .. } => check_by_value(record),
            RsTypeKind::TypeAlias { underlying_type, .. } => underlying_type.check_by_value(),
            _ => Ok(()),
        }
    }

    pub fn format_as_return_type_fragment(&self) -> TokenStream {
        match self {
            RsTypeKind::Unit => quote! {},
            other_type => {
                quote! { -> #other_type }
            }
        }
    }

    /// Formats this RsTypeKind as `&'a mut MaybeUninit<SomeStruct>`. This is
    /// used to format `__this` parameter in a constructor thunk.
    pub fn format_mut_ref_as_uninitialized(&self) -> Result<TokenStream> {
        match self {
            RsTypeKind::Reference { referent, lifetime, mutability: Mutability::Mut } => {
                let lifetime = lifetime.format_for_reference();
                Ok(quote! { & #lifetime mut ::std::mem::MaybeUninit< #referent > })
            }
            _ => bail!("Expected reference to format as MaybeUninit, got: {:?}", self),
        }
    }

    /// Formats this RsTypeKind as the `self` parameter: usually, `&'a self` or
    /// `&'a mut self`.
    ///
    /// If this is !Unpin, however, it uses `self: Pin<&mut Self>` instead.
    pub fn format_as_self_param(&self) -> Result<TokenStream> {
        let referent;
        let mutability;
        let lifetime;
        match self {
            RsTypeKind::Pointer { .. } => {
                // TODO(jeanpierreda): provide end-user-facing docs, and insert a link to e.g.
                // something like <internal link>
                bail!(
                    "`self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function."
                )
            }
            RsTypeKind::Reference {
                referent: reference_pointee,
                lifetime: reference_lifetime,
                mutability: reference_mutability,
            } => {
                referent = reference_pointee;
                mutability = reference_mutability;
                lifetime = reference_lifetime;
            }
            RsTypeKind::Record { .. } => {
                // This case doesn't happen for methods, but is needed for free functions mapped
                // to a trait impl that take the first argument by value.
                return Ok(quote! { self });
            }
            _ => bail!("Unexpected type of `self` parameter: {:?}", self),
        }
        let mut_ = mutability.format_for_reference();
        let lifetime = lifetime.format_for_reference();
        if mutability == &Mutability::Mut && !referent.is_unpin() {
            // TODO(b/239661934): Add a `use ::std::pin::Pin` to the crate, and use
            // `Pin`.
            Ok(quote! {self: ::std::pin::Pin< & #lifetime #mut_ Self>})
        } else {
            Ok(quote! { & #lifetime #mut_ self })
        }
    }

    /// Returns whether the type represented by `self` implements the `Copy`
    /// trait.
    pub fn implements_copy(&self) -> bool {
        // TODO(b/212696226): Verify results of `implements_copy` via static
        // assertions in the generated Rust code (because incorrect results
        // can silently lead to unsafe behavior).
        match self {
            RsTypeKind::Unit => true,
            RsTypeKind::Pointer { .. } => true,
            RsTypeKind::FuncPtr { .. } => true,
            RsTypeKind::Reference { mutability: Mutability::Const, .. } => true,
            RsTypeKind::Reference { mutability: Mutability::Mut, .. } => false,
            RsTypeKind::RvalueReference { .. } => false,
            RsTypeKind::IncompleteRecord { .. } => false,
            RsTypeKind::Record { record, .. } => should_derive_copy(record),
            RsTypeKind::TypeAlias { underlying_type, .. } => underlying_type.implements_copy(),
            RsTypeKind::Other { type_args, .. } => {
                // All types that may appear here without `type_args` (e.g.
                // primitive types like `i32`) implement `Copy`. Generic types
                // that may be present here (e.g. Option<...>) are `Copy` if all
                // of their `type_args` are `Copy`.
                type_args.iter().all(|t| t.implements_copy())
            }
        }
    }

    pub fn is_ref_to(&self, expected_record: &Record) -> bool {
        match self {
            RsTypeKind::Reference { referent, .. } => referent.is_record(expected_record),
            _ => false,
        }
    }

    pub fn is_shared_ref_to(&self, expected_record: &Record) -> bool {
        match self {
            RsTypeKind::Reference { referent, mutability: Mutability::Const, .. } => {
                referent.is_record(expected_record)
            }
            _ => false,
        }
    }

    pub fn is_record(&self, expected_record: &Record) -> bool {
        match self {
            RsTypeKind::Record { record: actual_record, .. } => {
                actual_record.id == expected_record.id
            }
            _ => false,
        }
    }

    /// Iterates over `self` and all the nested types (e.g. pointees, generic
    /// type args, etc.) in DFS order.
    pub fn dfs_iter<'ty>(&'ty self) -> impl Iterator<Item = &'ty RsTypeKind> + '_ {
        RsTypeKindIter::new(self)
    }

    /// Iterates over all `LifetimeId`s in `self` and in all the nested types.
    /// Note that the results might contain duplicate LifetimeId values (e.g.
    /// if the same LifetimeId is used in two `type_args`).
    pub fn lifetimes(&self) -> impl Iterator<Item = Lifetime> + '_ {
        self.dfs_iter().filter_map(Self::lifetime)
    }

    /// Returns the pointer or reference target.
    pub fn referent(&self) -> Option<&RsTypeKind> {
        match self {
            Self::Pointer { pointee: p, .. }
            | Self::Reference { referent: p, .. }
            | Self::RvalueReference { referent: p, .. } => Some(&**p),
            _ => None,
        }
    }

    /// Returns the reference lifetime, or None if this is not a reference.
    pub fn lifetime(&self) -> Option<Lifetime> {
        match self {
            Self::Reference { lifetime, .. } | Self::RvalueReference { lifetime, .. } => {
                Some(lifetime.clone())
            }
            _ => None,
        }
    }
}

impl ToTokens for RsTypeKind {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_token_stream().to_tokens(tokens)
    }

    fn to_token_stream(&self) -> TokenStream {
        match self {
            RsTypeKind::Pointer { pointee, mutability } => {
                let mutability = mutability.format_for_pointer();
                quote! {* #mutability #pointee}
            }
            RsTypeKind::Reference { referent, mutability, lifetime } => {
                let mut_ = mutability.format_for_reference();
                let lifetime = lifetime.format_for_reference();
                let reference = quote! {& #lifetime #mut_ #referent};
                if mutability == &Mutability::Mut && !referent.is_unpin() {
                    // TODO(b/239661934): Add a `use ::std::pin::Pin` to the crate, and use
                    // `Pin`. This either requires deciding how to qualify pin at
                    // RsTypeKind-creation time, or returning an RsSnippet from here (and not
                    // implementing ToTokens, but instead some other interface.)
                    quote! {::std::pin::Pin< #reference >}
                } else {
                    reference
                }
            }
            RsTypeKind::RvalueReference { referent, mutability, lifetime } => {
                // TODO(b/239661934): Add a `use ::ctor::RvalueReference` (etc.) to the crate.
                if mutability == &Mutability::Mut {
                    quote! {::ctor::RvalueReference<#lifetime, #referent>}
                } else {
                    quote! {::ctor::ConstRvalueReference<#lifetime, #referent>}
                }
            }
            RsTypeKind::FuncPtr { abi, return_type, param_types } => {
                let return_frag = return_type.format_as_return_type_fragment();
                quote! { extern #abi fn( #( #param_types ),* ) #return_frag }
            }
            RsTypeKind::IncompleteRecord {
                incomplete_record,
                namespace_qualifier,
                crate_ident,
            } => {
                let record_ident = make_rs_ident(&incomplete_record.rs_name);
                match crate_ident {
                    Some(ci) => {
                        quote! { #ci #namespace_qualifier #record_ident }
                    }
                    None => {
                        quote! { crate:: #namespace_qualifier #record_ident }
                    }
                }
            }
            RsTypeKind::Record { record, namespace_qualifier, crate_ident } => {
                let ident = make_rs_ident(&record.rs_name);
                match crate_ident {
                    Some(ci) => {
                        quote! { #ci:: #namespace_qualifier #ident }
                    }
                    None => {
                        quote! { crate:: #namespace_qualifier #ident }
                    }
                }
            }
            RsTypeKind::TypeAlias { type_alias, namespace_qualifier, crate_ident, .. } => {
                let ident = make_rs_ident(&type_alias.identifier.identifier);
                match crate_ident {
                    Some(ci) => {
                        quote! { #ci:: #namespace_qualifier #ident }
                    }
                    None => {
                        quote! { crate:: #namespace_qualifier #ident }
                    }
                }
            }
            RsTypeKind::Unit => quote! {()},
            RsTypeKind::Other { name, type_args } => {
                let ident = make_rs_ident(name);
                let generic_params = format_generic_params(type_args.iter());
                quote! {#ident #generic_params}
            }
        }
    }
}

struct RsTypeKindIter<'ty> {
    todo: Vec<&'ty RsTypeKind>,
}

impl<'ty> RsTypeKindIter<'ty> {
    pub fn new(ty: &'ty RsTypeKind) -> Self {
        Self { todo: vec![ty] }
    }
}

impl<'ty> Iterator for RsTypeKindIter<'ty> {
    type Item = &'ty RsTypeKind;

    fn next(&mut self) -> Option<Self::Item> {
        match self.todo.pop() {
            None => None,
            Some(curr) => {
                match curr {
                    RsTypeKind::Unit
                    | RsTypeKind::IncompleteRecord { .. }
                    | RsTypeKind::Record { .. } => {}
                    RsTypeKind::Pointer { pointee, .. } => self.todo.push(pointee),
                    RsTypeKind::Reference { referent, .. } => self.todo.push(referent),
                    RsTypeKind::RvalueReference { referent, .. } => self.todo.push(referent),
                    RsTypeKind::TypeAlias { underlying_type: t, .. } => self.todo.push(t),
                    RsTypeKind::FuncPtr { return_type, param_types, .. } => {
                        self.todo.push(return_type);
                        self.todo.extend(param_types.iter().rev());
                    }
                    RsTypeKind::Other { type_args, .. } => self.todo.extend(type_args.iter().rev()),
                };
                Some(curr)
            }
        }
    }
}

fn unique_lifetimes<'a>(
    types: impl IntoIterator<Item = &'a RsTypeKind> + 'a,
) -> impl Iterator<Item = Lifetime> + 'a {
    let mut unordered_lifetimes = HashSet::new();
    types
        .into_iter()
        .flat_map(|ty| ty.lifetimes())
        .filter(move |lifetime| unordered_lifetimes.insert(lifetime.clone()))
}

fn rs_type_kind(db: &dyn BindingsGenerator, ty: ir::RsType) -> Result<RsTypeKind> {
    let ir = db.ir();
    // The lambdas deduplicate code needed by multiple `match` branches.
    let get_type_args = || -> Result<Vec<RsTypeKind>> {
        ty.type_args.iter().map(|type_arg| db.rs_type_kind(type_arg.clone())).collect()
    };
    let get_pointee = || -> Result<Rc<RsTypeKind>> {
        if ty.type_args.len() != 1 {
            bail!("Missing pointee/referent type (need exactly 1 type argument): {:?}", ty);
        }
        Ok(Rc::new(get_type_args()?.remove(0)))
    };
    let get_lifetime = || -> Result<Lifetime> {
        if ty.lifetime_args.len() != 1 {
            bail!("Missing reference lifetime (need exactly 1 lifetime argument): {:?}", ty);
        }
        let lifetime_id = ty.lifetime_args[0];
        ir.get_lifetime(lifetime_id)
            .ok_or_else(|| anyhow!("no known lifetime with id {lifetime_id:?}"))
            .map(Lifetime::from)
    };

    let result = match ty.name.as_deref() {
        None => {
            ensure!(
                ty.type_args.is_empty(),
                "Type arguments on records nor type aliases are not yet supported: {:?}",
                ty
            );
            match ir.item_for_type(&ty)? {
                Item::IncompleteRecord(incomplete_record) => RsTypeKind::IncompleteRecord {
                    incomplete_record: incomplete_record.clone(),
                    namespace_qualifier: RcEq::new(format_rs_namespace_qualifier(incomplete_record.id, &ir)?),
                    crate_ident: rs_imported_crate_name(&incomplete_record.owning_target, &ir),
                },
                Item::Record(record) => RsTypeKind::new_record(record.clone(), &ir)?,
                Item::TypeAlias(type_alias) => RsTypeKind::TypeAlias {
                    type_alias: type_alias.clone(),
                    namespace_qualifier: RcEq::new(format_rs_namespace_qualifier(type_alias.id, &ir)?),
                    crate_ident: rs_imported_crate_name(&type_alias.owning_target, &ir),
                    underlying_type: Rc::new(
                        db.rs_type_kind(type_alias.underlying_type.rs_type.clone())?,
                    ),
                },
                other_item => bail!("Item does not define a type: {:?}", other_item),
            }
        }
        Some(name) => match name {
            "()" => {
                if !ty.type_args.is_empty() {
                    bail!("Unit type must not have type arguments: {:?}", ty);
                }
                RsTypeKind::Unit
            }
            "*mut" => RsTypeKind::Pointer { pointee: get_pointee()?, mutability: Mutability::Mut },
            "*const" => {
                RsTypeKind::Pointer { pointee: get_pointee()?, mutability: Mutability::Const }
            }
            "&mut" => RsTypeKind::Reference {
                referent: get_pointee()?,
                mutability: Mutability::Mut,
                lifetime: get_lifetime()?,
            },
            "&" => RsTypeKind::Reference {
                referent: get_pointee()?,
                mutability: Mutability::Const,
                lifetime: get_lifetime()?,
            },
            "#RvalueReference mut" => RsTypeKind::RvalueReference {
                referent: get_pointee()?,
                mutability: Mutability::Mut,
                lifetime: get_lifetime()?,
            },
            "#RvalueReference const" => RsTypeKind::RvalueReference {
                referent: get_pointee()?,
                mutability: Mutability::Const,
                lifetime: get_lifetime()?,
            },
            name => {
                let mut type_args = get_type_args()?;
                match name.strip_prefix("#funcPtr ") {
                    None => RsTypeKind::Other { name: name.into(), type_args: Rc::from(type_args) },
                    Some(abi) => {
                        // TODO(b/217419782): Consider enforcing `'static` lifetime.
                        ensure!(!type_args.is_empty(), "No return type in fn type: {:?}", ty);
                        RsTypeKind::FuncPtr {
                            abi: abi.into(),
                            return_type: Rc::new(type_args.remove(type_args.len() - 1)),
                            param_types: Rc::from(type_args),
                        }
                    }
                }
            }
        },
    };
    Ok(result)
}

fn cc_type_name_for_record(record: &Record, ir: &IR) -> Result<TokenStream> {
    let ident = format_cc_ident(&record.cc_name);
    let namespace_qualifier = format_cc_namespace_qualifier(record.id, ir)?;
    let tag_kind = cc_tag_kind(record);
    Ok(quote! { #tag_kind #namespace_qualifier #ident })
}

fn cc_type_name_for_item(item: &ir::Item, ir: &IR) -> Result<TokenStream> {
    match item {
        Item::IncompleteRecord(incomplete_record) => {
            let ident = format_cc_ident(&incomplete_record.cc_name);
            let namespace_qualifier = format_cc_namespace_qualifier(incomplete_record.id, ir)?;
            let tag_kind = incomplete_record.record_type;
            Ok(quote! { #tag_kind #namespace_qualifier #ident })
        }
        Item::Record(record) => cc_type_name_for_record(record, ir),
        Item::TypeAlias(type_alias) => {
            let ident = format_cc_ident(&type_alias.identifier.identifier);
            let namespace_qualifier = format_cc_namespace_qualifier(type_alias.id, ir)?;
            Ok(quote! { #namespace_qualifier #ident })
        }
        _ => bail!("Item does not define a type: {:?}", item),
    }
}

fn cc_tag_kind(record: &ir::Record) -> TokenStream {
    if record.is_anon_record_with_typedef {
        quote! {}
    } else {
        record.record_type.into_token_stream()
    }
}

// Maps a Rust ABI [1] into a Clang attribute. See also
// `ConvertCcCallConvIntoRsApi` in importer.cc.
// [1]
// https://doc.rust-lang.org/reference/items/functions.html#extern-function-qualifier
fn format_cc_call_conv_as_clang_attribute(rs_abi: &str) -> Result<TokenStream> {
    match rs_abi {
        "cdecl" => Ok(quote! {}),
        "fastcall" => Ok(quote! { __attribute__((fastcall)) }),
        "stdcall" => Ok(quote! { __attribute__((stdcall)) }),
        "thiscall" => Ok(quote! { __attribute__((thiscall)) }),
        "vectorcall" => Ok(quote! { __attribute__((vectorcall)) }),
        _ => bail!("Unsupported ABI: {}", rs_abi),
    }
}

fn format_cc_type(ty: &ir::CcType, ir: &IR) -> Result<TokenStream> {
    let const_fragment = if ty.is_const {
        quote! {const}
    } else {
        quote! {}
    };
    if let Some(ref name) = ty.name {
        match name.as_str() {
            // Formatting *both* pointers *and* references as pointers, because:
            // - Pointers and references have the same representation in the ABI.
            // - Clang's `-Wreturn-type-c-linkage` warns when using references in C++ function
            //   thunks declared as `extern "C"` (see b/238681766).
            "*" | "&" | "&&" => {
                if ty.type_args.len() != 1 {
                    bail!("Invalid pointer type (need exactly 1 type argument): {:?}", ty);
                }
                let nested_type = format_cc_type(&ty.type_args[0], ir)?;
                Ok(quote! {#nested_type * #const_fragment})
            }
            cc_type_name => match cc_type_name.strip_prefix("#funcValue ") {
                None => {
                    if !ty.type_args.is_empty() {
                        bail!("Type not yet supported: {:?}", ty);
                    }
                    let idents = cc_type_name.split_whitespace().map(format_cc_ident);
                    Ok(quote! {#( #idents )* #const_fragment})
                }
                Some(abi) => match ty.type_args.split_last() {
                    None => bail!("funcValue type without a return type: {:?}", ty),
                    Some((ret_type, param_types)) => {
                        let ret_type = format_cc_type(ret_type, ir)?;
                        let param_types = param_types
                            .iter()
                            .map(|t| format_cc_type(t, ir))
                            .collect::<Result<Vec<_>>>()?;
                        let attr = format_cc_call_conv_as_clang_attribute(abi)?;
                        // `type_identity_t` is used below to avoid having to
                        // emit spiral-like syntax where some syntax elements of
                        // an inner type (e.g. function type as below) can
                        // surround syntax elements of an outer type (e.g. a
                        // pointer type). Compare: `int (*foo)(int, int)` VS
                        // `type_identity_t<int(int, int)>* foo`.
                        Ok(quote! { crubit::type_identity_t<
                            #ret_type ( #( #param_types ),* ) #attr
                        >  })
                    }
                },
            },
        }
    } else {
        let item = ir.item_for_type(ty)?;
        let type_name = cc_type_name_for_item(item, ir)?;
        Ok(quote! {#const_fragment #type_name})
    }
}

fn cc_struct_layout_assertion(record: &Record, ir: &IR) -> Result<TokenStream> {
    if !ir.is_current_target(&record.owning_target) && !ir.is_stdlib_target(&record.owning_target) {
        return Ok(quote! {});
    }
    let record_ident = format_cc_ident(&record.cc_name);
    let namespace_qualifier = format_cc_namespace_qualifier(record.id, ir)?;
    let cc_size = Literal::usize_unsuffixed(record.original_cc_size);
    let alignment = Literal::usize_unsuffixed(record.alignment);
    let tag_kind = cc_tag_kind(record);
    let field_assertions = record
        .fields
        .iter()
        .filter(|f| f.access == AccessSpecifier::Public && f.identifier.is_some())
        // https://en.cppreference.com/w/cpp/types/offsetof points out that "if member is [...]
        // a bit-field [...] the behavior [of `offsetof` macro] is undefined.".  In such
        // scenario clang reports an error: cannot compute offset of bit-field 'field_name'.
        .filter(|f| !f.is_bitfield)
        .map(|field| {
            // The IR contains the offset in bits, while `CRUBIT_OFFSET_OF` returns the
            // offset in bytes, so we need to convert.  We can assert that
            // `field.offset` is always at field boundaries, because the
            // bitfields have been filtered out earlier.
            assert_eq!(field.offset % 8, 0);
            let expected_offset = Literal::usize_unsuffixed(field.offset / 8);

            let field_ident = format_cc_ident(&field.identifier.as_ref().unwrap().identifier);
            let actual_offset = quote! {
                CRUBIT_OFFSET_OF(#field_ident, #tag_kind #namespace_qualifier #record_ident)
            };

            quote! { static_assert( #actual_offset == #expected_offset); }
        });
    Ok(quote! {
        static_assert(sizeof(#tag_kind #namespace_qualifier #record_ident) == #cc_size);
        static_assert(alignof(#tag_kind #namespace_qualifier #record_ident) == #alignment);
        #( #field_assertions )*
    })
}

// Returns the accessor functions for no_unique_address member variables.
fn cc_struct_no_unique_address_impl(db: &Database, record: &Record) -> Result<TokenStream> {
    let mut fields = vec![];
    let mut types = vec![];
    for field in &record.fields {
        if field.access != AccessSpecifier::Public || !field.is_no_unique_address {
            continue;
        }
        // Can't use `get_field_rs_type_for_layout` here, because we want to dig into
        // no_unique_address fields, despite laying them out as opaque blobs of bytes.
        if let Ok(rs_type) = field.type_.as_ref().map(|t| t.rs_type.clone()) {
            fields.push(make_rs_ident(
                &field
                    .identifier
                    .as_ref()
                    .expect("Unnamed fields can't be annotated with [[no_unique_address]]")
                    .identifier,
            ));
            types.push(db.rs_type_kind(rs_type).with_context(|| {
                format!("Failed to format type for field {:?} on record {:?}", field, record)
            })?);
        }
    }

    if fields.is_empty() {
        return Ok(quote! {});
    }

    let ident = make_rs_ident(&record.rs_name);
    Ok(quote! {
        impl #ident {
            #(
                pub fn #fields(&self) -> &#types {
                    unsafe {&* (&self.#fields as *const _ as *const #types)}
                }
            )*
        }
    })
}

/// Returns the implementation of base class conversions, for converting a type
/// to its unambiguous public base classes.
fn cc_struct_upcast_impl(record: &Rc<Record>, ir: &IR) -> Result<GeneratedItem> {
    let mut impls = Vec::with_capacity(record.unambiguous_public_bases.len());
    let mut thunks = vec![];
    let mut cc_impls = vec![];
    for base in &record.unambiguous_public_bases {
        let base_record: &Rc<Record> = ir
            .find_decl(base.base_record_id)
            .with_context(|| format!("Can't find a base record of {:?}", record))?;
        let base_name = RsTypeKind::new_record(base_record.clone(), ir)?.into_token_stream();
        let derived_name = RsTypeKind::new_record(record.clone(), ir)?.into_token_stream();
        let body;
        if let Some(offset) = base.offset {
            let offset = Literal::i64_unsuffixed(offset);
            body = quote! {(derived as *const _ as *const u8).offset(#offset) as *const #base_name};
        } else {
            // TODO(b/216195042): use mangled names here, or otherwise guarantee
            // non-collision.
            let cast_fn_name = make_rs_ident(&format!(
                "__crubit_dynamic_upcast__{}__to__{}",
                record.rs_name, base_record.rs_name
            ));
            let base_cc_name = cc_type_name_for_record(base_record.as_ref(), ir)?;
            let derived_cc_name = cc_type_name_for_record(record.as_ref(), ir)?;
            cc_impls.push(quote! {
                extern "C" const #base_cc_name& #cast_fn_name(const #derived_cc_name& from) {
                    return from;
                }
            });
            thunks.push(quote! {
                pub fn #cast_fn_name (from: *const #derived_name) -> *const #base_name;
            });
            body = quote! {
                crate::detail::#cast_fn_name(derived)
            };
        }
        impls.push(quote! {
            unsafe impl oops::Inherits<#base_name> for #derived_name {
                unsafe fn upcast_ptr(derived: *const Self) -> *const #base_name {
                    #body
                }
            }
        });
    }

    Ok(GeneratedItem {
        item: quote! {#(#impls)*},
        thunks: quote! {#(#thunks)*},
        thunk_impls: quote! {#(#cc_impls)*},
        ..Default::default()
    })
}

fn thunk_ident(func: &Func) -> Ident {
    format_ident!("__rust_thunk__{}", func.mangled_name)
}

fn generate_rs_api_impl(db: &mut Database, crubit_support_path: &str) -> Result<TokenStream> {
    // This function uses quote! to generate C++ source code out of convenience.
    // This is a bold idea so we have to continously evaluate if it still makes
    // sense or the cost of working around differences in Rust and C++ tokens is
    // greather than the value added.
    //
    // See rs_bindings_from_cc/
    // token_stream_printer.rs for a list of supported placeholders.
    let mut thunks = vec![];
    let ir = db.ir();
    for func in ir.functions() {
        if can_skip_cc_thunk(db, func) {
            continue;
        }
        if db.generate_func(func.clone()).unwrap_or_default().is_none() {
            // No function was generated that will call this thunk.
            continue;
        }

        let thunk_ident = thunk_ident(func);
        let implementation_function = match &func.name {
            UnqualifiedIdentifier::Operator(op) => {
                let name = syn::parse_str::<TokenStream>(&op.name)?;
                quote! { operator #name }
            }
            UnqualifiedIdentifier::Identifier(id) => {
                let fn_ident = format_cc_ident(&id.identifier);
                match func.member_func_metadata.as_ref() {
                    Some(meta) => {
                        if let Some(_) = meta.instance_method_metadata {
                            quote! { #fn_ident }
                        } else {
                            let record: &Rc<Record> = ir.find_decl(meta.record_id)?;
                            let record_ident = format_cc_ident(&record.cc_name);
                            let namespace_qualifier = format_cc_namespace_qualifier(record.id, &ir)?;
                            quote! { #namespace_qualifier #record_ident :: #fn_ident }
                        }
                    }
                    None => {
                        let namespace_qualifier = format_cc_namespace_qualifier(func.id, &ir)?;
                        quote! { #namespace_qualifier #fn_ident }
                    }
                }
            }
            // Use `destroy_at` to avoid needing to spell out the class name. Destructor identiifers
            // use the name of the type itself, without namespace qualification, template
            // parameters, or aliases. We do not need to use that naming scheme anywhere else in
            // the bindings, and it can be difficult (impossible?) to spell in the general case. By
            // using destroy_at, we avoid needing to determine or remember what the correct spelling
            // is. Similar arguments apply to `construct_at`.
            UnqualifiedIdentifier::Constructor => {
                quote! { crubit::construct_at }
            }
            UnqualifiedIdentifier::Destructor => quote! {std::destroy_at},
        };

        let mut param_idents =
            func.params.iter().map(|p| format_cc_ident(&p.identifier.identifier)).collect_vec();

        let mut param_types = func
            .params
            .iter()
            .map(|p| {
                let formatted = format_cc_type(&p.type_.cc_type, &ir)?;
                if !db.rs_type_kind(p.type_.rs_type.clone())?.is_unpin() {
                    // non-Unpin types are wrapped by a pointer in the thunk.
                    Ok(quote! {#formatted *})
                } else {
                    Ok(formatted)
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let arg_expressions = func
            .params
            .iter()
            .map(|p| {
                let ident = format_cc_ident(&p.identifier.identifier);
                match p.type_.cc_type.name.as_deref() {
                    Some("&") => Ok(quote! { * #ident }),
                    Some("&&") => Ok(quote! { std::move(* #ident) }),
                    _ => {
                        // non-Unpin types are wrapped by a pointer in the thunk.
                        if !db.rs_type_kind(p.type_.rs_type.clone())?.is_unpin() {
                            Ok(quote! { std::move(* #ident) })
                        } else {
                            Ok(quote! { #ident })
                        }
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?;

        // Here, we add a __return parameter if the return type is not trivially
        // relocatable. (We do this after the arg_expressions computation, so
        // that it's only in the parameter list, not the argument list.)
        //
        // RsTypeKind is where, as much as anywhere, where the information about trivial
        // relocatability is stored.
        let is_trivial_return = db.rs_type_kind(func.return_type.rs_type.clone())?.is_unpin();
        let mut return_type_name = format_cc_type(&func.return_type.cc_type, &ir)?;
        if !is_trivial_return {
            param_idents.insert(0, format_cc_ident("__return"));
            param_types.insert(0, quote! {#return_type_name *});
            return_type_name = quote! {void};
        }

        let needs_this_deref = match &func.member_func_metadata {
            None => false,
            Some(meta) => match &func.name {
                UnqualifiedIdentifier::Constructor | UnqualifiedIdentifier::Destructor => false,
                UnqualifiedIdentifier::Identifier(_) | UnqualifiedIdentifier::Operator(_) => {
                    meta.instance_method_metadata.is_some()
                }
            },
        };
        let (implementation_function, arg_expressions) = if !needs_this_deref {
            (implementation_function, arg_expressions.clone())
        } else {
            let this_param = func
                .params
                .first()
                .ok_or_else(|| anyhow!("Instance methods must have `__this` param."))?;
            let this_arg = format_cc_ident(&this_param.identifier.identifier);
            (
                quote! { #this_arg -> #implementation_function},
                arg_expressions.iter().skip(1).cloned().collect_vec(),
            )
        };

        let return_expr = quote! {#implementation_function( #( #arg_expressions ),* )};
        let return_stmt = if !is_trivial_return {
            // Explicitly use placement new so that we get guaranteed copy elision in C++17.
            let out_param = &param_idents[0];
            quote! {new(#out_param) auto(#return_expr)}
        } else {
            match func.return_type.cc_type.name.as_deref() {
                Some("void") => return_expr,
                Some("&") => quote! { return & #return_expr },
                Some("&&") => {
                    // The code below replicates bits of `format_cc_type`, but formats an rvalue
                    // reference (which `format_cc_type` would format as a pointer).
                    // `const_fragment` from `format_cc_type` is ignored - it is not applicable for
                    // references.
                    let ty = &func.return_type.cc_type;
                    if ty.type_args.len() != 1 {
                        bail!("Invalid reference type (need exactly 1 type argument): {:?}", ty);
                    }
                    let nested_type = format_cc_type(&ty.type_args[0], &ir)?;
                    quote! {
                        #nested_type && lvalue = #return_expr;
                        return &lvalue
                    }
                }
                _ => quote! { return #return_expr },
            }
        };

        thunks.push(quote! {
            extern "C" #return_type_name #thunk_ident( #( #param_types #param_idents ),* ) {
                #return_stmt;
            }
        });
    }

    let layout_assertions = ir
        .records()
        .map(|record| cc_struct_layout_assertion(record, &ir))
        .collect::<Result<Vec<_>>>()?;

    let mut standard_headers = <BTreeSet<Ident>>::new();
    standard_headers.insert(format_ident!("memory")); // ubiquitous.
    if ir.records().next().is_some() {
        standard_headers.insert(format_ident!("cstddef"));
    };

    let mut includes = vec!["cxx20_backports.h", "offsetof.h"]
        .into_iter()
        .map(|hdr| format!("{}/{}", crubit_support_path, hdr))
        .collect_vec();

    // In order to generate C++ thunk in all the cases Clang needs to be able to
    // access declarations from public headers of the C++ library.
    includes.extend(ir.used_headers().map(|hdr| hdr.name.clone()));

    Ok(quote! {
        #( __HASH_TOKEN__ include <#standard_headers> __NEWLINE__)*
        __NEWLINE__
        #( __HASH_TOKEN__ include #includes __NEWLINE__)* __NEWLINE__
        __HASH_TOKEN__ pragma clang diagnostic push __NEWLINE__
        // Disable Clang thread-safety-analysis warnings that would otherwise
        // complain about thunks that call mutex locking functions in an unpaired way.
        __HASH_TOKEN__ pragma clang diagnostic ignored "-Wthread-safety-analysis" __NEWLINE__

        #( #thunks )* __NEWLINE__ __NEWLINE__

        #( #layout_assertions __NEWLINE__ __NEWLINE__ )*

        __NEWLINE__
        __HASH_TOKEN__ pragma clang diagnostic pop __NEWLINE__
        // To satisfy http://cs/symbol:devtools.metadata.Presubmit.CheckTerminatingNewline check.
        __NEWLINE__
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir_testing::{
        ir_from_cc, ir_from_cc_dependency, ir_record, make_ir_from_items, retrieve_func,
        with_lifetime_macros,
    };
    use static_assertions::{assert_impl_all, assert_not_impl_any};
    use token_stream_matchers::{
        assert_cc_matches, assert_cc_not_matches, assert_ir_matches, assert_rs_matches,
        assert_rs_not_matches,
    };
    use token_stream_printer::{rs_tokens_to_formatted_string_for_tests, tokens_to_string};

    fn generate_bindings_tokens(ir: Rc<IR>) -> Result<BindingsTokens> {
        super::generate_bindings_tokens(ir, "crubit/rs_bindings_support")
    }

    fn db_from_cc(cc_src: &str) -> Result<Database> {
        let mut db = Database::default();
        db.set_ir(ir_from_cc(cc_src)?);
        Ok(db)
    }

    #[test]
    fn test_disable_thread_safety_warnings() -> Result<()> {
        let ir = ir_from_cc("inline void foo() {}")?;
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                ...
                __HASH_TOKEN__ pragma clang diagnostic push
                __HASH_TOKEN__ pragma clang diagnostic ignored "-Wthread-safety-analysis"
                ...

                __HASH_TOKEN__ pragma clang diagnostic pop
                ...
            }
        );
        Ok(())
    }

    #[test]
    // TODO(hlopko): Move this test to a more principled place where it can access
    // `ir_testing`.
    fn test_duplicate_decl_ids_err() {
        let mut r1 = ir_record("R1");
        r1.id = ItemId::new_for_testing(42);
        let mut r2 = ir_record("R2");
        r2.id = ItemId::new_for_testing(42);
        let result = make_ir_from_items([r1.into(), r2.into()]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Duplicate decl_id found in"));
    }

    #[test]
    fn test_simple_function() -> Result<()> {
        let ir = ir_from_cc("int Add(int a, int b);")?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn Add(a: i32, b: i32) -> i32 {
                    unsafe { crate::detail::__rust_thunk___Z3Addii(a, b) }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    extern "C" {
                        #[link_name = "_Z3Addii"]
                        pub(crate) fn __rust_thunk___Z3Addii(a: i32, b: i32) -> i32;
                    }
                }
            }
        );

        assert_cc_not_matches!(rs_api_impl, quote! {__rust_thunk___Z3Addii});

        Ok(())
    }

    #[test]
    fn test_inline_function() -> Result<()> {
        let ir = ir_from_cc("inline int Add(int a, int b);")?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn Add(a: i32, b: i32) -> i32 {
                    unsafe { crate::detail::__rust_thunk___Z3Addii(a, b) }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    extern "C" {
                        pub(crate) fn __rust_thunk___Z3Addii(a: i32, b: i32) -> i32;
                    }
                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" int __rust_thunk___Z3Addii(int a, int b) {
                    return Add(a, b);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_simple_function_with_types_from_other_target() -> Result<()> {
        let ir = ir_from_cc_dependency(
            "inline ReturnStruct DoSomething(ParamStruct param);",
            "struct ReturnStruct final {}; struct ParamStruct final {};",
        )?;

        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn DoSomething(param: dependency::ParamStruct)
                    -> dependency::ReturnStruct {
                    unsafe { crate::detail::__rust_thunk___Z11DoSomething11ParamStruct(param) }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
            mod detail {
                #[allow(unused_imports)]
                use super::*;
                extern "C" {
                    pub(crate) fn __rust_thunk___Z11DoSomething11ParamStruct(
                        param: dependency::ParamStruct) -> dependency::ReturnStruct;
                }
            }}
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" struct ReturnStruct __rust_thunk___Z11DoSomething11ParamStruct(
                        struct ParamStruct param) {
                    return DoSomething(param);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_template_in_dependency_and_alias_in_current_target() -> Result<()> {
        // See also the test with the same name in `ir_from_cc_test.rs`.
        let ir = {
            let dependency_src = r#" #pragma clang lifetime_elision
                    template <typename T>
                    struct MyTemplate {
                        T GetValue() { return field; }
                        T field;
                    }; "#;
            let current_target_src = r#" #pragma clang lifetime_elision
                    using MyAliasOfTemplate = MyTemplate<int>; "#;
            ir_from_cc_dependency(current_target_src, dependency_src)?
        };

        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C)]
                pub struct __CcTemplateInst10MyTemplateIiE {
                    pub field: i32,
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                impl __CcTemplateInst10MyTemplateIiE {
                    #[inline(always)]
                    pub fn GetValue<'a>(self: ... Pin<&'a mut Self>) -> i32 { unsafe {
                        crate::detail::__rust_thunk___ZN10MyTemplateIiE8GetValueEv__2f_2ftest_3atesting_5ftarget(
                            self)
                    }}
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                pub type MyAliasOfTemplate = crate::__CcTemplateInst10MyTemplateIiE;
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail { ...  extern "C" {
                    ...
                    pub(crate) fn
                    __rust_thunk___ZN10MyTemplateIiE8GetValueEv__2f_2ftest_3atesting_5ftarget<'a>(
                        __this: ... Pin<&'a mut crate::__CcTemplateInst10MyTemplateIiE>
                    ) -> i32;
                    ...
                } }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C"
                int __rust_thunk___ZN10MyTemplateIiE8GetValueEv__2f_2ftest_3atesting_5ftarget(
                        struct MyTemplate<int>* __this) {
                    return __this->GetValue();
                }
            }
        );

        Ok(())
    }

    #[test]
    fn test_template_with_out_of_line_definition() -> Result<()> {
        // See also an end-to-end test in the `test/templates/out_of_line_definition`
        // directory.
        let ir = ir_from_cc(
            r#" #pragma clang lifetime_elision
                template <typename T>
                class MyTemplate final {
                 public:
                  static MyTemplate Create(T value);
                  const T& value() const;

                 private:
                  T value_;
                };

                using MyTypeAlias = MyTemplate<int>; "#,
        )?;

        let BindingsTokens { rs_api_impl, .. } = generate_bindings_tokens(ir)?;

        // Even though the member functions above are *not* defined inline (e.g.
        // IR::Func::is_inline is false), they still need to have thunks generated for
        // them (to force/guarantee that the class template and its members get
        // instantiated).  This is also covered in the following end-to-end
        // tests:
        // - test/templates/out_of_line_definition/ - without a thunk, the template
        //   won't be instantiated and Rust bindings won't be able to call the member
        //   function (there will be no instantiation of the member function in the C++
        //   object files)
        // - test/templates/definition_in_cc/ - the instantiation happens in the .cc
        //   file and therefore the thunk is not *required* (but it doesn't hurt to have
        //   the thunk)
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" class MyTemplate<int>
                __rust_thunk___ZN10MyTemplateIiE6CreateEi__2f_2ftest_3atesting_5ftarget(
                        int value) {
                    return MyTemplate<int>::Create(value);
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" int const*
                __rust_thunk___ZNK10MyTemplateIiE5valueEv__2f_2ftest_3atesting_5ftarget(
                        const class MyTemplate<int>*__this) {
                    return &__this->value();
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_simple_struct() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct SomeStruct final {
                ~SomeStruct() {}
                int public_int;
              protected:
                int protected_int;
              private:
               int private_int;
            };
        "#,
        )?;

        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[::ctor::recursively_pinned(PinnedDrop)]
                #[repr(C, align(4))]
                pub struct SomeStruct {
                    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
                    pub public_int: i32,
                    #[doc = " Reason for representing this field as a blob of bytes:\n Types of non-public C++ fields can be elided away"]
                    pub(crate) protected_int: [::std::mem::MaybeUninit<u8>; 4],
                    #[doc = " Reason for representing this field as a blob of bytes:\n Types of non-public C++ fields can be elided away"]
                    pub(crate) private_int: [::std::mem::MaybeUninit<u8>; 4],
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());
                const _: () = assert!(::std::mem::size_of::<crate::SomeStruct>() == 12);
                const _: () = assert!(::std::mem::align_of::<crate::SomeStruct>() == 4);
                const _: () = { static_assertions::assert_not_impl_any!(crate::SomeStruct: Copy); };
                const _: () = { static_assertions::assert_impl_all!(crate::SomeStruct: Drop); };
                const _: () = assert!(memoffset_unstable_const::offset_of!(crate::SomeStruct, public_int) == 0);
                const _: () = assert!(memoffset_unstable_const::offset_of!(crate::SomeStruct, protected_int) == 4);
                const _: () = assert!(memoffset_unstable_const::offset_of!(crate::SomeStruct, private_int) == 8);
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN10SomeStructD1Ev(struct SomeStruct * __this) {
                    std::destroy_at(__this);
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                static_assert(sizeof(struct SomeStruct) == 12);
                static_assert(alignof(struct SomeStruct) == 4);
                static_assert(CRUBIT_OFFSET_OF(public_int, struct SomeStruct) == 0);
            }
        );
        Ok(())
    }

    #[test]
    fn test_struct_vs_class() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct SomeStruct final {
                SomeStruct() {}
                int field;
            };
            class SomeClass final {
              public:
                SomeClass() {}
                int field;
            };
        "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;

        // A Rust `struct` is generated for both `SomeStruct` and `SomeClass`.
        assert_rs_matches!(rs_api, quote! { pub struct SomeStruct },);
        assert_rs_matches!(rs_api, quote! { pub struct SomeClass },);

        // But in C++ we still should refer to `struct SomeStruct` and `class
        // SomeClass`. See also b/238212337.
        assert_cc_matches!(rs_api_impl, quote! { struct SomeStruct * __this });
        assert_cc_matches!(rs_api_impl, quote! { class SomeClass * __this });
        assert_cc_matches!(rs_api_impl, quote! { static_assert(sizeof(struct SomeStruct) == 4); });
        assert_cc_matches!(rs_api_impl, quote! { static_assert(sizeof(class SomeClass) == 4); });
        Ok(())
    }

    #[test]
    fn test_struct_vs_typedefed_struct() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct SomeStruct final {
              int x;
            } __attribute__((aligned(16)));
            typedef struct {
              int x;
            } SomeAnonStruct __attribute__((aligned(16)));
        "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;

        // A `struct` is generated for both `SomeStruct` and `SomeAnonStruct`, both
        // in Rust and in C++.
        assert_rs_matches!(rs_api, quote! { pub struct SomeStruct },);
        assert_rs_matches!(rs_api, quote! { pub struct SomeAnonStruct },);
        assert_rs_matches!(rs_api_impl, quote! { struct SomeStruct * __this },);
        assert_rs_matches!(rs_api_impl, quote! { SomeAnonStruct * __this },);

        // In C++, both have align == 16, but size for `SomeAnonStruct` is not aligned.
        // `SomeAnonStruct` won't have `struct` in the assert.
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(alignof(struct SomeStruct) == 16); }
        );
        assert_cc_matches!(rs_api_impl, quote! { static_assert(alignof(SomeAnonStruct) == 16); });
        assert_cc_matches!(rs_api_impl, quote! { static_assert(sizeof(struct SomeStruct) == 16); });
        assert_cc_matches!(rs_api_impl, quote! { static_assert(sizeof(SomeAnonStruct) == 4); });

        // In Rust, both have align == 16 and size == 16.
        assert_rs_matches!(
            rs_api,
            quote! { assert!(::std::mem::size_of::<crate::SomeStruct>() == 16); }
        );
        assert_rs_matches!(
            rs_api,
            quote! { assert!(::std::mem::align_of::<crate::SomeStruct>() == 16); }
        );
        assert_rs_matches!(
            rs_api,
            quote! { assert!(::std::mem::size_of::<crate::SomeAnonStruct>() == 16); }
        );
        assert_rs_matches!(
            rs_api,
            quote! { assert!(::std::mem::align_of::<crate::SomeAnonStruct>() == 16); }
        );

        Ok(())
    }

    #[test]
    fn test_ref_to_struct_in_thunk_impls() -> Result<()> {
        let ir = ir_from_cc("struct S{}; inline void foo(S& s) {} ")?;
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z3fooR1S(struct S* s) {
                    foo(*s);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_const_ref_to_struct_in_thunk_impls() -> Result<()> {
        let ir = ir_from_cc("struct S{}; inline void foo(const S& s) {} ")?;
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z3fooRK1S(const struct S* s) {
                    foo(*s);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_unsigned_int_in_thunk_impls() -> Result<()> {
        let ir = ir_from_cc("inline void foo(unsigned int i) {} ")?;
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z3fooj(unsigned int i) {
                    foo(i);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_record_static_methods_qualify_call_in_thunk() -> Result<()> {
        let ir = ir_from_cc(&tokens_to_string(quote! {
            struct SomeStruct {
                static inline int some_func() { return 42; }
            };
        })?)?;

        assert_cc_matches!(
            generate_bindings_tokens(ir)?.rs_api_impl,
            quote! {
                extern "C" int __rust_thunk___ZN10SomeStruct9some_funcEv() {
                    return SomeStruct::some_func();
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_record_instance_methods_deref_this_in_thunk() -> Result<()> {
        let ir = ir_from_cc(&tokens_to_string(quote! {
            struct SomeStruct {
                inline int some_func(int arg) const { return 42 + arg; }
            };
        })?)?;

        assert_cc_matches!(
            generate_bindings_tokens(ir)?.rs_api_impl,
            quote! {
                extern "C" int __rust_thunk___ZNK10SomeStruct9some_funcEi(
                        const struct SomeStruct* __this, int arg) {
                    return __this->some_func(arg);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_record_with_unsupported_field_type() -> Result<()> {
        // Using a nested struct because it's currently not supported.
        // But... any other unsupported type would also work for this test.
        let ir = ir_from_cc(
            r#"
            struct StructWithUnsupportedField {
              struct NestedStruct {
                int nested_field;
              };

              // Doc comment for `my_field`.
              NestedStruct my_field;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(4))]
                pub struct StructWithUnsupportedField {
                    #[doc = " Doc comment for `my_field`.\n \n Reason for representing this field as a blob of bytes:\n Unsupported type 'struct StructWithUnsupportedField::NestedStruct': No generated bindings found for 'NestedStruct'"]
                    pub(crate) my_field: [::std::mem::MaybeUninit<u8>; 4],
                }
                ...
                const _: () = assert!(
                    memoffset_unstable_const::offset_of!(
                        crate::StructWithUnsupportedField, my_field) == 0);
            }
        );
        Ok(())
    }

    #[test]
    fn test_struct_with_unnamed_bitfield_member() -> Result<()> {
        // This test input causes `field_decl->getName()` to return an empty string.
        // This example is based on `struct timex` from
        // /usr/grte/v5/include/bits/timex.h
        let ir = ir_from_cc(
            r#"
            struct SomeStruct {
                int first_field;
                int :32;
                int last_field;
            }; "#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
               #[repr(C)]
               pub struct SomeStruct {
                   pub first_field: i32, ...
                   __bitfields1: [::std::mem::MaybeUninit<u8>; 4],
                   pub last_field: i32,
               }
               ...
               const _: () = assert!(memoffset_unstable_const::offset_of!(
                       crate::SomeStruct, first_field) == 0);
               const _: () = assert!(memoffset_unstable_const::offset_of!(
                       crate::SomeStruct, last_field) == 8);
            }
        );
        Ok(())
    }

    #[test]
    fn test_struct_with_inheritable_field() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            struct TrivialButInheritable {
              int x;
            };
            struct StructWithInheritable final {
              TrivialButInheritable t;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Copy ... )});
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Clone ... )});
        Ok(())
    }

    #[test]
    fn test_union_with_inheritable_field() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            struct TrivialButInheritable {
              int x;
            };
            union UnionWithInheritable {
              TrivialButInheritable t;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Copy ... )});
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Clone ... )});
        Ok(())
    }

    /// Classes with a non-public destructor shouldn't be constructible, not
    /// even via Copy/Clone.
    #[test]
    fn test_trivial_nonpublic_destructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct Indestructible final {
              Indestructible() = default;
              Indestructible(int);
              Indestructible(const Indestructible&) = default;
              void Foo() const;
             private:
              ~Indestructible() = default;
            };

            Indestructible ReturnsValue();
            void TakesValue(Indestructible);
            void TakesReference(const Indestructible& x);
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        // It isn't available by value:
        assert_rs_not_matches!(rs_api, quote! {Default});
        assert_rs_not_matches!(rs_api, quote! {From});
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Copy ... )});
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Clone ... )});
        assert_rs_not_matches!(rs_api, quote! {ReturnsValue});
        assert_rs_not_matches!(rs_api, quote! {TakesValue});
        // ... but it is otherwise available:
        assert_rs_matches!(rs_api, quote! {struct Indestructible});
        assert_rs_matches!(rs_api, quote! {fn Foo<'a>(&'a self)});
        assert_rs_matches!(rs_api, quote! {fn TakesReference<'a>(x: &'a crate::Indestructible)});
        Ok(())
    }

    #[test]
    fn test_nontrivial_nonpublic_destructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct Indestructible final {
              Indestructible() = default;
              Indestructible(int);
              Indestructible(const Indestructible&) = default;
              void Foo() const;
             private:
              ~Indestructible() {}
            };

            Indestructible ReturnsValue();
            void TakesValue(Indestructible);
            void TakesReference(const Indestructible& x);
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        // It isn't available by value:
        assert_rs_not_matches!(rs_api, quote! {CtorNew});
        assert_rs_not_matches!(rs_api, quote! {ReturnsValue});
        assert_rs_not_matches!(rs_api, quote! {TakesValue});
        // ... but it is otherwise available:
        assert_rs_matches!(rs_api, quote! {struct Indestructible});
        assert_rs_matches!(rs_api, quote! {fn Foo<'a>(&'a self)});
        assert_rs_matches!(rs_api, quote! {fn TakesReference<'a>(x: &'a crate::Indestructible)});
        Ok(())
    }

    /// trivial abstract structs shouldn't be constructible, not even via
    /// Copy/Clone.
    ///
    /// Right now, a struct can only be Copy/Clone if it's final, but that
    /// restriction will likely be lifted later.
    #[test]
    fn test_trivial_abstract_by_value() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct Abstract final {
              Abstract() = default;
              Abstract(int);
              Abstract(const Abstract&) = default;
              virtual void Foo() const = 0;
              void Nonvirtual() const;
            };
            void TakesAbstract(const Abstract& a);
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        // It isn't available by value:
        assert_rs_not_matches!(rs_api, quote! {Default});
        assert_rs_not_matches!(rs_api, quote! {From});
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Copy ... )});
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Clone ... )});
        // ... but it is otherwise available:
        assert_rs_matches!(rs_api, quote! {struct Abstract});
        assert_rs_matches!(rs_api, quote! {fn Foo<'a>(&'a self)});
        assert_rs_matches!(rs_api, quote! {fn Nonvirtual<'a>(&'a self)});
        assert_rs_matches!(rs_api, quote! {fn TakesAbstract<'a>(a: &'a crate::Abstract)});
        Ok(())
    }

    #[test]
    fn test_nontrivial_abstract_by_value() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct Abstract final {
              Abstract() {};
              Abstract(int);
              Abstract(const Abstract&) {}
              virtual void Foo() const = 0;
              void Nonvirtual() const;
            };
            void TakesAbstract(const Abstract& a);
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {CtorNew});
        // ... but it is otherwise available:
        assert_rs_matches!(rs_api, quote! {struct Abstract});
        assert_rs_matches!(rs_api, quote! {fn Foo<'a>(&'a self)});
        assert_rs_matches!(rs_api, quote! {fn Nonvirtual<'a>(&'a self)});
        assert_rs_matches!(rs_api, quote! {fn TakesAbstract<'a>(a: &'a crate::Abstract)});
        Ok(())
    }

    #[test]
    fn test_struct_with_unnamed_struct_and_union_members() -> Result<()> {
        // This test input causes `field_decl->getName()` to return an empty string.
        // See also:
        // - https://en.cppreference.com/w/c/language/struct: "[...] an unnamed member
        //   of a struct whose type is a struct without name is known as anonymous
        //   struct."
        // - https://rust-lang.github.io/rfcs/2102-unnamed-fields.html
        let ir = ir_from_cc(
            r#"
            struct StructWithUnnamedMembers {
              int first_field;

              struct {
                int anonymous_struct_field_1;
                int anonymous_struct_field_2;
              };
              union {
                int anonymous_union_field_1;
                int anonymous_union_field_2;
              };

              int last_field;
            }; "#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        // TODO(b/200067824): Once nested structs anhd unions are supported,
        // `__unnamed_field1` and `__unnamed_field2` should have a real, usable
        // type.
        assert_rs_matches!(
            rs_api,
            quote! {
               #[repr(C, align(4))]
               pub struct StructWithUnnamedMembers {
                   pub first_field: i32,
                   #[doc=" Reason for representing this field as a blob of bytes:\n Unsupported type 'struct StructWithUnnamedMembers::(anonymous at ir_from_cc_virtual_header.h:7:15)': No generated bindings found for ''"]
                   pub(crate) __unnamed_field1: [::std::mem::MaybeUninit<u8>; 8],
                   #[doc=" Reason for representing this field as a blob of bytes:\n Unsupported type 'union StructWithUnnamedMembers::(anonymous at ir_from_cc_virtual_header.h:11:15)': No generated bindings found for ''"]
                   pub(crate) __unnamed_field2: [::std::mem::MaybeUninit<u8>; 4],
                   pub last_field: i32,
               }
               ...
               const _: () = assert!(memoffset_unstable_const::offset_of!(
                       crate::StructWithUnnamedMembers, first_field) == 0);
               const _: () = assert!(memoffset_unstable_const::offset_of!(
                       crate::StructWithUnnamedMembers, __unnamed_field1) == 4);
               const _: () = assert!(memoffset_unstable_const::offset_of!(
                       crate::StructWithUnnamedMembers, __unnamed_field2) == 12);
               const _: () = assert!(memoffset_unstable_const::offset_of!(
                       crate::StructWithUnnamedMembers, last_field) == 16);
            }
        );
        Ok(())
    }

    #[test]
    fn test_struct_from_other_target() -> Result<()> {
        let ir = ir_from_cc_dependency("// intentionally empty", "struct SomeStruct {};")?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_not_matches!(rs_api, quote! { SomeStruct });
        assert_cc_not_matches!(rs_api_impl, quote! { SomeStruct });
        Ok(())
    }

    #[test]
    fn test_copy_derives() {
        let record = ir_record("S");
        assert_eq!(generate_derives(&record), &["Clone", "Copy"]);
    }

    #[test]
    fn test_copy_derives_not_is_trivial_abi() {
        let mut record = ir_record("S");
        record.is_trivial_abi = false;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    /// Even if it's trivially relocatable, !Unpin C++ type cannot be
    /// cloned/copied or otherwise used by value, because values would allow
    /// assignment into the Pin.
    ///
    /// All !Unpin C++ types, not just non trivially relocatable ones, are
    /// unsafe to assign in the Rust sense.
    #[test]
    fn test_copy_derives_not_final() {
        let mut record = ir_record("S");
        record.is_inheritable = true;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    #[test]
    fn test_copy_derives_ctor_deleted() {
        let mut record = ir_record("S");
        record.copy_constructor = ir::SpecialMemberFunc::Unavailable;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    #[test]
    fn test_copy_derives_ctor_nontrivial_members() {
        let mut record = ir_record("S");
        record.copy_constructor = ir::SpecialMemberFunc::NontrivialMembers;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    #[test]
    fn test_copy_derives_ctor_nontrivial_self() {
        let mut record = ir_record("S");
        record.copy_constructor = ir::SpecialMemberFunc::NontrivialUserDefined;
        assert_eq!(generate_derives(&record), &[""; 0]);
    }

    /// In Rust, a Drop type cannot be Copy.
    #[test]
    fn test_copy_derives_dtor_nontrivial_self() {
        let mut record = ir_record("S");
        for definition in
            [ir::SpecialMemberFunc::NontrivialUserDefined, ir::SpecialMemberFunc::NontrivialMembers]
        {
            record.destructor = definition;
            assert_eq!(generate_derives(&record), &["Clone"]);
        }
    }

    #[test]
    fn test_ptr_func() -> Result<()> {
        let ir = ir_from_cc(&tokens_to_string(quote! {
            inline int* Deref(int*const* p);
        })?)?;

        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub unsafe fn Deref(p: *const *mut i32) -> *mut i32 {
                    crate::detail::__rust_thunk___Z5DerefPKPi(p)
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    extern "C" {
                        pub(crate) fn __rust_thunk___Z5DerefPKPi(p: *const *mut i32) -> *mut i32;
                    }
                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" int* __rust_thunk___Z5DerefPKPi(int* const * p) {
                    return Deref(p);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_const_char_ptr_func() -> Result<()> {
        // This is a regression test: We used to include the "const" in the name
        // of the CcType, which caused a panic in the code generator
        // ('"const char" is not a valid Ident').
        // It's therefore important that f() is inline so that we need to
        // generate a thunk for it (where we then process the CcType).
        let ir = ir_from_cc(&tokens_to_string(quote! {
            inline void f(const char *str);
        })?)?;

        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub unsafe fn f(str: *const i8) {
                    crate::detail::__rust_thunk___Z1fPKc(str)
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                extern "C" {
                    pub(crate) fn __rust_thunk___Z1fPKc(str: *const i8);
                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z1fPKc(char const * str){ f(str); }
            }
        );
        Ok(())
    }

    #[test]
    fn test_func_ptr_where_params_are_primitive_types() -> Result<()> {
        let ir = ir_from_cc(r#" int (*get_ptr_to_func())(float, double); "#)?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn get_ptr_to_func() -> Option<extern "C" fn (f32, f64) -> i32> {
                    unsafe { crate::detail::__rust_thunk___Z15get_ptr_to_funcv() }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    extern "C" {
                        #[link_name = "_Z15get_ptr_to_funcv"]
                        pub(crate) fn __rust_thunk___Z15get_ptr_to_funcv()
                        -> Option<extern "C" fn(f32, f64) -> i32>;
                    }
                }
            }
        );
        // Verify that no C++ thunk got generated.
        assert_cc_not_matches!(rs_api_impl, quote! { __rust_thunk___Z15get_ptr_to_funcv });

        // TODO(b/217419782): Add another test for more exotic calling conventions /
        // abis.

        // TODO(b/217419782): Add another test for pointer to a function that
        // takes/returns non-trivially-movable types by value. See also
        // <internal link>

        Ok(())
    }

    #[test]
    fn test_func_ref() -> Result<()> {
        let ir = ir_from_cc(r#" int (&get_ref_to_func())(float, double); "#)?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn get_ref_to_func() -> extern "C" fn (f32, f64) -> i32 {
                    unsafe { crate::detail::__rust_thunk___Z15get_ref_to_funcv() }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_func_ptr_with_non_static_lifetime() -> Result<()> {
        let ir = ir_from_cc(&with_lifetime_macros(
            r#"
            int (* $a get_ptr_to_func())(float, double); "#,
        ))?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                // Error while generating bindings for item 'get_ptr_to_func':
                // Return type is not supported: Function pointers with non-'static lifetimes are not supported: int (*)(float, double)
            }
        );
        Ok(())
    }

    #[test]
    fn test_func_ptr_where_params_are_raw_ptrs() -> Result<()> {
        let ir = ir_from_cc(r#" const int* (*get_ptr_to_func())(const int*); "#)?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn get_ptr_to_func() -> Option<extern "C" fn (*const i32) -> *const i32> {
                    unsafe { crate::detail::__rust_thunk___Z15get_ptr_to_funcv() }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    extern "C" {
                        #[link_name = "_Z15get_ptr_to_funcv"]
                        pub(crate) fn __rust_thunk___Z15get_ptr_to_funcv()
                        -> Option<extern "C" fn(*const i32) -> *const i32>;
                    }
                }
            }
        );
        // Verify that no C++ thunk got generated.
        assert_cc_not_matches!(rs_api_impl, quote! { __rust_thunk___Z15get_ptr_to_funcv });

        // TODO(b/217419782): Add another test where params (and the return
        // type) are references with lifetimes.  Something like this:
        //     #pragma clang lifetime_elision
        //     const int& (*get_ptr_to_func())(const int&, const int&); "#)?;
        // 1) Need to investigate why this fails - seeing raw pointers in Rust
        //    seems to indicate that no lifetimes are present at the `importer.cc`
        //    level. Maybe lifetime elision doesn't support this scenario? Unclear
        //    how to explicitly apply [[clang::annotate("lifetimes", "a, b -> a")]]
        //    to the _inner_ function.
        // 2) It is important to have 2 reference parameters, so see if the problem
        //    of passing `lifetimes` by value would have been caught - see:
        //    cl/428079010/depot/rs_bindings_from_cc/
        // importer.cc?version=s6#823

        // TODO(b/217419782): Decide what to do if the C++ pointer is *not*
        // annotated with a lifetime - emit `unsafe fn(...) -> ...` in that
        // case?

        Ok(())
    }

    #[test]
    fn test_func_ptr_with_custom_abi() -> Result<()> {
        let ir = ir_from_cc(r#" int (*get_ptr_to_func())(float, double) [[clang::vectorcall]]; "#)?;

        // Verify that the test input correctly represents what we intend to
        // test - we want [[clang::vectorcall]] to apply to the returned
        // function pointer, but *not* apply to the `get_ptr_to_func` function.
        assert_ir_matches!(
            ir,
            quote! {
                Func(Func {
                    name: "get_ptr_to_func", ...
                    return_type: MappedType {
                        rs_type: RsType {
                            name: Some("Option"), ...
                            type_args: [RsType { name: Some("#funcPtr vectorcall"), ... }], ...
                        },
                        cc_type: CcType {
                            name: Some("*"), ...
                            type_args: [CcType { name: Some("#funcValue vectorcall"), ... }], ...
                        },
                    }, ...
                    has_c_calling_convention: true, ...
                }),
            }
        );

        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        // Check that the custom "vectorcall" ABI gets propagated into the
        // return type (i.e. into `extern "vectorcall" fn`).
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn get_ptr_to_func() -> Option<extern "vectorcall" fn (f32, f64) -> i32> {
                    unsafe { crate::detail::__rust_thunk___Z15get_ptr_to_funcv() }
                }
            }
        );

        // The usual `extern "C"` ABI should be used for "get_ptr_to_func".
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    extern "C" {
                        #[link_name = "_Z15get_ptr_to_funcv"]
                        pub(crate) fn __rust_thunk___Z15get_ptr_to_funcv()
                        -> Option<extern "vectorcall" fn(f32, f64) -> i32>;
                    }
                }
            }
        );

        // Verify that no C++ thunk got generated.
        assert_cc_not_matches!(rs_api_impl, quote! { __rust_thunk___Z15get_ptr_to_funcv });
        Ok(())
    }

    #[test]
    fn test_func_ptr_thunk() -> Result<()> {
        // Using an `inline` keyword forces generation of a C++ thunk in
        // `rs_api_impl` (i.e. exercises `format_cc_type` and similar code).
        let ir = ir_from_cc(
            r#"
            int multiply(int x, int y);
            inline int (*inline_get_pointer_to_function())(int, int) {
                return multiply;
            }
        "#,
        )?;
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" crubit::type_identity_t<int(int , int)>*
                __rust_thunk___Z30inline_get_pointer_to_functionv() {
                    return inline_get_pointer_to_function();
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_func_ptr_with_custom_abi_thunk() -> Result<()> {
        // Using an `inline` keyword forces generation of a C++ thunk in
        // `rs_api_impl` (i.e. exercises `format_cc_type`,
        // `format_cc_call_conv_as_clang_attribute` and similar code).
        let ir = ir_from_cc(
            r#"
            inline int (*inline_get_ptr_to_func())(float, double) [[clang::vectorcall]];
        "#,
        )?;

        // Verify that the test input correctly represents what we intend to
        // test - we want [[clang::vectorcall]] to apply to the returned
        // function pointer, but *not* apply to the `get_ptr_to_func` function.
        assert_ir_matches!(
            ir,
            quote! {
                Func(Func {
                    name: "inline_get_ptr_to_func", ...
                    return_type: MappedType {
                        rs_type: RsType {
                            name: Some("Option"), ...
                            type_args: [RsType { name: Some("#funcPtr vectorcall"), ... }], ...
                        },
                        cc_type: CcType {
                            name: Some("*"), ...
                            type_args: [CcType { name: Some("#funcValue vectorcall"), ... }], ...
                        },
                    }, ...
                    has_c_calling_convention: true, ...
                }),
            }
        );

        // This test is quite similar to `test_func_ptr_thunk` - the main
        // difference is verification of the `__attribute__((vectorcall))` in
        // the expected signature of the generated thunk below.
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" crubit::type_identity_t<
                        int(float , double) __attribute__((vectorcall))
                    >* __rust_thunk___Z22inline_get_ptr_to_funcv() {
                    return inline_get_ptr_to_func();
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_item_order() -> Result<()> {
        let ir = ir_from_cc(
            "int first_func();
             struct FirstStruct {};
             int second_func();
             struct SecondStruct {};",
        )?;

        let rs_api = rs_tokens_to_formatted_string_for_tests(generate_bindings_tokens(ir)?.rs_api)?;

        let idx = |s: &str| rs_api.find(s).ok_or_else(|| anyhow!("'{}' missing", s));

        let f1 = idx("fn first_func")?;
        let f2 = idx("fn second_func")?;
        let s1 = idx("struct FirstStruct")?;
        let s2 = idx("struct SecondStruct")?;
        let t1 = idx("fn __rust_thunk___Z10first_funcv")?;
        let t2 = idx("fn __rust_thunk___Z11second_funcv")?;

        assert!(f1 < s1);
        assert!(s1 < f2);
        assert!(f2 < s2);
        assert!(s2 < t1);
        assert!(t1 < t2);

        Ok(())
    }

    #[test]
    fn test_base_class_subobject_layout() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            // We use a class here to force `Derived::z` to live inside the tail padding of `Base`.
            // On the Itanium ABI, this would not happen if `Base` were a POD type.
            class Base {__INT64_TYPE__ x; char y;};
            struct Derived final : Base {__INT16_TYPE__ z;};
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                pub struct Derived {
                    __non_field_data: [::std::mem::MaybeUninit<u8>; 10],
                    pub z: i16,
                }
            }
        );
        Ok(())
    }

    /// The same as test_base_class_subobject_layout, but with multiple
    /// inheritance.
    #[test]
    fn test_base_class_multiple_inheritance_subobject_layout() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base1 {__INT64_TYPE__ x;};
            class Base2 {char y;};
            struct Derived final : Base1, Base2 {__INT16_TYPE__ z;};
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                pub struct Derived {
                    __non_field_data: [::std::mem::MaybeUninit<u8>; 10],
                    pub z: i16,
                }
            }
        );
        Ok(())
    }

    /// The same as test_base_class_subobject_layout, but with a chain of
    /// inheritance.
    #[test]
    fn test_base_class_deep_inheritance_subobject_layout() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base1 {__INT64_TYPE__ x;};
            class Base2 : Base1 {char y;};
            struct Derived final : Base2 {__INT16_TYPE__ z;};
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                pub struct Derived {
                    __non_field_data: [::std::mem::MaybeUninit<u8>; 10],
                    pub z: i16,
                }
            }
        );
        Ok(())
    }

    /// For derived classes with no data members, we can't use the offset of the
    /// first member to determine the size of the base class subobjects.
    #[test]
    fn test_base_class_subobject_fieldless_layout() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base {__INT64_TYPE__ x; char y;};
            struct Derived final : Base {};
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                pub struct Derived {
                    __non_field_data: [::std::mem::MaybeUninit<u8>; 16],
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_base_class_subobject_empty_fieldless() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base {};
            struct Derived final : Base {};
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C)]
                pub struct Derived {
                    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_base_class_subobject_empty() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Base {};
            struct Derived final : Base {
                __INT16_TYPE__ x;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub struct Derived {
                    // TODO(b/232984274): delete this.
                    // Currently, our tests use C++14 instead of C++17. In C++14, `Derived`
                    // is not an aggregate, because it has a base class. C++17 removed this
                    // restriction, and allows aggregates to have base classes.
                    __non_field_data:  [::std::mem::MaybeUninit<u8>; 0],
                    pub x: i16,
                }
            }
        );
        Ok(())
    }

    /// Non-aggregate structs can't be directly initialized, because we add
    /// a zero-sized private field to the bindings.
    #[test]
    fn test_non_aggregate_struct_private_field() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            struct NonAggregate {
                NonAggregate() {}

                __INT16_TYPE__ x = 0;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub struct NonAggregate {
                    __non_field_data:  [::std::mem::MaybeUninit<u8>; 0],
                    pub x: i16,
                }
            }
        );
        Ok(())
    }

    /// When a field is [[no_unique_address]], it occupies the space up to the
    /// next field.
    #[test]
    fn test_no_unique_address() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Field1 {__INT64_TYPE__ x;};
            class Field2 {char y;};
            struct Struct final {
                [[no_unique_address]] Field1 field1;
                [[no_unique_address]] Field2 field2;
                __INT16_TYPE__ z;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                pub struct Struct {
                    pub(crate) field1: [::std::mem::MaybeUninit<u8>; 8],
                    pub(crate) field2: [::std::mem::MaybeUninit<u8>; 2],
                    pub z: i16,
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                impl Struct {
                    pub fn field1(&self) -> &crate::Field1 {
                        unsafe {&* (&self.field1 as *const _ as *const crate::Field1)}
                    }
                    pub fn field2(&self) -> &crate::Field2 {
                        unsafe {&* (&self.field2 as *const _ as *const crate::Field2)}
                    }
                }
            }
        );
        Ok(())
    }

    /// When a [[no_unique_address]] field is the last one, it occupies the rest
    /// of the object.
    #[test]
    fn test_no_unique_address_last_field() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Field1 {__INT64_TYPE__ x;};
            class Field2 {char y;};
            struct Struct final {
                [[no_unique_address]] Field1 field1;
                [[no_unique_address]] Field2 field2;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(8))]
                pub struct Struct {
                    pub(crate) field1: [::std::mem::MaybeUninit<u8>; 8],
                    pub(crate) field2: [::std::mem::MaybeUninit<u8>; 8],
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_no_unique_address_empty() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Field {};
            struct Struct final {
                [[no_unique_address]] Field field;
                int x;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(4))]
                pub struct Struct {
                    pub(crate) field: [::std::mem::MaybeUninit<u8>; 0],
                    pub x: i32,
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_base_class_subobject_empty_last_field() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            class Field {};
            struct Struct final {
                [[no_unique_address]] Field field;
            };
        "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C)]
                pub struct Struct {
                    pub(crate) field: [::std::mem::MaybeUninit<u8>; 1],
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_generate_enum_basic() -> Result<()> {
        let ir = ir_from_cc("enum Color { kRed = 5, kBlue };")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                pub struct Color(u32);
                impl Color {
                    pub const kRed: Color = Color(5);
                    pub const kBlue: Color = Color(6);
                }
                impl From<u32> for Color {
                    fn from(value: u32) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for u32 {
                    fn from(value: Color) -> u32 {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_generate_scoped_enum_basic() -> Result<()> {
        let ir = ir_from_cc("enum class Color { kRed = -5, kBlue };")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                pub struct Color(i32);
                impl Color {
                    pub const kRed: Color = Color(-5);
                    pub const kBlue: Color = Color(-4);
                }
                impl From<i32> for Color {
                    fn from(value: i32) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for i32 {
                    fn from(value: Color) -> i32 {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_generate_enum_with_64_bit_signed_vals() -> Result<()> {
        let ir = ir_from_cc(
            "enum Color : long { kViolet = -9223372036854775807 - 1LL, kRed = -5, kBlue, kGreen = 3, kMagenta = 9223372036854775807 };",
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                pub struct Color(i64);
                impl Color {
                    pub const kViolet: Color = Color(-9223372036854775808);
                    pub const kRed: Color = Color(-5);
                    pub const kBlue: Color = Color(-4);
                    pub const kGreen: Color = Color(3);
                    pub const kMagenta: Color = Color(9223372036854775807);
                }
                impl From<i64> for Color {
                    fn from(value: i64) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for i64 {
                    fn from(value: Color) -> i64 {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_generate_enum_with_64_bit_unsigned_vals() -> Result<()> {
        let ir = ir_from_cc(
            "enum Color: unsigned long { kRed, kBlue, kLimeGreen = 18446744073709551615 };",
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                pub struct Color(u64);
                impl Color {
                    pub const kRed: Color = Color(0);
                    pub const kBlue: Color = Color(1);
                    pub const kLimeGreen: Color = Color(18446744073709551615);
                }
                impl From<u64> for Color {
                    fn from(value: u64) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for u64 {
                    fn from(value: Color) -> u64 {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_generate_enum_with_32_bit_signed_vals() -> Result<()> {
        let ir = ir_from_cc(
            "enum Color { kViolet = -2147483647 - 1, kRed = -5, kBlue, kGreen = 3, kMagenta = 2147483647 };",
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                pub struct Color(i32);
                impl Color {
                    pub const kViolet: Color = Color(-2147483648);
                    pub const kRed: Color = Color(-5);
                    pub const kBlue: Color = Color(-4);
                    pub const kGreen: Color = Color(3);
                    pub const kMagenta: Color = Color(2147483647);
                }
                impl From<i32> for Color {
                    fn from(value: i32) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for i32 {
                    fn from(value: Color) -> i32 {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_generate_enum_with_32_bit_unsigned_vals() -> Result<()> {
        let ir = ir_from_cc("enum Color: unsigned int { kRed, kBlue, kLimeGreen = 4294967295 };")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(transparent)]
                #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
                pub struct Color(u32);
                impl Color {
                    pub const kRed: Color = Color(0);
                    pub const kBlue: Color = Color(1);
                    pub const kLimeGreen: Color = Color(4294967295);
                }
                impl From<u32> for Color {
                    fn from(value: u32) -> Color {
                        Color(value)
                    }
                }
                impl From<Color> for u32 {
                    fn from(value: Color) -> u32 {
                        value.0
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_doc_comment_func() -> Result<()> {
        let ir = ir_from_cc(
            "
        // Doc Comment
        // with two lines
        int func();",
        )?;

        assert_rs_matches!(
            generate_bindings_tokens(ir)?.rs_api,
            // leading space is intentional so there is a space between /// and the text of the
            // comment
            quote! {
                #[doc = " Doc Comment\n with two lines"]
                #[inline(always)]
                pub fn func
            }
        );

        Ok(())
    }

    #[test]
    fn test_doc_comment_record() -> Result<()> {
        let ir = ir_from_cc(
            "// Doc Comment\n\
            //\n\
            //  * with bullet\n\
            struct SomeStruct final {\n\
                // Field doc\n\
                int field;\
            };",
        )?;

        assert_rs_matches!(
            generate_bindings_tokens(ir)?.rs_api,
            quote! {
                #[doc = " Doc Comment\n \n  * with bullet"]
                #[derive(Clone, Copy)]
                #[repr(C)]
                pub struct SomeStruct {
                    # [doc = " Field doc"]
                    pub field: i32,
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_basic_union() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            union SomeUnion {
                int some_field;
                long long some_bigger_field;
            };
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C)]
                pub union SomeUnion {
                    pub some_field: i32,
                    pub some_bigger_field: i64,
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN9SomeUnionC1Ev(union SomeUnion*__this) {...}
            }
        );
        assert_cc_matches!(rs_api_impl, quote! { static_assert(sizeof(union SomeUnion)==8) });
        assert_cc_matches!(rs_api_impl, quote! { static_assert(alignof(union SomeUnion)==8) });
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(CRUBIT_OFFSET_OF(some_field, union SomeUnion)==0) }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(CRUBIT_OFFSET_OF(some_bigger_field, union SomeUnion)==0) }
        );
        Ok(())
    }

    #[test]
    fn test_union_with_opaque_field() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            union MyUnion {
                char first_field[56];
                int second_field;
              };
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C, align(4))]
                pub union MyUnion { ...
                    first_field: [::std::mem::MaybeUninit<u8>; 56],
                    pub second_field: i32,
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! { const _: () = assert!(::std::mem::size_of::<crate::MyUnion>() == 56); }
        );
        assert_rs_matches!(
            rs_api,
            quote! {  const _: () = assert!(::std::mem::align_of::<crate::MyUnion>() == 4); }
        );
        Ok(())
    }

    #[test]
    // TODO(https://github.com/Gilnaa/memoffset/issues/66): generate assertions for unions once
    // offsetof supports them.
    fn test_currently_no_offset_assertions_for_unions() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            union SomeUnion {
                int some_field;
                long long some_bigger_field;
            };
            "#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;

        assert_rs_not_matches!(rs_api, quote! { offset_of! });
        Ok(())
    }

    #[test]
    fn test_union_with_private_fields() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            union SomeUnionWithPrivateFields {
              public:
                int public_field;
              private:
                long long private_field;
            };
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C, align(8))]
                pub union SomeUnionWithPrivateFields {
                    pub public_field: i32,
                    #[doc = " Reason for representing this field as a blob of bytes:\n Types of non-public C++ fields can be elided away"]
                    pub(crate) private_field: [::std::mem::MaybeUninit<u8>; 8],
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = assert!(::std::mem::size_of::<crate::SomeUnionWithPrivateFields>() == 8);
                const _: () = assert!(::std::mem::align_of::<crate::SomeUnionWithPrivateFields>() == 8);
                const _: () = {
                  static_assertions::assert_impl_all!(crate::SomeUnionWithPrivateFields: Clone);
                };
                const _: () = {
                  static_assertions::assert_impl_all!(crate::SomeUnionWithPrivateFields: Copy);
                };
                const _: () = {
                  static_assertions::assert_not_impl_any!(crate::SomeUnionWithPrivateFields: Drop);
                };
            }
        );
        Ok(())
    }

    #[test]
    fn test_nontrivial_unions() -> Result<()> {
        let ir = ir_from_cc_dependency(
            r#"
            union UnionWithNontrivialField {
                NonTrivialStruct my_field;
            };
            "#,
            r#"
            struct NonTrivialStruct {
                NonTrivialStruct(NonTrivialStruct&&);
            };
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_not_matches!(rs_api, quote! {derive ( ... Copy ... )});
        assert_rs_not_matches!(rs_api, quote! {derive ( ... Clone ... )});
        assert_rs_matches!(
            rs_api,
            quote! {
                #[::ctor::recursively_pinned]
                #[repr(C)]
                pub union UnionWithNontrivialField { ... }
            }
        );
        Ok(())
    }

    #[test]
    fn test_empty_struct() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            struct EmptyStruct final {};
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C)]
                pub struct EmptyStruct {
                    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = assert!(::std::mem::size_of::<crate::EmptyStruct>() == 1);
                const _: () = assert!(::std::mem::align_of::<crate::EmptyStruct>() == 1);
            }
        );

        Ok(())
    }

    #[test]
    fn test_empty_union() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            union EmptyUnion {};
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C)]
                pub union EmptyUnion {
                    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = assert!(::std::mem::size_of::<crate::EmptyUnion>() == 1);
                const _: () = assert!(::std::mem::align_of::<crate::EmptyUnion>() == 1);
            }
        );

        Ok(())
    }

    #[test]
    fn test_union_field_with_nontrivial_destructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            struct NontrivialStruct { ~NontrivialStruct(); };
            union UnionWithNontrivialField {
                int trivial_field;
                NontrivialStruct nontrivial_field;
            };
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[repr(C)]
                pub union UnionWithNontrivialField {
                    pub trivial_field: i32,
                    pub nontrivial_field: ::std::mem::ManuallyDrop<crate::NontrivialStruct>,
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                const _: () = assert!(::std::mem::size_of::<crate::UnionWithNontrivialField>() == 4);
                const _: () = assert!(::std::mem::align_of::<crate::UnionWithNontrivialField>() == 4);
            }
        );
        Ok(())
    }

    #[test]
    fn test_union_with_constructors() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            union UnionWithDefaultConstructors {
                int a;
            };
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                #[derive(Clone, Copy)]
                #[repr(C)]
                pub union UnionWithDefaultConstructors {
                    pub a: i32,
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                impl Default for UnionWithDefaultConstructors {
                    #[inline(always)]
                    fn default() -> Self {
                        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN28UnionWithDefaultConstructorsC1Ev(&mut tmp);
                            tmp.assume_init()
                        }
                    }
                }
            }
        );

        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> From<::ctor::RvalueReference<'b, crate::UnionWithDefaultConstructors>> for UnionWithDefaultConstructors {
                    #[inline(always)]
                    fn from(__param_0: ::ctor::RvalueReference<'b, crate::UnionWithDefaultConstructors>) -> Self {
                        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN28UnionWithDefaultConstructorsC1EOS_(&mut tmp, __param_0);
                            tmp.assume_init()
                        }
                    }
                }
            }
        );

        Ok(())
    }

    #[test]
    fn test_unambiguous_public_bases() -> Result<()> {
        let ir = ir_from_cc_dependency(
            "
            struct VirtualBase {};
            struct PrivateBase {};
            struct ProtectedBase {};
            struct UnambiguousPublicBase {};
            struct AmbiguousPublicBase {};
            struct MultipleInheritance : UnambiguousPublicBase, AmbiguousPublicBase {};
            struct Derived : private PrivateBase, protected ProtectedBase, MultipleInheritance, AmbiguousPublicBase, virtual VirtualBase {};
        ",
            "",
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                unsafe impl oops::Inherits<crate::VirtualBase> for crate::Derived {
                    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::VirtualBase {
                        crate::detail::__crubit_dynamic_upcast__Derived__to__VirtualBase(derived)
                    }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! { unsafe impl oops::Inherits<crate::UnambiguousPublicBase> for crate::Derived }
        );
        assert_rs_matches!(
            rs_api,
            quote! { unsafe impl oops::Inherits<crate::MultipleInheritance> for crate::Derived }
        );
        assert_rs_not_matches!(
            rs_api,
            quote! {unsafe impl oops::Inherits<crate::PrivateBase> for crate::Derived}
        );
        assert_rs_not_matches!(
            rs_api,
            quote! {unsafe impl oops::Inherits<crate::ProtectedBase> for crate::Derived}
        );
        assert_rs_not_matches!(
            rs_api,
            quote! {unsafe impl oops::Inherits<crate::AmbiguousPublicBase> for crate::Derived}
        );
        Ok(())
    }

    /// Contrary to intuitions: a base class conversion is ambiguous even if the
    /// ambiguity is from a private base class cast that you can't even
    /// perform.
    ///
    /// Explanation (courtesy James Dennett):
    ///
    /// > Once upon a time, there was a rule in C++ that changing all access
    /// > specifiers to "public" would not change the meaning of code.
    /// > That's no longer true, but some of its effects can still be seen.
    ///
    /// So, we need to be sure to not allow casting to privately-ambiguous
    /// bases.
    #[test]
    fn test_unambiguous_public_bases_private_ambiguity() -> Result<()> {
        let ir = ir_from_cc_dependency(
            "
            struct Base {};
            struct Intermediate : public Base {};
            struct Derived : Base, private Intermediate {};
        ",
            "",
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(
            rs_api,
            quote! { unsafe impl oops::Inherits<crate::Base> for Derived }
        );
        Ok(())
    }

    #[test]
    fn test_virtual_thunk() -> Result<()> {
        let ir = ir_from_cc("struct Polymorphic { virtual void Foo(); };")?;

        assert_cc_matches!(
            generate_bindings_tokens(ir)?.rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN11Polymorphic3FooEv(struct Polymorphic * __this)
            }
        );
        Ok(())
    }

    #[test]
    fn test_custom_abi_thunk() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            float f_vectorcall_calling_convention(float p1, float p2) [[clang::vectorcall]];
            double f_c_calling_convention(double p1, double p2);
        "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn f_vectorcall_calling_convention(p1: f32, p2: f32) -> f32 {
                    unsafe {
                        crate::detail::__rust_thunk___Z31f_vectorcall_calling_conventionff(p1, p2)
                    }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                #[inline(always)]
                pub fn f_c_calling_convention(p1: f64, p2: f64) -> f64 {
                    unsafe { crate::detail::__rust_thunk___Z22f_c_calling_conventiondd(p1, p2) }
                }
            }
        );
        // `link_name` (i.e. no thunk) for `f_c_calling_convention`. No
        // `link_name` (i.e. indicates presence of a thunk) for
        // `f_vectorcall_calling_convention`.
        assert_rs_matches!(
            rs_api,
            quote! {
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    extern "C" {
                        pub(crate) fn __rust_thunk___Z31f_vectorcall_calling_conventionff(
                            p1: f32, p2: f32) -> f32;
                        #[link_name = "_Z22f_c_calling_conventiondd"]
                        pub(crate) fn __rust_thunk___Z22f_c_calling_conventiondd(
                            p1: f64, p2: f64) -> f64;
                    }
                }
            }
        );
        // C++ thunk needed for `f_vectorcall_calling_convention`.
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" float __rust_thunk___Z31f_vectorcall_calling_conventionff(
                    float p1, float p2) {
                        return f_vectorcall_calling_convention(p1, p2);
                }
            }
        );
        // No C++ thunk expected for `f_c_calling_convention`.
        assert_cc_not_matches!(rs_api_impl, quote! { f_c_calling_convention });
        Ok(())
    }

    /// A trivially relocatable final struct is safe to use in Rust as normal,
    /// and is Unpin.
    #[test]
    fn test_no_negative_impl_unpin() -> Result<()> {
        let ir = ir_from_cc("struct Trivial final {};")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {#[::ctor::recursively_pinned]});
        Ok(())
    }

    /// A non-final struct, even if it's trivial, is not usable by mut
    /// reference, and so is !Unpin.
    #[test]
    fn test_negative_impl_unpin_nonfinal() -> Result<()> {
        let ir = ir_from_cc("struct Nonfinal {};")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned]});
        Ok(())
    }

    /// At the least, a trivial type should have no drop impl if or until we add
    /// empty drop impls.
    #[test]
    fn test_no_impl_drop() -> Result<()> {
        let ir = ir_from_cc("struct Trivial {};")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl Drop});
        assert_rs_not_matches!(rs_api, quote! {impl ::ctor::PinnedDrop});
        Ok(())
    }

    /// User-defined destructors *must* become Drop impls with ManuallyDrop
    /// fields
    #[test]
    fn test_impl_drop_user_defined_destructor() -> Result<()> {
        let ir = ir_from_cc(
            r#" struct NontrivialStruct { ~NontrivialStruct(); };
            struct UserDefinedDestructor {
                ~UserDefinedDestructor();
                int x;
                NontrivialStruct nts;
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl ::ctor::PinnedDrop for UserDefinedDestructor {
                    #[inline(always)]
                    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
                        crate::detail::__rust_thunk___ZN21UserDefinedDestructorD1Ev(self)
                    }
                }
            }
        );
        assert_rs_matches!(rs_api, quote! {pub x: i32,});
        assert_rs_matches!(
            rs_api,
            quote! {pub nts: ::std::mem::ManuallyDrop<crate::NontrivialStruct>,}
        );
        Ok(())
    }

    /// nontrivial types without user-defined destructors should invoke
    /// the C++ destructor to preserve the order of field destructions.
    #[test]
    fn test_impl_drop_nontrivial_member_destructor() -> Result<()> {
        // TODO(jeanpierreda): This would be cleaner if the UserDefinedDestructor code were
        // omitted. For example, we simulate it so that UserDefinedDestructor
        // comes from another library.
        let ir = ir_from_cc(
            r#"struct UserDefinedDestructor final {
                ~UserDefinedDestructor();
            };
            struct TrivialStruct final { int i; };
            struct NontrivialMembers final {
                UserDefinedDestructor udd;
                TrivialStruct ts;
                int x;
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl ::ctor::PinnedDrop for NontrivialMembers {
                    #[inline(always)]
                    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
                        crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self)
                    }
                }
            }
        );
        assert_rs_matches!(rs_api, quote! {pub x: i32,});
        assert_rs_matches!(rs_api, quote! {pub ts: crate::TrivialStruct,});
        assert_rs_matches!(
            rs_api,
            quote! {pub udd: ::std::mem::ManuallyDrop<crate::UserDefinedDestructor>,}
        );
        Ok(())
    }

    /// Trivial types (at least those that are mapped to Copy rust types) do not
    /// get a Drop impl.
    #[test]
    fn test_impl_drop_trivial() -> Result<()> {
        let ir = ir_from_cc(
            r#"struct Trivial final {
                ~Trivial() = default;
                int x;
            };"#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_not_matches!(rs_api, quote! {impl Drop});
        assert_rs_not_matches!(rs_api, quote! {impl ::ctor::PinnedDrop});
        assert_rs_matches!(rs_api, quote! {pub x: i32});
        assert_cc_not_matches!(rs_api_impl, quote! { std::destroy_at });
        Ok(())
    }

    #[test]
    fn test_impl_default_explicitly_defaulted_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct DefaultedConstructor final {
                DefaultedConstructor() = default;
            };"#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl Default for DefaultedConstructor {
                    #[inline(always)]
                    fn default() -> Self {
                        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN20DefaultedConstructorC1Ev(&mut tmp);
                            tmp.assume_init()
                        }
                    }
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN20DefaultedConstructorC1Ev(
                        struct DefaultedConstructor* __this) {
                    crubit::construct_at(__this);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_impl_clone_that_propagates_lifetime() -> Result<()> {
        // This test covers the case where a single lifetime applies to 1)
        // the `__this` parameter and 2) other constructor parameters. For
        // example, maybe the newly constructed object needs to have the
        // same lifetime as the constructor's parameter. (This might require
        // annotating the whole C++ struct with a lifetime, so maybe the
        // example below is not fully realistic/accurate...).
        let ir = ir_from_cc(&with_lifetime_macros(
            r#"#pragma clang lifetime_elision
            struct Foo final {
                Foo(const int& $a i) $a;
            };"#,
        ))?;
        let ctor: &Func = ir
            .items()
            .filter_map(|item| match item {
                Item::Func(func) => Some(&**func),
                _ => None,
            })
            .find(|f| {
                matches!(&f.name, UnqualifiedIdentifier::Constructor)
                    && f.params.get(1).map(|p| p.identifier.identifier == "i").unwrap_or_default()
            })
            .unwrap();
        {
            // Double-check that the test scenario set up above uses the same lifetime
            // for both of the constructor's parameters: `__this` and `i`.
            assert_eq!(ctor.params.len(), 2);
            let this_lifetime: LifetimeId =
                *ctor.params[0].type_.rs_type.lifetime_args.first().unwrap();
            let i_lifetime: LifetimeId =
                *ctor.params[1].type_.rs_type.lifetime_args.first().unwrap();
            assert_eq!(i_lifetime, this_lifetime);
        }

        // Before cl/423346348 the generated Rust code would incorrectly look
        // like this (note the mismatched 'a and 'b lifetimes):
        //     fn from<'b>(i: &'a i32) -> Self
        // After this CL, this scenario will result in an explicit error.
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl From});
        let rs_api_str = tokens_to_string(rs_api)?;
        assert!(rs_api_str.contains(
            "// The lifetime of `__this` is unexpectedly also used by another parameter"
        ));
        Ok(())
    }

    #[test]
    fn test_impl_default_non_trivial_struct() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct NonTrivialStructWithConstructors final {
                NonTrivialStructWithConstructors();
                ~NonTrivialStructWithConstructors();  // Non-trivial
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl Default});
        Ok(())
    }

    #[test]
    fn test_impl_from_for_explicit_conversion_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                explicit SomeStruct(int i);
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        // As discussed in b/214020567 for now we only generate `From::from` bindings
        // for *implicit* C++ conversion constructors.
        assert_rs_not_matches!(rs_api, quote! {impl From});
        Ok(())
    }

    #[test]
    fn test_impl_from_for_implicit_conversion_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                SomeStruct(int i);  // implicit - no `explicit` keyword
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        // As discussed in b/214020567 we generate `From::from` bindings for
        // *implicit* C++ conversion constructors.
        assert_rs_matches!(
            rs_api,
            quote! {
                impl From<i32> for SomeStruct {
                    #[inline(always)]
                    fn from(i: i32) -> Self {
                        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN10SomeStructC1Ei(&mut tmp, i);
                            tmp.assume_init()
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_impl_from_for_implicit_conversion_from_reference() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeOtherStruct final { int i; };
            struct StructUnderTest final {
                StructUnderTest(const SomeOtherStruct& other);  // implicit - no `explicit` keyword
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        // This is a regression test for b/223800038: We want to ensure that the
        // code says `impl<'b>` (instead of incorrectly declaring that lifetime
        // in `fn from<'b>`).
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> From<&'b crate::SomeOtherStruct> for StructUnderTest {
                    #[inline(always)]
                    fn from(other: &'b crate::SomeOtherStruct) -> Self {
                        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN15StructUnderTestC1ERK15SomeOtherStruct(
                                &mut tmp, other);
                            tmp.assume_init()
                        }
                    }
                }
            },
        );
        Ok(())
    }

    /// Methods with missing lifetimes for `self` should give a useful error
    /// message.
    #[test]
    fn test_eq_nolifetime() -> Result<()> {
        // Missing lifetimes currently only causes hard errors for trait impls,
        // not For inherent methods.
        let ir = ir_from_cc("struct SomeStruct{SomeStruct& operator=(const SomeStruct&);};")?;

        let rs_api = rs_tokens_to_formatted_string_for_tests(generate_bindings_tokens(ir)?.rs_api)?;
        assert!(rs_api.contains(
            "// Error while generating bindings for item 'SomeStruct::operator=':\n\
             // `self` has no lifetime. Use lifetime annotations or \
                `#pragma clang lifetime_elision` to create bindings for this function."
        ));
        Ok(())
    }

    #[test]
    fn test_impl_eq_for_member_function() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                inline bool operator==(const SomeStruct& other) const {
                    return i == other.i;
                }
                int i;
            };"#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl PartialEq<crate::SomeStruct> for SomeStruct {
                    #[inline(always)]
                    fn eq<'a, 'b>(&'a self, other: &'b crate::SomeStruct) -> bool {
                        unsafe { crate::detail::__rust_thunk___ZNK10SomeStructeqERKS_(self, other) }
                    }
                }
            }
        );
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" bool __rust_thunk___ZNK10SomeStructeqERKS_(
                        const struct SomeStruct* __this, const struct SomeStruct* other) {
                    return __this->operator==(*other);
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_impl_eq_for_free_function() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final { int i; };
            bool operator==(const SomeStruct& lhs, const SomeStruct& rhs) {
                return lhs.i == rhs.i;
            }"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl PartialEq<crate::SomeStruct> for SomeStruct {
                    #[inline(always)]
                    fn eq<'a, 'b>(&'a self, rhs: &'b crate::SomeStruct) -> bool {
                        unsafe { crate::detail::__rust_thunk___ZeqRK10SomeStructS1_(self, rhs) }
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_assign() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct SomeStruct {
                SomeStruct& operator=(const SomeStruct& other);
            };"#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> ::ctor::Assign<&'b crate::SomeStruct> for SomeStruct {
                    #[inline(always)]
                    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, other: &'b crate::SomeStruct) {
                        unsafe {
                            crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, other);
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_assign_nonreference_other() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct SomeStruct {
                SomeStruct& operator=(int other);
            };"#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> ::ctor::Assign<&'b crate::SomeStruct> for SomeStruct {
                    #[inline(always)]
                    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b crate::SomeStruct) {
                        unsafe {
                            crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, __param_0);
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_assign_nonreference_return() -> Result<()> {
        let ir = ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct SomeStruct {
                int operator=(const SomeStruct& other);
            };"#,
        )?;
        let BindingsTokens { rs_api, .. } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> ::ctor::Assign<&'b crate::SomeStruct> for SomeStruct {
                    #[inline(always)]
                    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, other: &'b crate::SomeStruct) {
                        unsafe {
                            crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, other);
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_impl_eq_non_const_member_function() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                bool operator==(const SomeStruct& other) /* no `const` here */;
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl PartialEq});
        Ok(())
    }

    #[test]
    fn test_impl_eq_rhs_by_value() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                bool operator==(SomeStruct other) const;
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_not_matches!(rs_api, quote! {impl PartialEq});
        Ok(())
    }

    #[test]
    fn test_thunk_ident_function() -> Result<()> {
        let ir = ir_from_cc("inline int foo() {}")?;
        let func = retrieve_func(&ir, "foo");
        assert_eq!(thunk_ident(&func), make_rs_ident("__rust_thunk___Z3foov"));
        Ok(())
    }

    #[test]
    fn test_thunk_ident_special_names() {
        let ir = ir_from_cc("struct Class {};").unwrap();

        let destructor =
            ir.functions().find(|f| f.name == UnqualifiedIdentifier::Destructor).unwrap();
        assert_eq!(thunk_ident(destructor), make_rs_ident("__rust_thunk___ZN5ClassD1Ev"));

        let default_constructor = ir
            .functions()
            .find(|f| f.name == UnqualifiedIdentifier::Constructor && f.params.len() == 1)
            .unwrap();
        assert_eq!(thunk_ident(default_constructor), make_rs_ident("__rust_thunk___ZN5ClassC1Ev"));
    }

    #[test]
    fn test_elided_lifetimes() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
          struct S final {
            int& f(int& i);
          };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn f<'a, 'b>(&'a mut self, i: &'b mut i32) -> &'a mut i32 { ... }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                pub(crate) fn __rust_thunk___ZN1S1fERi<'a, 'b>(__this: &'a mut crate::S, i: &'b mut i32)
                    -> &'a mut i32;
            }
        );
        Ok(())
    }

    #[test]
    fn test_annotated_lifetimes() -> Result<()> {
        let ir = ir_from_cc(&with_lifetime_macros(
            r#"
          int& $a f(int& $a i1, int& $a i2);
          "#,
        ))?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn f<'a>(i1: &'a mut i32, i2: &'a mut i32) -> &'a mut i32 { ... }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                pub(crate) fn __rust_thunk___Z1fRiS_<'a>(i1: &'a mut i32, i2: &'a mut i32)
                    -> &'a mut i32;
            }
        );
        Ok(())
    }

    #[test]
    fn test_format_generic_params() -> Result<()> {
        assert_rs_matches!(format_generic_params(std::iter::empty::<syn::Ident>()), quote! {});

        let idents = ["T1", "T2"].iter().map(|s| make_rs_ident(s));
        assert_rs_matches!(format_generic_params(idents), quote! { < T1, T2 > });

        let lifetimes = ["a", "b"]
            .iter()
            .map(|s| syn::Lifetime::new(&format!("'{}", s), proc_macro2::Span::call_site()));
        assert_rs_matches!(format_generic_params(lifetimes), quote! { < 'a, 'b > });

        Ok(())
    }

    #[test]
    fn test_format_tuple_except_singleton() {
        fn format(xs: &[TokenStream]) -> TokenStream {
            format_tuple_except_singleton(xs)
        }
        assert_rs_matches!(format(&[]), quote! {()});
        assert_rs_matches!(format(&[quote! {a}]), quote! {a});
        assert_rs_matches!(format(&[quote! {a}, quote! {b}]), quote! {(a, b)});
    }

    #[test]
    fn test_overloaded_functions() -> Result<()> {
        // TODO(b/213280424): We don't support creating bindings for overloaded
        // functions yet, except in the case of overloaded constructors with a
        // single parameter.
        let ir = ir_from_cc(
            r#" #pragma clang lifetime_elision
                void f();
                void f(int i);
                struct S1 final {
                  void f();
                  void f(int i);
                };
                struct S2 final {
                  void f();
                };
                struct S3 final {
                  S3(int i);
                  S3(double d);
                };

                namespace foo { void not_overloaded(); }
                namespace bar { void not_overloaded(); }
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        let rs_api_str = tokens_to_string(rs_api.clone())?;

        // Cannot overload free functions.
        assert!(rs_api_str.contains("Error while generating bindings for item 'f'"));
        assert_rs_not_matches!(rs_api, quote! {pub fn f()});
        assert_rs_not_matches!(rs_api, quote! {pub fn f(i: i32)});

        // Cannot overload member functions.
        assert!(rs_api_str.contains("Error while generating bindings for item 'S1::f'"));
        assert_rs_not_matches!(rs_api, quote! {pub fn f(... S1 ...)});

        // But we can import member functions that have the same name as a free
        // function.
        assert_rs_matches!(rs_api, quote! {pub fn f<'a>(&'a mut self)});

        // We can also import overloaded single-parameter constructors.
        assert_rs_matches!(rs_api, quote! {impl From<i32> for S3});
        assert_rs_matches!(rs_api, quote! {impl From<f64> for S3});

        // And we can import functions that have the same name + signature, but that are
        // in 2 different namespaces.
        assert_rs_matches!(rs_api, quote! { pub fn not_overloaded() });
        Ok(())
    }

    #[test]
    fn test_type_alias() -> Result<()> {
        let ir = ir_from_cc(
            r#"
                // MyTypedefDecl doc comment
                typedef int MyTypedefDecl;

                using MyTypeAliasDecl = int;
                using MyTypeAliasDecl_Alias = MyTypeAliasDecl;

                struct S final {};
                using S_Alias = S;
                using S_Alias_Alias = S_Alias;

                inline void f(MyTypedefDecl t) {}
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                #[doc = " MyTypedefDecl doc comment"]
                pub type MyTypedefDecl = i32;
            }
        );
        assert_rs_matches!(rs_api, quote! { pub type MyTypeAliasDecl = i32; });
        assert_rs_matches!(
            rs_api,
            quote! { pub type MyTypeAliasDecl_Alias = crate::MyTypeAliasDecl; }
        );
        assert_rs_matches!(rs_api, quote! { pub type S_Alias = crate::S; });
        assert_rs_matches!(rs_api, quote! { pub type S_Alias_Alias = crate::S_Alias; });
        assert_rs_matches!(rs_api, quote! { pub fn f(t: crate::MyTypedefDecl) });
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z1fi(MyTypedefDecl t) { f(t); }
            }
        );
        Ok(())
    }

    #[test]
    fn test_rs_type_kind_implements_copy() -> Result<()> {
        let template = r#" LIFETIMES
            struct [[clang::trivial_abi]] TrivialStruct final { int i; };
            struct [[clang::trivial_abi]] UserDefinedCopyConstructor final {
                UserDefinedCopyConstructor(const UserDefinedCopyConstructor&);
            };
            using IntAlias = int;
            using TrivialAlias = TrivialStruct;
            using NonTrivialAlias = UserDefinedCopyConstructor;
            void func(PARAM_TYPE some_param);
        "#;
        assert_impl_all!(i32: Copy);
        assert_impl_all!(&i32: Copy);
        assert_not_impl_any!(&mut i32: Copy);
        assert_impl_all!(Option<&i32>: Copy);
        assert_not_impl_any!(Option<&mut i32>: Copy);
        assert_impl_all!(*const i32: Copy);
        assert_impl_all!(*mut i32: Copy);
        struct Test {
            // Test inputs:
            cc: &'static str,
            lifetimes: bool,
            // Expected test outputs:
            rs: &'static str,
            is_copy: bool,
        }
        let tests = vec![
            // Validity of the next few tests is verified via
            // `assert_[not_]impl_all!` static assertions above.
            Test { cc: "int", lifetimes: true, rs: "i32", is_copy: true },
            Test { cc: "const int&", lifetimes: true, rs: "&'a i32", is_copy: true },
            Test { cc: "int&", lifetimes: true, rs: "&'a mut i32", is_copy: false },
            Test { cc: "const int*", lifetimes: true, rs: "Option<&'a i32>", is_copy: true },
            Test { cc: "int*", lifetimes: true, rs: "Option<&'a mut i32>", is_copy: false },
            Test { cc: "const int*", lifetimes: false, rs: "*const i32", is_copy: true },
            Test { cc: "int*", lifetimes: false, rs: "*mut i32", is_copy: true },
            // Tests below have been thought-through and verified "manually".
            // TrivialStruct is expected to derive Copy.
            Test {
                cc: "TrivialStruct",
                lifetimes: true,
                rs: "crate::TrivialStruct",
                is_copy: true,
            },
            Test {
                cc: "UserDefinedCopyConstructor",
                lifetimes: true,
                rs: "crate::UserDefinedCopyConstructor",
                is_copy: false,
            },
            Test { cc: "IntAlias", lifetimes: true, rs: "crate::IntAlias", is_copy: true },
            Test { cc: "TrivialAlias", lifetimes: true, rs: "crate::TrivialAlias", is_copy: true },
            Test {
                cc: "NonTrivialAlias",
                lifetimes: true,
                rs: "crate::NonTrivialAlias",
                is_copy: false,
            },
        ];
        for test in tests.iter() {
            let test_name = format!("cc='{}', lifetimes={}", test.cc, test.lifetimes);
            let cc_input = template.replace("PARAM_TYPE", test.cc).replace(
                "LIFETIMES",
                if test.lifetimes { "#pragma clang lifetime_elision" } else { "" },
            );
            let db = db_from_cc(&cc_input)?;
            let ir = db.ir();

            let f = retrieve_func(&ir, "func");
            let t = db.rs_type_kind(f.params[0].type_.rs_type.clone())?;

            let fmt = tokens_to_string(t.to_token_stream())?;
            assert_eq!(test.rs, fmt, "Testing: {}", test_name);

            assert_eq!(test.is_copy, t.implements_copy(), "Testing: {}", test_name);
        }
        Ok(())
    }

    #[test]
    fn test_rs_type_kind_is_shared_ref_to_with_lifetimes() -> Result<()> {
        let db = db_from_cc(
            "#pragma clang lifetime_elision
             struct SomeStruct {};
             void foo(const SomeStruct& foo_param);
             void bar(SomeStruct& bar_param);",
        )?;
        let ir = db.ir();
        let record = ir.records().next().unwrap();
        let foo_func = retrieve_func(&ir, "foo");
        let bar_func = retrieve_func(&ir, "bar");

        // const-ref + lifetimes in C++  ===>  shared-ref in Rust
        assert_eq!(foo_func.params.len(), 1);
        let foo_param = &foo_func.params[0];
        assert_eq!(&foo_param.identifier.identifier, "foo_param");
        let foo_type = db.rs_type_kind(foo_param.type_.rs_type.clone())?;
        assert!(foo_type.is_shared_ref_to(record));
        assert!(matches!(foo_type, RsTypeKind::Reference { mutability: Mutability::Const, .. }));

        // non-const-ref + lifetimes in C++  ===>  mutable-ref in Rust
        assert_eq!(bar_func.params.len(), 1);
        let bar_param = &bar_func.params[0];
        assert_eq!(&bar_param.identifier.identifier, "bar_param");
        let bar_type = db.rs_type_kind(bar_param.type_.rs_type.clone())?;
        assert!(!bar_type.is_shared_ref_to(record));
        assert!(matches!(bar_type, RsTypeKind::Reference { mutability: Mutability::Mut, .. }));

        Ok(())
    }

    #[test]
    fn test_rs_type_kind_is_shared_ref_to_without_lifetimes() -> Result<()> {
        let db = db_from_cc(
            "struct SomeStruct {};
             void foo(const SomeStruct& foo_param);",
        )?;
        let ir = db.ir();
        let record = ir.records().next().unwrap();
        let foo_func = retrieve_func(&ir, "foo");

        // const-ref + *no* lifetimes in C++  ===>  const-pointer in Rust
        assert_eq!(foo_func.params.len(), 1);
        let foo_param = &foo_func.params[0];
        assert_eq!(&foo_param.identifier.identifier, "foo_param");
        let foo_type = db.rs_type_kind(foo_param.type_.rs_type.clone())?;
        assert!(!foo_type.is_shared_ref_to(record));
        assert!(matches!(foo_type, RsTypeKind::Pointer { mutability: Mutability::Const, .. }));

        Ok(())
    }

    #[test]
    fn test_rs_type_kind_dfs_iter_ordering() {
        // Set up a test input representing: A<B<C>, D<E>>.
        let a = {
            let b = {
                let c = RsTypeKind::Other { name: "C".into(), type_args: Rc::from([]) };
                RsTypeKind::Other { name: "B".into(), type_args: Rc::from([c]) }
            };
            let d = {
                let e = RsTypeKind::Other { name: "E".into(), type_args: Rc::from([]) };
                RsTypeKind::Other { name: "D".into(), type_args: Rc::from([e]) }
            };
            RsTypeKind::Other { name: "A".into(), type_args: Rc::from([b, d]) }
        };
        let dfs_names = a
            .dfs_iter()
            .map(|t| match t {
                RsTypeKind::Other { name, .. } => &**name,
                _ => unreachable!("Only 'other' types are used in this test"),
            })
            .collect_vec();
        assert_eq!(vec!["A", "B", "C", "D", "E"], dfs_names);
    }

    #[test]
    fn test_rs_type_kind_dfs_iter_ordering_for_func_ptr() {
        // Set up a test input representing: fn(A, B) -> C
        let f = {
            let a = RsTypeKind::Other { name: "A".into(), type_args: Rc::from(&[][..]) };
            let b = RsTypeKind::Other { name: "B".into(), type_args: Rc::from(&[][..]) };
            let c = RsTypeKind::Other { name: "C".into(), type_args: Rc::from(&[][..]) };
            RsTypeKind::FuncPtr {
                abi: "blah".into(),
                param_types: Rc::from([a, b]),
                return_type: Rc::new(c),
            }
        };
        let dfs_names = f
            .dfs_iter()
            .map(|t| match t {
                RsTypeKind::FuncPtr { .. } => "fn",
                RsTypeKind::Other { name, .. } => &**name,
                _ => unreachable!("Only FuncPtr and Other kinds are used in this test"),
            })
            .collect_vec();
        assert_eq!(vec!["fn", "A", "B", "C"], dfs_names);
    }

    #[test]
    fn test_rs_type_kind_lifetimes() -> Result<()> {
        let db = db_from_cc(
            r#"
            #pragma clang lifetime_elision
            using TypeAlias = int&;
            struct SomeStruct {};
            void foo(int a, int& b, int&& c, int* d, int** e, TypeAlias f, SomeStruct g); "#,
        )?;
        let ir = db.ir();
        let func = retrieve_func(&ir, "foo");
        let ret = db.rs_type_kind(func.return_type.rs_type.clone())?;
        let a = db.rs_type_kind(func.params[0].type_.rs_type.clone())?;
        let b = db.rs_type_kind(func.params[1].type_.rs_type.clone())?;
        let c = db.rs_type_kind(func.params[2].type_.rs_type.clone())?;
        let d = db.rs_type_kind(func.params[3].type_.rs_type.clone())?;
        let e = db.rs_type_kind(func.params[4].type_.rs_type.clone())?;
        let f = db.rs_type_kind(func.params[5].type_.rs_type.clone())?;
        let g = db.rs_type_kind(func.params[6].type_.rs_type.clone())?;

        assert_eq!(0, ret.lifetimes().count()); // No lifetimes on `void`.
        assert_eq!(0, a.lifetimes().count()); // No lifetimes on `int`.
        assert_eq!(1, b.lifetimes().count()); // `&'a i32` has a single lifetime.
        assert_eq!(1, c.lifetimes().count()); // `RvalueReference<'a, i32>` has a single lifetime.
        assert_eq!(1, d.lifetimes().count()); // `Option<&'b i32>` has a single lifetime.
        assert_eq!(2, e.lifetimes().count()); // `&'c Option<&'d i32>` has two lifetimes.
        assert_eq!(1, f.lifetimes().count()); // Lifetime of underlying type should show through.
        assert_eq!(0, g.lifetimes().count()); // No lifetimes on structs (yet).
        Ok(())
    }

    #[test]
    fn test_rs_type_kind_lifetimes_raw_ptr() -> Result<()> {
        let db = db_from_cc("void foo(int* a);")?;
        let ir = db.ir();
        let f = retrieve_func(&ir, "foo");
        let a = db.rs_type_kind(f.params[0].type_.rs_type.clone())?;
        assert_eq!(0, a.lifetimes().count()); // No lifetimes on `int*`.
        Ok(())
    }

    #[test]
    fn test_rust_keywords_are_escaped_in_rs_api_file() -> Result<()> {
        let ir = ir_from_cc("struct type { int dyn; };")?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(rs_api, quote! { struct r#type { ... r#dyn: i32 ... } });
        Ok(())
    }

    #[test]
    fn test_rust_keywords_are_not_escaped_in_rs_api_impl_file() -> Result<()> {
        let ir = ir_from_cc("struct type { int dyn; };")?;
        let rs_api_impl = generate_bindings_tokens(ir)?.rs_api_impl;
        assert_cc_matches!(
            rs_api_impl,
            quote! { static_assert(CRUBIT_OFFSET_OF(dyn, struct type) ... ) }
        );
        Ok(())
    }

    #[test]
    fn test_no_aligned_attr() {
        let ir = ir_from_cc("struct SomeStruct {};").unwrap();
        let rs_api = generate_bindings_tokens(ir).unwrap().rs_api;

        assert_rs_matches! {rs_api, quote! {
         #[repr(C)]
         pub struct SomeStruct { ... }
        }};
    }

    #[test]
    fn test_aligned_attr() {
        let ir = ir_from_cc("struct SomeStruct {} __attribute__((aligned(64)));").unwrap();
        let rs_api = generate_bindings_tokens(ir).unwrap().rs_api;

        assert_rs_matches! {rs_api, quote! {
           #[repr(C, align(64))]
           pub struct SomeStruct { ... }
          }
        };
    }

    /// !Unpin references should not be pinned.
    #[test]
    fn test_nonunpin_ref_param() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct S {~S();};
            void Function(const S& s);
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                fn Function<'a>(s: &'a crate::S) { ... }
            }
        );
        Ok(())
    }

    /// !Unpin mut references must be pinned.
    #[test]
    fn test_nonunpin_mut_param() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct S {~S();};
            void Function(S& s);
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                fn Function<'a>(s: ::std::pin::Pin<&'a mut crate::S>) { ... }
            }
        );
        Ok(())
    }

    /// !Unpin &self should not be pinned.
    #[test]
    fn test_nonunpin_ref_self() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct S {
              ~S();
              void Function() const;
            };
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                fn Function<'a>(&'a self) { ... }
            }
        );
        Ok(())
    }

    /// !Unpin &mut self must be pinned.
    #[test]
    fn test_nonunpin_mut_self() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            #pragma clang lifetime_elision
            struct S {
              ~S();
              void Function();
            };
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                fn Function<'a>(self: ::std::pin::Pin<&'a mut Self>) { ... }
            }
        );
        Ok(())
    }

    /// Drop::drop must not use self : Pin<...>.
    #[test]
    fn test_nonunpin_drop() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            struct S {~S();};
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) { ... }
            }
        );
        Ok(())
    }

    #[test]
    fn test_nonunpin_0_arg_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct HasConstructor {explicit HasConstructor() {}};"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned]});
        assert_rs_matches!(
            rs_api,
            quote! {
                impl ::ctor::CtorNew<()> for HasConstructor {
                    type CtorType = impl ::ctor::Ctor<Output = Self>;

                    #[inline (always)]
                    fn ctor_new(args: ()) -> Self::CtorType {
                        let () = args;
                        unsafe {
                            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::HasConstructor>>| {
                                crate::detail::__rust_thunk___ZN14HasConstructorC1Ev(::std::pin::Pin::into_inner_unchecked(dest));
                            })
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_nonunpin_1_arg_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct HasConstructor {explicit HasConstructor(unsigned char input) {}};"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned]});
        assert_rs_matches!(
            rs_api,
            quote! {
                impl ::ctor::CtorNew<u8> for HasConstructor {
                    type CtorType = impl ::ctor::Ctor<Output = Self>;

                    #[inline (always)]
                    fn ctor_new(args: u8) -> Self::CtorType {
                        let input = args;
                        unsafe {
                            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::HasConstructor>>| {
                                crate::detail::__rust_thunk___ZN14HasConstructorC1Eh(::std::pin::Pin::into_inner_unchecked(dest), input);
                            })
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_nonunpin_2_arg_constructor() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct HasConstructor {explicit HasConstructor(unsigned char input1, signed char input2) {}};"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned]});
        assert_rs_matches!(
            rs_api,
            quote! {
                impl ::ctor::CtorNew<(u8, i8)> for HasConstructor {
                    type CtorType = impl ::ctor::Ctor<Output = Self>;

                    #[inline (always)]
                    fn ctor_new(args: (u8, i8)) -> Self::CtorType {
                        let (input1, input2) = args;
                        unsafe {
                            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::HasConstructor>>| {
                                crate::detail::__rust_thunk___ZN14HasConstructorC1Eha(::std::pin::Pin::into_inner_unchecked(dest), input1, input2);
                            })
                        }
                    }
                }
            }
        );
        Ok(())
    }

    /// Traits which monomorphize the `Ctor` parameter into the caller must
    /// synthesize an RvalueReference parameter, with an appropriate
    /// lifetime parameter.
    #[test]
    fn test_nonunpin_by_value_params() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct HasConstructor {
                // int& x is here to create a 'b lifetime, which collides with a synthesized
                // lifetime name. But that's OK! We handle collisions!
                // (`a` would also work, but that's just because the left hand doesn't know what
                // the right is doing: the `a` lifetime is present in some places, but eventually
                // removed from the public interface.)
                explicit HasConstructor(const int& x, HasConstructor y, HasConstructor b) {}
            };"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(rs_api, quote! {#[::ctor::recursively_pinned]});
        assert_rs_matches!(
            rs_api,
            quote! {
                impl <'b, 'y, 'b_2> ::ctor::CtorNew<(
                    &'b i32,
                    ::ctor::RvalueReference<'y, crate::HasConstructor>,
                    ::ctor::RvalueReference<'b_2, crate::HasConstructor>)
                > for HasConstructor {
                    // The captures are why we need explicit lifetimes for the two rvalue reference
                    // parameters.
                    type CtorType = impl ::ctor::Ctor<Output = Self>
                        + ::ctor::Captures<'b>
                        + ::ctor::Captures<'y>
                        + ::ctor::Captures<'b_2>;

                    #[inline (always)]
                    fn ctor_new(args: (
                        &'b i32,
                        ::ctor::RvalueReference<'y, crate::HasConstructor>,
                        ::ctor::RvalueReference<'b_2, crate::HasConstructor>)
                    ) -> Self::CtorType {
                        let (x, y, b) = args;
                        unsafe {
                            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::HasConstructor>>| {
                                crate::detail::__rust_thunk___ZN14HasConstructorC1ERKiS_S_(::std::pin::Pin::into_inner_unchecked(dest), x, y, b);
                            })
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_nonunpin_return() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct Nontrivial {~Nontrivial();};

            Nontrivial ReturnsByValue(const int& x, const int& y);
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn ReturnsByValue<'a, 'b>(x: &'a i32, y: &'b i32)
                -> impl ::ctor::Ctor<Output=crate::Nontrivial>
                 + ::ctor::Captures<'a>
                 + ::ctor::Captures<'b> {
                    unsafe {
                        ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nontrivial>>| {
                            crate::detail::__rust_thunk___Z14ReturnsByValueRKiS0_(::std::pin::Pin::into_inner_unchecked(dest), x, y);
                        })
                    }

                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z14ReturnsByValueRKiS0_(
                        struct Nontrivial* __return, int const* x, int const* y) {
                    new(__return) auto(ReturnsByValue(*x, *y));
                }
            }
        );
        Ok(())
    }

    /// Assignment is special in that it discards the return type.
    /// So if the return type is !Unpin, it needs to emplace!() it.
    #[test]
    fn test_nonunpin_return_assign() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct Nontrivial {
                ~Nontrivial();
                Nontrivial operator=(const Nontrivial& other);
            };
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'b> ::ctor::Assign<&'b crate::Nontrivial> for Nontrivial {
                    #[inline(always)]
                    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, other: &'b crate::Nontrivial) {
                        unsafe {
                            let _ = ::ctor::emplace!(::ctor::FnCtor::new(
                                move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nontrivial>>| {
                                    crate::detail::__rust_thunk___ZN10NontrivialaSERKS_(
                                        ::std::pin::Pin::into_inner_unchecked(dest),
                                        self,
                                        other
                                    );
                                }
                            ));
                        }
                    }
                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN10NontrivialaSERKS_(
                    struct Nontrivial* __return, struct Nontrivial* __this,
                    const struct Nontrivial* other
                ) {
                    new(__return) auto(__this->operator=(*other));
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_nonunpin_param() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct Nontrivial {
                Nontrivial(Nontrivial&&);
                ~Nontrivial();
            };

            void TakesByValue(Nontrivial x);
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub fn TakesByValue(x: impl ::ctor::Ctor<Output=crate::Nontrivial>) {
                    unsafe {
                        crate::detail::__rust_thunk___Z12TakesByValue10Nontrivial(::std::pin::Pin::into_inner_unchecked(::ctor::emplace!(x)))
                    }
                }
            }
        );

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___Z12TakesByValue10Nontrivial(struct Nontrivial*x) {
                    TakesByValue(std::move(*x));
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_nonunpin_trait_param() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin`.
            struct Nontrivial {
                Nontrivial(Nontrivial&&);
                Nontrivial& operator=(Nontrivial) {}
                ~Nontrivial();
            };

            struct Trivial final {
                /*implicit*/ Trivial(Nontrivial) {}
            };
            "#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl<'__param_0> From<::ctor::RvalueReference<'__param_0, crate::Nontrivial>> for Trivial {
                    #[inline(always)]
                    fn from(__param_0: ::ctor::RvalueReference<'__param_0, crate::Nontrivial>) -> Self {
                        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
                        unsafe {
                            crate::detail::__rust_thunk___ZN7TrivialC1E10Nontrivial(
                                &mut tmp,
                                __param_0
                            );
                            tmp.assume_init()
                        }
                    }
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_nonmovable_param() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            // This type must be `!Unpin` and non-move constructible.
            struct Nonmovable {
                Nonmovable(Nonmovable&&) = delete;
            };

            void TakesByValue(Nonmovable) {}
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        // Bindings for TakesByValue cannot be generated.
        assert_rs_not_matches!(rs_api, quote! {TakesByValue});
        assert_cc_not_matches!(rs_api_impl, quote! {TakesByValue});
        Ok(())
    }

    #[test]
    fn test_function_returning_rvalue_reference() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct SomeStruct final {
                // Inline to force generation (and test coverage) of C++ thunks.
                inline SomeStruct&& GetRValueReference() {
                  return static_cast<SomeStruct&&>(*this);
                }
                int field;
            };
            "#,
        )?;
        let BindingsTokens { rs_api, rs_api_impl } = generate_bindings_tokens(ir)?;
        assert_rs_matches!(
            rs_api,
            quote! {
                impl SomeStruct {
                    ...
                    #[inline(always)]
                    pub fn GetRValueReference<'a>(&'a mut self)
                            -> ::ctor::RvalueReference<'a, crate::SomeStruct> {
                        unsafe {
                            crate::detail::__rust_thunk___ZN10SomeStruct18GetRValueReferenceEv(self)
                        }
                    }
                }
            }
        );
        assert_rs_matches!(
            rs_api,
            quote! {
                extern "C" {
                    ...
                    pub(crate) fn __rust_thunk___ZN10SomeStruct18GetRValueReferenceEv<'a>(
                            __this: &'a mut crate::SomeStruct
                       ) -> ::ctor::RvalueReference<'a, crate::SomeStruct>;
                    ...
                }
            }
        );

        // Note that you can't just convert directly from xvalue to lvalue:
        //
        //     return &static_cast<SomeStruct&>(__this->GetRValueReference());
        //
        // For the above, Clang will emit an error that "non-const lvalue reference to
        // type 'struct SomeStruct' cannot bind to a temporary of type
        // 'SomeStruct'" (This is somewhat misleading, because there are no
        // temporaries here).  We must first bind the return value to a name
        // (`lvalue` below), so that it becomes an lvalue. Only then can it be
        // converted to a pointer.
        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" struct SomeStruct*
                __rust_thunk___ZN10SomeStruct18GetRValueReferenceEv(struct SomeStruct* __this) {
                    struct SomeStruct&& lvalue = __this->GetRValueReference();
                    return &lvalue;
                }
            }
        );

        Ok(())
    }

    #[test]
    fn test_forward_declared() -> Result<()> {
        let ir = ir_from_cc(
            r#"#pragma clang lifetime_elision
            struct ForwardDeclared;"#,
        )?;
        let rs_api = generate_bindings_tokens(ir)?.rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                forward_declare::forward_declare!(pub ForwardDeclared = forward_declare::symbol!("ForwardDeclared"));
            }
        );
        assert_rs_not_matches!(rs_api, quote! {struct ForwardDeclared});
        Ok(())
    }

    #[test]
    fn test_namespace_module_items() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            namespace test_namespace_bindings {
                int func();
                struct S {};
                namespace inner {
                    int inner_func();
                    struct InnerS {};
                }
            }
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub mod test_namespace_bindings {
                    ...
                    pub fn func() -> i32 { ... }
                    ...
                    pub struct S { ... }
                    ...
                    pub mod inner {
                        ...
                        pub fn inner_func() -> i32 { ... }
                        ...
                        pub struct InnerS { ... }
                        ...
                    }
                    ...
                }
            }
        );
        Ok(())
    }

    #[test]
    fn test_detail_outside_of_namespace_module() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            namespace test_namespace_bindings {
                int f();
            }
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub mod test_namespace_bindings {
                    ...
                }
                ...
                mod detail {
                    #[allow(unused_imports)]
                    use super::*;
                    extern "C" {
                        #[link_name = "_ZN23test_namespace_bindings1fEv"]
                        pub(crate) fn __rust_thunk___ZN23test_namespace_bindings1fEv() -> i32;
                    }
                }
                ...
            }
        );
        Ok(())
    }

    #[test]
    fn test_assertions_outside_of_namespace_module() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            namespace test_namespace_bindings {
                struct S {
                    int i;
                };
            }
        "#,
        )?)?
        .rs_api;
        assert_rs_matches!(
            rs_api,
            quote! {
                pub mod test_namespace_bindings {
                    ...
                }
                ...
                const _: () = assert!(::std::mem::size_of::<crate::test_namespace_bindings::S>() == 4);
                const _: () = assert!(::std::mem::align_of::<crate::test_namespace_bindings::S>() == 4);
                ...
                const _: () = assert!(memoffset_unstable_const::offset_of!(crate::test_namespace_bindings::S, i) == 0);
            }
        );
        Ok(())
    }

    #[test]
    fn test_reopened_namespaces() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
        namespace test_namespace_bindings {
        namespace inner {}
        }  // namespace test_namespace_bindings

        namespace test_namespace_bindings {
        namespace inner {}
        }  // namespace test_namespace_bindings"#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                ...
                pub mod test_namespace_bindings_0 {
                    pub mod inner_0 {} ...
                }
                ...
                pub mod test_namespace_bindings {
                    pub use super::test_namespace_bindings_0::*;
                    ...
                    pub mod inner {
                        pub use super::inner_0::*;
                        ...
                    }
                }
                ...
            }
        );
        Ok(())
    }

    #[test]
    fn test_qualified_identifiers_in_impl_file() -> Result<()> {
        let rs_api_impl = generate_bindings_tokens(ir_from_cc(
            r#"
        namespace test_namespace_bindings {
            inline void f() {};
            struct S final {};
        }
        inline void useS(test_namespace_bindings::S s) {};"#,
        )?)?
        .rs_api_impl;

        assert_cc_matches!(
            rs_api_impl,
            quote! {
                extern "C" void __rust_thunk___ZN23test_namespace_bindings1fEv() {
                    test_namespace_bindings::f();
                }
                ...
                extern "C" void __rust_thunk___Z4useSN23test_namespace_bindings1SE(
                    struct test_namespace_bindings::S s) { useS(s); }
                ...
            }
        );
        Ok(())
    }

    #[test]
    fn test_inline_namespace() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            namespace test_namespace_bindings {
                inline namespace inner {
                    struct MyStruct final {};
                }
                void processMyStruct(MyStruct s);
            }
            void processMyStructOutsideNamespace(test_namespace_bindings::inner::MyStruct s);
            void processMyStructSkipInlineNamespaceQualifier(test_namespace_bindings::MyStruct s);
            "#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                ...
                pub mod test_namespace_bindings {
                    ...
                    pub mod inner {
                        ...
                        pub struct MyStruct {...} ...
                    }
                    ...
                    pub fn processMyStruct(s: crate::test_namespace_bindings::inner::MyStruct)
                    ...
                }
                ...
                pub fn processMyStructOutsideNamespace(s: crate::test_namespace_bindings::inner::MyStruct)
                ...
                pub fn processMyStructSkipInlineNamespaceQualifier(s: crate::test_namespace_bindings::inner::MyStruct)
                ...
            }
        );
        Ok(())
    }

    #[test]
    fn test_implicit_template_specializations_are_sorted_by_mangled_name() -> Result<()> {
        let bindings = generate_bindings_tokens(ir_from_cc(
            r#"
                template <typename T>
                struct MyStruct {
                    T getT();
                };

                using Alias1 = MyStruct<int>;
                using Alias2 = MyStruct<double>;

                namespace test_namespace_bindings {
                    using Alias3 = MyStruct<bool>;
                }
                "#,
        )?)?;

        // Mangled name order: bool < double < int
        let my_struct_bool = make_rs_ident("__CcTemplateInst8MyStructIbE");
        let my_struct_double = make_rs_ident("__CcTemplateInst8MyStructIdE");
        let my_struct_int = make_rs_ident("__CcTemplateInst8MyStructIiE");

        assert_rs_matches!(
            &bindings.rs_api,
            quote! {
                ...
                pub struct #my_struct_bool {...}
                ...
                pub struct #my_struct_double {...}
                ...
                pub struct #my_struct_int {...}
                ...
                const _: () = assert!(::std::mem::size_of::<crate::#my_struct_bool>() == 1);
                ...
                const _: () = assert!(::std::mem::size_of::<crate::#my_struct_double>() == 1);
                ...
                const _: () = assert!(::std::mem::size_of::<crate::#my_struct_int>() == 1);
                ...
            }
        );

        // User defined methods in mangled name order
        let my_struct_bool_method =
            make_rs_ident("__rust_thunk___ZN8MyStructIbE4getTEv__2f_2ftest_3atesting_5ftarget");
        let my_struct_double_method =
            make_rs_ident("__rust_thunk___ZN8MyStructIdE4getTEv__2f_2ftest_3atesting_5ftarget");
        let my_struct_int_method =
            make_rs_ident("__rust_thunk___ZN8MyStructIiE4getTEv__2f_2ftest_3atesting_5ftarget");

        assert_cc_matches!(
            &bindings.rs_api_impl,
            quote! {
                ...
                extern "C" bool #my_struct_bool_method(struct MyStruct<bool>*__this) {...} ...
                extern "C" double #my_struct_double_method(struct MyStruct<double>*__this) {...} ...
                extern "C" int #my_struct_int_method(struct MyStruct<int>*__this) {...} ...
            }
        );
        Ok(())
    }

    #[test]
    fn test_implicit_template_specialization_namespace_qualifier() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#" #pragma clang lifetime_elision
                namespace test_namespace_bindings {
                    template <typename T>
                    struct MyTemplate final {
                        T value_;
                    };

                    using MyTypeAlias = MyTemplate<int>;
                }"#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                ...
                pub mod test_namespace_bindings {
                    ...
                    pub type MyTypeAlias = crate::__CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE;
                    ...
                }
                ...
                pub struct __CcTemplateInstN23test_namespace_bindings10MyTemplateIiEE {
                    pub value_: i32,
                }
                ...
            }
        );
        Ok(())
    }

    #[test]
    fn test_forward_declared_class_template_specialization_symbol() -> Result<()> {
        let rs_api = generate_bindings_tokens(ir_from_cc(
            r#"
            namespace test_namespace_bindings {
              template <typename T>
              struct MyTemplate {
                void processT(T t);
              };

              struct Param {};

              template<> struct MyTemplate<Param>;
            }"#,
        )?)?
        .rs_api;

        assert_rs_matches!(
            rs_api,
            quote! {
                ...
                pub mod test_namespace_bindings {
                    ...
                    forward_declare::forward_declare!(pub __CcTemplateInstN23test_namespace_bindings10MyTemplateINS_5ParamEEE = forward_declare::symbol!("__CcTemplateInstN23test_namespace_bindings10MyTemplateINS_5ParamEEE"));
                    ...
                }
                ...
            }
        );
        Ok(())
    }

    #[test]
    fn test_lifetime_elision_for_references() {
        let type_args: &[RsTypeKind] = &[];
        let referent = Rc::new(RsTypeKind::Other { name: "T".into(), type_args: type_args.into() });
        let reference = RsTypeKind::Reference {
            referent: referent,
            mutability: Mutability::Const,
            lifetime: Lifetime::new("_"),
        };
        assert_rs_matches!(quote! {#reference}, quote! {&T});
    }

    #[test]
    fn test_lifetime_elision_for_rvalue_references() {
        let type_args: &[RsTypeKind] = &[];
        let referent = Rc::new(RsTypeKind::Other { name: "T".into(), type_args: type_args.into() });
        let reference = RsTypeKind::RvalueReference {
            referent: referent,
            mutability: Mutability::Mut,
            lifetime: Lifetime::new("_"),
        };
        assert_rs_matches!(quote! {#reference}, quote! {RvalueReference<'_, T>});
    }
}
