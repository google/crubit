// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#![deny(missing_docs)]

//! Adds default lifetimes to (partially-) unannotated IR.

use arc_anyhow::Result;
use database::BindingsGenerator;
use error_report::bail;
use ir::{
    make_ir, CcType, CcTypeVariant, FlatIR, Func, Item, ItemId, PointerType, PointerTypeKind,
    Record, IR,
};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// A binding for one (possibly renamed) variable.
pub struct VariableBinding {
    /// A stack of renamings for this variable.
    pub renamed: Vec<Rc<str>>,
}

/// Keeps track of bound variables.
pub struct BindingContext {
    /// Maps user variable names to bindings.
    pub bindings: HashMap<Rc<str>, VariableBinding>,
    /// All variable names we've seen.
    pub names: HashSet<Rc<str>>,
}

/// Manages bindings for lifetime names. We expect to start with a `new` `BindingContext` for each
/// item I being imported. This `BindingContext` should contain bindings for all parent items of I.
/// The `BindingContext` will rename bindings that are shadowed and will never use the same renaming
/// twice.
impl BindingContext {
    /// Binds the variable `id`, returning a unique name for it.
    pub fn push_new_binding(&mut self, id: &Rc<str>) -> Rc<str> {
        let mut binding =
            self.bindings.remove(id).unwrap_or_else(|| VariableBinding { renamed: vec![] });
        let fresh_name = self.fresh_name_for(id);
        binding.renamed.push(fresh_name.clone());
        self.bindings.insert(id.clone(), binding);
        self.names.insert(fresh_name.clone());
        fresh_name
    }

    /// Returns a fresh variable name.
    pub fn push_fresh_binding(&mut self, hint: Option<&Rc<str>>) -> Rc<str> {
        let fresh_name = match hint {
            Some(id) => self.fresh_name_for(id),
            None => self.fresh_name_for(&Rc::from("lt")),
        };
        self.push_new_binding(&fresh_name)
    }

    /// If `id` is already bound, returns its renaming; otherwise, creates a new binding for it,
    /// and calls the provided function with the renaming.
    pub fn get_or_push_new_binding<F: FnOnce(&Rc<str>)>(&mut self, id: &Rc<str>, f: F) -> Rc<str> {
        if let Some(binding) = self.bindings.get(id) {
            return binding.renamed.last().unwrap().clone();
        }
        let new_binding = self.push_new_binding(id);
        f(&new_binding);
        new_binding
    }

    /// Returns a fresh variable name based on `id`.
    pub fn fresh_name_for(&self, id: &Rc<str>) -> Rc<str> {
        if !self.names.contains(id) {
            return id.clone();
        }
        let mut ix = 0;
        loop {
            let name = format!("{}_{}", id, ix);
            if !self.names.contains(name.as_str()) {
                return Rc::from(name);
            }
            ix += 1;
        }
    }

    /// Removes the last binding for `id`.
    pub fn pop_binding(&mut self, id: &Rc<str>) {
        let mut last_binding = self.bindings.remove(id).unwrap();
        last_binding.renamed.pop();
        if !last_binding.renamed.is_empty() {
            self.bindings.insert(id.clone(), last_binding);
        }
    }

    /// Creates a binding context with builtin lifetimes configured.
    pub fn new() -> Self {
        let mut ctx = BindingContext { bindings: HashMap::new(), names: HashSet::new() };
        ctx.push_new_binding(&Rc::from("static"));
        ctx.push_new_binding(&Rc::from("unknown"));
        ctx
    }
}

impl Default for BindingContext {
    fn default() -> Self {
        BindingContext::new()
    }
}

struct LifetimeDefaults<'a> {
    ir: &'a IR,
    bindings: BindingContext,
}

/// Used to keep track of the state we're in when ascribing lifetimes.
#[derive(Clone, Debug, PartialEq)]
enum LifetimeState {
    /// No lifetimes have been seen.
    Unseen,
    /// A single lifetime has been seen.
    Single(Rc<str>),
    /// We can't ascribe a lifetime.
    Unknown,
}

impl LifetimeState {
    fn update(&mut self, next: &LifetimeState) {
        match (&self, next) {
            (LifetimeState::Unseen, n) => *self = n.clone(),
            (_, LifetimeState::Unseen) => (),
            (LifetimeState::Single(a), LifetimeState::Single(b)) if a == b => (),
            _ => *self = LifetimeState::Unknown,
        }
    }
}

/// The result of adding lifetimes to a type.
struct LifetimeResult {
    /// The rewritten type.
    ty: CcType,
    /// Output state for default lifetime assignment.
    state: LifetimeState,
    /// Output state for default lifetime assignment on `this`.
    this_state: LifetimeState,
}

impl<'a> LifetimeDefaults<'a> {
    fn new(ir: &'a IR) -> Self {
        LifetimeDefaults { ir, bindings: BindingContext::new() }
    }

    /// Returns a state representing the given `lifetime`.
    fn get_state_for_annotated_lifetime(
        &mut self,
        lifetime: &[Rc<str>],
        new_bindings: &mut Vec<Rc<str>>,
    ) -> LifetimeState {
        match lifetime {
            [] => LifetimeState::Unseen,
            [id] => {
                let binding = self
                    .bindings
                    .get_or_push_new_binding(id, |name| new_bindings.push(name.clone()));
                if binding.as_ref() == "unknown" {
                    LifetimeState::Unknown
                } else {
                    LifetimeState::Single(binding)
                }
            }
            // TODO(b/454627672): multiple variables.
            _ => LifetimeState::Unknown,
        }
    }

    /// Returns a lifetime (suitable for use as an annotation) from `state` if `state`
    /// unambiguously refers to a lifetime.
    fn get_lifetime_for_state(&mut self, state: &LifetimeState) -> Vec<Rc<str>> {
        match state {
            LifetimeState::Single(lifetime) => vec![lifetime.clone()],
            _ => vec![],
        }
    }

    fn decl_binds_lifetimes(&mut self, id: &ItemId) -> bool {
        match self.ir.find_untyped_decl(*id) {
            // TODO(zarko): Here, we look for the explicit renaming we do in type_alias.cc. What
            // we actually want to do is recursively check ta.underlying_type (since anyone's free
            // to invent their own aliases for string_view). More generally, a type alias can bind
            // and apply arbitrary lifetimes.
            Item::TypeAlias(ta) if ta.rs_name == "raw_string_view" => true,
            // We seem to lose the typedef sugar if it's annotated.
            Item::Record(rc)
                if matches!(
                    **rc,
                    Record {
                        template_specialization: Some(ir::TemplateSpecialization {
                            kind: ir::TemplateSpecializationKind::StdStringView,
                            ..
                        }),
                        ..
                    }
                ) =>
            {
                true
            }
            _ => false,
        }
    }

    /// Adds lifetimes to a type in input position. Returns the new type paired with a LifetimeState
    /// describing the lifetimes we encountered and a list of any lifetimes we had to bind.
    /// `name_hint` is used to name the lifetime parameter when we need to make one.
    /// `is_this` must be `true` if this type is being used for the C++ implicit `this`.
    fn add_lifetime_to_input_type(
        &mut self,
        is_this: bool,
        is_constructor: bool,
        name_hint: Option<&Rc<str>>,
        new_bindings: &mut Vec<Rc<str>>,
        ty: &CcType,
    ) -> LifetimeResult {
        match &ty.variant {
            CcTypeVariant::Decl(d) if self.decl_binds_lifetimes(d) => {
                let mut state =
                    self.get_state_for_annotated_lifetime(&ty.explicit_lifetimes, new_bindings);
                if state == LifetimeState::Unseen {
                    let lifetime = self.bindings.push_fresh_binding(name_hint);
                    new_bindings.push(lifetime.clone());
                    state = LifetimeState::Single(lifetime);
                }
                let mut new_ty = ty.clone();
                new_ty.explicit_lifetimes = self.get_lifetime_for_state(&state);
                LifetimeResult { ty: new_ty, state, this_state: LifetimeState::Unseen }
            }
            CcTypeVariant::Pointer(pty) if is_this || pty.kind == PointerTypeKind::LValueRef => {
                let LifetimeResult { ty: pointee_type, .. } = self.add_lifetime_to_input_type(
                    false,
                    false,
                    name_hint,
                    new_bindings,
                    &pty.pointee_type,
                );
                let mut state =
                    self.get_state_for_annotated_lifetime(&ty.explicit_lifetimes, new_bindings);
                if state == LifetimeState::Unseen {
                    let lifetime = self.bindings.push_fresh_binding(name_hint);
                    new_bindings.push(lifetime.clone());
                    state = LifetimeState::Single(lifetime);
                }
                let mut new_ty = ty.clone();
                new_ty.variant = CcTypeVariant::Pointer(PointerType {
                    pointee_type: pointee_type.into(),
                    ..pty.clone()
                });
                new_ty.explicit_lifetimes = self.get_lifetime_for_state(&state);
                if is_this {
                    LifetimeResult { ty: new_ty, state: LifetimeState::Unseen, this_state: state }
                } else {
                    LifetimeResult { ty: new_ty, state, this_state: LifetimeState::Unseen }
                }
            }
            _ => LifetimeResult {
                ty: ty.clone(),
                state: LifetimeState::Unseen,
                this_state: LifetimeState::Unseen,
            },
        }
    }

    /// Adds lifetimes to a type in output position. `lifetime_hint` is used to assign a lifetime
    /// when one is not otherwise available. If `lifetime_hint` is empty, no new lifetimes will be
    /// assigned.
    fn add_lifetime_to_output_type(
        &mut self,
        lifetime_hint: &Vec<Rc<str>>,
        new_bindings: &mut Vec<Rc<str>>,
        ty: &CcType,
    ) -> CcType {
        match &ty.variant {
            CcTypeVariant::Decl(d) if self.decl_binds_lifetimes(d) => {
                let mut new_ty = ty.clone();
                // If there's a previously-annotated lifetime, use that.
                if !ty.explicit_lifetimes.is_empty() {
                    new_ty.explicit_lifetimes = ty
                        .explicit_lifetimes
                        .iter()
                        .map(|l| {
                            self.bindings
                                .get_or_push_new_binding(l, |name| new_bindings.push(name.clone()))
                        })
                        .collect();
                    return new_ty;
                }
                // If there is no viable inferred lifetime, there is nothing to do.
                if lifetime_hint.is_empty() {
                    return new_ty;
                }
                new_ty.explicit_lifetimes = lifetime_hint.clone();
                new_ty
            }
            CcTypeVariant::Pointer(pty) if pty.kind == PointerTypeKind::LValueRef => {
                let mut new_ty = ty.clone();
                // If there's a previously-annotated lifetime, use that.
                if !ty.explicit_lifetimes.is_empty() {
                    new_ty.explicit_lifetimes = ty
                        .explicit_lifetimes
                        .iter()
                        .map(|l| {
                            self.bindings
                                .get_or_push_new_binding(l, |name| new_bindings.push(name.clone()))
                        })
                        .collect();
                    return new_ty;
                }
                // If there is no viable inferred lifetime, we need to downgrade this to a raw
                // pointer. We can at least mark it non-null. (An argument could be made about
                // doing this later on provided we have a fuller treatement of safe/unsafe types
                // selected by the presence of lifetime inputs.)
                let kind =
                    if lifetime_hint.is_empty() { PointerTypeKind::NonNull } else { pty.kind };
                let pointee_type = self.add_lifetime_to_output_type(
                    lifetime_hint,
                    new_bindings,
                    &pty.pointee_type,
                );
                new_ty.variant = CcTypeVariant::Pointer(PointerType {
                    pointee_type: pointee_type.into(),
                    kind,
                    ..pty.clone()
                });
                new_ty.explicit_lifetimes = lifetime_hint.clone();
                new_ty
            }
            _ => ty.clone(),
        }
    }

    /// Returns the number of lifetime parameters `ty` expects to take, where `ty` may be a type
    /// that has not yet been transformed.
    fn get_lifetime_arity(&mut self, ty: &CcType) -> Result<usize> {
        // TODO(b/454627672): Support other types.
        match &ty.variant {
            CcTypeVariant::Pointer(_) => Ok(1),
            CcTypeVariant::Primitive(_) => Ok(0),
            CcTypeVariant::FuncPointer { .. } => {
                bail!("TODO(b/454627672): function pointer returns are unsupported")
            }
            CcTypeVariant::Decl(_) => bail!("TODO(b/454627672): decl returns are unsupported"),
            CcTypeVariant::Error(msg) => bail!("encountered error type: {:?}", msg),
        }
    }

    /// Transforms a function with Clang lifetime annotations into a function with Crubit-style
    /// lifetime annotations. This function will not rename any existing lifetimes.
    fn lower_clang_annotations(&mut self, func: &mut Func) -> Result<()> {
        // TODO(b/475407556): Support lifetime_capture_by.
        let mut return_lifetime: Vec<Rc<str>> = func.return_type.explicit_lifetimes.clone();
        let mut has_lifetimebound = false;
        let is_constructor = func.cc_name == ir::UnqualifiedIdentifier::Constructor;
        // First, check to see if there are any existing lifetime annotations that we need to
        // respect.
        for (ix, param) in func.params.iter().enumerate() {
            if param.clang_lifetimebound || (is_constructor && ix == 0) {
                has_lifetimebound |= param.clang_lifetimebound;
                if return_lifetime.is_empty() {
                    // If a [[lifetimebound]] parameter already has a lifetime annotation and we
                    // don't have a lifetime for the return value yet, use the parameter's
                    // annotation.
                    return_lifetime = param.type_.explicit_lifetimes.clone();
                } else if !param.type_.explicit_lifetimes.is_empty()
                    && param.type_.explicit_lifetimes != return_lifetime
                {
                    // If there's a conflict between what we believe is the [[lifetimebound]]
                    // lifetime and the one annotated on a parameter, return a diagnostic.
                    bail!(
                        "lifetimebound: lifetime mismatch in function {:#?} between parameter {:#?} with lifetime {:#?} and return with lifetime {:#?}",
                        &func.cc_name,
                        &param.identifier.identifier,
                        &param.type_.explicit_lifetimes,
                        &return_lifetime
                    );
                }
            }
        }
        if !has_lifetimebound {
            // If there are no [[lifetimebound]] parameters, we don't need to change anything.
            return Ok(());
        }
        if return_lifetime.is_empty() {
            // Since there are [[lifetimebound]] parameters but none were given lifetime
            // annotations, we need to create new lifetime variables for the return value.
            // Use a reserved name for these so we don't conflict with lifetimes embedded in
            // types or on non-[[lifetimebound]] parameters.
            let arity = self.get_lifetime_arity(if is_constructor {
                &func.params[0].type_
            } else {
                &func.return_type
            })?;
            for _ in 0..arity {
                let name = if is_constructor { &Rc::from("__this") } else { &Rc::from("__rv") };
                return_lifetime.push(self.bindings.fresh_name_for(name))
            }
        }
        for (ix, param) in func.params.iter_mut().enumerate() {
            if param.clang_lifetimebound || (is_constructor && ix == 0) {
                param.type_.explicit_lifetimes = return_lifetime.clone();
            }
        }
        if !is_constructor {
            func.return_type.explicit_lifetimes = return_lifetime;
        }
        Ok(())
    }

    fn bind_lifetime_inputs(&mut self, id: Option<ItemId>) -> Result<()> {
        let item = if let Some(id) = id {
            self.ir.find_untyped_decl(id)
        } else {
            return Ok(());
        };
        // Bind inputs from ancestors first.
        self.bind_lifetime_inputs(item.enclosing_item_id())?;
        match item {
            Item::Func(func) => {
                func.lifetime_inputs.iter().for_each(|name| {
                    self.bindings.push_new_binding(name);
                });
            }
            Item::Record(record) => {
                record.lifetime_inputs.iter().for_each(|name| {
                    self.bindings.push_new_binding(name);
                });
            }
            _ => (),
        };
        Ok(())
    }

    /// Transforms a function to use default lifetime rules.
    fn add_lifetime_to_func(&mut self, func: &Func) -> Result<Func> {
        let mut new_func = func.clone();
        let mut state = LifetimeState::Unseen;
        let mut this_state = LifetimeState::Unseen;
        let mut had_this = false;
        new_func.lifetime_inputs.clear();
        // Note that we generate a new LifetimeDefaults per Item that we're importing, so we don't
        // need to pop these bindings. (We *do* need to worry about unbinding names for internal
        // binders, like function types.)
        self.bind_lifetime_inputs(func.enclosing_item_id)?;
        // Rename local bindings (and remember how we've renamed them).
        func.lifetime_inputs
            .iter()
            .for_each(|name| new_func.lifetime_inputs.push(self.bindings.push_new_binding(name)));
        self.lower_clang_annotations(&mut new_func)?;
        new_func.params.iter_mut().enumerate().for_each(|(ix, param)| {
            let is_constructor = func.cc_name == ir::UnqualifiedIdentifier::Constructor;
            // `this` in a constructor is strange. The !is_constructor restriction fixes some
            // situations where we would bind a `'__this` in a constructor and then not use it
            // (because the actual `__this` is a void*).
            let is_this = ix == 0 && &*param.identifier.identifier == "__this" && !is_constructor;
            had_this |= is_this;
            let LifetimeResult { ty: new_type, state: new_state, this_state: new_this_state } =
                self.add_lifetime_to_input_type(
                    is_this,
                    is_constructor,
                    Some(&param.identifier.identifier),
                    &mut new_func.lifetime_inputs,
                    &param.type_,
                );
            state.update(&new_state);
            this_state.update(&new_this_state);
            param.type_ = new_type;
        });
        let lifetime = match this_state {
            LifetimeState::Unseen => self.get_lifetime_for_state(&state),
            _ => self.get_lifetime_for_state(&this_state),
        };
        new_func.return_type = self.add_lifetime_to_output_type(
            &lifetime,
            &mut new_func.lifetime_inputs,
            &new_func.return_type,
        );
        if had_this {
            // See if we can promote the type of `this` to a reference.
            let this = new_func.params.get_mut(0).unwrap();
            if !this.type_.explicit_lifetimes.is_empty()
                && let CcTypeVariant::Pointer(pty) = &mut this.type_.variant
            {
                pty.kind = PointerTypeKind::LValueRef;
            }
        }
        Ok(new_func)
    }

    /// Transforms a record to use default lifetime rules.
    fn add_lifetime_to_record(&mut self, record: &Record) -> Result<Record> {
        let mut new_record = record.clone();
        new_record.lifetime_inputs.clear();
        self.bind_lifetime_inputs(record.enclosing_item_id)?;
        // Rename local bindings (and remember how we've renamed them).
        record
            .lifetime_inputs
            .iter()
            .for_each(|name| new_record.lifetime_inputs.push(self.bindings.push_new_binding(name)));
        Ok(new_record)
    }

    /// Since we keep all item ids stable, we only have to deep-clone the objects that we need to
    /// change. We may need to introduce lifetime param binders whenever we see a type (but not on
    /// decls).
    fn add_lifetime_to_item(&mut self, item: &Item) -> Result<Item> {
        match item {
            Item::Func(func) => Ok(Item::Func(self.add_lifetime_to_func(func)?.into())),
            Item::Record(record) => Ok(Item::Record(self.add_lifetime_to_record(record)?.into())),
            _ => Ok(item.clone()),
        }
    }
}

/// Creates a copy of `func` with default lifetimes filled in.
pub fn lifetime_defaults_transform_func(db: &BindingsGenerator, func: &Func) -> Result<Func> {
    LifetimeDefaults::new(db.ir()).add_lifetime_to_func(func)
}

/// Creates a copy of `record` with default lifetimes filled in.
pub fn lifetime_defaults_transform_record(
    db: &BindingsGenerator,
    record: &Record,
) -> Result<Record> {
    LifetimeDefaults::new(db.ir()).add_lifetime_to_record(record)
}

/// Creates a copy of `ir` with default lifetimes filled in. This is mostly useful for testing;
/// prefer to transform items on demand.
pub fn lifetime_defaults_transform(ir: &IR) -> Result<IR> {
    let mut new_items = vec![];
    for item in ir.items() {
        let new_item = LifetimeDefaults::new(ir).add_lifetime_to_item(item)?;
        new_items.push(new_item);
    }
    Ok(make_ir(FlatIR {
        public_headers: ir.flat_ir().public_headers.clone(),
        current_target: ir.flat_ir().current_target.clone(),
        items: new_items,
        top_level_item_ids: ir.flat_ir().top_level_item_ids.clone(),
        crate_root_path: ir.flat_ir().crate_root_path.clone(),
        crubit_features: ir.flat_ir().crubit_features.clone(),
    }))
}
